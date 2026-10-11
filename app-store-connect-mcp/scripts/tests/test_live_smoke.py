"""No live credentials or network access: python3 -m unittest discover -s scripts/tests."""
import contextlib
import importlib.util
import io
import json
import os
from pathlib import Path
import stat
import subprocess
import tempfile
import unittest
from unittest import mock

SPEC = importlib.util.spec_from_file_location("live_smoke", Path(__file__).resolve().parents[1] / "live-smoke.py")
smoke = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(smoke)


def frame(identifier, result):
    return json.dumps({"jsonrpc": "2.0", "id": identifier, "result": result}) + "\n"


class FakeProcess:
    def __init__(self, frames="", timeout_once=False):
        self.stdin, self.stdout = io.StringIO(), io.StringIO(frames)
        self.terminated = self.killed = False
        self.timeout_once = timeout_once
        self.waits = 0

    def terminate(self):
        self.terminated = True

    def kill(self):
        self.killed = True

    def wait(self, timeout):
        self.waits += 1
        if self.timeout_once and self.waits == 1:
            raise subprocess.TimeoutExpired("fake", timeout)
        return 0


def frames(extra="", tools=None):
    return frame(1, {}) + frame(2, {"tools": [{"name": name} for name in (tools or smoke.DEFAULT_TOOLS)]}) + extra


class SessionTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.credentials = self.root / "credentials.json"
        self.credentials.write_text(json.dumps({"key": "PRIVATE TEST KEY", "key_id": "TEST_KEY_ID",
                                                 "issuer_id": "TEST_ISSUER"}))
        self.records = []

    def session(self, popen, **kwargs):
        return smoke.Session(self.root / "fake-binary", self.credentials, self.root, "workflow",
                             self.records, smoke.Budget(10), popen=popen, **kwargs)

    def test_allowed_reads_and_strict_boolean_dry_runs(self):
        for name in smoke.READ_TOOLS:
            smoke.validate_call(name, {})
        for name in smoke.DRY_TOOLS:
            smoke.validate_call(name, {"dryRun": True})
            for value in (False, 1, "true", None, [], {}):
                with self.assertRaises(smoke.SmokeError):
                    smoke.validate_call(name, {"dryRun": value})
            with self.assertRaises(smoke.SmokeError):
                smoke.validate_call(name, {})
        for name in ("asset_upload", "analyticsReportRequests_createInstance", "apps_updateInstance",
                     "appScreenshots_deleteInstance", "analytics_segment_download"):
            with self.assertRaises(smoke.SmokeError):
                smoke.validate_call(name, {"dryRun": True})
        with self.assertRaises(smoke.SmokeError):
            smoke.validate_call("report_query", {"output": "artifact"})
        with self.assertRaises(smoke.SmokeError):
            smoke.validate_call("app_get", [])

    def test_rpc_and_send_cannot_bypass_tool_allowlist(self):
        session = self.session(lambda *a, **kw: None)
        with self.assertRaises(smoke.SmokeError):
            session._rpc("tools/call", {"name": "asset_upload", "arguments": {}})
        with self.assertRaises(smoke.SmokeError):
            session._send({"method": "tools/call", "params": {"name": "release", "arguments": {"dryRun": 1}}})
        with self.assertRaises(smoke.SmokeError):
            session._rpc("resources/read", {})

    def test_credentials_are_mode_0600_and_removed_after_success(self):
        process = FakeProcess(frames())
        observed = {}
        def popen(command, **kwargs):
            observed.update(kwargs)
            observed["command"] = command
            key = Path(kwargs["env"]["ASC_PRIVATE_KEY_PATH"])
            observed["key"] = key
            observed["config"] = Path(kwargs["env"]["MCP_FACTORY_CONFIG"])
            self.assertEqual(stat.S_IMODE(key.stat().st_mode), 0o600)
            self.assertTrue(key.read_text() == "PRIVATE TEST KEY")
            self.assertEqual(kwargs["stderr"], subprocess.DEVNULL)
            self.assertNotIn("shell", kwargs)
            self.assertEqual(kwargs["env"]["MCP_FACTORY_BASE_URL"], "https://api.appstoreconnect.apple.com/")
            return process
        with mock.patch.dict(os.environ, {"ASC_OLD": "stale", "MCP_FACTORY_BEARER_TOKEN": "stale", "MCP_FACTORY_CONFIG": "stale"}):
            with self.session(popen):
                self.assertTrue(observed["key"].exists())
                self.assertNotIn("ASC_OLD", observed["env"])
                self.assertNotIn("MCP_FACTORY_BEARER_TOKEN", observed["env"])
        self.assertFalse(observed["key"].exists())
        self.assertFalse(observed["config"].exists())
        self.assertTrue(process.terminated)

    def test_popen_enter_failure_removes_key_and_config(self):
        observed = {}
        def fail(*args, **kwargs):
            observed["key"] = Path(kwargs["env"]["ASC_PRIVATE_KEY_PATH"])
            observed["config"] = Path(kwargs["env"]["MCP_FACTORY_CONFIG"])
            raise OSError("provider secret that must never reach output")
        with self.assertRaises(OSError):
            self.session(fail).__enter__()
        self.assertFalse(observed["key"].exists())
        self.assertFalse(observed["config"].exists())

    def test_handshake_enter_failure_terminates_process_and_removes_key(self):
        process = FakeProcess(frame(1, {}), timeout_once=True)
        session = self.session(lambda *args, **kwargs: process)
        with self.assertRaises(smoke.SmokeError):
            session.__enter__()
        self.assertFalse(Path(session.key.name).exists())
        self.assertFalse(Path(session.work.name).exists())
        self.assertTrue(process.terminated)
        self.assertTrue(process.killed)

    def test_second_wait_failure_still_removes_key(self):
        process = FakeProcess(frames())
        session = self.session(lambda *args, **kwargs: process)
        session.__enter__()
        process.wait = mock.Mock(side_effect=subprocess.TimeoutExpired("fake", 3))
        with self.assertRaises(subprocess.TimeoutExpired):
            session.close()
        self.assertFalse(Path(session.key.name).exists())
        self.assertFalse(Path(session.work.name).exists())

    def test_request_timeout_is_sanitized(self):
        session = self.session(lambda *a, **kw: None, timeout=0.01)
        session.process = FakeProcess()
        with self.assertRaisesRegex(smoke.SmokeError, "request_timeout"):
            session._rpc("tools/list", {})
        session.close()

    def test_invalid_credentials_fail_without_starting_subprocess(self):
        self.credentials.write_text('{"key": "secret", "key_id": null}')
        popen = mock.Mock()
        with self.assertRaisesRegex(smoke.SmokeError, "credentials_invalid"):
            self.session(popen).__enter__()
        popen.assert_not_called()

    def test_normalized_record_never_copies_provider_data(self):
        secret = "Authorization: Bearer PROVIDER_SECRET"
        body = {"dryRun": True, "changes": [{"credential": secret}], "unchanged": 1, "warnings": [secret],
                "rows": [{"privateKey": secret}], "details": {"url": secret}, "bundleId": secret}
        process = FakeProcess(frames(frame(3, {"structuredContent": body})))
        with self.session(lambda *args, **kwargs: process) as session:
            session.call("metadata_apply", {"dryRun": True, "bundleId": secret})
        output = json.dumps(self.records)
        self.assertNotIn(secret, output)
        self.assertNotIn("bundleId", output)
        self.assertEqual(self.records[0]["counts"], {"changes": 1, "unchanged": 1, "warnings": 1})
        self.assertEqual(self.records[0]["status"], "pass")

    def test_expected_blockers_negative_and_unexpected_are_distinct(self):
        blocked = smoke.normalize("release", "workflow", {"stage": "preflight", "message": "private"}, True, 0)
        auth = smoke.normalize("release", "workflow", {"stage": "lookup", "message": "401 private"}, True, 0)
        missing = smoke.normalize("app_get", "workflow", {"stage": "lookup", "message": "app selection matched 0 resources; expected exactly one"}, True, 0, True)
        negative_auth = smoke.normalize("app_get", "workflow", {"stage": "lookup", "message": "401"}, True, 0, True)
        unexpected_success = smoke.normalize("app_get", "workflow", {}, False, 0, True)
        self.assertEqual(blocked["status"], "expected_blocker")
        self.assertEqual(auth["status"], "unexpected_error")
        self.assertEqual(missing["status"], "expected_negative")
        self.assertEqual(negative_auth["status"], "unexpected_error")
        self.assertEqual(unexpected_success["status"], "unexpected_error")

    def test_default_surface_mismatch_is_detected_and_cleaned(self):
        session = self.session(lambda *a, **kw: FakeProcess(frames(tools={"asset_upload"})))
        with self.assertRaisesRegex(smoke.SmokeError, "default_tool_surface_mismatch"):
            session.__enter__()
        self.assertFalse(Path(session.key.name).exists())

    def test_budget_exhaustion_skips_without_sending(self):
        session = self.session(lambda *a, **kw: None)
        session.tools = {"app_get"}
        session.budget = smoke.Budget(0)
        self.assertEqual(session.call("app_get", {}), {})
        self.assertEqual(self.records[0]["reason"], "request_budget_exhausted")

    def test_empty_or_mutated_success_contracts_are_rejected(self):
        valid = {
            "app_get": {"id": "a", "bundleId": "example"},
            "version_get": {"app": {"id": "a"}, "version": {"id": "v"}},
            "release_status": {"app": {"id": "a"}, "version": {"id": "v"}},
            "release_prepare": {"readyForSubmission": True, "blockingIssues": []},
            "metadata_get": {"localizations": []},
            "screenshots_get": {"screenshots": []},
            "testflight_status": {"build": {"id": "b"}, "groups": []},
            "report_query": {"ok": True, "rowCount": 0, "columns": [], "sums": {}},
            "apps_getCollection": {"data": []},
        }
        valid.update({name: {"dryRun": True, "changes": []} for name in smoke.DRY_TOOLS})
        for name, body in valid.items():
            with self.subTest(tool=name):
                self.assertEqual(smoke.normalize(name, "workflow", body, False, 0)["status"], "pass")
                mutated = dict(body)
                mutated.pop(next(iter(mutated)))
                self.assertEqual(smoke.normalize(name, "workflow", mutated, False, 0)["reason"], "semantic_failure")
                self.assertEqual(smoke.normalize(name, "workflow", {}, False, 0)["status"], "unexpected_error")
        for body in ({"dryRun": 1, "changes": []}, {"dryRun": True, "changes": {}},
                     {"readyForSubmission": 1, "blockingIssues": []}, {"ok": True, "rowCount": True, "columns": [], "sums": {}}):
            name = "release_prepare" if "readyForSubmission" in body else "report_query" if "rowCount" in body else "metadata_apply"
            self.assertEqual(smoke.normalize(name, "workflow", body, False, 0)["reason"], "semantic_failure")

    def test_noop_and_change_counts_are_independent_semantic_assertions(self):
        no_op = {"dryRun": True, "changes": [], "unchanged": ["en-US"]}
        changed = {"dryRun": True, "changes": [{"action": "update", "resource": "localization"}], "unchanged": []}
        for body, expected in ((no_op, 0), (changed, 1)):
            self.assertEqual(smoke.normalize("metadata_apply", "workflow", body, False, 0, expected_changes=expected)["status"], "pass")
        for body, expected in ((no_op, 1), (changed, 0)):
            report = smoke.normalize("metadata_apply", "workflow", body, False, 0, expected_changes=expected)
            self.assertEqual(report["status"], "unexpected_error")
            self.assertEqual(report["reason"], "semantic_failure")
            self.assertEqual(report["expectedChanges"], expected)

    def test_call_rejects_false_success_and_reports_only_byte_count(self):
        body = {"dryRun": True, "changes": [{"private": "SOURCE_PRIVATE"}]}
        process = FakeProcess(frames(frame(3, {"structuredContent": body})))
        with self.session(lambda *a, **kw: process) as session:
            self.assertEqual(session.call("metadata_apply", {"dryRun": True}, expected_changes=0), {})
        self.assertEqual(self.records[0]["reason"], "semantic_failure")
        expected_bytes = len(json.dumps(body, separators=(",", ":"), ensure_ascii=False).encode())
        self.assertEqual(self.records[0]["summaryBytes"], expected_bytes)
        self.assertNotIn("SOURCE_PRIVATE", json.dumps(self.records))

    def test_workflow_noop_uses_only_existing_metadata_and_platform_display(self):
        class FixtureSession:
            def __init__(self, localizations):
                self.localizations, self.calls, self.skips = localizations, [], []
            def call(self, name, args, **options):
                smoke.validate_call(name, args)
                self.calls.append((name, args, options))
                return {"localizations": self.localizations} if name == "metadata_get" else {}
            def skip(self, name, reason):
                self.skips.append((name, reason))
        def plan(platform):
            return {"bundle": "example", "locale": "en-US", "versions": [{"attributes": {"versionString": "1", "platform": platform}}], "builds": [], "groups": []}
        missing = FixtureSession([])
        smoke.check_workflows(missing, [plan("IOS")], [], False)
        metadata = [call for call in missing.calls if call[0] == "metadata_apply"]
        self.assertEqual(len(metadata), 1)
        self.assertEqual(metadata[0][2]["expected_changes"], 1)
        self.assertIn(("metadata_apply", "no_localization_for_noop"), missing.skips)
        for platform, display in (("IOS", "APP_IPHONE_67"), ("MAC_OS", "APP_DESKTOP")):
            fixture = FixtureSession([{"locale": "en-US"}])
            smoke.check_workflows(fixture, [plan(platform)], [], True)
            no_op = next(c for c in fixture.calls if c[0] == "metadata_apply" and c[2].get("expected_changes") == 0)
            self.assertEqual(no_op[1]["localizations"], {"en-US": {}})
            screenshot = next(c for c in fixture.calls if c[0] == "screenshots_sync")
            self.assertEqual(screenshot[1]["displayType"], display)
            self.assertEqual(screenshot[1]["files"], ["smoke.png"])
        unsupported = FixtureSession([{"locale": "en-US"}])
        smoke.check_workflows(unsupported, [plan("TV_OS")], [], True)
        self.assertFalse(any(c[0] == "screenshots_sync" for c in unsupported.calls))
        self.assertIn(("screenshots_sync", "screenshot_platform_unsupported"), unsupported.skips)


class RunnerTests(unittest.TestCase):
    def test_no_opt_in_does_not_start_session(self):
        factory = mock.Mock()
        with contextlib.redirect_stderr(io.StringIO()), self.assertRaises(SystemExit) as error:
            smoke.main([], session_factory=factory)
        self.assertEqual(error.exception.code, 2)
        factory.assert_not_called()

    def test_opaque_failure_prints_only_normalized_reason(self):
        def factory(*a, **kw):
            raise OSError("AuthKey secret provider dump")
        out = io.StringIO()
        with contextlib.redirect_stdout(out):
            code = smoke.main(["--credentials", "fake.json"], session_factory=factory)
        self.assertEqual(code, 1)
        summary = json.loads(out.getvalue())
        self.assertEqual(summary["error"], "runner_failure")
        self.assertNotIn("AuthKey", out.getvalue())
        self.assertNotIn("fake.json", out.getvalue())

    def test_empty_discovery_skips_honestly_and_writes_0600_summary(self):
        class EmptySession:
            def __init__(self, binary, credentials, media, profile, records, budget, timeout):
                self.records, self.profile, self.media = records, profile, media
                self.budget = budget
            def __enter__(self):
                self.assert_fixture = (self.media / "smoke.png").read_bytes().startswith(b"\x89PNG")
                if not self.assert_fixture:
                    raise AssertionError()
                return self
            def __exit__(self, *_):
                pass
            def call(self, name, args, negative=False, expected_changes=None):
                smoke.validate_call(name, args)
                self.budget.take()
                self.records.append(smoke.normalize(name, self.profile,
                    {"stage": "lookup", "message": "matched 0 resources"} if negative else {"data": []}, negative, 0, negative))
                return {}
            def skip(self, name, reason):
                self.records.append({"tool": name, "profile": self.profile, "status": "skipped", "reason": reason})
        with tempfile.TemporaryDirectory() as directory:
            output = Path(directory) / "summary.json"
            out = io.StringIO()
            with contextlib.redirect_stdout(out):
                code = smoke.main(["--credentials", "fake.json", "--output", str(output), "--screenshots"], session_factory=EmptySession)
            self.assertEqual(code, 0)
            summary = json.loads(output.read_text())
            self.assertEqual(summary["statuses"]["skipped"], 2)
            self.assertEqual(summary["statuses"]["expected_negative"], 1)
            self.assertEqual(stat.S_IMODE(output.stat().st_mode), 0o600)
            self.assertEqual(json.loads(out.getvalue()), summary)

    def test_unexpected_tool_error_produces_nonzero_exit(self):
        class FailedSession:
            def __init__(self, binary, credentials, media, profile, records, budget, timeout):
                self.records = records
            def __enter__(self):
                self.records.append({"tool": "apps_getCollection", "profile": "all", "status": "unexpected_error"})
                return self
            def __exit__(self, *_):
                pass
            def call(self, *a, **kw):
                return {}
            def skip(self, name, reason):
                pass
        with contextlib.redirect_stdout(io.StringIO()):
            self.assertEqual(smoke.main(["--credentials", "fake.json"], session_factory=FailedSession), 1)

    def test_output_cannot_overwrite_credentials_or_follow_final_symlink(self):
        with tempfile.TemporaryDirectory() as directory:
            credentials = Path(directory) / "credentials.json"
            credentials.write_text("PRIVATE FIXTURE")
            factory = mock.Mock()
            with contextlib.redirect_stderr(io.StringIO()), self.assertRaises(SystemExit):
                smoke.main(["--credentials", str(credentials), "--output", str(credentials)], session_factory=factory)
            factory.assert_not_called()
            self.assertTrue(credentials.read_text() == "PRIVATE FIXTURE")
            hardlink = Path(directory) / "hardlink.json"
            os.link(credentials, hardlink)
            with contextlib.redirect_stderr(io.StringIO()), self.assertRaises(SystemExit):
                smoke.main(["--credentials", str(credentials), "--output", str(hardlink)], session_factory=factory)
            self.assertTrue(credentials.read_text() == "PRIVATE FIXTURE")
            if not hasattr(os, "O_NOFOLLOW"):
                return
            output = Path(directory) / "output.json"
            output.symlink_to(credentials)
            with contextlib.redirect_stderr(io.StringIO()), self.assertRaises(SystemExit):
                smoke.main(["--credentials", str(credentials), "--output", str(output)], session_factory=factory)
            self.assertTrue(credentials.read_text() == "PRIVATE FIXTURE")
            target = Path(directory) / "other.json"
            target.write_text("PRESERVE")
            output.unlink()
            output.symlink_to(target)
            def fail(*a, **kw):
                raise smoke.SmokeError("server_exited")
            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                self.assertEqual(smoke.main(["--credentials", str(credentials), "--output", str(output)], session_factory=fail), 1)
            self.assertTrue(target.read_text() == "PRESERVE")
            self.assertEqual(json.loads(stdout.getvalue())["error"], "summary_write_failed")

    def test_utc_hash_and_per_tool_byte_maxima_are_normalized(self):
        class FixtureSession:
            def __init__(self, binary, credentials, media, profile, records, budget, timeout):
                self.records, self.profile = records, profile
            def __enter__(self):
                for size in (12, 20):
                    self.records.append({"tool": "metadata_get", "profile": self.profile, "status": "pass", "summaryBytes": size})
                return self
            def __exit__(self, *_):
                pass
            def call(self, *a, **kw):
                return {}
            def skip(self, name, reason):
                pass
        with tempfile.TemporaryDirectory() as directory:
            binary = Path(directory) / "binary"
            binary.write_bytes(b"fixture")
            output = io.StringIO()
            with contextlib.redirect_stdout(output):
                self.assertEqual(smoke.main(["--credentials", "fake.json", "--binary", str(binary)], session_factory=FixtureSession), 0)
            summary = json.loads(output.getvalue())
            self.assertEqual(summary["summaryBytesTotal"], 64)
            self.assertEqual(summary["maxSummaryBytesByTool"], {"metadata_get": 20})
            self.assertEqual(len(summary["binarySha256"]), 64)
            self.assertEqual(len(summary["runnerSha256"]), 64)
            self.assertTrue(summary["finishedAtUtc"].endswith("+00:00"))
            self.assertNotIn(str(binary), output.getvalue())

    def test_default_credential_path_requires_explicit_live_flag(self):
        observed = {}
        def factory(binary, credentials, *args):
            observed["credentials"] = credentials
            raise smoke.SmokeError("server_exited")
        with mock.patch.object(smoke.Path, "home", return_value=Path("/fake-home")), contextlib.redirect_stdout(io.StringIO()):
            self.assertEqual(smoke.main(["--live"], session_factory=factory), 1)
        self.assertEqual(observed["credentials"], Path("/fake-home/.appstoreconnect/api_key.json"))


if __name__ == "__main__":
    unittest.main()
