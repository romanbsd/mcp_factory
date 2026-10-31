use super::{GoogleAdcAuthProvider, GoogleServiceAccountAuthProvider};
use mcp_factory_core::ProxyError;

#[test]
fn invalid_google_options_fail_before_credentials_are_read() {
    for scopes in [
        vec![],
        vec!["".into()],
        vec!["  ".into()],
        vec![" read".into()],
    ] {
        assert!(matches!(
            GoogleAdcAuthProvider::new(&scopes),
            Err(ProxyError::Config(_))
        ));
        assert!(matches!(
            GoogleServiceAccountAuthProvider::from_env(
                "MCP_FACTORY_TEST_INVALID_SCOPES_KEY",
                &scopes
            ),
            Err(ProxyError::Config(_))
        ));
    }
    for quota in ["", "  ", " project", "project\nheader"] {
        assert!(matches!(
            GoogleAdcAuthProvider::with_quota_project(&["read".into()], Some(quota.into())),
            Err(ProxyError::Config(_))
        ));
    }
}

#[test]
fn key_file_env_and_invalid_files_report_configuration_errors() {
    const NAME: &str = "MCP_FACTORY_TEST_SERVICE_KEY_PATH_VALIDATION";
    struct Restore(Option<std::ffi::OsString>);
    impl Drop for Restore {
        fn drop(&mut self) {
            if let Some(value) = &self.0 {
                std::env::set_var(NAME, value)
            } else {
                std::env::remove_var(NAME)
            }
        }
    }
    let _restore = Restore(std::env::var_os(NAME));
    std::env::remove_var(NAME);
    let scopes = vec!["read".into()];
    assert!(
        matches!(GoogleServiceAccountAuthProvider::from_env(NAME,&scopes),Err(ProxyError::Config(message)) if message.contains("missing or blank"))
    );
    std::env::set_var(NAME, "   ");
    assert!(
        matches!(GoogleServiceAccountAuthProvider::from_env(NAME,&scopes),Err(ProxyError::Config(message)) if message.contains("missing or blank"))
    );
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("invalid.json");
    std::env::set_var(NAME, &path);
    assert!(
        matches!(GoogleServiceAccountAuthProvider::from_env(NAME,&scopes),Err(ProxyError::Config(message)) if message.contains("failed to load"))
    );
    std::fs::write(&path, "invalid-json").unwrap();
    std::env::set_var(NAME, format!(" {} ", path.display()));
    assert!(
        matches!(GoogleServiceAccountAuthProvider::from_env(NAME,&scopes),Err(ProxyError::Config(message)) if message.contains("failed to load"))
    );
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStringExt;
        std::env::set_var(NAME, std::ffi::OsString::from_vec(vec![0xff]));
        assert!(
            matches!(GoogleServiceAccountAuthProvider::from_env(NAME,&scopes),Err(ProxyError::Config(message)) if message.contains("not valid Unicode"))
        );
    }
}
