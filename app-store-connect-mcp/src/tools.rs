use mcp_factory_core::ExecutionKind;
use mcp_factory_core::ParamBinding;
use mcp_factory_core::ParamLocation;
use mcp_factory_core::RestOperation;
use mcp_factory_core::ToolHints;
use mcp_factory_core::ToolSpec;
pub fn build_tools() -> Vec<ToolSpec> {
    vec![
        ToolSpec {
            name: "ageRatingDeclarations_updateInstance".to_string(),
            description: "Update age-rating questionnaire answers. The values must come from the user; the resulting rating is shown on the App Info.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"advertising":{"nullable":true,"type":"boolean"},"ageAssurance":{"nullable":true,"type":"boolean"},"ageRatingOverride":{"deprecated":true,"enum":["NONE","NINE_PLUS","THIRTEEN_PLUS","SIXTEEN_PLUS","SEVENTEEN_PLUS","UNRATED"],"nullable":true,"type":"string"},"ageRatingOverrideV2":{"enum":["NONE","NINE_PLUS","THIRTEEN_PLUS","SIXTEEN_PLUS","EIGHTEEN_PLUS","UNRATED"],"nullable":true,"type":"string"},"alcoholTobaccoOrDrugUseOrReferences":{"enum":["NONE","INFREQUENT_OR_MILD","FREQUENT_OR_INTENSE","INFREQUENT","FREQUENT"],"nullable":true,"type":"string"},"contests":{"enum":["NONE","INFREQUENT_OR_MILD","FREQUENT_OR_INTENSE","INFREQUENT","FREQUENT"],"nullable":true,"type":"string"},"developerAgeRatingInfoUrl":{"format":"uri","nullable":true,"type":"string"},"gambling":{"nullable":true,"type":"boolean"},"gamblingSimulated":{"enum":["NONE","INFREQUENT_OR_MILD","FREQUENT_OR_INTENSE","INFREQUENT","FREQUENT"],"nullable":true,"type":"string"},"gracRatingClassificationNumber":{"nullable":true,"type":"string"},"gunsOrOtherWeapons":{"enum":["NONE","INFREQUENT_OR_MILD","FREQUENT_OR_INTENSE","INFREQUENT","FREQUENT"],"nullable":true,"type":"string"},"healthOrWellnessTopics":{"nullable":true,"type":"boolean"},"horrorOrFearThemes":{"enum":["NONE","INFREQUENT_OR_MILD","FREQUENT_OR_INTENSE","INFREQUENT","FREQUENT"],"nullable":true,"type":"string"},"kidsAgeBand":{"enum":["FIVE_AND_UNDER","SIX_TO_EIGHT","NINE_TO_ELEVEN"],"type":"string"},"koreaAgeRatingOverride":{"enum":["NONE","ALL","TWELVE_PLUS","FIFTEEN_PLUS","NINETEEN_PLUS"],"nullable":true,"type":"string"},"lootBox":{"nullable":true,"type":"boolean"},"matureOrSuggestiveThemes":{"enum":["NONE","INFREQUENT_OR_MILD","FREQUENT_OR_INTENSE","INFREQUENT","FREQUENT"],"nullable":true,"type":"string"},"medicalOrTreatmentInformation":{"enum":["NONE","INFREQUENT_OR_MILD","FREQUENT_OR_INTENSE","INFREQUENT","FREQUENT"],"nullable":true,"type":"string"},"messagingAndChat":{"nullable":true,"type":"boolean"},"parentalControls":{"nullable":true,"type":"boolean"},"profanityOrCrudeHumor":{"enum":["NONE","INFREQUENT_OR_MILD","FREQUENT_OR_INTENSE","INFREQUENT","FREQUENT"],"nullable":true,"type":"string"},"sexualContentGraphicAndNudity":{"enum":["NONE","INFREQUENT_OR_MILD","FREQUENT_OR_INTENSE","INFREQUENT","FREQUENT"],"nullable":true,"type":"string"},"sexualContentOrNudity":{"enum":["NONE","INFREQUENT_OR_MILD","FREQUENT_OR_INTENSE","INFREQUENT","FREQUENT"],"nullable":true,"type":"string"},"socialMedia":{"nullable":true,"type":"boolean"},"socialMediaAgeRestricted":{"nullable":true,"type":"boolean"},"unrestrictedWebAccess":{"nullable":true,"type":"boolean"},"userGeneratedContent":{"nullable":true,"type":"boolean"},"violenceCartoonOrFantasy":{"enum":["NONE","INFREQUENT_OR_MILD","FREQUENT_OR_INTENSE","INFREQUENT","FREQUENT"],"nullable":true,"type":"string"},"violenceRealistic":{"enum":["NONE","INFREQUENT_OR_MILD","FREQUENT_OR_INTENSE","INFREQUENT","FREQUENT"],"nullable":true,"type":"string"},"violenceRealisticProlongedGraphicOrSadistic":{"enum":["NONE","INFREQUENT_OR_MILD","FREQUENT_OR_INTENSE","INFREQUENT","FREQUENT"],"nullable":true,"type":"string"}},"type":"object"},"id":{"type":"string"},"type":{"enum":["ageRatingDeclarations"],"type":"string"}},"required":["id","type"],"type":"object"},"id":{"type":"string"}},"required":["id","data"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "PATCH".to_string(),
                path_template: "/v1/ageRatingDeclarations/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "analyticsReportInstances_getInstance".to_string(),
            description: "Get one `analyticsReportInstances` resource by id.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[analyticsReportInstances]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/analyticsReportInstances/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[analyticsReportInstances]".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "analyticsReportRequests_createInstance".to_string(),
            description: "Create one `analyticsReportRequests` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"accessType":{"enum":["ONE_TIME_SNAPSHOT","ONGOING"],"type":"string"}},"required":["accessType"],"type":"object"},"relationships":{"properties":{"app":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["apps"],"type":"string"}},"required":["id","type"],"type":"object"}},"required":["data"],"type":"object"}},"required":["app"],"type":"object"},"type":{"enum":["analyticsReportRequests"],"type":"string"}},"required":["relationships","attributes","type"],"type":"object"}},"required":["data"],"title":"AnalyticsReportRequestCreateRequest","type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "POST".to_string(),
                path_template: "/v1/analyticsReportRequests".to_string(),
                params: vec![
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "analyticsReportRequests_getInstance".to_string(),
            description: "Get one `analyticsReportRequests` resource by id.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[analyticsReportRequests]":{"items":{"type":"string"},"type":"array"},"fields[analyticsReports]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["reports"],"type":"string"},"type":"array"},"limit[reports]":{"maximum":50,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/analyticsReportRequests/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[analyticsReportRequests]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[analyticsReports]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[reports]".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "analyticsReportRequests_deleteInstance".to_string(),
            description: "Delete one `analyticsReportRequests` resource. This cannot be undone.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"id":{"type":"string"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "DELETE".to_string(),
                path_template: "/v1/analyticsReportRequests/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(true),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "analyticsReportSegments_getInstance".to_string(),
            description: "Get an analytics report segment, including its download `url`. Use `analytics_segment_download` to fetch and decompress it.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[analyticsReportSegments]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/analyticsReportSegments/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[analyticsReportSegments]".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "analyticsReports_getInstance".to_string(),
            description: "Get one `analyticsReports` resource by id.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[analyticsReports]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/analyticsReports/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[analyticsReports]".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appAvailabilitiesV2_createInstance".to_string(),
            description: "Create one `appAvailabilities` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"availableInNewTerritories":{"type":"boolean"}},"required":["availableInNewTerritories"],"type":"object"},"relationships":{"properties":{"app":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["apps"],"type":"string"}},"required":["id","type"],"type":"object"}},"required":["data"],"type":"object"},"territoryAvailabilities":{"properties":{"data":{"items":{"properties":{"id":{"type":"string"},"type":{"enum":["territoryAvailabilities"],"type":"string"}},"required":["id","type"],"type":"object"},"type":"array"}},"required":["data"],"type":"object"}},"required":["app","territoryAvailabilities"],"type":"object"},"type":{"enum":["appAvailabilities"],"type":"string"}},"required":["relationships","attributes","type"],"type":"object"},"included":{"items":{"properties":{"attributes":{"properties":{"available":{"nullable":true,"type":"boolean"},"preOrderEnabled":{"nullable":true,"type":"boolean"},"releaseDate":{"format":"date","nullable":true,"type":"string"}},"type":"object"},"id":{"type":"string"},"relationships":{"properties":{"territory":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["territories"],"type":"string"}},"required":["id","type"],"type":"object"}},"type":"object"}},"type":"object"},"type":{"enum":["territoryAvailabilities"],"type":"string"}},"required":["type"],"type":"object"},"type":"array"}},"required":["data"],"title":"AppAvailabilityV2CreateRequest","type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "POST".to_string(),
                path_template: "/v2/appAvailabilities".to_string(),
                params: vec![
                ],
                body_fields: vec![
                    "data".to_string(),
                    "included".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appAvailabilitiesV2_getInstance".to_string(),
            description: "Get one `appAvailabilities` resource by id.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[appAvailabilities]":{"items":{"type":"string"},"type":"array"},"fields[territoryAvailabilities]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["territoryAvailabilities"],"type":"string"},"type":"array"},"limit[territoryAvailabilities]":{"maximum":50,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v2/appAvailabilities/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[appAvailabilities]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[territoryAvailabilities]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[territoryAvailabilities]".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appCategories_getCollection".to_string(),
            description: "List `appCategories` resources; supports the filters and pagination parameters below.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"exists[parent]":{"type":"boolean"},"fields[appCategories]":{"items":{"type":"string"},"type":"array"},"filter[platforms]":{"items":{"enum":["IOS","MAC_OS","TV_OS","VISION_OS"],"type":"string"},"type":"array"},"include":{"items":{"enum":["subcategories","parent"],"type":"string"},"type":"array"},"limit":{"maximum":200,"type":"integer"},"limit[subcategories]":{"maximum":50,"type":"integer"}},"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/appCategories".to_string(),
                params: vec![
                    ParamBinding {
                        name: "filter[platforms]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "exists[parent]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appCategories]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[subcategories]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appCategories_getInstance".to_string(),
            description: "Get one `appCategories` resource by id.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[appCategories]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["subcategories","parent"],"type":"string"},"type":"array"},"limit[subcategories]":{"maximum":50,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/appCategories/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[appCategories]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[subcategories]".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appEncryptionDeclarationDocuments_createInstance".to_string(),
            description: "Create one `appEncryptionDeclarationDocuments` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"fileName":{"type":"string"},"fileSize":{"type":"integer"}},"required":["fileName","fileSize"],"type":"object"},"relationships":{"properties":{"appEncryptionDeclaration":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["appEncryptionDeclarations"],"type":"string"}},"required":["id","type"],"type":"object"}},"required":["data"],"type":"object"}},"required":["appEncryptionDeclaration"],"type":"object"},"type":{"enum":["appEncryptionDeclarationDocuments"],"type":"string"}},"required":["relationships","attributes","type"],"type":"object"}},"required":["data"],"title":"AppEncryptionDeclarationDocumentCreateRequest","type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "POST".to_string(),
                path_template: "/v1/appEncryptionDeclarationDocuments".to_string(),
                params: vec![
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appEncryptionDeclarationDocuments_getInstance".to_string(),
            description: "Get one `appEncryptionDeclarationDocuments` resource by id.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[appEncryptionDeclarationDocuments]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/appEncryptionDeclarationDocuments/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[appEncryptionDeclarationDocuments]".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appEncryptionDeclarationDocuments_updateInstance".to_string(),
            description: "Update attributes or relationships of one `appEncryptionDeclarationDocuments` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"sourceFileChecksum":{"nullable":true,"type":"string"},"uploaded":{"nullable":true,"type":"boolean"}},"type":"object"},"id":{"type":"string"},"type":{"enum":["appEncryptionDeclarationDocuments"],"type":"string"}},"required":["id","type"],"type":"object"},"id":{"type":"string"}},"required":["id","data"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "PATCH".to_string(),
                path_template: "/v1/appEncryptionDeclarationDocuments/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appEncryptionDeclarations_getCollection".to_string(),
            description: "List `appEncryptionDeclarations` resources; supports the filters and pagination parameters below.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[appEncryptionDeclarationDocuments]":{"items":{"type":"string"},"type":"array"},"fields[appEncryptionDeclarations]":{"items":{"type":"string"},"type":"array"},"fields[apps]":{"items":{"type":"string"},"type":"array"},"fields[builds]":{"items":{"type":"string"},"type":"array"},"filter[app]":{"items":{"type":"string"},"type":"array"},"filter[builds]":{"items":{"type":"string"},"type":"array"},"filter[platform]":{"items":{"enum":["IOS","MAC_OS","TV_OS","VISION_OS"],"type":"string"},"type":"array"},"include":{"items":{"enum":["app","builds","appEncryptionDeclarationDocument"],"type":"string"},"type":"array"},"limit":{"maximum":200,"type":"integer"},"limit[builds]":{"maximum":50,"type":"integer"}},"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/appEncryptionDeclarations".to_string(),
                params: vec![
                    ParamBinding {
                        name: "filter[platform]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[app]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[builds]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appEncryptionDeclarations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[apps]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[builds]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appEncryptionDeclarationDocuments]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[builds]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appEncryptionDeclarations_createInstance".to_string(),
            description: "Create an export-compliance declaration from answers supplied by the user. This tool never decides compliance.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"appDescription":{"type":"string"},"availableOnFrenchStore":{"type":"boolean"},"containsProprietaryCryptography":{"type":"boolean"},"containsThirdPartyCryptography":{"type":"boolean"}},"required":["availableOnFrenchStore","appDescription","containsThirdPartyCryptography","containsProprietaryCryptography"],"type":"object"},"relationships":{"properties":{"app":{"deprecated":true,"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["apps"],"type":"string"}},"required":["id","type"],"type":"object"}},"required":["data"],"type":"object"}},"required":["app"],"type":"object"},"type":{"enum":["appEncryptionDeclarations"],"type":"string"}},"required":["relationships","attributes","type"],"type":"object"}},"required":["data"],"title":"AppEncryptionDeclarationCreateRequest","type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "POST".to_string(),
                path_template: "/v1/appEncryptionDeclarations".to_string(),
                params: vec![
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appEncryptionDeclarations_getInstance".to_string(),
            description: "Get one `appEncryptionDeclarations` resource by id.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[appEncryptionDeclarationDocuments]":{"items":{"type":"string"},"type":"array"},"fields[appEncryptionDeclarations]":{"items":{"type":"string"},"type":"array"},"fields[apps]":{"items":{"type":"string"},"type":"array"},"fields[builds]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["app","builds","appEncryptionDeclarationDocument"],"type":"string"},"type":"array"},"limit[builds]":{"maximum":50,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/appEncryptionDeclarations/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[appEncryptionDeclarations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[apps]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[builds]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appEncryptionDeclarationDocuments]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[builds]".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appInfoLocalizations_createInstance".to_string(),
            description: "Create one `appInfoLocalizations` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"locale":{"type":"string"},"name":{"type":"string"},"privacyChoicesUrl":{"nullable":true,"type":"string"},"privacyPolicyText":{"nullable":true,"type":"string"},"privacyPolicyUrl":{"nullable":true,"type":"string"},"subtitle":{"nullable":true,"type":"string"}},"required":["name","locale"],"type":"object"},"relationships":{"properties":{"appInfo":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["appInfos"],"type":"string"}},"required":["id","type"],"type":"object"}},"required":["data"],"type":"object"}},"required":["appInfo"],"type":"object"},"type":{"enum":["appInfoLocalizations"],"type":"string"}},"required":["relationships","attributes","type"],"type":"object"}},"required":["data"],"title":"AppInfoLocalizationCreateRequest","type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "POST".to_string(),
                path_template: "/v1/appInfoLocalizations".to_string(),
                params: vec![
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appInfoLocalizations_getInstance".to_string(),
            description: "Get one `appInfoLocalizations` resource by id.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[appInfoLocalizations]":{"items":{"type":"string"},"type":"array"},"fields[appInfos]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["appInfo"],"type":"string"},"type":"array"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/appInfoLocalizations/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[appInfoLocalizations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appInfos]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appInfoLocalizations_updateInstance".to_string(),
            description: "Update app-level text for one locale: `name`, `subtitle`, `privacyPolicyUrl`, `privacyChoicesUrl`, `privacyPolicyText`.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"name":{"nullable":true,"type":"string"},"privacyChoicesUrl":{"nullable":true,"type":"string"},"privacyPolicyText":{"nullable":true,"type":"string"},"privacyPolicyUrl":{"nullable":true,"type":"string"},"subtitle":{"nullable":true,"type":"string"}},"type":"object"},"id":{"type":"string"},"type":{"enum":["appInfoLocalizations"],"type":"string"}},"required":["id","type"],"type":"object"},"id":{"type":"string"}},"required":["id","data"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "PATCH".to_string(),
                path_template: "/v1/appInfoLocalizations/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appInfoLocalizations_deleteInstance".to_string(),
            description: "Delete one `appInfoLocalizations` resource. This cannot be undone.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"id":{"type":"string"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "DELETE".to_string(),
                path_template: "/v1/appInfoLocalizations/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(true),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appInfos_getInstance".to_string(),
            description: "Get one `appInfos` resource by id.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[ageRatingDeclarations]":{"items":{"type":"string"},"type":"array"},"fields[appCategories]":{"items":{"type":"string"},"type":"array"},"fields[appInfoLocalizations]":{"items":{"type":"string"},"type":"array"},"fields[appInfos]":{"items":{"type":"string"},"type":"array"},"fields[apps]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["app","ageRatingDeclaration","appInfoLocalizations","primaryCategory","primarySubcategoryOne","primarySubcategoryTwo","secondaryCategory","secondarySubcategoryOne","secondarySubcategoryTwo"],"type":"string"},"type":"array"},"limit[appInfoLocalizations]":{"maximum":50,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/appInfos/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[appInfos]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[apps]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[ageRatingDeclarations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appInfoLocalizations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appCategories]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[appInfoLocalizations]".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appInfos_updateInstance".to_string(),
            description: "Update App Info relationships such as `primaryCategory`/`secondaryCategory` (IDs from `appCategories_getCollection`).".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"id":{"type":"string"},"relationships":{"properties":{"primaryCategory":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["appCategories"],"type":"string"}},"required":["id","type"],"type":"object"}},"type":"object"},"primarySubcategoryOne":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["appCategories"],"type":"string"}},"required":["id","type"],"type":"object"}},"type":"object"},"primarySubcategoryTwo":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["appCategories"],"type":"string"}},"required":["id","type"],"type":"object"}},"type":"object"},"secondaryCategory":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["appCategories"],"type":"string"}},"required":["id","type"],"type":"object"}},"type":"object"},"secondarySubcategoryOne":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["appCategories"],"type":"string"}},"required":["id","type"],"type":"object"}},"type":"object"},"secondarySubcategoryTwo":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["appCategories"],"type":"string"}},"required":["id","type"],"type":"object"}},"type":"object"}},"type":"object"},"type":{"enum":["appInfos"],"type":"string"}},"required":["id","type"],"type":"object"},"id":{"type":"string"}},"required":["id","data"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "PATCH".to_string(),
                path_template: "/v1/appInfos/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appPreviewSets_createInstance".to_string(),
            description: "Create one `appPreviewSets` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"previewType":{"enum":["IPHONE_67","IPHONE_61","IPHONE_65","IPHONE_58","IPHONE_55","IPHONE_47","IPHONE_40","IPHONE_35","IPAD_PRO_3GEN_129","IPAD_PRO_3GEN_11","IPAD_PRO_129","IPAD_105","IPAD_97","DESKTOP","APPLE_TV","APPLE_VISION_PRO"],"type":"string"}},"required":["previewType"],"type":"object"},"relationships":{"properties":{"appCustomProductPageLocalization":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["appCustomProductPageLocalizations"],"type":"string"}},"required":["id","type"],"type":"object"}},"type":"object"},"appStoreVersionExperimentTreatmentLocalization":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["appStoreVersionExperimentTreatmentLocalizations"],"type":"string"}},"required":["id","type"],"type":"object"}},"type":"object"},"appStoreVersionLocalization":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["appStoreVersionLocalizations"],"type":"string"}},"required":["id","type"],"type":"object"}},"type":"object"}},"type":"object"},"type":{"enum":["appPreviewSets"],"type":"string"}},"required":["attributes","type"],"type":"object"}},"required":["data"],"title":"AppPreviewSetCreateRequest","type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "POST".to_string(),
                path_template: "/v1/appPreviewSets".to_string(),
                params: vec![
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appPreviewSets_getInstance".to_string(),
            description: "Get one `appPreviewSets` resource by id.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[appCustomProductPageLocalizations]":{"items":{"type":"string"},"type":"array"},"fields[appPreviewSets]":{"items":{"type":"string"},"type":"array"},"fields[appPreviews]":{"items":{"type":"string"},"type":"array"},"fields[appStoreVersionExperimentTreatmentLocalizations]":{"items":{"type":"string"},"type":"array"},"fields[appStoreVersionLocalizations]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["appStoreVersionLocalization","appCustomProductPageLocalization","appStoreVersionExperimentTreatmentLocalization","appPreviews"],"type":"string"},"type":"array"},"limit[appPreviews]":{"maximum":50,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/appPreviewSets/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[appPreviewSets]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appStoreVersionLocalizations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appCustomProductPageLocalizations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appStoreVersionExperimentTreatmentLocalizations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appPreviews]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[appPreviews]".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appPreviewSets_deleteInstance".to_string(),
            description: "Delete one `appPreviewSets` resource. This cannot be undone.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"id":{"type":"string"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "DELETE".to_string(),
                path_template: "/v1/appPreviewSets/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(true),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appPreviews_createInstance".to_string(),
            description: "Low-level step 1 of an upload (reserve). Prefer `asset_upload`.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"fileName":{"type":"string"},"fileSize":{"type":"integer"},"mimeType":{"nullable":true,"type":"string"},"previewFrameTimeCode":{"nullable":true,"type":"string"}},"required":["fileName","fileSize"],"type":"object"},"relationships":{"properties":{"appPreviewSet":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["appPreviewSets"],"type":"string"}},"required":["id","type"],"type":"object"}},"required":["data"],"type":"object"}},"required":["appPreviewSet"],"type":"object"},"type":{"enum":["appPreviews"],"type":"string"}},"required":["relationships","attributes","type"],"type":"object"}},"required":["data"],"title":"AppPreviewCreateRequest","type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "POST".to_string(),
                path_template: "/v1/appPreviews".to_string(),
                params: vec![
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appPreviews_getInstance".to_string(),
            description: "Get one `appPreviews` resource by id.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[appPreviewSets]":{"items":{"type":"string"},"type":"array"},"fields[appPreviews]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["appPreviewSet"],"type":"string"},"type":"array"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/appPreviews/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[appPreviews]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appPreviewSets]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appPreviews_updateInstance".to_string(),
            description: "Low-level commit step or `previewFrameTimeCode` update. Prefer `asset_upload` for uploads.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"previewFrameTimeCode":{"nullable":true,"type":"string"},"sourceFileChecksum":{"nullable":true,"type":"string"},"uploaded":{"nullable":true,"type":"boolean"}},"type":"object"},"id":{"type":"string"},"type":{"enum":["appPreviews"],"type":"string"}},"required":["id","type"],"type":"object"},"id":{"type":"string"}},"required":["id","data"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "PATCH".to_string(),
                path_template: "/v1/appPreviews/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appPreviews_deleteInstance".to_string(),
            description: "Delete one `appPreviews` resource. This cannot be undone.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"id":{"type":"string"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "DELETE".to_string(),
                path_template: "/v1/appPreviews/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(true),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appPricePointsV3_getInstance".to_string(),
            description: "Get one `appPricePoints` resource by id.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[appPricePoints]":{"items":{"type":"string"},"type":"array"},"fields[apps]":{"items":{"type":"string"},"type":"array"},"fields[territories]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["app","territory"],"type":"string"},"type":"array"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v3/appPricePoints/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[appPricePoints]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[apps]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[territories]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appPriceSchedules_createInstance".to_string(),
            description: "Create one `appPriceSchedules` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"relationships":{"properties":{"app":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["apps"],"type":"string"}},"required":["id","type"],"type":"object"}},"required":["data"],"type":"object"},"baseTerritory":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["territories"],"type":"string"}},"required":["id","type"],"type":"object"}},"required":["data"],"type":"object"},"manualPrices":{"properties":{"data":{"items":{"properties":{"id":{"type":"string"},"type":{"enum":["appPrices"],"type":"string"}},"required":["id","type"],"type":"object"},"type":"array"}},"required":["data"],"type":"object"}},"required":["app","manualPrices","baseTerritory"],"type":"object"},"type":{"enum":["appPriceSchedules"],"type":"string"}},"required":["relationships","type"],"type":"object"},"included":{"items":{"oneOf":[{"properties":{"attributes":{"properties":{"endDate":{"format":"date","nullable":true,"type":"string"},"startDate":{"format":"date","nullable":true,"type":"string"}},"type":"object"},"id":{"type":"string"},"relationships":{"properties":{"appPricePoint":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["appPricePoints"],"type":"string"}},"required":["id","type"],"type":"object"}},"type":"object"}},"type":"object"},"type":{"enum":["appPrices"],"type":"string"}},"required":["type"],"type":"object"},{"properties":{"id":{"type":"string"},"type":{"enum":["territories"],"type":"string"}},"required":["type"],"type":"object"}]},"type":"array"}},"required":["data"],"title":"AppPriceScheduleCreateRequest","type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "POST".to_string(),
                path_template: "/v1/appPriceSchedules".to_string(),
                params: vec![
                ],
                body_fields: vec![
                    "data".to_string(),
                    "included".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appPriceSchedules_getInstance".to_string(),
            description: "Get one `appPriceSchedules` resource by id.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[appPriceSchedules]":{"items":{"type":"string"},"type":"array"},"fields[appPrices]":{"items":{"type":"string"},"type":"array"},"fields[apps]":{"items":{"type":"string"},"type":"array"},"fields[territories]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["app","baseTerritory","manualPrices","automaticPrices"],"type":"string"},"type":"array"},"limit[automaticPrices]":{"maximum":50,"type":"integer"},"limit[manualPrices]":{"maximum":50,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/appPriceSchedules/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[appPriceSchedules]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[apps]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[territories]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appPrices]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[automaticPrices]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[manualPrices]".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appScreenshotSets_createInstance".to_string(),
            description: "Create a screenshot set for one `screenshotDisplayType` (e.g. APP_IPHONE_67) under a version localization. List existing sets first.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"screenshotDisplayType":{"enum":["APP_IPHONE_67","APP_IPHONE_61","APP_IPHONE_65","APP_IPHONE_58","APP_IPHONE_55","APP_IPHONE_47","APP_IPHONE_40","APP_IPHONE_35","APP_IPAD_PRO_3GEN_129","APP_IPAD_PRO_3GEN_11","APP_IPAD_PRO_129","APP_IPAD_105","APP_IPAD_97","APP_DESKTOP","APP_WATCH_ULTRA","APP_WATCH_SERIES_10","APP_WATCH_SERIES_7","APP_WATCH_SERIES_4","APP_WATCH_SERIES_3","APP_APPLE_TV","APP_APPLE_VISION_PRO","IMESSAGE_APP_IPHONE_67","IMESSAGE_APP_IPHONE_61","IMESSAGE_APP_IPHONE_65","IMESSAGE_APP_IPHONE_58","IMESSAGE_APP_IPHONE_55","IMESSAGE_APP_IPHONE_47","IMESSAGE_APP_IPHONE_40","IMESSAGE_APP_IPAD_PRO_3GEN_129","IMESSAGE_APP_IPAD_PRO_3GEN_11","IMESSAGE_APP_IPAD_PRO_129","IMESSAGE_APP_IPAD_105","IMESSAGE_APP_IPAD_97"],"type":"string"}},"required":["screenshotDisplayType"],"type":"object"},"relationships":{"properties":{"appCustomProductPageLocalization":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["appCustomProductPageLocalizations"],"type":"string"}},"required":["id","type"],"type":"object"}},"type":"object"},"appStoreVersionExperimentTreatmentLocalization":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["appStoreVersionExperimentTreatmentLocalizations"],"type":"string"}},"required":["id","type"],"type":"object"}},"type":"object"},"appStoreVersionLocalization":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["appStoreVersionLocalizations"],"type":"string"}},"required":["id","type"],"type":"object"}},"type":"object"}},"type":"object"},"type":{"enum":["appScreenshotSets"],"type":"string"}},"required":["attributes","type"],"type":"object"}},"required":["data"],"title":"AppScreenshotSetCreateRequest","type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "POST".to_string(),
                path_template: "/v1/appScreenshotSets".to_string(),
                params: vec![
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appScreenshotSets_getInstance".to_string(),
            description: "Get one `appScreenshotSets` resource by id.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[appCustomProductPageLocalizations]":{"items":{"type":"string"},"type":"array"},"fields[appScreenshotSets]":{"items":{"type":"string"},"type":"array"},"fields[appScreenshots]":{"items":{"type":"string"},"type":"array"},"fields[appStoreVersionExperimentTreatmentLocalizations]":{"items":{"type":"string"},"type":"array"},"fields[appStoreVersionLocalizations]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["appStoreVersionLocalization","appCustomProductPageLocalization","appStoreVersionExperimentTreatmentLocalization","appScreenshots"],"type":"string"},"type":"array"},"limit[appScreenshots]":{"maximum":50,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/appScreenshotSets/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[appScreenshotSets]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appStoreVersionLocalizations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appCustomProductPageLocalizations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appStoreVersionExperimentTreatmentLocalizations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appScreenshots]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[appScreenshots]".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appScreenshotSets_deleteInstance".to_string(),
            description: "Delete one `appScreenshotSets` resource. This cannot be undone.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"id":{"type":"string"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "DELETE".to_string(),
                path_template: "/v1/appScreenshotSets/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(true),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appScreenshots_createInstance".to_string(),
            description: "Low-level step 1 of an upload (reserve). Prefer `asset_upload`, which performs reserve, upload, commit and processing checks.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"fileName":{"type":"string"},"fileSize":{"type":"integer"}},"required":["fileName","fileSize"],"type":"object"},"relationships":{"properties":{"appScreenshotSet":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["appScreenshotSets"],"type":"string"}},"required":["id","type"],"type":"object"}},"required":["data"],"type":"object"}},"required":["appScreenshotSet"],"type":"object"},"type":{"enum":["appScreenshots"],"type":"string"}},"required":["relationships","attributes","type"],"type":"object"}},"required":["data"],"title":"AppScreenshotCreateRequest","type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "POST".to_string(),
                path_template: "/v1/appScreenshots".to_string(),
                params: vec![
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appScreenshots_getInstance".to_string(),
            description: "Get one `appScreenshots` resource by id.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[appScreenshotSets]":{"items":{"type":"string"},"type":"array"},"fields[appScreenshots]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["appScreenshotSet"],"type":"string"},"type":"array"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/appScreenshots/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[appScreenshots]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appScreenshotSets]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appScreenshots_updateInstance".to_string(),
            description: "Low-level step 3 of an upload (commit with `uploaded: true` and `sourceFileChecksum`). Prefer `asset_upload`.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"sourceFileChecksum":{"nullable":true,"type":"string"},"uploaded":{"nullable":true,"type":"boolean"}},"type":"object"},"id":{"type":"string"},"type":{"enum":["appScreenshots"],"type":"string"}},"required":["id","type"],"type":"object"},"id":{"type":"string"}},"required":["id","data"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "PATCH".to_string(),
                path_template: "/v1/appScreenshots/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appScreenshots_deleteInstance".to_string(),
            description: "Delete a screenshot from its set. This cannot be undone.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"id":{"type":"string"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "DELETE".to_string(),
                path_template: "/v1/appScreenshots/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(true),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appStoreReviewAttachments_createInstance".to_string(),
            description: "Create one `appStoreReviewAttachments` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"fileName":{"type":"string"},"fileSize":{"type":"integer"}},"required":["fileName","fileSize"],"type":"object"},"relationships":{"properties":{"appStoreReviewDetail":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["appStoreReviewDetails"],"type":"string"}},"required":["id","type"],"type":"object"}},"required":["data"],"type":"object"}},"required":["appStoreReviewDetail"],"type":"object"},"type":{"enum":["appStoreReviewAttachments"],"type":"string"}},"required":["relationships","attributes","type"],"type":"object"}},"required":["data"],"title":"AppStoreReviewAttachmentCreateRequest","type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "POST".to_string(),
                path_template: "/v1/appStoreReviewAttachments".to_string(),
                params: vec![
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appStoreReviewAttachments_getInstance".to_string(),
            description: "Get one `appStoreReviewAttachments` resource by id.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[appStoreReviewAttachments]":{"items":{"type":"string"},"type":"array"},"fields[appStoreReviewDetails]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["appStoreReviewDetail"],"type":"string"},"type":"array"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/appStoreReviewAttachments/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[appStoreReviewAttachments]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appStoreReviewDetails]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appStoreReviewAttachments_updateInstance".to_string(),
            description: "Update attributes or relationships of one `appStoreReviewAttachments` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"sourceFileChecksum":{"nullable":true,"type":"string"},"uploaded":{"nullable":true,"type":"boolean"}},"type":"object"},"id":{"type":"string"},"type":{"enum":["appStoreReviewAttachments"],"type":"string"}},"required":["id","type"],"type":"object"},"id":{"type":"string"}},"required":["id","data"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "PATCH".to_string(),
                path_template: "/v1/appStoreReviewAttachments/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appStoreReviewAttachments_deleteInstance".to_string(),
            description: "Delete one `appStoreReviewAttachments` resource. This cannot be undone.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"id":{"type":"string"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "DELETE".to_string(),
                path_template: "/v1/appStoreReviewAttachments/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(true),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appStoreReviewDetails_createInstance".to_string(),
            description: "Create App Review details (contact, demo account, notes) for a version that has none; check `appStoreVersions_appStoreReviewDetail_getToOneRelated` first.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"contactEmail":{"nullable":true,"type":"string"},"contactFirstName":{"nullable":true,"type":"string"},"contactLastName":{"nullable":true,"type":"string"},"contactPhone":{"nullable":true,"type":"string"},"demoAccountName":{"nullable":true,"type":"string"},"demoAccountPassword":{"nullable":true,"type":"string"},"demoAccountRequired":{"nullable":true,"type":"boolean"},"notes":{"nullable":true,"type":"string"}},"type":"object"},"relationships":{"properties":{"appStoreVersion":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["appStoreVersions"],"type":"string"}},"required":["id","type"],"type":"object"}},"required":["data"],"type":"object"}},"required":["appStoreVersion"],"type":"object"},"type":{"enum":["appStoreReviewDetails"],"type":"string"}},"required":["relationships","type"],"type":"object"}},"required":["data"],"title":"AppStoreReviewDetailCreateRequest","type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "POST".to_string(),
                path_template: "/v1/appStoreReviewDetails".to_string(),
                params: vec![
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appStoreReviewDetails_getInstance".to_string(),
            description: "Get one `appStoreReviewDetails` resource by id.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[appStoreReviewAttachments]":{"items":{"type":"string"},"type":"array"},"fields[appStoreReviewDetails]":{"items":{"type":"string"},"type":"array"},"fields[appStoreVersions]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["appStoreVersion","appStoreReviewAttachments"],"type":"string"},"type":"array"},"limit[appStoreReviewAttachments]":{"maximum":50,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/appStoreReviewDetails/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[appStoreReviewDetails]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appStoreVersions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appStoreReviewAttachments]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[appStoreReviewAttachments]".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appStoreReviewDetails_updateInstance".to_string(),
            description: "Update attributes or relationships of one `appStoreReviewDetails` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"contactEmail":{"nullable":true,"type":"string"},"contactFirstName":{"nullable":true,"type":"string"},"contactLastName":{"nullable":true,"type":"string"},"contactPhone":{"nullable":true,"type":"string"},"demoAccountName":{"nullable":true,"type":"string"},"demoAccountPassword":{"nullable":true,"type":"string"},"demoAccountRequired":{"nullable":true,"type":"boolean"},"notes":{"nullable":true,"type":"string"}},"type":"object"},"id":{"type":"string"},"type":{"enum":["appStoreReviewDetails"],"type":"string"}},"required":["id","type"],"type":"object"},"id":{"type":"string"}},"required":["id","data"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "PATCH".to_string(),
                path_template: "/v1/appStoreReviewDetails/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appStoreVersionLocalizations_createInstance".to_string(),
            description: "Create one `appStoreVersionLocalizations` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"description":{"nullable":true,"type":"string"},"keywords":{"nullable":true,"type":"string"},"locale":{"type":"string"},"marketingUrl":{"format":"uri","nullable":true,"type":"string"},"promotionalText":{"nullable":true,"type":"string"},"supportUrl":{"format":"uri","nullable":true,"type":"string"},"whatsNew":{"nullable":true,"type":"string"}},"required":["locale"],"type":"object"},"relationships":{"properties":{"appStoreVersion":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["appStoreVersions"],"type":"string"}},"required":["id","type"],"type":"object"}},"required":["data"],"type":"object"}},"required":["appStoreVersion"],"type":"object"},"type":{"enum":["appStoreVersionLocalizations"],"type":"string"}},"required":["relationships","attributes","type"],"type":"object"}},"required":["data"],"title":"AppStoreVersionLocalizationCreateRequest","type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "POST".to_string(),
                path_template: "/v1/appStoreVersionLocalizations".to_string(),
                params: vec![
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appStoreVersionLocalizations_getInstance".to_string(),
            description: "Get one `appStoreVersionLocalizations` resource by id.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[appPreviewSets]":{"items":{"type":"string"},"type":"array"},"fields[appScreenshotSets]":{"items":{"type":"string"},"type":"array"},"fields[appStoreVersionLocalizations]":{"items":{"type":"string"},"type":"array"},"fields[appStoreVersions]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["appStoreVersion","appScreenshotSets","appPreviewSets","searchKeywords"],"type":"string"},"type":"array"},"limit[appPreviewSets]":{"maximum":50,"type":"integer"},"limit[appScreenshotSets]":{"maximum":50,"type":"integer"},"limit[searchKeywords]":{"maximum":50,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/appStoreVersionLocalizations/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[appStoreVersionLocalizations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appStoreVersions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appScreenshotSets]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appPreviewSets]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[appPreviewSets]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[appScreenshotSets]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[searchKeywords]".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appStoreVersionLocalizations_updateInstance".to_string(),
            description: "Update store text for one locale of a version: `description`, `keywords`, `whatsNew`, `promotionalText`, `marketingUrl`, `supportUrl`. Read it first with `appStoreVersionLocalizations_getInstance`.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"description":{"nullable":true,"type":"string"},"keywords":{"nullable":true,"type":"string"},"marketingUrl":{"format":"uri","nullable":true,"type":"string"},"promotionalText":{"nullable":true,"type":"string"},"supportUrl":{"format":"uri","nullable":true,"type":"string"},"whatsNew":{"nullable":true,"type":"string"}},"type":"object"},"id":{"type":"string"},"type":{"enum":["appStoreVersionLocalizations"],"type":"string"}},"required":["id","type"],"type":"object"},"id":{"type":"string"}},"required":["id","data"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "PATCH".to_string(),
                path_template: "/v1/appStoreVersionLocalizations/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appStoreVersionLocalizations_deleteInstance".to_string(),
            description: "Delete one `appStoreVersionLocalizations` resource. This cannot be undone.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"id":{"type":"string"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "DELETE".to_string(),
                path_template: "/v1/appStoreVersionLocalizations/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(true),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appStoreVersionPhasedReleases_createInstance".to_string(),
            description: "Enable a 7-day phased release for an App Store version.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"phasedReleaseState":{"enum":["INACTIVE","ACTIVE","PAUSED","COMPLETE"],"type":"string"}},"type":"object"},"relationships":{"properties":{"appStoreVersion":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["appStoreVersions"],"type":"string"}},"required":["id","type"],"type":"object"}},"required":["data"],"type":"object"}},"required":["appStoreVersion"],"type":"object"},"type":{"enum":["appStoreVersionPhasedReleases"],"type":"string"}},"required":["relationships","type"],"type":"object"}},"required":["data"],"title":"AppStoreVersionPhasedReleaseCreateRequest","type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "POST".to_string(),
                path_template: "/v1/appStoreVersionPhasedReleases".to_string(),
                params: vec![
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appStoreVersionPhasedReleases_updateInstance".to_string(),
            description: "Control a phased release: `phasedReleaseState` ACTIVE (resume), PAUSED, or COMPLETE (release to everyone now).".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"phasedReleaseState":{"enum":["INACTIVE","ACTIVE","PAUSED","COMPLETE"],"type":"string"}},"type":"object"},"id":{"type":"string"},"type":{"enum":["appStoreVersionPhasedReleases"],"type":"string"}},"required":["id","type"],"type":"object"},"id":{"type":"string"}},"required":["id","data"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "PATCH".to_string(),
                path_template: "/v1/appStoreVersionPhasedReleases/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appStoreVersionPhasedReleases_deleteInstance".to_string(),
            description: "Remove a phased release from a version that has not started releasing, so it releases to everyone at once.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"id":{"type":"string"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "DELETE".to_string(),
                path_template: "/v1/appStoreVersionPhasedReleases/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(true),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appStoreVersionReleaseRequests_createInstance".to_string(),
            description: "Release an approved App Store version that uses manual release (state PENDING_DEVELOPER_RELEASE). This publishes the app.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"relationships":{"properties":{"appStoreVersion":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["appStoreVersions"],"type":"string"}},"required":["id","type"],"type":"object"}},"required":["data"],"type":"object"}},"required":["appStoreVersion"],"type":"object"},"type":{"enum":["appStoreVersionReleaseRequests"],"type":"string"}},"required":["relationships","type"],"type":"object"}},"required":["data"],"title":"AppStoreVersionReleaseRequestCreateRequest","type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "POST".to_string(),
                path_template: "/v1/appStoreVersionReleaseRequests".to_string(),
                params: vec![
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appStoreVersions_createInstance".to_string(),
            description: "Create a new App Store version (`versionString`, `platform`) for an app. Localizations are copied from the previous version.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"copyright":{"nullable":true,"type":"string"},"earliestReleaseDate":{"format":"date-time","nullable":true,"type":"string"},"platform":{"enum":["IOS","MAC_OS","TV_OS","VISION_OS"],"type":"string"},"releaseType":{"enum":["MANUAL","AFTER_APPROVAL","SCHEDULED"],"nullable":true,"type":"string"},"reviewType":{"enum":["APP_STORE","NOTARIZATION"],"nullable":true,"type":"string"},"usesIdfa":{"deprecated":true,"nullable":true,"type":"boolean"},"versionString":{"type":"string"}},"required":["versionString","platform"],"type":"object"},"relationships":{"properties":{"app":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["apps"],"type":"string"}},"required":["id","type"],"type":"object"}},"required":["data"],"type":"object"},"appStoreVersionLocalizations":{"properties":{"data":{"items":{"properties":{"id":{"type":"string"},"type":{"enum":["appStoreVersionLocalizations"],"type":"string"}},"required":["id","type"],"type":"object"},"type":"array"}},"type":"object"},"build":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["builds"],"type":"string"}},"required":["id","type"],"type":"object"}},"type":"object"}},"required":["app"],"type":"object"},"type":{"enum":["appStoreVersions"],"type":"string"}},"required":["relationships","attributes","type"],"type":"object"}},"required":["data"],"title":"AppStoreVersionCreateRequest","type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "POST".to_string(),
                path_template: "/v1/appStoreVersions".to_string(),
                params: vec![
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appStoreVersions_getInstance".to_string(),
            description: "Get one `appStoreVersions` resource by id.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[alternativeDistributionPackages]":{"items":{"type":"string"},"type":"array"},"fields[appClipDefaultExperiences]":{"items":{"type":"string"},"type":"array"},"fields[appStoreReviewDetails]":{"items":{"type":"string"},"type":"array"},"fields[appStoreVersionExperiments]":{"items":{"type":"string"},"type":"array"},"fields[appStoreVersionLocalizations]":{"items":{"type":"string"},"type":"array"},"fields[appStoreVersionPhasedReleases]":{"items":{"type":"string"},"type":"array"},"fields[appStoreVersionSubmissions]":{"items":{"type":"string"},"type":"array"},"fields[appStoreVersions]":{"items":{"type":"string"},"type":"array"},"fields[apps]":{"items":{"type":"string"},"type":"array"},"fields[builds]":{"items":{"type":"string"},"type":"array"},"fields[gameCenterAppVersions]":{"items":{"type":"string"},"type":"array"},"fields[routingAppCoverages]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["app","appStoreVersionLocalizations","build","appStoreVersionPhasedRelease","gameCenterAppVersion","routingAppCoverage","appStoreReviewDetail","appStoreVersionSubmission","appClipDefaultExperience","appStoreVersionExperiments","appStoreVersionExperimentsV2","alternativeDistributionPackage"],"type":"string"},"type":"array"},"limit[appStoreVersionExperimentsV2]":{"maximum":50,"type":"integer"},"limit[appStoreVersionExperiments]":{"maximum":50,"type":"integer"},"limit[appStoreVersionLocalizations]":{"maximum":50,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/appStoreVersions/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[appStoreVersions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[apps]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appStoreVersionLocalizations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[builds]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appStoreVersionPhasedReleases]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[gameCenterAppVersions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[routingAppCoverages]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appStoreReviewDetails]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appStoreVersionSubmissions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appClipDefaultExperiences]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appStoreVersionExperiments]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[alternativeDistributionPackages]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[appStoreVersionExperiments]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[appStoreVersionExperimentsV2]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[appStoreVersionLocalizations]".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appStoreVersions_updateInstance".to_string(),
            description: "Update an App Store version: `versionString`, `releaseType` (MANUAL, AFTER_APPROVAL, SCHEDULED), `earliestReleaseDate`, `copyright`.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"copyright":{"nullable":true,"type":"string"},"downloadable":{"nullable":true,"type":"boolean"},"earliestReleaseDate":{"format":"date-time","nullable":true,"type":"string"},"releaseType":{"enum":["MANUAL","AFTER_APPROVAL","SCHEDULED"],"nullable":true,"type":"string"},"reviewType":{"enum":["APP_STORE","NOTARIZATION"],"nullable":true,"type":"string"},"usesIdfa":{"deprecated":true,"nullable":true,"type":"boolean"},"versionString":{"nullable":true,"type":"string"}},"type":"object"},"id":{"type":"string"},"relationships":{"properties":{"appClipDefaultExperience":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["appClipDefaultExperiences"],"type":"string"}},"required":["id","type"],"type":"object"}},"type":"object"},"build":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["builds"],"type":"string"}},"required":["id","type"],"type":"object"}},"type":"object"}},"type":"object"},"type":{"enum":["appStoreVersions"],"type":"string"}},"required":["id","type"],"type":"object"},"id":{"type":"string"}},"required":["id","data"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "PATCH".to_string(),
                path_template: "/v1/appStoreVersions/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appStoreVersions_deleteInstance".to_string(),
            description: "Delete one `appStoreVersions` resource. This cannot be undone.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"id":{"type":"string"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "DELETE".to_string(),
                path_template: "/v1/appStoreVersions/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(true),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "apps_getCollection".to_string(),
            description: "Find apps. Filter by `filter[bundleId]` or `filter[name]` to resolve an app ID; never guess IDs.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"exists[gameCenterEnabledVersions]":{"type":"boolean"},"fields[androidToIosAppMappingDetails]":{"items":{"type":"string"},"type":"array"},"fields[appClips]":{"items":{"type":"string"},"type":"array"},"fields[appCustomProductPages]":{"items":{"type":"string"},"type":"array"},"fields[appEncryptionDeclarations]":{"items":{"type":"string"},"type":"array"},"fields[appEvents]":{"items":{"type":"string"},"type":"array"},"fields[appInfos]":{"items":{"type":"string"},"type":"array"},"fields[appStoreVersionExperiments]":{"items":{"type":"string"},"type":"array"},"fields[appStoreVersions]":{"items":{"type":"string"},"type":"array"},"fields[apps]":{"items":{"type":"string"},"type":"array"},"fields[betaAppLocalizations]":{"items":{"type":"string"},"type":"array"},"fields[betaAppReviewDetails]":{"items":{"type":"string"},"type":"array"},"fields[betaGroups]":{"items":{"type":"string"},"type":"array"},"fields[betaLicenseAgreements]":{"items":{"type":"string"},"type":"array"},"fields[buildIcons]":{"items":{"type":"string"},"type":"array"},"fields[builds]":{"items":{"type":"string"},"type":"array"},"fields[ciProducts]":{"items":{"type":"string"},"type":"array"},"fields[endUserLicenseAgreements]":{"items":{"type":"string"},"type":"array"},"fields[gameCenterDetails]":{"items":{"type":"string"},"type":"array"},"fields[gameCenterEnabledVersions]":{"items":{"type":"string"},"type":"array"},"fields[inAppPurchases]":{"items":{"type":"string"},"type":"array"},"fields[preReleaseVersions]":{"items":{"type":"string"},"type":"array"},"fields[promotedPurchases]":{"items":{"type":"string"},"type":"array"},"fields[reviewSubmissions]":{"items":{"type":"string"},"type":"array"},"fields[subscriptionGracePeriods]":{"items":{"type":"string"},"type":"array"},"fields[subscriptionGroups]":{"items":{"type":"string"},"type":"array"},"filter[appStoreVersions.appStoreState]":{"items":{"enum":["ACCEPTED","DEVELOPER_REMOVED_FROM_SALE","DEVELOPER_REJECTED","IN_REVIEW","INVALID_BINARY","METADATA_REJECTED","PENDING_APPLE_RELEASE","PENDING_CONTRACT","PENDING_DEVELOPER_RELEASE","PREPARE_FOR_SUBMISSION","PREORDER_READY_FOR_SALE","PROCESSING_FOR_APP_STORE","READY_FOR_REVIEW","READY_FOR_SALE","REJECTED","REMOVED_FROM_SALE","WAITING_FOR_EXPORT_COMPLIANCE","WAITING_FOR_REVIEW","REPLACED_WITH_NEW_VERSION","NOT_APPLICABLE"],"type":"string"},"type":"array"},"filter[appStoreVersions.appVersionState]":{"items":{"enum":["ACCEPTED","DEVELOPER_REJECTED","IN_REVIEW","INVALID_BINARY","METADATA_REJECTED","PENDING_APPLE_RELEASE","PENDING_DEVELOPER_RELEASE","PREPARE_FOR_SUBMISSION","PROCESSING_FOR_DISTRIBUTION","READY_FOR_DISTRIBUTION","READY_FOR_REVIEW","REJECTED","REPLACED_WITH_NEW_VERSION","WAITING_FOR_EXPORT_COMPLIANCE","WAITING_FOR_REVIEW"],"type":"string"},"type":"array"},"filter[appStoreVersions.platform]":{"items":{"enum":["IOS","MAC_OS","TV_OS","VISION_OS"],"type":"string"},"type":"array"},"filter[appStoreVersions]":{"items":{"type":"string"},"type":"array"},"filter[bundleId]":{"items":{"type":"string"},"type":"array"},"filter[id]":{"items":{"type":"string"},"type":"array"},"filter[name]":{"items":{"type":"string"},"type":"array"},"filter[reviewSubmissions.platform]":{"items":{"enum":["IOS","MAC_OS","TV_OS","VISION_OS"],"type":"string"},"type":"array"},"filter[reviewSubmissions.state]":{"items":{"enum":["READY_FOR_REVIEW","WAITING_FOR_REVIEW","IN_REVIEW","UNRESOLVED_ISSUES","CANCELING","COMPLETING","COMPLETE"],"type":"string"},"type":"array"},"filter[sku]":{"items":{"type":"string"},"type":"array"},"include":{"items":{"enum":["appEncryptionDeclarations","appStoreIcon","ciProduct","betaGroups","appStoreVersions","preReleaseVersions","betaAppLocalizations","builds","betaLicenseAgreement","betaAppReviewDetail","appInfos","appClips","endUserLicenseAgreement","inAppPurchases","subscriptionGroups","gameCenterEnabledVersions","appCustomProductPages","inAppPurchasesV2","promotedPurchases","appEvents","reviewSubmissions","subscriptionGracePeriod","gameCenterDetail","appStoreVersionExperimentsV2","androidToIosAppMappingDetails"],"type":"string"},"type":"array"},"limit":{"maximum":200,"type":"integer"},"limit[androidToIosAppMappingDetails]":{"maximum":50,"type":"integer"},"limit[appClips]":{"maximum":50,"type":"integer"},"limit[appCustomProductPages]":{"maximum":50,"type":"integer"},"limit[appEncryptionDeclarations]":{"maximum":50,"type":"integer"},"limit[appEvents]":{"maximum":50,"type":"integer"},"limit[appInfos]":{"maximum":50,"type":"integer"},"limit[appStoreVersionExperimentsV2]":{"maximum":50,"type":"integer"},"limit[appStoreVersions]":{"maximum":50,"type":"integer"},"limit[betaAppLocalizations]":{"maximum":50,"type":"integer"},"limit[betaGroups]":{"maximum":50,"type":"integer"},"limit[builds]":{"maximum":50,"type":"integer"},"limit[gameCenterEnabledVersions]":{"maximum":50,"type":"integer"},"limit[inAppPurchasesV2]":{"maximum":50,"type":"integer"},"limit[inAppPurchases]":{"maximum":50,"type":"integer"},"limit[preReleaseVersions]":{"maximum":50,"type":"integer"},"limit[promotedPurchases]":{"maximum":50,"type":"integer"},"limit[reviewSubmissions]":{"maximum":50,"type":"integer"},"limit[subscriptionGroups]":{"maximum":50,"type":"integer"},"sort":{"items":{"enum":["name","-name","bundleId","-bundleId","sku","-sku"],"type":"string"},"type":"array"}},"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/apps".to_string(),
                params: vec![
                    ParamBinding {
                        name: "filter[name]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[bundleId]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[sku]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[appStoreVersions.appStoreState]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[appStoreVersions.platform]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[appStoreVersions.appVersionState]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[reviewSubmissions.state]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[reviewSubmissions.platform]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[appStoreVersions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[id]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "exists[gameCenterEnabledVersions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "sort".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[apps]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appEncryptionDeclarations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[buildIcons]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[ciProducts]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[betaGroups]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appStoreVersions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[preReleaseVersions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[betaAppLocalizations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[builds]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[betaLicenseAgreements]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[betaAppReviewDetails]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appInfos]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appClips]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[endUserLicenseAgreements]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[inAppPurchases]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[subscriptionGroups]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[gameCenterEnabledVersions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appCustomProductPages]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[promotedPurchases]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appEvents]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[reviewSubmissions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[subscriptionGracePeriods]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[gameCenterDetails]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appStoreVersionExperiments]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[androidToIosAppMappingDetails]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[androidToIosAppMappingDetails]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[appClips]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[appCustomProductPages]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[appEncryptionDeclarations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[appEvents]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[appInfos]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[appStoreVersionExperimentsV2]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[appStoreVersions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[betaAppLocalizations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[betaGroups]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[builds]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[gameCenterEnabledVersions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[inAppPurchases]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[inAppPurchasesV2]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[preReleaseVersions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[promotedPurchases]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[reviewSubmissions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[subscriptionGroups]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "apps_getInstance".to_string(),
            description: "Get one `apps` resource by id.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[androidToIosAppMappingDetails]":{"items":{"type":"string"},"type":"array"},"fields[appClips]":{"items":{"type":"string"},"type":"array"},"fields[appCustomProductPages]":{"items":{"type":"string"},"type":"array"},"fields[appEncryptionDeclarations]":{"items":{"type":"string"},"type":"array"},"fields[appEvents]":{"items":{"type":"string"},"type":"array"},"fields[appInfos]":{"items":{"type":"string"},"type":"array"},"fields[appStoreVersionExperiments]":{"items":{"type":"string"},"type":"array"},"fields[appStoreVersions]":{"items":{"type":"string"},"type":"array"},"fields[apps]":{"items":{"type":"string"},"type":"array"},"fields[betaAppLocalizations]":{"items":{"type":"string"},"type":"array"},"fields[betaAppReviewDetails]":{"items":{"type":"string"},"type":"array"},"fields[betaGroups]":{"items":{"type":"string"},"type":"array"},"fields[betaLicenseAgreements]":{"items":{"type":"string"},"type":"array"},"fields[buildIcons]":{"items":{"type":"string"},"type":"array"},"fields[builds]":{"items":{"type":"string"},"type":"array"},"fields[ciProducts]":{"items":{"type":"string"},"type":"array"},"fields[endUserLicenseAgreements]":{"items":{"type":"string"},"type":"array"},"fields[gameCenterDetails]":{"items":{"type":"string"},"type":"array"},"fields[gameCenterEnabledVersions]":{"items":{"type":"string"},"type":"array"},"fields[inAppPurchases]":{"items":{"type":"string"},"type":"array"},"fields[preReleaseVersions]":{"items":{"type":"string"},"type":"array"},"fields[promotedPurchases]":{"items":{"type":"string"},"type":"array"},"fields[reviewSubmissions]":{"items":{"type":"string"},"type":"array"},"fields[subscriptionGracePeriods]":{"items":{"type":"string"},"type":"array"},"fields[subscriptionGroups]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["appEncryptionDeclarations","appStoreIcon","ciProduct","betaGroups","appStoreVersions","preReleaseVersions","betaAppLocalizations","builds","betaLicenseAgreement","betaAppReviewDetail","appInfos","appClips","endUserLicenseAgreement","inAppPurchases","subscriptionGroups","gameCenterEnabledVersions","appCustomProductPages","inAppPurchasesV2","promotedPurchases","appEvents","reviewSubmissions","subscriptionGracePeriod","gameCenterDetail","appStoreVersionExperimentsV2","androidToIosAppMappingDetails"],"type":"string"},"type":"array"},"limit[androidToIosAppMappingDetails]":{"maximum":50,"type":"integer"},"limit[appClips]":{"maximum":50,"type":"integer"},"limit[appCustomProductPages]":{"maximum":50,"type":"integer"},"limit[appEncryptionDeclarations]":{"maximum":50,"type":"integer"},"limit[appEvents]":{"maximum":50,"type":"integer"},"limit[appInfos]":{"maximum":50,"type":"integer"},"limit[appStoreVersionExperimentsV2]":{"maximum":50,"type":"integer"},"limit[appStoreVersions]":{"maximum":50,"type":"integer"},"limit[betaAppLocalizations]":{"maximum":50,"type":"integer"},"limit[betaGroups]":{"maximum":50,"type":"integer"},"limit[builds]":{"maximum":50,"type":"integer"},"limit[gameCenterEnabledVersions]":{"maximum":50,"type":"integer"},"limit[inAppPurchasesV2]":{"maximum":50,"type":"integer"},"limit[inAppPurchases]":{"maximum":50,"type":"integer"},"limit[preReleaseVersions]":{"maximum":50,"type":"integer"},"limit[promotedPurchases]":{"maximum":50,"type":"integer"},"limit[reviewSubmissions]":{"maximum":50,"type":"integer"},"limit[subscriptionGroups]":{"maximum":50,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/apps/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[apps]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appEncryptionDeclarations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[buildIcons]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[ciProducts]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[betaGroups]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appStoreVersions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[preReleaseVersions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[betaAppLocalizations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[builds]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[betaLicenseAgreements]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[betaAppReviewDetails]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appInfos]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appClips]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[endUserLicenseAgreements]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[inAppPurchases]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[subscriptionGroups]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[gameCenterEnabledVersions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appCustomProductPages]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[promotedPurchases]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appEvents]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[reviewSubmissions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[subscriptionGracePeriods]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[gameCenterDetails]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appStoreVersionExperiments]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[androidToIosAppMappingDetails]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[androidToIosAppMappingDetails]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[appClips]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[appCustomProductPages]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[appEncryptionDeclarations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[appEvents]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[appInfos]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[appStoreVersionExperimentsV2]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[appStoreVersions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[betaAppLocalizations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[betaGroups]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[builds]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[gameCenterEnabledVersions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[inAppPurchases]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[inAppPurchasesV2]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[preReleaseVersions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[promotedPurchases]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[reviewSubmissions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[subscriptionGroups]".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "apps_updateInstance".to_string(),
            description: "Update attributes or relationships of one `apps` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"accessibilityUrl":{"format":"uri","nullable":true,"type":"string"},"bundleId":{"nullable":true,"type":"string"},"contentRightsDeclaration":{"enum":["DOES_NOT_USE_THIRD_PARTY_CONTENT","USES_THIRD_PARTY_CONTENT"],"nullable":true,"type":"string"},"primaryLocale":{"nullable":true,"type":"string"},"streamlinedPurchasingEnabled":{"nullable":true,"type":"boolean"},"subscriptionStatusUrl":{"format":"uri","nullable":true,"type":"string"},"subscriptionStatusUrlForSandbox":{"format":"uri","nullable":true,"type":"string"},"subscriptionStatusUrlVersion":{"enum":["V1","V2"],"type":"string"},"subscriptionStatusUrlVersionForSandbox":{"enum":["V1","V2"],"type":"string"}},"type":"object"},"id":{"type":"string"},"type":{"enum":["apps"],"type":"string"}},"required":["id","type"],"type":"object"},"id":{"type":"string"}},"required":["id","data"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "PATCH".to_string(),
                path_template: "/v1/apps/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "betaAppLocalizations_getCollection".to_string(),
            description: "List `betaAppLocalizations` resources; supports the filters and pagination parameters below.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[apps]":{"items":{"type":"string"},"type":"array"},"fields[betaAppLocalizations]":{"items":{"type":"string"},"type":"array"},"filter[app]":{"items":{"type":"string"},"type":"array"},"filter[locale]":{"items":{"type":"string"},"type":"array"},"include":{"items":{"enum":["app"],"type":"string"},"type":"array"},"limit":{"maximum":200,"type":"integer"}},"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/betaAppLocalizations".to_string(),
                params: vec![
                    ParamBinding {
                        name: "filter[locale]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[app]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[betaAppLocalizations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[apps]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "betaAppLocalizations_createInstance".to_string(),
            description: "Create one `betaAppLocalizations` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"description":{"nullable":true,"type":"string"},"feedbackEmail":{"nullable":true,"type":"string"},"locale":{"type":"string"},"marketingUrl":{"nullable":true,"type":"string"},"privacyPolicyUrl":{"nullable":true,"type":"string"},"tvOsPrivacyPolicy":{"nullable":true,"type":"string"}},"required":["locale"],"type":"object"},"relationships":{"properties":{"app":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["apps"],"type":"string"}},"required":["id","type"],"type":"object"}},"required":["data"],"type":"object"}},"required":["app"],"type":"object"},"type":{"enum":["betaAppLocalizations"],"type":"string"}},"required":["relationships","attributes","type"],"type":"object"}},"required":["data"],"title":"BetaAppLocalizationCreateRequest","type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "POST".to_string(),
                path_template: "/v1/betaAppLocalizations".to_string(),
                params: vec![
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "betaAppLocalizations_getInstance".to_string(),
            description: "Get one `betaAppLocalizations` resource by id.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[apps]":{"items":{"type":"string"},"type":"array"},"fields[betaAppLocalizations]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["app"],"type":"string"},"type":"array"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/betaAppLocalizations/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[betaAppLocalizations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[apps]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "betaAppLocalizations_updateInstance".to_string(),
            description: "Update attributes or relationships of one `betaAppLocalizations` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"description":{"nullable":true,"type":"string"},"feedbackEmail":{"nullable":true,"type":"string"},"marketingUrl":{"nullable":true,"type":"string"},"privacyPolicyUrl":{"nullable":true,"type":"string"},"tvOsPrivacyPolicy":{"nullable":true,"type":"string"}},"type":"object"},"id":{"type":"string"},"type":{"enum":["betaAppLocalizations"],"type":"string"}},"required":["id","type"],"type":"object"},"id":{"type":"string"}},"required":["id","data"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "PATCH".to_string(),
                path_template: "/v1/betaAppLocalizations/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "betaAppLocalizations_deleteInstance".to_string(),
            description: "Delete one `betaAppLocalizations` resource. This cannot be undone.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"id":{"type":"string"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "DELETE".to_string(),
                path_template: "/v1/betaAppLocalizations/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(true),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "betaAppReviewDetails_getCollection".to_string(),
            description: "List `betaAppReviewDetails` resources; supports the filters and pagination parameters below.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[apps]":{"items":{"type":"string"},"type":"array"},"fields[betaAppReviewDetails]":{"items":{"type":"string"},"type":"array"},"filter[app]":{"items":{"type":"string"},"type":"array"},"include":{"items":{"enum":["app"],"type":"string"},"type":"array"},"limit":{"maximum":200,"type":"integer"}},"required":["filter[app]"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/betaAppReviewDetails".to_string(),
                params: vec![
                    ParamBinding {
                        name: "filter[app]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[betaAppReviewDetails]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[apps]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "betaAppReviewDetails_getInstance".to_string(),
            description: "Get one `betaAppReviewDetails` resource by id.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[apps]":{"items":{"type":"string"},"type":"array"},"fields[betaAppReviewDetails]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["app"],"type":"string"},"type":"array"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/betaAppReviewDetails/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[betaAppReviewDetails]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[apps]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "betaAppReviewDetails_updateInstance".to_string(),
            description: "Update Beta App Review contact, demo account (`demoAccountRequired`, `demoAccountName`, `demoAccountPassword`) and notes. Apple creates this resource with the app; there is no create.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"contactEmail":{"nullable":true,"type":"string"},"contactFirstName":{"nullable":true,"type":"string"},"contactLastName":{"nullable":true,"type":"string"},"contactPhone":{"nullable":true,"type":"string"},"demoAccountName":{"nullable":true,"type":"string"},"demoAccountPassword":{"nullable":true,"type":"string"},"demoAccountRequired":{"nullable":true,"type":"boolean"},"notes":{"nullable":true,"type":"string"}},"type":"object"},"id":{"type":"string"},"type":{"enum":["betaAppReviewDetails"],"type":"string"}},"required":["id","type"],"type":"object"},"id":{"type":"string"}},"required":["id","data"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "PATCH".to_string(),
                path_template: "/v1/betaAppReviewDetails/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "betaAppReviewSubmissions_getCollection".to_string(),
            description: "List `betaAppReviewSubmissions` resources; supports the filters and pagination parameters below.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[betaAppReviewSubmissions]":{"items":{"type":"string"},"type":"array"},"fields[builds]":{"items":{"type":"string"},"type":"array"},"filter[betaReviewState]":{"items":{"enum":["WAITING_FOR_REVIEW","IN_REVIEW","REJECTED","APPROVED"],"type":"string"},"type":"array"},"filter[build]":{"items":{"type":"string"},"type":"array"},"include":{"items":{"enum":["build"],"type":"string"},"type":"array"},"limit":{"maximum":200,"type":"integer"}},"required":["filter[build]"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/betaAppReviewSubmissions".to_string(),
                params: vec![
                    ParamBinding {
                        name: "filter[betaReviewState]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[build]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[betaAppReviewSubmissions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[builds]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "betaAppReviewSubmissions_createInstance".to_string(),
            description: "Submit a build to Beta App Review so external TestFlight groups can test it.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"relationships":{"properties":{"build":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["builds"],"type":"string"}},"required":["id","type"],"type":"object"}},"required":["data"],"type":"object"}},"required":["build"],"type":"object"},"type":{"enum":["betaAppReviewSubmissions"],"type":"string"}},"required":["relationships","type"],"type":"object"}},"required":["data"],"title":"BetaAppReviewSubmissionCreateRequest","type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "POST".to_string(),
                path_template: "/v1/betaAppReviewSubmissions".to_string(),
                params: vec![
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "betaAppReviewSubmissions_getInstance".to_string(),
            description: "Get one `betaAppReviewSubmissions` resource by id.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[betaAppReviewSubmissions]":{"items":{"type":"string"},"type":"array"},"fields[builds]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["build"],"type":"string"},"type":"array"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/betaAppReviewSubmissions/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[betaAppReviewSubmissions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[builds]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "betaBuildLocalizations_getCollection".to_string(),
            description: "List `betaBuildLocalizations` resources; supports the filters and pagination parameters below.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[betaBuildLocalizations]":{"items":{"type":"string"},"type":"array"},"fields[builds]":{"items":{"type":"string"},"type":"array"},"filter[build]":{"items":{"type":"string"},"type":"array"},"filter[locale]":{"items":{"type":"string"},"type":"array"},"include":{"items":{"enum":["build"],"type":"string"},"type":"array"},"limit":{"maximum":200,"type":"integer"}},"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/betaBuildLocalizations".to_string(),
                params: vec![
                    ParamBinding {
                        name: "filter[locale]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[build]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[betaBuildLocalizations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[builds]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "betaBuildLocalizations_createInstance".to_string(),
            description: "Create one `betaBuildLocalizations` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"locale":{"type":"string"},"whatsNew":{"nullable":true,"type":"string"}},"required":["locale"],"type":"object"},"relationships":{"properties":{"build":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["builds"],"type":"string"}},"required":["id","type"],"type":"object"}},"required":["data"],"type":"object"}},"required":["build"],"type":"object"},"type":{"enum":["betaBuildLocalizations"],"type":"string"}},"required":["relationships","attributes","type"],"type":"object"}},"required":["data"],"title":"BetaBuildLocalizationCreateRequest","type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "POST".to_string(),
                path_template: "/v1/betaBuildLocalizations".to_string(),
                params: vec![
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "betaBuildLocalizations_getInstance".to_string(),
            description: "Get one `betaBuildLocalizations` resource by id.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[betaBuildLocalizations]":{"items":{"type":"string"},"type":"array"},"fields[builds]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["build"],"type":"string"},"type":"array"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/betaBuildLocalizations/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[betaBuildLocalizations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[builds]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "betaBuildLocalizations_updateInstance".to_string(),
            description: "Update TestFlight 'What to Test' (`whatsNew`) for one locale of a build.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"whatsNew":{"nullable":true,"type":"string"}},"type":"object"},"id":{"type":"string"},"type":{"enum":["betaBuildLocalizations"],"type":"string"}},"required":["id","type"],"type":"object"},"id":{"type":"string"}},"required":["id","data"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "PATCH".to_string(),
                path_template: "/v1/betaBuildLocalizations/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "betaBuildLocalizations_deleteInstance".to_string(),
            description: "Delete one `betaBuildLocalizations` resource. This cannot be undone.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"id":{"type":"string"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "DELETE".to_string(),
                path_template: "/v1/betaBuildLocalizations/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(true),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "betaGroups_getCollection".to_string(),
            description: "List `betaGroups` resources; supports the filters and pagination parameters below.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[apps]":{"items":{"type":"string"},"type":"array"},"fields[betaGroups]":{"items":{"type":"string"},"type":"array"},"fields[betaRecruitmentCriteria]":{"items":{"type":"string"},"type":"array"},"fields[betaTesters]":{"items":{"type":"string"},"type":"array"},"fields[builds]":{"items":{"type":"string"},"type":"array"},"filter[app]":{"items":{"type":"string"},"type":"array"},"filter[builds]":{"items":{"type":"string"},"type":"array"},"filter[id]":{"items":{"type":"string"},"type":"array"},"filter[isInternalGroup]":{"items":{"type":"string"},"type":"array"},"filter[name]":{"items":{"type":"string"},"type":"array"},"filter[publicLinkEnabled]":{"items":{"type":"string"},"type":"array"},"filter[publicLinkLimitEnabled]":{"items":{"type":"string"},"type":"array"},"filter[publicLink]":{"items":{"type":"string"},"type":"array"},"include":{"items":{"enum":["app","builds","betaTesters","betaRecruitmentCriteria"],"type":"string"},"type":"array"},"limit":{"maximum":200,"type":"integer"},"limit[betaTesters]":{"maximum":50,"type":"integer"},"limit[builds]":{"maximum":1000,"type":"integer"},"sort":{"items":{"enum":["name","-name","createdDate","-createdDate","publicLinkEnabled","-publicLinkEnabled","publicLinkLimit","-publicLinkLimit"],"type":"string"},"type":"array"}},"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/betaGroups".to_string(),
                params: vec![
                    ParamBinding {
                        name: "filter[name]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[isInternalGroup]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[publicLinkEnabled]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[publicLinkLimitEnabled]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[publicLink]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[app]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[builds]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[id]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "sort".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[betaGroups]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[apps]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[builds]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[betaTesters]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[betaRecruitmentCriteria]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[betaTesters]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[builds]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "betaGroups_createInstance".to_string(),
            description: "Create a TestFlight group for an app. `isInternalGroup` true for App Store Connect users; external groups need Beta App Review before testers can install.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"feedbackEnabled":{"nullable":true,"type":"boolean"},"hasAccessToAllBuilds":{"nullable":true,"type":"boolean"},"isInternalGroup":{"nullable":true,"type":"boolean"},"name":{"type":"string"},"publicLinkEnabled":{"nullable":true,"type":"boolean"},"publicLinkLimit":{"nullable":true,"type":"integer"},"publicLinkLimitEnabled":{"nullable":true,"type":"boolean"}},"required":["name"],"type":"object"},"relationships":{"properties":{"app":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["apps"],"type":"string"}},"required":["id","type"],"type":"object"}},"required":["data"],"type":"object"},"betaTesters":{"properties":{"data":{"items":{"properties":{"id":{"type":"string"},"type":{"enum":["betaTesters"],"type":"string"}},"required":["id","type"],"type":"object"},"type":"array"}},"type":"object"},"builds":{"properties":{"data":{"items":{"properties":{"id":{"type":"string"},"type":{"enum":["builds"],"type":"string"}},"required":["id","type"],"type":"object"},"type":"array"}},"type":"object"}},"required":["app"],"type":"object"},"type":{"enum":["betaGroups"],"type":"string"}},"required":["relationships","attributes","type"],"type":"object"}},"required":["data"],"title":"BetaGroupCreateRequest","type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "POST".to_string(),
                path_template: "/v1/betaGroups".to_string(),
                params: vec![
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "betaGroups_getInstance".to_string(),
            description: "Get one `betaGroups` resource by id.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[apps]":{"items":{"type":"string"},"type":"array"},"fields[betaGroups]":{"items":{"type":"string"},"type":"array"},"fields[betaRecruitmentCriteria]":{"items":{"type":"string"},"type":"array"},"fields[betaTesters]":{"items":{"type":"string"},"type":"array"},"fields[builds]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["app","builds","betaTesters","betaRecruitmentCriteria"],"type":"string"},"type":"array"},"limit[betaTesters]":{"maximum":50,"type":"integer"},"limit[builds]":{"maximum":1000,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/betaGroups/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[betaGroups]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[apps]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[builds]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[betaTesters]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[betaRecruitmentCriteria]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[betaTesters]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[builds]".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "betaGroups_updateInstance".to_string(),
            description: "Update attributes or relationships of one `betaGroups` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"feedbackEnabled":{"nullable":true,"type":"boolean"},"iosBuildsAvailableForAppleSiliconMac":{"nullable":true,"type":"boolean"},"iosBuildsAvailableForAppleVision":{"nullable":true,"type":"boolean"},"name":{"nullable":true,"type":"string"},"publicLinkEnabled":{"nullable":true,"type":"boolean"},"publicLinkLimit":{"nullable":true,"type":"integer"},"publicLinkLimitEnabled":{"nullable":true,"type":"boolean"}},"type":"object"},"id":{"type":"string"},"type":{"enum":["betaGroups"],"type":"string"}},"required":["id","type"],"type":"object"},"id":{"type":"string"}},"required":["id","data"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "PATCH".to_string(),
                path_template: "/v1/betaGroups/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "betaGroups_deleteInstance".to_string(),
            description: "Delete one `betaGroups` resource. This cannot be undone.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"id":{"type":"string"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "DELETE".to_string(),
                path_template: "/v1/betaGroups/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(true),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "betaLicenseAgreements_getCollection".to_string(),
            description: "List `betaLicenseAgreements` resources; supports the filters and pagination parameters below.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[apps]":{"items":{"type":"string"},"type":"array"},"fields[betaLicenseAgreements]":{"items":{"type":"string"},"type":"array"},"filter[app]":{"items":{"type":"string"},"type":"array"},"include":{"items":{"enum":["app"],"type":"string"},"type":"array"},"limit":{"maximum":200,"type":"integer"}},"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/betaLicenseAgreements".to_string(),
                params: vec![
                    ParamBinding {
                        name: "filter[app]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[betaLicenseAgreements]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[apps]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "betaLicenseAgreements_getInstance".to_string(),
            description: "Get one `betaLicenseAgreements` resource by id.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[apps]":{"items":{"type":"string"},"type":"array"},"fields[betaLicenseAgreements]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["app"],"type":"string"},"type":"array"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/betaLicenseAgreements/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[betaLicenseAgreements]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[apps]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "betaLicenseAgreements_updateInstance".to_string(),
            description: "Update attributes or relationships of one `betaLicenseAgreements` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"agreementText":{"nullable":true,"type":"string"}},"type":"object"},"id":{"type":"string"},"type":{"enum":["betaLicenseAgreements"],"type":"string"}},"required":["id","type"],"type":"object"},"id":{"type":"string"}},"required":["id","data"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "PATCH".to_string(),
                path_template: "/v1/betaLicenseAgreements/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "betaTesters_getCollection".to_string(),
            description: "Find TestFlight testers. Filter by `filter[email]`, `filter[apps]` or `filter[betaGroups]`.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[apps]":{"items":{"type":"string"},"type":"array"},"fields[betaGroups]":{"items":{"type":"string"},"type":"array"},"fields[betaTesters]":{"items":{"type":"string"},"type":"array"},"fields[builds]":{"items":{"type":"string"},"type":"array"},"filter[apps]":{"items":{"type":"string"},"type":"array"},"filter[betaGroups]":{"items":{"type":"string"},"type":"array"},"filter[builds]":{"items":{"type":"string"},"type":"array"},"filter[email]":{"items":{"type":"string"},"type":"array"},"filter[firstName]":{"items":{"type":"string"},"type":"array"},"filter[id]":{"items":{"type":"string"},"type":"array"},"filter[inviteType]":{"items":{"enum":["EMAIL","PUBLIC_LINK"],"type":"string"},"type":"array"},"filter[lastName]":{"items":{"type":"string"},"type":"array"},"include":{"items":{"enum":["apps","betaGroups","builds"],"type":"string"},"type":"array"},"limit":{"maximum":200,"type":"integer"},"limit[apps]":{"maximum":50,"type":"integer"},"limit[betaGroups]":{"maximum":50,"type":"integer"},"limit[builds]":{"maximum":50,"type":"integer"},"sort":{"items":{"enum":["firstName","-firstName","lastName","-lastName","email","-email","inviteType","-inviteType","state","-state"],"type":"string"},"type":"array"}},"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/betaTesters".to_string(),
                params: vec![
                    ParamBinding {
                        name: "filter[firstName]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[lastName]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[email]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[inviteType]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[apps]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[betaGroups]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[builds]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[id]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "sort".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[betaTesters]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[apps]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[betaGroups]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[builds]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[apps]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[betaGroups]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[builds]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "betaTesters_createInstance".to_string(),
            description: "Invite a tester by email into one or more beta groups (relationship `betaGroups`) or builds. Sends an invitation email.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"email":{"format":"email","type":"string"},"firstName":{"nullable":true,"type":"string"},"lastName":{"nullable":true,"type":"string"}},"required":["email"],"type":"object"},"relationships":{"properties":{"betaGroups":{"properties":{"data":{"items":{"properties":{"id":{"type":"string"},"type":{"enum":["betaGroups"],"type":"string"}},"required":["id","type"],"type":"object"},"type":"array"}},"type":"object"},"builds":{"properties":{"data":{"items":{"properties":{"id":{"type":"string"},"type":{"enum":["builds"],"type":"string"}},"required":["id","type"],"type":"object"},"type":"array"}},"type":"object"}},"type":"object"},"type":{"enum":["betaTesters"],"type":"string"}},"required":["attributes","type"],"type":"object"}},"required":["data"],"title":"BetaTesterCreateRequest","type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "POST".to_string(),
                path_template: "/v1/betaTesters".to_string(),
                params: vec![
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "betaTesters_getInstance".to_string(),
            description: "Get one `betaTesters` resource by id.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[apps]":{"items":{"type":"string"},"type":"array"},"fields[betaGroups]":{"items":{"type":"string"},"type":"array"},"fields[betaTesters]":{"items":{"type":"string"},"type":"array"},"fields[builds]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["apps","betaGroups","builds"],"type":"string"},"type":"array"},"limit[apps]":{"maximum":50,"type":"integer"},"limit[betaGroups]":{"maximum":50,"type":"integer"},"limit[builds]":{"maximum":50,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/betaTesters/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[betaTesters]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[apps]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[betaGroups]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[builds]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[apps]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[betaGroups]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[builds]".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "betaTesters_deleteInstance".to_string(),
            description: "Remove a tester from all apps and groups. This cannot be undone.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"id":{"type":"string"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "DELETE".to_string(),
                path_template: "/v1/betaTesters/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(true),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "buildBetaDetails_getCollection".to_string(),
            description: "List `buildBetaDetails` resources; supports the filters and pagination parameters below.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[buildBetaDetails]":{"items":{"type":"string"},"type":"array"},"fields[builds]":{"items":{"type":"string"},"type":"array"},"filter[build]":{"items":{"type":"string"},"type":"array"},"filter[id]":{"items":{"type":"string"},"type":"array"},"include":{"items":{"enum":["build"],"type":"string"},"type":"array"},"limit":{"maximum":200,"type":"integer"}},"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/buildBetaDetails".to_string(),
                params: vec![
                    ParamBinding {
                        name: "filter[build]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[id]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[buildBetaDetails]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[builds]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "buildBetaDetails_getInstance".to_string(),
            description: "Get one `buildBetaDetails` resource by id.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[buildBetaDetails]":{"items":{"type":"string"},"type":"array"},"fields[builds]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["build"],"type":"string"},"type":"array"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/buildBetaDetails/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[buildBetaDetails]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[builds]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "buildBetaDetails_updateInstance".to_string(),
            description: "Update attributes or relationships of one `buildBetaDetails` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"autoNotifyEnabled":{"nullable":true,"type":"boolean"}},"type":"object"},"id":{"type":"string"},"type":{"enum":["buildBetaDetails"],"type":"string"}},"required":["id","type"],"type":"object"},"id":{"type":"string"}},"required":["id","data"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "PATCH".to_string(),
                path_template: "/v1/buildBetaDetails/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "builds_getCollection".to_string(),
            description: "Find builds. Filter by `filter[app]`, `filter[version]` (build number), `filter[preReleaseVersion.version]` (marketing version) and `filter[processingState]=VALID` to find a build ready to attach.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"exists[usesNonExemptEncryption]":{"type":"boolean"},"fields[appEncryptionDeclarations]":{"items":{"type":"string"},"type":"array"},"fields[appStoreVersions]":{"items":{"type":"string"},"type":"array"},"fields[apps]":{"items":{"type":"string"},"type":"array"},"fields[betaAppReviewSubmissions]":{"items":{"type":"string"},"type":"array"},"fields[betaBuildLocalizations]":{"items":{"type":"string"},"type":"array"},"fields[betaGroups]":{"items":{"type":"string"},"type":"array"},"fields[betaTesters]":{"items":{"type":"string"},"type":"array"},"fields[buildBetaDetails]":{"items":{"type":"string"},"type":"array"},"fields[buildBundles]":{"items":{"type":"string"},"type":"array"},"fields[buildIcons]":{"items":{"type":"string"},"type":"array"},"fields[buildUploads]":{"items":{"type":"string"},"type":"array"},"fields[builds]":{"items":{"type":"string"},"type":"array"},"fields[preReleaseVersions]":{"items":{"type":"string"},"type":"array"},"filter[appStoreVersion]":{"items":{"type":"string"},"type":"array"},"filter[app]":{"items":{"type":"string"},"type":"array"},"filter[betaAppReviewSubmission.betaReviewState]":{"items":{"enum":["WAITING_FOR_REVIEW","IN_REVIEW","REJECTED","APPROVED"],"type":"string"},"type":"array"},"filter[betaGroups]":{"items":{"type":"string"},"type":"array"},"filter[buildAudienceType]":{"items":{"enum":["INTERNAL_ONLY","APP_STORE_ELIGIBLE"],"type":"string"},"type":"array"},"filter[expired]":{"items":{"type":"string"},"type":"array"},"filter[id]":{"items":{"type":"string"},"type":"array"},"filter[preReleaseVersion.platform]":{"items":{"enum":["IOS","MAC_OS","TV_OS","VISION_OS"],"type":"string"},"type":"array"},"filter[preReleaseVersion.version]":{"items":{"type":"string"},"type":"array"},"filter[preReleaseVersion]":{"items":{"type":"string"},"type":"array"},"filter[processingState]":{"items":{"enum":["PROCESSING","FAILED","INVALID","VALID"],"type":"string"},"type":"array"},"filter[usesNonExemptEncryption]":{"items":{"type":"string"},"type":"array"},"filter[version]":{"items":{"type":"string"},"type":"array"},"include":{"items":{"enum":["preReleaseVersion","individualTesters","betaGroups","betaBuildLocalizations","appEncryptionDeclaration","betaAppReviewSubmission","app","buildBetaDetail","appStoreVersion","icons","buildBundles","buildUpload"],"type":"string"},"type":"array"},"limit":{"maximum":200,"type":"integer"},"limit[betaBuildLocalizations]":{"maximum":50,"type":"integer"},"limit[betaGroups]":{"maximum":50,"type":"integer"},"limit[buildBundles]":{"maximum":50,"type":"integer"},"limit[icons]":{"maximum":50,"type":"integer"},"limit[individualTesters]":{"maximum":50,"type":"integer"},"sort":{"items":{"enum":["version","-version","uploadedDate","-uploadedDate","preReleaseVersion","-preReleaseVersion"],"type":"string"},"type":"array"}},"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/builds".to_string(),
                params: vec![
                    ParamBinding {
                        name: "filter[version]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[expired]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[processingState]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[betaAppReviewSubmission.betaReviewState]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[usesNonExemptEncryption]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[preReleaseVersion.version]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[preReleaseVersion.platform]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[buildAudienceType]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[preReleaseVersion]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[app]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[betaGroups]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[appStoreVersion]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[id]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "exists[usesNonExemptEncryption]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "sort".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[builds]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[preReleaseVersions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[betaTesters]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[betaGroups]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[betaBuildLocalizations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appEncryptionDeclarations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[betaAppReviewSubmissions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[apps]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[buildBetaDetails]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appStoreVersions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[buildIcons]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[buildBundles]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[buildUploads]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[betaBuildLocalizations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[betaGroups]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[buildBundles]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[icons]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[individualTesters]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "builds_getInstance".to_string(),
            description: "Get one `builds` resource by id.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[appEncryptionDeclarations]":{"items":{"type":"string"},"type":"array"},"fields[appStoreVersions]":{"items":{"type":"string"},"type":"array"},"fields[apps]":{"items":{"type":"string"},"type":"array"},"fields[betaAppReviewSubmissions]":{"items":{"type":"string"},"type":"array"},"fields[betaBuildLocalizations]":{"items":{"type":"string"},"type":"array"},"fields[betaGroups]":{"items":{"type":"string"},"type":"array"},"fields[betaTesters]":{"items":{"type":"string"},"type":"array"},"fields[buildBetaDetails]":{"items":{"type":"string"},"type":"array"},"fields[buildBundles]":{"items":{"type":"string"},"type":"array"},"fields[buildIcons]":{"items":{"type":"string"},"type":"array"},"fields[buildUploads]":{"items":{"type":"string"},"type":"array"},"fields[builds]":{"items":{"type":"string"},"type":"array"},"fields[preReleaseVersions]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["preReleaseVersion","individualTesters","betaGroups","betaBuildLocalizations","appEncryptionDeclaration","betaAppReviewSubmission","app","buildBetaDetail","appStoreVersion","icons","buildBundles","buildUpload"],"type":"string"},"type":"array"},"limit[betaBuildLocalizations]":{"maximum":50,"type":"integer"},"limit[betaGroups]":{"maximum":50,"type":"integer"},"limit[buildBundles]":{"maximum":50,"type":"integer"},"limit[icons]":{"maximum":50,"type":"integer"},"limit[individualTesters]":{"maximum":50,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/builds/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[builds]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[preReleaseVersions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[betaTesters]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[betaGroups]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[betaBuildLocalizations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appEncryptionDeclarations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[betaAppReviewSubmissions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[apps]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[buildBetaDetails]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appStoreVersions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[buildIcons]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[buildBundles]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[buildUploads]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[betaBuildLocalizations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[betaGroups]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[buildBundles]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[icons]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[individualTesters]".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "builds_updateInstance".to_string(),
            description: "Update a build: `usesNonExemptEncryption` (export compliance answer supplied by the user), `expired=true` to expire it from TestFlight.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"expired":{"nullable":true,"type":"boolean"},"usesNonExemptEncryption":{"nullable":true,"type":"boolean"}},"type":"object"},"id":{"type":"string"},"relationships":{"properties":{"appEncryptionDeclaration":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["appEncryptionDeclarations"],"type":"string"}},"required":["id","type"],"type":"object"}},"type":"object"}},"type":"object"},"type":{"enum":["builds"],"type":"string"}},"required":["id","type"],"type":"object"},"id":{"type":"string"}},"required":["id","data"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "PATCH".to_string(),
                path_template: "/v1/builds/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "bundleIdCapabilities_createInstance".to_string(),
            description: "Create one `bundleIdCapabilities` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"capabilityType":{"enum":["ICLOUD","IN_APP_PURCHASE","GAME_CENTER","PUSH_NOTIFICATIONS","WALLET","INTER_APP_AUDIO","MAPS","ASSOCIATED_DOMAINS","PERSONAL_VPN","APP_GROUPS","HEALTHKIT","HOMEKIT","WIRELESS_ACCESSORY_CONFIGURATION","APPLE_PAY","DATA_PROTECTION","SIRIKIT","NETWORK_EXTENSIONS","MULTIPATH","HOT_SPOT","NFC_TAG_READING","CLASSKIT","AUTOFILL_CREDENTIAL_PROVIDER","ACCESS_WIFI_INFORMATION","NETWORK_CUSTOM_PROTOCOL","COREMEDIA_HLS_LOW_LATENCY","SYSTEM_EXTENSION_INSTALL","USER_MANAGEMENT","APPLE_ID_AUTH"],"type":"string"},"settings":{"items":{"properties":{"allowedInstances":{"enum":["ENTRY","SINGLE","MULTIPLE"],"type":"string"},"description":{"type":"string"},"enabledByDefault":{"type":"boolean"},"key":{"enum":["ICLOUD_VERSION","DATA_PROTECTION_PERMISSION_LEVEL","APPLE_ID_AUTH_APP_CONSENT"],"type":"string"},"minInstances":{"type":"integer"},"name":{"type":"string"},"options":{"items":{"properties":{"description":{"type":"string"},"enabled":{"type":"boolean"},"enabledByDefault":{"type":"boolean"},"key":{"enum":["XCODE_5","XCODE_6","COMPLETE_PROTECTION","PROTECTED_UNLESS_OPEN","PROTECTED_UNTIL_FIRST_USER_AUTH","PRIMARY_APP_CONSENT"],"type":"string"},"name":{"type":"string"},"supportsWildcard":{"type":"boolean"}},"type":"object"},"type":"array"},"visible":{"type":"boolean"}},"type":"object"},"nullable":true,"type":"array"}},"required":["capabilityType"],"type":"object"},"relationships":{"properties":{"bundleId":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["bundleIds"],"type":"string"}},"required":["id","type"],"type":"object"}},"required":["data"],"type":"object"}},"required":["bundleId"],"type":"object"},"type":{"enum":["bundleIdCapabilities"],"type":"string"}},"required":["relationships","attributes","type"],"type":"object"}},"required":["data"],"title":"BundleIdCapabilityCreateRequest","type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "POST".to_string(),
                path_template: "/v1/bundleIdCapabilities".to_string(),
                params: vec![
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "bundleIdCapabilities_updateInstance".to_string(),
            description: "Update attributes or relationships of one `bundleIdCapabilities` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"capabilityType":{"enum":["ICLOUD","IN_APP_PURCHASE","GAME_CENTER","PUSH_NOTIFICATIONS","WALLET","INTER_APP_AUDIO","MAPS","ASSOCIATED_DOMAINS","PERSONAL_VPN","APP_GROUPS","HEALTHKIT","HOMEKIT","WIRELESS_ACCESSORY_CONFIGURATION","APPLE_PAY","DATA_PROTECTION","SIRIKIT","NETWORK_EXTENSIONS","MULTIPATH","HOT_SPOT","NFC_TAG_READING","CLASSKIT","AUTOFILL_CREDENTIAL_PROVIDER","ACCESS_WIFI_INFORMATION","NETWORK_CUSTOM_PROTOCOL","COREMEDIA_HLS_LOW_LATENCY","SYSTEM_EXTENSION_INSTALL","USER_MANAGEMENT","APPLE_ID_AUTH"],"type":"string"},"settings":{"items":{"properties":{"allowedInstances":{"enum":["ENTRY","SINGLE","MULTIPLE"],"type":"string"},"description":{"type":"string"},"enabledByDefault":{"type":"boolean"},"key":{"enum":["ICLOUD_VERSION","DATA_PROTECTION_PERMISSION_LEVEL","APPLE_ID_AUTH_APP_CONSENT"],"type":"string"},"minInstances":{"type":"integer"},"name":{"type":"string"},"options":{"items":{"properties":{"description":{"type":"string"},"enabled":{"type":"boolean"},"enabledByDefault":{"type":"boolean"},"key":{"enum":["XCODE_5","XCODE_6","COMPLETE_PROTECTION","PROTECTED_UNLESS_OPEN","PROTECTED_UNTIL_FIRST_USER_AUTH","PRIMARY_APP_CONSENT"],"type":"string"},"name":{"type":"string"},"supportsWildcard":{"type":"boolean"}},"type":"object"},"type":"array"},"visible":{"type":"boolean"}},"type":"object"},"nullable":true,"type":"array"}},"type":"object"},"id":{"type":"string"},"type":{"enum":["bundleIdCapabilities"],"type":"string"}},"required":["id","type"],"type":"object"},"id":{"type":"string"}},"required":["id","data"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "PATCH".to_string(),
                path_template: "/v1/bundleIdCapabilities/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "bundleIdCapabilities_deleteInstance".to_string(),
            description: "Delete one `bundleIdCapabilities` resource. This cannot be undone.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"id":{"type":"string"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "DELETE".to_string(),
                path_template: "/v1/bundleIdCapabilities/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(true),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "bundleIds_getCollection".to_string(),
            description: "List `bundleIds` resources; supports the filters and pagination parameters below.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[apps]":{"items":{"type":"string"},"type":"array"},"fields[bundleIdCapabilities]":{"items":{"type":"string"},"type":"array"},"fields[bundleIds]":{"items":{"type":"string"},"type":"array"},"fields[profiles]":{"items":{"type":"string"},"type":"array"},"filter[id]":{"items":{"type":"string"},"type":"array"},"filter[identifier]":{"items":{"type":"string"},"type":"array"},"filter[name]":{"items":{"type":"string"},"type":"array"},"filter[platform]":{"items":{"enum":["IOS","MAC_OS","UNIVERSAL"],"type":"string"},"type":"array"},"filter[seedId]":{"items":{"type":"string"},"type":"array"},"include":{"items":{"enum":["profiles","bundleIdCapabilities","app"],"type":"string"},"type":"array"},"limit":{"maximum":200,"type":"integer"},"limit[bundleIdCapabilities]":{"maximum":50,"type":"integer"},"limit[profiles]":{"maximum":50,"type":"integer"},"sort":{"items":{"enum":["name","-name","platform","-platform","identifier","-identifier","seedId","-seedId","id","-id"],"type":"string"},"type":"array"}},"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/bundleIds".to_string(),
                params: vec![
                    ParamBinding {
                        name: "filter[name]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[platform]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[identifier]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[seedId]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[id]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "sort".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[bundleIds]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[profiles]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[bundleIdCapabilities]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[apps]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[bundleIdCapabilities]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[profiles]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "bundleIds_createInstance".to_string(),
            description: "Create one `bundleIds` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"identifier":{"type":"string"},"name":{"type":"string"},"platform":{"enum":["IOS","MAC_OS","UNIVERSAL"],"type":"string"},"seedId":{"nullable":true,"type":"string"}},"required":["identifier","name","platform"],"type":"object"},"type":{"enum":["bundleIds"],"type":"string"}},"required":["attributes","type"],"type":"object"}},"required":["data"],"title":"BundleIdCreateRequest","type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "POST".to_string(),
                path_template: "/v1/bundleIds".to_string(),
                params: vec![
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "bundleIds_getInstance".to_string(),
            description: "Get one `bundleIds` resource by id.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[apps]":{"items":{"type":"string"},"type":"array"},"fields[bundleIdCapabilities]":{"items":{"type":"string"},"type":"array"},"fields[bundleIds]":{"items":{"type":"string"},"type":"array"},"fields[profiles]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["profiles","bundleIdCapabilities","app"],"type":"string"},"type":"array"},"limit[bundleIdCapabilities]":{"maximum":50,"type":"integer"},"limit[profiles]":{"maximum":50,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/bundleIds/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[bundleIds]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[profiles]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[bundleIdCapabilities]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[apps]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[bundleIdCapabilities]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[profiles]".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "bundleIds_updateInstance".to_string(),
            description: "Update attributes or relationships of one `bundleIds` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"name":{"nullable":true,"type":"string"}},"type":"object"},"id":{"type":"string"},"type":{"enum":["bundleIds"],"type":"string"}},"required":["id","type"],"type":"object"},"id":{"type":"string"}},"required":["id","data"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "PATCH".to_string(),
                path_template: "/v1/bundleIds/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "bundleIds_deleteInstance".to_string(),
            description: "Delete one `bundleIds` resource. This cannot be undone.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"id":{"type":"string"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "DELETE".to_string(),
                path_template: "/v1/bundleIds/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(true),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "certificates_getCollection".to_string(),
            description: "List `certificates` resources; supports the filters and pagination parameters below.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[certificates]":{"items":{"type":"string"},"type":"array"},"fields[passTypeIds]":{"items":{"type":"string"},"type":"array"},"filter[certificateType]":{"items":{"enum":["APPLE_PAY","APPLE_PAY_MERCHANT_IDENTITY","APPLE_PAY_PSP_IDENTITY","APPLE_PAY_RSA","DEVELOPER_ID_KEXT","DEVELOPER_ID_KEXT_G2","DEVELOPER_ID_APPLICATION","DEVELOPER_ID_APPLICATION_G2","DEVELOPMENT","DISTRIBUTION","IDENTITY_ACCESS","IOS_DEVELOPMENT","IOS_DISTRIBUTION","MAC_APP_DISTRIBUTION","MAC_INSTALLER_DISTRIBUTION","MAC_APP_DEVELOPMENT","PASS_TYPE_ID","PASS_TYPE_ID_WITH_NFC"],"type":"string"},"type":"array"},"filter[displayName]":{"items":{"type":"string"},"type":"array"},"filter[id]":{"items":{"type":"string"},"type":"array"},"filter[serialNumber]":{"items":{"type":"string"},"type":"array"},"include":{"items":{"enum":["passTypeId"],"type":"string"},"type":"array"},"limit":{"maximum":200,"type":"integer"},"sort":{"items":{"enum":["displayName","-displayName","certificateType","-certificateType","serialNumber","-serialNumber","id","-id"],"type":"string"},"type":"array"}},"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/certificates".to_string(),
                params: vec![
                    ParamBinding {
                        name: "filter[displayName]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[certificateType]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[serialNumber]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[id]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "sort".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[certificates]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[passTypeIds]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "certificates_createInstance".to_string(),
            description: "Create a signing certificate from a certificate signing request (`csrContent`, `certificateType`).".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"certificateType":{"enum":["APPLE_PAY","APPLE_PAY_MERCHANT_IDENTITY","APPLE_PAY_PSP_IDENTITY","APPLE_PAY_RSA","DEVELOPER_ID_KEXT","DEVELOPER_ID_KEXT_G2","DEVELOPER_ID_APPLICATION","DEVELOPER_ID_APPLICATION_G2","DEVELOPMENT","DISTRIBUTION","IDENTITY_ACCESS","IOS_DEVELOPMENT","IOS_DISTRIBUTION","MAC_APP_DISTRIBUTION","MAC_INSTALLER_DISTRIBUTION","MAC_APP_DEVELOPMENT","PASS_TYPE_ID","PASS_TYPE_ID_WITH_NFC"],"type":"string"},"csrContent":{"type":"string"}},"required":["csrContent","certificateType"],"type":"object"},"relationships":{"properties":{"merchantId":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["merchantIds"],"type":"string"}},"required":["id","type"],"type":"object"}},"type":"object"},"passTypeId":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["passTypeIds"],"type":"string"}},"required":["id","type"],"type":"object"}},"type":"object"}},"type":"object"},"type":{"enum":["certificates"],"type":"string"}},"required":["attributes","type"],"type":"object"}},"required":["data"],"title":"CertificateCreateRequest","type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "POST".to_string(),
                path_template: "/v1/certificates".to_string(),
                params: vec![
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "certificates_getInstance".to_string(),
            description: "Get one `certificates` resource by id.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[certificates]":{"items":{"type":"string"},"type":"array"},"fields[passTypeIds]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["passTypeId"],"type":"string"},"type":"array"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/certificates/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[certificates]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[passTypeIds]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "certificates_updateInstance".to_string(),
            description: "Update attributes or relationships of one `certificates` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"activated":{"nullable":true,"type":"boolean"}},"type":"object"},"id":{"type":"string"},"type":{"enum":["certificates"],"type":"string"}},"required":["id","type"],"type":"object"},"id":{"type":"string"}},"required":["id","data"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "PATCH".to_string(),
                path_template: "/v1/certificates/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "certificates_deleteInstance".to_string(),
            description: "Revoke a signing certificate. Apps and profiles that use it stop working. This cannot be undone.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"id":{"type":"string"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "DELETE".to_string(),
                path_template: "/v1/certificates/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(true),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "customerReviewResponses_createInstance".to_string(),
            description: "Publish a developer response to a customer review. If a response exists it is replaced.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"responseBody":{"type":"string"}},"required":["responseBody"],"type":"object"},"relationships":{"properties":{"review":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["customerReviews"],"type":"string"}},"required":["id","type"],"type":"object"}},"required":["data"],"type":"object"}},"required":["review"],"type":"object"},"type":{"enum":["customerReviewResponses"],"type":"string"}},"required":["relationships","attributes","type"],"type":"object"}},"required":["data"],"title":"CustomerReviewResponseV1CreateRequest","type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "POST".to_string(),
                path_template: "/v1/customerReviewResponses".to_string(),
                params: vec![
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "customerReviewResponses_getInstance".to_string(),
            description: "Get one `customerReviewResponses` resource by id.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[customerReviewResponses]":{"items":{"type":"string"},"type":"array"},"fields[customerReviews]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["review"],"type":"string"},"type":"array"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/customerReviewResponses/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[customerReviewResponses]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[customerReviews]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "customerReviewResponses_deleteInstance".to_string(),
            description: "Delete the developer response to a customer review. This cannot be undone.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"id":{"type":"string"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "DELETE".to_string(),
                path_template: "/v1/customerReviewResponses/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(true),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "customerReviews_getInstance".to_string(),
            description: "Get one `customerReviews` resource by id.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[customerReviewResponses]":{"items":{"type":"string"},"type":"array"},"fields[customerReviews]":{"items":{"type":"string"},"type":"array"},"fields[territories]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["response","reviewTerritory"],"type":"string"},"type":"array"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/customerReviews/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[customerReviews]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[customerReviewResponses]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[territories]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "devices_getCollection".to_string(),
            description: "List `devices` resources; supports the filters and pagination parameters below.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[devices]":{"items":{"type":"string"},"type":"array"},"filter[id]":{"items":{"type":"string"},"type":"array"},"filter[name]":{"items":{"type":"string"},"type":"array"},"filter[platform]":{"items":{"enum":["IOS","MAC_OS","UNIVERSAL"],"type":"string"},"type":"array"},"filter[status]":{"items":{"enum":["ENABLED","DISABLED"],"type":"string"},"type":"array"},"filter[udid]":{"items":{"type":"string"},"type":"array"},"limit":{"maximum":200,"type":"integer"},"sort":{"items":{"enum":["name","-name","platform","-platform","udid","-udid","status","-status","id","-id"],"type":"string"},"type":"array"}},"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/devices".to_string(),
                params: vec![
                    ParamBinding {
                        name: "filter[name]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[platform]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[udid]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[status]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[id]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "sort".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[devices]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "devices_createInstance".to_string(),
            description: "Register a device UDID for development/ad hoc provisioning. Registrations count against the yearly device limit.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"name":{"type":"string"},"platform":{"enum":["IOS","MAC_OS","UNIVERSAL"],"type":"string"},"udid":{"type":"string"}},"required":["name","udid","platform"],"type":"object"},"type":{"enum":["devices"],"type":"string"}},"required":["attributes","type"],"type":"object"}},"required":["data"],"title":"DeviceCreateRequest","type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "POST".to_string(),
                path_template: "/v1/devices".to_string(),
                params: vec![
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "devices_getInstance".to_string(),
            description: "Get one `devices` resource by id.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[devices]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/devices/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[devices]".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "devices_updateInstance".to_string(),
            description: "Update attributes or relationships of one `devices` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"name":{"nullable":true,"type":"string"},"status":{"enum":["ENABLED","DISABLED"],"nullable":true,"type":"string"}},"type":"object"},"id":{"type":"string"},"type":{"enum":["devices"],"type":"string"}},"required":["id","type"],"type":"object"},"id":{"type":"string"}},"required":["id","data"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "PATCH".to_string(),
                path_template: "/v1/devices/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "financeReports_getCollection".to_string(),
            description: "Download a finance report (gzip TSV, returned as text). Requires `filter[vendorNumber]`, `filter[reportType]`, `filter[regionCode]`, `filter[reportDate]`.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"filter[regionCode]":{"items":{"type":"string"},"type":"array"},"filter[reportDate]":{"items":{"type":"string"},"type":"array"},"filter[reportType]":{"items":{"enum":["FINANCIAL","FINANCE_DETAIL"],"type":"string"},"type":"array"},"filter[vendorNumber]":{"items":{"type":"string"},"type":"array"}},"required":["filter[vendorNumber]","filter[reportType]","filter[regionCode]","filter[reportDate]"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/financeReports".to_string(),
                params: vec![
                    ParamBinding {
                        name: "filter[vendorNumber]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[reportType]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[regionCode]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[reportDate]".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "inAppPurchaseAppStoreReviewScreenshots_createInstance".to_string(),
            description: "Create one `inAppPurchaseAppStoreReviewScreenshots` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"fileName":{"type":"string"},"fileSize":{"type":"integer"}},"required":["fileName","fileSize"],"type":"object"},"relationships":{"properties":{"inAppPurchaseV2":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["inAppPurchases"],"type":"string"}},"required":["id","type"],"type":"object"}},"required":["data"],"type":"object"}},"required":["inAppPurchaseV2"],"type":"object"},"type":{"enum":["inAppPurchaseAppStoreReviewScreenshots"],"type":"string"}},"required":["relationships","attributes","type"],"type":"object"}},"required":["data"],"title":"InAppPurchaseAppStoreReviewScreenshotCreateRequest","type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "POST".to_string(),
                path_template: "/v1/inAppPurchaseAppStoreReviewScreenshots".to_string(),
                params: vec![
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "inAppPurchaseAppStoreReviewScreenshots_getInstance".to_string(),
            description: "Get one `inAppPurchaseAppStoreReviewScreenshots` resource by id.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[inAppPurchaseAppStoreReviewScreenshots]":{"items":{"type":"string"},"type":"array"},"fields[inAppPurchases]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["inAppPurchaseV2"],"type":"string"},"type":"array"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/inAppPurchaseAppStoreReviewScreenshots/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[inAppPurchaseAppStoreReviewScreenshots]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[inAppPurchases]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "inAppPurchaseAppStoreReviewScreenshots_updateInstance".to_string(),
            description: "Update attributes or relationships of one `inAppPurchaseAppStoreReviewScreenshots` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"sourceFileChecksum":{"nullable":true,"type":"string"},"uploaded":{"nullable":true,"type":"boolean"}},"type":"object"},"id":{"type":"string"},"type":{"enum":["inAppPurchaseAppStoreReviewScreenshots"],"type":"string"}},"required":["id","type"],"type":"object"},"id":{"type":"string"}},"required":["id","data"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "PATCH".to_string(),
                path_template: "/v1/inAppPurchaseAppStoreReviewScreenshots/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "inAppPurchaseAppStoreReviewScreenshots_deleteInstance".to_string(),
            description: "Delete one `inAppPurchaseAppStoreReviewScreenshots` resource. This cannot be undone.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"id":{"type":"string"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "DELETE".to_string(),
                path_template: "/v1/inAppPurchaseAppStoreReviewScreenshots/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(true),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "inAppPurchaseAvailabilities_createInstance".to_string(),
            description: "Create one `inAppPurchaseAvailabilities` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"availableInNewTerritories":{"type":"boolean"}},"required":["availableInNewTerritories"],"type":"object"},"relationships":{"properties":{"availableTerritories":{"properties":{"data":{"items":{"properties":{"id":{"type":"string"},"type":{"enum":["territories"],"type":"string"}},"required":["id","type"],"type":"object"},"type":"array"}},"required":["data"],"type":"object"},"inAppPurchase":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["inAppPurchases"],"type":"string"}},"required":["id","type"],"type":"object"}},"required":["data"],"type":"object"}},"required":["inAppPurchase","availableTerritories"],"type":"object"},"type":{"enum":["inAppPurchaseAvailabilities"],"type":"string"}},"required":["relationships","attributes","type"],"type":"object"}},"required":["data"],"title":"InAppPurchaseAvailabilityCreateRequest","type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "POST".to_string(),
                path_template: "/v1/inAppPurchaseAvailabilities".to_string(),
                params: vec![
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "inAppPurchaseAvailabilities_getInstance".to_string(),
            description: "Get one `inAppPurchaseAvailabilities` resource by id.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[inAppPurchaseAvailabilities]":{"items":{"type":"string"},"type":"array"},"fields[territories]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["availableTerritories"],"type":"string"},"type":"array"},"limit[availableTerritories]":{"maximum":50,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/inAppPurchaseAvailabilities/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[inAppPurchaseAvailabilities]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[territories]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[availableTerritories]".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "inAppPurchaseLocalizations_createInstance".to_string(),
            description: "Create one `inAppPurchaseLocalizations` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"description":{"nullable":true,"type":"string"},"locale":{"type":"string"},"name":{"type":"string"}},"required":["name","locale"],"type":"object"},"relationships":{"properties":{"inAppPurchaseV2":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["inAppPurchases"],"type":"string"}},"required":["id","type"],"type":"object"}},"required":["data"],"type":"object"}},"required":["inAppPurchaseV2"],"type":"object"},"type":{"enum":["inAppPurchaseLocalizations"],"type":"string"}},"required":["relationships","attributes","type"],"type":"object"}},"required":["data"],"title":"InAppPurchaseLocalizationCreateRequest","type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "POST".to_string(),
                path_template: "/v1/inAppPurchaseLocalizations".to_string(),
                params: vec![
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "inAppPurchaseLocalizations_getInstance".to_string(),
            description: "Get one `inAppPurchaseLocalizations` resource by id.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[inAppPurchaseLocalizations]":{"items":{"type":"string"},"type":"array"},"fields[inAppPurchases]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["inAppPurchaseV2"],"type":"string"},"type":"array"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/inAppPurchaseLocalizations/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[inAppPurchaseLocalizations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[inAppPurchases]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "inAppPurchaseLocalizations_updateInstance".to_string(),
            description: "Update attributes or relationships of one `inAppPurchaseLocalizations` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"description":{"nullable":true,"type":"string"},"name":{"nullable":true,"type":"string"}},"type":"object"},"id":{"type":"string"},"type":{"enum":["inAppPurchaseLocalizations"],"type":"string"}},"required":["id","type"],"type":"object"},"id":{"type":"string"}},"required":["id","data"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "PATCH".to_string(),
                path_template: "/v1/inAppPurchaseLocalizations/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "inAppPurchaseLocalizations_deleteInstance".to_string(),
            description: "Delete one `inAppPurchaseLocalizations` resource. This cannot be undone.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"id":{"type":"string"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "DELETE".to_string(),
                path_template: "/v1/inAppPurchaseLocalizations/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(true),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "inAppPurchasePriceSchedules_createInstance".to_string(),
            description: "Create one `inAppPurchasePriceSchedules` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"relationships":{"properties":{"baseTerritory":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["territories"],"type":"string"}},"required":["id","type"],"type":"object"}},"required":["data"],"type":"object"},"inAppPurchase":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["inAppPurchases"],"type":"string"}},"required":["id","type"],"type":"object"}},"required":["data"],"type":"object"},"manualPrices":{"properties":{"data":{"items":{"properties":{"id":{"type":"string"},"type":{"enum":["inAppPurchasePrices"],"type":"string"}},"required":["id","type"],"type":"object"},"type":"array"}},"required":["data"],"type":"object"}},"required":["inAppPurchase","manualPrices","baseTerritory"],"type":"object"},"type":{"enum":["inAppPurchasePriceSchedules"],"type":"string"}},"required":["relationships","type"],"type":"object"},"included":{"items":{"oneOf":[{"properties":{"attributes":{"properties":{"endDate":{"format":"date","nullable":true,"type":"string"},"startDate":{"format":"date","nullable":true,"type":"string"}},"type":"object"},"id":{"type":"string"},"relationships":{"properties":{"inAppPurchasePricePoint":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["inAppPurchasePricePoints"],"type":"string"}},"required":["id","type"],"type":"object"}},"type":"object"},"inAppPurchaseV2":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["inAppPurchases"],"type":"string"}},"required":["id","type"],"type":"object"}},"type":"object"}},"type":"object"},"type":{"enum":["inAppPurchasePrices"],"type":"string"}},"required":["type"],"type":"object"},{"properties":{"id":{"type":"string"},"type":{"enum":["territories"],"type":"string"}},"required":["type"],"type":"object"}]},"type":"array"}},"required":["data"],"title":"InAppPurchasePriceScheduleCreateRequest","type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "POST".to_string(),
                path_template: "/v1/inAppPurchasePriceSchedules".to_string(),
                params: vec![
                ],
                body_fields: vec![
                    "data".to_string(),
                    "included".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "inAppPurchasePriceSchedules_getInstance".to_string(),
            description: "Get one `inAppPurchasePriceSchedules` resource by id.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[inAppPurchasePriceSchedules]":{"items":{"type":"string"},"type":"array"},"fields[inAppPurchasePrices]":{"items":{"type":"string"},"type":"array"},"fields[territories]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["baseTerritory","manualPrices","automaticPrices"],"type":"string"},"type":"array"},"limit[automaticPrices]":{"maximum":50,"type":"integer"},"limit[manualPrices]":{"maximum":50,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/inAppPurchasePriceSchedules/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[inAppPurchasePriceSchedules]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[territories]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[inAppPurchasePrices]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[automaticPrices]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[manualPrices]".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "inAppPurchaseSubmissions_createInstance".to_string(),
            description: "Create one `inAppPurchaseSubmissions` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"relationships":{"properties":{"inAppPurchaseV2":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["inAppPurchases"],"type":"string"}},"required":["id","type"],"type":"object"}},"required":["data"],"type":"object"}},"required":["inAppPurchaseV2"],"type":"object"},"type":{"enum":["inAppPurchaseSubmissions"],"type":"string"}},"required":["relationships","type"],"type":"object"}},"required":["data"],"title":"InAppPurchaseSubmissionCreateRequest","type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "POST".to_string(),
                path_template: "/v1/inAppPurchaseSubmissions".to_string(),
                params: vec![
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "inAppPurchasesV2_createInstance".to_string(),
            description: "Create one `inAppPurchases` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"familySharable":{"nullable":true,"type":"boolean"},"inAppPurchaseType":{"enum":["CONSUMABLE","NON_CONSUMABLE","NON_RENEWING_SUBSCRIPTION"],"type":"string"},"name":{"type":"string"},"productId":{"type":"string"},"reviewNote":{"nullable":true,"type":"string"}},"required":["productId","name","inAppPurchaseType"],"type":"object"},"relationships":{"properties":{"app":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["apps"],"type":"string"}},"required":["id","type"],"type":"object"}},"required":["data"],"type":"object"}},"required":["app"],"type":"object"},"type":{"enum":["inAppPurchases"],"type":"string"}},"required":["relationships","attributes","type"],"type":"object"}},"required":["data"],"title":"InAppPurchaseV2CreateRequest","type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "POST".to_string(),
                path_template: "/v2/inAppPurchases".to_string(),
                params: vec![
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "inAppPurchasesV2_getInstance".to_string(),
            description: "Get one `inAppPurchases` resource by id.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[inAppPurchaseAppStoreReviewScreenshots]":{"items":{"type":"string"},"type":"array"},"fields[inAppPurchaseAvailabilities]":{"items":{"type":"string"},"type":"array"},"fields[inAppPurchaseContents]":{"items":{"type":"string"},"type":"array"},"fields[inAppPurchaseImages]":{"items":{"type":"string"},"type":"array"},"fields[inAppPurchaseLocalizations]":{"items":{"type":"string"},"type":"array"},"fields[inAppPurchaseOfferCodes]":{"items":{"type":"string"},"type":"array"},"fields[inAppPurchasePricePoints]":{"items":{"type":"string"},"type":"array"},"fields[inAppPurchasePriceSchedules]":{"items":{"type":"string"},"type":"array"},"fields[inAppPurchaseVersions]":{"items":{"type":"string"},"type":"array"},"fields[inAppPurchases]":{"items":{"type":"string"},"type":"array"},"fields[promotedPurchases]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["inAppPurchaseLocalizations","pricePoints","content","appStoreReviewScreenshot","promotedPurchase","iapPriceSchedule","inAppPurchaseAvailability","images","offerCodes","versions"],"type":"string"},"type":"array"},"limit[images]":{"maximum":50,"type":"integer"},"limit[inAppPurchaseLocalizations]":{"maximum":50,"type":"integer"},"limit[offerCodes]":{"maximum":50,"type":"integer"},"limit[pricePoints]":{"maximum":8000,"type":"integer"},"limit[versions]":{"maximum":50,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v2/inAppPurchases/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[inAppPurchases]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[inAppPurchaseLocalizations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[inAppPurchasePricePoints]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[inAppPurchaseContents]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[inAppPurchaseAppStoreReviewScreenshots]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[promotedPurchases]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[inAppPurchasePriceSchedules]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[inAppPurchaseAvailabilities]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[inAppPurchaseImages]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[inAppPurchaseOfferCodes]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[inAppPurchaseVersions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[images]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[inAppPurchaseLocalizations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[offerCodes]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[pricePoints]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[versions]".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "inAppPurchasesV2_updateInstance".to_string(),
            description: "Update attributes or relationships of one `inAppPurchases` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"familySharable":{"nullable":true,"type":"boolean"},"name":{"nullable":true,"type":"string"},"reviewNote":{"nullable":true,"type":"string"}},"type":"object"},"id":{"type":"string"},"type":{"enum":["inAppPurchases"],"type":"string"}},"required":["id","type"],"type":"object"},"id":{"type":"string"}},"required":["id","data"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "PATCH".to_string(),
                path_template: "/v2/inAppPurchases/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "inAppPurchasesV2_deleteInstance".to_string(),
            description: "Delete one `inAppPurchases` resource. This cannot be undone.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"id":{"type":"string"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "DELETE".to_string(),
                path_template: "/v2/inAppPurchases/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(true),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "preReleaseVersions_getCollection".to_string(),
            description: "List `preReleaseVersions` resources; supports the filters and pagination parameters below.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[apps]":{"items":{"type":"string"},"type":"array"},"fields[builds]":{"items":{"type":"string"},"type":"array"},"fields[preReleaseVersions]":{"items":{"type":"string"},"type":"array"},"filter[app]":{"items":{"type":"string"},"type":"array"},"filter[builds.buildAudienceType]":{"items":{"enum":["INTERNAL_ONLY","APP_STORE_ELIGIBLE"],"type":"string"},"type":"array"},"filter[builds.expired]":{"items":{"type":"string"},"type":"array"},"filter[builds.processingState]":{"items":{"enum":["PROCESSING","FAILED","INVALID","VALID"],"type":"string"},"type":"array"},"filter[builds.version]":{"items":{"type":"string"},"type":"array"},"filter[builds]":{"items":{"type":"string"},"type":"array"},"filter[platform]":{"items":{"enum":["IOS","MAC_OS","TV_OS","VISION_OS"],"type":"string"},"type":"array"},"filter[version]":{"items":{"type":"string"},"type":"array"},"include":{"items":{"enum":["builds","app"],"type":"string"},"type":"array"},"limit":{"maximum":200,"type":"integer"},"limit[builds]":{"maximum":50,"type":"integer"},"sort":{"items":{"enum":["version","-version"],"type":"string"},"type":"array"}},"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/preReleaseVersions".to_string(),
                params: vec![
                    ParamBinding {
                        name: "filter[builds.buildAudienceType]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[builds.expired]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[builds.processingState]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[builds.version]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[platform]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[version]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[app]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[builds]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "sort".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[preReleaseVersions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[builds]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[apps]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[builds]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "profiles_getCollection".to_string(),
            description: "List `profiles` resources; supports the filters and pagination parameters below.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[bundleIds]":{"items":{"type":"string"},"type":"array"},"fields[certificates]":{"items":{"type":"string"},"type":"array"},"fields[devices]":{"items":{"type":"string"},"type":"array"},"fields[profiles]":{"items":{"type":"string"},"type":"array"},"filter[id]":{"items":{"type":"string"},"type":"array"},"filter[name]":{"items":{"type":"string"},"type":"array"},"filter[profileState]":{"items":{"enum":["ACTIVE","INVALID"],"type":"string"},"type":"array"},"filter[profileType]":{"items":{"enum":["IOS_APP_DEVELOPMENT","IOS_APP_STORE","IOS_APP_ADHOC","IOS_APP_INHOUSE","MAC_APP_DEVELOPMENT","MAC_APP_STORE","MAC_APP_DIRECT","TVOS_APP_DEVELOPMENT","TVOS_APP_STORE","TVOS_APP_ADHOC","TVOS_APP_INHOUSE","MAC_CATALYST_APP_DEVELOPMENT","MAC_CATALYST_APP_STORE","MAC_CATALYST_APP_DIRECT"],"type":"string"},"type":"array"},"include":{"items":{"enum":["bundleId","devices","certificates"],"type":"string"},"type":"array"},"limit":{"maximum":200,"type":"integer"},"limit[certificates]":{"maximum":50,"type":"integer"},"limit[devices]":{"maximum":50,"type":"integer"},"sort":{"items":{"enum":["name","-name","profileType","-profileType","profileState","-profileState","id","-id"],"type":"string"},"type":"array"}},"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/profiles".to_string(),
                params: vec![
                    ParamBinding {
                        name: "filter[name]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[profileType]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[profileState]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[id]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "sort".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[profiles]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[bundleIds]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[devices]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[certificates]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[certificates]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[devices]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "profiles_createInstance".to_string(),
            description: "Create a provisioning profile for a bundle ID, certificates and (for development/ad hoc) devices. The response includes `profileContent` (base64).".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"name":{"type":"string"},"profileType":{"enum":["IOS_APP_DEVELOPMENT","IOS_APP_STORE","IOS_APP_ADHOC","IOS_APP_INHOUSE","MAC_APP_DEVELOPMENT","MAC_APP_STORE","MAC_APP_DIRECT","TVOS_APP_DEVELOPMENT","TVOS_APP_STORE","TVOS_APP_ADHOC","TVOS_APP_INHOUSE","MAC_CATALYST_APP_DEVELOPMENT","MAC_CATALYST_APP_STORE","MAC_CATALYST_APP_DIRECT"],"type":"string"}},"required":["profileType","name"],"type":"object"},"relationships":{"properties":{"bundleId":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["bundleIds"],"type":"string"}},"required":["id","type"],"type":"object"}},"required":["data"],"type":"object"},"certificates":{"properties":{"data":{"items":{"properties":{"id":{"type":"string"},"type":{"enum":["certificates"],"type":"string"}},"required":["id","type"],"type":"object"},"type":"array"}},"required":["data"],"type":"object"},"devices":{"properties":{"data":{"items":{"properties":{"id":{"type":"string"},"type":{"enum":["devices"],"type":"string"}},"required":["id","type"],"type":"object"},"type":"array"}},"type":"object"}},"required":["certificates","bundleId"],"type":"object"},"type":{"enum":["profiles"],"type":"string"}},"required":["relationships","attributes","type"],"type":"object"}},"required":["data"],"title":"ProfileCreateRequest","type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "POST".to_string(),
                path_template: "/v1/profiles".to_string(),
                params: vec![
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "profiles_getInstance".to_string(),
            description: "Get one `profiles` resource by id.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[bundleIds]":{"items":{"type":"string"},"type":"array"},"fields[certificates]":{"items":{"type":"string"},"type":"array"},"fields[devices]":{"items":{"type":"string"},"type":"array"},"fields[profiles]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["bundleId","devices","certificates"],"type":"string"},"type":"array"},"limit[certificates]":{"maximum":50,"type":"integer"},"limit[devices]":{"maximum":50,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/profiles/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[profiles]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[bundleIds]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[devices]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[certificates]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[certificates]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[devices]".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "profiles_deleteInstance".to_string(),
            description: "Delete a provisioning profile. This cannot be undone.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"id":{"type":"string"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "DELETE".to_string(),
                path_template: "/v1/profiles/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(true),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "reviewSubmissionItems_createInstance".to_string(),
            description: "Add an item (e.g. an `appStoreVersion`) to a review submission that has not been submitted yet.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"relationships":{"properties":{"appCustomProductPageVersion":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["appCustomProductPageVersions"],"type":"string"}},"required":["id","type"],"type":"object"}},"type":"object"},"appEvent":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["appEvents"],"type":"string"}},"required":["id","type"],"type":"object"}},"type":"object"},"appStoreVersion":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["appStoreVersions"],"type":"string"}},"required":["id","type"],"type":"object"}},"type":"object"},"appStoreVersionExperiment":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["appStoreVersionExperiments"],"type":"string"}},"required":["id","type"],"type":"object"}},"type":"object"},"appStoreVersionExperimentV2":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["appStoreVersionExperiments"],"type":"string"}},"required":["id","type"],"type":"object"}},"type":"object"},"backgroundAssetVersion":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["backgroundAssetVersions"],"type":"string"}},"required":["id","type"],"type":"object"}},"type":"object"},"gameCenterAchievementVersion":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["gameCenterAchievementVersions"],"type":"string"}},"required":["id","type"],"type":"object"}},"type":"object"},"gameCenterActivityVersion":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["gameCenterActivityVersions"],"type":"string"}},"required":["id","type"],"type":"object"}},"type":"object"},"gameCenterChallengeVersion":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["gameCenterChallengeVersions"],"type":"string"}},"required":["id","type"],"type":"object"}},"type":"object"},"gameCenterLeaderboardSetVersion":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["gameCenterLeaderboardSetVersions"],"type":"string"}},"required":["id","type"],"type":"object"}},"type":"object"},"gameCenterLeaderboardVersion":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["gameCenterLeaderboardVersions"],"type":"string"}},"required":["id","type"],"type":"object"}},"type":"object"},"inAppPurchaseVersion":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["inAppPurchaseVersions"],"type":"string"}},"required":["id","type"],"type":"object"}},"type":"object"},"reviewSubmission":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["reviewSubmissions"],"type":"string"}},"required":["id","type"],"type":"object"}},"required":["data"],"type":"object"},"subscriptionGroupVersion":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["subscriptionGroupVersions"],"type":"string"}},"required":["id","type"],"type":"object"}},"type":"object"},"subscriptionVersion":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["subscriptionVersions"],"type":"string"}},"required":["id","type"],"type":"object"}},"type":"object"}},"required":["reviewSubmission"],"type":"object"},"type":{"enum":["reviewSubmissionItems"],"type":"string"}},"required":["relationships","type"],"type":"object"}},"required":["data"],"title":"ReviewSubmissionItemCreateRequest","type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "POST".to_string(),
                path_template: "/v1/reviewSubmissionItems".to_string(),
                params: vec![
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "reviewSubmissionItems_updateInstance".to_string(),
            description: "Update attributes or relationships of one `reviewSubmissionItems` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"removed":{"nullable":true,"type":"boolean"},"resolved":{"nullable":true,"type":"boolean"}},"type":"object"},"id":{"type":"string"},"type":{"enum":["reviewSubmissionItems"],"type":"string"}},"required":["id","type"],"type":"object"},"id":{"type":"string"}},"required":["id","data"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "PATCH".to_string(),
                path_template: "/v1/reviewSubmissionItems/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "reviewSubmissionItems_deleteInstance".to_string(),
            description: "Delete one `reviewSubmissionItems` resource. This cannot be undone.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"id":{"type":"string"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "DELETE".to_string(),
                path_template: "/v1/reviewSubmissionItems/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(true),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "reviewSubmissions_getCollection".to_string(),
            description: "List review submissions. Filter by `filter[app]` and `filter[state]` (READY_FOR_REVIEW, WAITING_FOR_REVIEW, IN_REVIEW, UNRESOLVED_ISSUES, COMPLETING, COMPLETE, CANCELING) to inspect review status.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[actors]":{"items":{"type":"string"},"type":"array"},"fields[appStoreVersions]":{"items":{"type":"string"},"type":"array"},"fields[apps]":{"items":{"type":"string"},"type":"array"},"fields[reviewSubmissionItems]":{"items":{"type":"string"},"type":"array"},"fields[reviewSubmissions]":{"items":{"type":"string"},"type":"array"},"filter[app]":{"items":{"type":"string"},"type":"array"},"filter[platform]":{"items":{"enum":["IOS","MAC_OS","TV_OS","VISION_OS"],"type":"string"},"type":"array"},"filter[state]":{"items":{"enum":["READY_FOR_REVIEW","WAITING_FOR_REVIEW","IN_REVIEW","UNRESOLVED_ISSUES","CANCELING","COMPLETING","COMPLETE"],"type":"string"},"type":"array"},"include":{"items":{"enum":["app","items","appStoreVersionForReview","submittedByActor","lastUpdatedByActor"],"type":"string"},"type":"array"},"limit":{"maximum":200,"type":"integer"},"limit[items]":{"maximum":50,"type":"integer"}},"required":["filter[app]"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/reviewSubmissions".to_string(),
                params: vec![
                    ParamBinding {
                        name: "filter[platform]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[state]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[app]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[reviewSubmissions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[apps]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[reviewSubmissionItems]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appStoreVersions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[actors]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[items]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "reviewSubmissions_createInstance".to_string(),
            description: "Start a review submission for an app and platform. Then add items with `reviewSubmissionItems_createInstance` and submit with `reviewSubmissions_updateInstance`.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"platform":{"enum":["IOS","MAC_OS","TV_OS","VISION_OS"],"type":"string"}},"type":"object"},"relationships":{"properties":{"app":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["apps"],"type":"string"}},"required":["id","type"],"type":"object"}},"required":["data"],"type":"object"}},"required":["app"],"type":"object"},"type":{"enum":["reviewSubmissions"],"type":"string"}},"required":["relationships","type"],"type":"object"}},"required":["data"],"title":"ReviewSubmissionCreateRequest","type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "POST".to_string(),
                path_template: "/v1/reviewSubmissions".to_string(),
                params: vec![
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "reviewSubmissions_getInstance".to_string(),
            description: "Get one `reviewSubmissions` resource by id.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[actors]":{"items":{"type":"string"},"type":"array"},"fields[appStoreVersions]":{"items":{"type":"string"},"type":"array"},"fields[apps]":{"items":{"type":"string"},"type":"array"},"fields[reviewSubmissionItems]":{"items":{"type":"string"},"type":"array"},"fields[reviewSubmissions]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["app","items","appStoreVersionForReview","submittedByActor","lastUpdatedByActor"],"type":"string"},"type":"array"},"limit[items]":{"maximum":50,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/reviewSubmissions/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[reviewSubmissions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[apps]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[reviewSubmissionItems]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appStoreVersions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[actors]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[items]".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "reviewSubmissions_updateInstance".to_string(),
            description: "Submit a review submission to App Review (`submitted: true`) or cancel it (`canceled: true`).".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"canceled":{"nullable":true,"type":"boolean"},"platform":{"enum":["IOS","MAC_OS","TV_OS","VISION_OS"],"type":"string"},"submitted":{"nullable":true,"type":"boolean"}},"type":"object"},"id":{"type":"string"},"type":{"enum":["reviewSubmissions"],"type":"string"}},"required":["id","type"],"type":"object"},"id":{"type":"string"}},"required":["id","data"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "PATCH".to_string(),
                path_template: "/v1/reviewSubmissions/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "salesReports_getCollection".to_string(),
            description: "Download a sales report (gzip TSV, returned as text). Requires `filter[vendorNumber]`, `filter[reportType]`, `filter[reportSubType]`, `filter[frequency]`.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"filter[frequency]":{"items":{"enum":["DAILY","WEEKLY","MONTHLY","YEARLY"],"type":"string"},"type":"array"},"filter[reportDate]":{"items":{"type":"string"},"type":"array"},"filter[reportSubType]":{"items":{"enum":["SUMMARY","DETAILED","SUMMARY_INSTALL_TYPE","SUMMARY_TERRITORY","SUMMARY_CHANNEL"],"type":"string"},"type":"array"},"filter[reportType]":{"items":{"enum":["SALES","PRE_ORDER","NEWSSTAND","SUBSCRIPTION","SUBSCRIPTION_EVENT","SUBSCRIBER","SUBSCRIPTION_OFFER_CODE_REDEMPTION","INSTALLS","FIRST_ANNUAL","WIN_BACK_ELIGIBILITY"],"type":"string"},"type":"array"},"filter[vendorNumber]":{"items":{"type":"string"},"type":"array"},"filter[version]":{"items":{"type":"string"},"type":"array"}},"required":["filter[vendorNumber]","filter[reportType]","filter[reportSubType]","filter[frequency]"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/salesReports".to_string(),
                params: vec![
                    ParamBinding {
                        name: "filter[vendorNumber]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[reportType]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[reportSubType]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[frequency]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[reportDate]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[version]".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "subscriptionAppStoreReviewScreenshots_createInstance".to_string(),
            description: "Create one `subscriptionAppStoreReviewScreenshots` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"fileName":{"type":"string"},"fileSize":{"type":"integer"}},"required":["fileName","fileSize"],"type":"object"},"relationships":{"properties":{"subscription":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["subscriptions"],"type":"string"}},"required":["id","type"],"type":"object"}},"required":["data"],"type":"object"}},"required":["subscription"],"type":"object"},"type":{"enum":["subscriptionAppStoreReviewScreenshots"],"type":"string"}},"required":["relationships","attributes","type"],"type":"object"}},"required":["data"],"title":"SubscriptionAppStoreReviewScreenshotCreateRequest","type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "POST".to_string(),
                path_template: "/v1/subscriptionAppStoreReviewScreenshots".to_string(),
                params: vec![
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "subscriptionAppStoreReviewScreenshots_getInstance".to_string(),
            description: "Get one `subscriptionAppStoreReviewScreenshots` resource by id.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[subscriptionAppStoreReviewScreenshots]":{"items":{"type":"string"},"type":"array"},"fields[subscriptions]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["subscription"],"type":"string"},"type":"array"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/subscriptionAppStoreReviewScreenshots/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[subscriptionAppStoreReviewScreenshots]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[subscriptions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "subscriptionAppStoreReviewScreenshots_updateInstance".to_string(),
            description: "Update attributes or relationships of one `subscriptionAppStoreReviewScreenshots` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"sourceFileChecksum":{"nullable":true,"type":"string"},"uploaded":{"nullable":true,"type":"boolean"}},"type":"object"},"id":{"type":"string"},"type":{"enum":["subscriptionAppStoreReviewScreenshots"],"type":"string"}},"required":["id","type"],"type":"object"},"id":{"type":"string"}},"required":["id","data"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "PATCH".to_string(),
                path_template: "/v1/subscriptionAppStoreReviewScreenshots/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "subscriptionAppStoreReviewScreenshots_deleteInstance".to_string(),
            description: "Delete one `subscriptionAppStoreReviewScreenshots` resource. This cannot be undone.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"id":{"type":"string"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "DELETE".to_string(),
                path_template: "/v1/subscriptionAppStoreReviewScreenshots/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(true),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "subscriptionGroupLocalizations_createInstance".to_string(),
            description: "Create one `subscriptionGroupLocalizations` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"customAppName":{"nullable":true,"type":"string"},"locale":{"type":"string"},"name":{"type":"string"}},"required":["name","locale"],"type":"object"},"relationships":{"properties":{"subscriptionGroup":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["subscriptionGroups"],"type":"string"}},"required":["id","type"],"type":"object"}},"required":["data"],"type":"object"}},"required":["subscriptionGroup"],"type":"object"},"type":{"enum":["subscriptionGroupLocalizations"],"type":"string"}},"required":["relationships","attributes","type"],"type":"object"}},"required":["data"],"title":"SubscriptionGroupLocalizationCreateRequest","type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "POST".to_string(),
                path_template: "/v1/subscriptionGroupLocalizations".to_string(),
                params: vec![
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "subscriptionGroupLocalizations_getInstance".to_string(),
            description: "Get one `subscriptionGroupLocalizations` resource by id.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[subscriptionGroupLocalizations]":{"items":{"type":"string"},"type":"array"},"fields[subscriptionGroups]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["subscriptionGroup"],"type":"string"},"type":"array"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/subscriptionGroupLocalizations/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[subscriptionGroupLocalizations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[subscriptionGroups]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "subscriptionGroupLocalizations_updateInstance".to_string(),
            description: "Update attributes or relationships of one `subscriptionGroupLocalizations` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"customAppName":{"nullable":true,"type":"string"},"name":{"nullable":true,"type":"string"}},"type":"object"},"id":{"type":"string"},"type":{"enum":["subscriptionGroupLocalizations"],"type":"string"}},"required":["id","type"],"type":"object"},"id":{"type":"string"}},"required":["id","data"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "PATCH".to_string(),
                path_template: "/v1/subscriptionGroupLocalizations/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "subscriptionGroupLocalizations_deleteInstance".to_string(),
            description: "Delete one `subscriptionGroupLocalizations` resource. This cannot be undone.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"id":{"type":"string"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "DELETE".to_string(),
                path_template: "/v1/subscriptionGroupLocalizations/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(true),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "subscriptionGroups_createInstance".to_string(),
            description: "Create one `subscriptionGroups` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"referenceName":{"type":"string"}},"required":["referenceName"],"type":"object"},"relationships":{"properties":{"app":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["apps"],"type":"string"}},"required":["id","type"],"type":"object"}},"required":["data"],"type":"object"}},"required":["app"],"type":"object"},"type":{"enum":["subscriptionGroups"],"type":"string"}},"required":["relationships","attributes","type"],"type":"object"}},"required":["data"],"title":"SubscriptionGroupCreateRequest","type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "POST".to_string(),
                path_template: "/v1/subscriptionGroups".to_string(),
                params: vec![
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "subscriptionGroups_getInstance".to_string(),
            description: "Get one `subscriptionGroups` resource by id.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[subscriptionGroupLocalizations]":{"items":{"type":"string"},"type":"array"},"fields[subscriptionGroupVersions]":{"items":{"type":"string"},"type":"array"},"fields[subscriptionGroups]":{"items":{"type":"string"},"type":"array"},"fields[subscriptions]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["subscriptions","subscriptionGroupLocalizations","versions"],"type":"string"},"type":"array"},"limit[subscriptionGroupLocalizations]":{"maximum":50,"type":"integer"},"limit[subscriptions]":{"maximum":50,"type":"integer"},"limit[versions]":{"maximum":50,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/subscriptionGroups/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[subscriptionGroups]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[subscriptions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[subscriptionGroupLocalizations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[subscriptionGroupVersions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[subscriptionGroupLocalizations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[subscriptions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[versions]".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "subscriptionGroups_updateInstance".to_string(),
            description: "Update attributes or relationships of one `subscriptionGroups` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"referenceName":{"nullable":true,"type":"string"}},"type":"object"},"id":{"type":"string"},"type":{"enum":["subscriptionGroups"],"type":"string"}},"required":["id","type"],"type":"object"},"id":{"type":"string"}},"required":["id","data"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "PATCH".to_string(),
                path_template: "/v1/subscriptionGroups/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "subscriptionGroups_deleteInstance".to_string(),
            description: "Delete one `subscriptionGroups` resource. This cannot be undone.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"id":{"type":"string"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "DELETE".to_string(),
                path_template: "/v1/subscriptionGroups/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(true),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "subscriptionIntroductoryOffers_createInstance".to_string(),
            description: "Create one `subscriptionIntroductoryOffers` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"duration":{"enum":["THREE_DAYS","ONE_WEEK","TWO_WEEKS","ONE_MONTH","TWO_MONTHS","THREE_MONTHS","SIX_MONTHS","ONE_YEAR"],"type":"string"},"endDate":{"format":"date","nullable":true,"type":"string"},"numberOfPeriods":{"type":"integer"},"offerMode":{"enum":["PAY_AS_YOU_GO","PAY_UP_FRONT","FREE_TRIAL"],"type":"string"},"startDate":{"format":"date","nullable":true,"type":"string"},"targetSubscriptionPlanType":{"enum":["MONTHLY","UPFRONT"],"type":"string"}},"required":["duration","numberOfPeriods","offerMode"],"type":"object"},"relationships":{"properties":{"subscription":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["subscriptions"],"type":"string"}},"required":["id","type"],"type":"object"}},"required":["data"],"type":"object"},"subscriptionPricePoint":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["subscriptionPricePoints"],"type":"string"}},"required":["id","type"],"type":"object"}},"type":"object"},"territory":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["territories"],"type":"string"}},"required":["id","type"],"type":"object"}},"type":"object"}},"required":["subscription"],"type":"object"},"type":{"enum":["subscriptionIntroductoryOffers"],"type":"string"}},"required":["relationships","attributes","type"],"type":"object"},"included":{"items":{"properties":{"id":{"type":"string"},"type":{"enum":["subscriptionPricePoints"],"type":"string"}},"required":["type"],"type":"object"},"type":"array"}},"required":["data"],"title":"SubscriptionIntroductoryOfferCreateRequest","type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "POST".to_string(),
                path_template: "/v1/subscriptionIntroductoryOffers".to_string(),
                params: vec![
                ],
                body_fields: vec![
                    "data".to_string(),
                    "included".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "subscriptionIntroductoryOffers_updateInstance".to_string(),
            description: "Update attributes or relationships of one `subscriptionIntroductoryOffers` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"endDate":{"format":"date","nullable":true,"type":"string"}},"type":"object"},"id":{"type":"string"},"type":{"enum":["subscriptionIntroductoryOffers"],"type":"string"}},"required":["id","type"],"type":"object"},"id":{"type":"string"}},"required":["id","data"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "PATCH".to_string(),
                path_template: "/v1/subscriptionIntroductoryOffers/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "subscriptionIntroductoryOffers_deleteInstance".to_string(),
            description: "Delete one `subscriptionIntroductoryOffers` resource. This cannot be undone.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"id":{"type":"string"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "DELETE".to_string(),
                path_template: "/v1/subscriptionIntroductoryOffers/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(true),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "subscriptionLocalizations_createInstance".to_string(),
            description: "Create one `subscriptionLocalizations` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"description":{"nullable":true,"type":"string"},"locale":{"type":"string"},"name":{"type":"string"}},"required":["name","locale"],"type":"object"},"relationships":{"properties":{"subscription":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["subscriptions"],"type":"string"}},"required":["id","type"],"type":"object"}},"required":["data"],"type":"object"}},"required":["subscription"],"type":"object"},"type":{"enum":["subscriptionLocalizations"],"type":"string"}},"required":["relationships","attributes","type"],"type":"object"}},"required":["data"],"title":"SubscriptionLocalizationCreateRequest","type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "POST".to_string(),
                path_template: "/v1/subscriptionLocalizations".to_string(),
                params: vec![
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "subscriptionLocalizations_getInstance".to_string(),
            description: "Get one `subscriptionLocalizations` resource by id.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[subscriptionLocalizations]":{"items":{"type":"string"},"type":"array"},"fields[subscriptions]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["subscription"],"type":"string"},"type":"array"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/subscriptionLocalizations/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[subscriptionLocalizations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[subscriptions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "subscriptionLocalizations_updateInstance".to_string(),
            description: "Update attributes or relationships of one `subscriptionLocalizations` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"description":{"nullable":true,"type":"string"},"name":{"nullable":true,"type":"string"}},"type":"object"},"id":{"type":"string"},"type":{"enum":["subscriptionLocalizations"],"type":"string"}},"required":["id","type"],"type":"object"},"id":{"type":"string"}},"required":["id","data"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "PATCH".to_string(),
                path_template: "/v1/subscriptionLocalizations/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "subscriptionLocalizations_deleteInstance".to_string(),
            description: "Delete one `subscriptionLocalizations` resource. This cannot be undone.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"id":{"type":"string"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "DELETE".to_string(),
                path_template: "/v1/subscriptionLocalizations/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(true),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "subscriptionPlanAvailabilities_createInstance".to_string(),
            description: "Create one `subscriptionPlanAvailabilities` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"availableInNewTerritories":{"nullable":true,"type":"boolean"},"planType":{"enum":["MONTHLY","UPFRONT"],"type":"string"}},"required":["planType"],"type":"object"},"relationships":{"properties":{"availableTerritories":{"properties":{"data":{"items":{"properties":{"id":{"type":"string"},"type":{"enum":["territories"],"type":"string"}},"required":["id","type"],"type":"object"},"type":"array"}},"required":["data"],"type":"object"},"subscription":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["subscriptions"],"type":"string"}},"required":["id","type"],"type":"object"}},"required":["data"],"type":"object"}},"required":["availableTerritories","subscription"],"type":"object"},"type":{"enum":["subscriptionPlanAvailabilities"],"type":"string"}},"required":["relationships","attributes","type"],"type":"object"}},"required":["data"],"title":"SubscriptionPlanAvailabilityCreateRequest","type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "POST".to_string(),
                path_template: "/v1/subscriptionPlanAvailabilities".to_string(),
                params: vec![
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "subscriptionPlanAvailabilities_getInstance".to_string(),
            description: "Get one `subscriptionPlanAvailabilities` resource by id.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[subscriptionPlanAvailabilities]":{"items":{"type":"string"},"type":"array"},"fields[territories]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["availableTerritories"],"type":"string"},"type":"array"},"limit[availableTerritories]":{"maximum":50,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/subscriptionPlanAvailabilities/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[subscriptionPlanAvailabilities]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[territories]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[availableTerritories]".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "subscriptionPlanAvailabilities_updateInstance".to_string(),
            description: "Update attributes or relationships of one `subscriptionPlanAvailabilities` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"availableInNewTerritories":{"nullable":true,"type":"boolean"}},"type":"object"},"id":{"type":"string"},"relationships":{"properties":{"availableTerritories":{"properties":{"data":{"items":{"properties":{"id":{"type":"string"},"type":{"enum":["territories"],"type":"string"}},"required":["id","type"],"type":"object"},"type":"array"}},"type":"object"}},"type":"object"},"type":{"enum":["subscriptionPlanAvailabilities"],"type":"string"}},"required":["id","type"],"type":"object"},"id":{"type":"string"}},"required":["id","data"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "PATCH".to_string(),
                path_template: "/v1/subscriptionPlanAvailabilities/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "subscriptionPricePoints_getInstance".to_string(),
            description: "Get one `subscriptionPricePoints` resource by id.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[subscriptionPricePoints]":{"items":{"type":"string"},"type":"array"},"fields[territories]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["territory"],"type":"string"},"type":"array"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/subscriptionPricePoints/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[subscriptionPricePoints]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[territories]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "subscriptionPrices_createInstance".to_string(),
            description: "Create one `subscriptionPrices` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"planType":{"enum":["MONTHLY","UPFRONT"],"type":"string"},"preserveCurrentPrice":{"nullable":true,"type":"boolean"},"startDate":{"format":"date","nullable":true,"type":"string"}},"type":"object"},"relationships":{"properties":{"subscription":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["subscriptions"],"type":"string"}},"required":["id","type"],"type":"object"}},"required":["data"],"type":"object"},"subscriptionPricePoint":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["subscriptionPricePoints"],"type":"string"}},"required":["id","type"],"type":"object"}},"required":["data"],"type":"object"},"territory":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["territories"],"type":"string"}},"required":["id","type"],"type":"object"}},"type":"object"}},"required":["subscription","subscriptionPricePoint"],"type":"object"},"type":{"enum":["subscriptionPrices"],"type":"string"}},"required":["relationships","type"],"type":"object"}},"required":["data"],"title":"SubscriptionPriceCreateRequest","type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "POST".to_string(),
                path_template: "/v1/subscriptionPrices".to_string(),
                params: vec![
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "subscriptionPrices_deleteInstance".to_string(),
            description: "Delete one `subscriptionPrices` resource. This cannot be undone.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"id":{"type":"string"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "DELETE".to_string(),
                path_template: "/v1/subscriptionPrices/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(true),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "subscriptionPromotionalOffers_createInstance".to_string(),
            description: "Create one `subscriptionPromotionalOffers` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"duration":{"enum":["THREE_DAYS","ONE_WEEK","TWO_WEEKS","ONE_MONTH","TWO_MONTHS","THREE_MONTHS","SIX_MONTHS","ONE_YEAR"],"type":"string"},"name":{"type":"string"},"numberOfPeriods":{"type":"integer"},"offerCode":{"type":"string"},"offerMode":{"enum":["PAY_AS_YOU_GO","PAY_UP_FRONT","FREE_TRIAL"],"type":"string"},"targetSubscriptionPlanType":{"enum":["MONTHLY","UPFRONT"],"type":"string"}},"required":["duration","offerCode","name","numberOfPeriods","offerMode"],"type":"object"},"relationships":{"properties":{"prices":{"properties":{"data":{"items":{"properties":{"id":{"type":"string"},"type":{"enum":["subscriptionPromotionalOfferPrices"],"type":"string"}},"required":["id","type"],"type":"object"},"type":"array"}},"required":["data"],"type":"object"},"subscription":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["subscriptions"],"type":"string"}},"required":["id","type"],"type":"object"}},"required":["data"],"type":"object"}},"required":["subscription","prices"],"type":"object"},"type":{"enum":["subscriptionPromotionalOffers"],"type":"string"}},"required":["relationships","attributes","type"],"type":"object"},"included":{"items":{"properties":{"id":{"type":"string"},"relationships":{"properties":{"subscriptionPricePoint":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["subscriptionPricePoints"],"type":"string"}},"required":["id","type"],"type":"object"}},"type":"object"},"territory":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["territories"],"type":"string"}},"required":["id","type"],"type":"object"}},"type":"object"}},"type":"object"},"type":{"enum":["subscriptionPromotionalOfferPrices"],"type":"string"}},"required":["type"],"type":"object"},"type":"array"}},"required":["data"],"title":"SubscriptionPromotionalOfferCreateRequest","type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "POST".to_string(),
                path_template: "/v1/subscriptionPromotionalOffers".to_string(),
                params: vec![
                ],
                body_fields: vec![
                    "data".to_string(),
                    "included".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "subscriptionPromotionalOffers_getInstance".to_string(),
            description: "Get one `subscriptionPromotionalOffers` resource by id.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[subscriptionPromotionalOfferPrices]":{"items":{"type":"string"},"type":"array"},"fields[subscriptionPromotionalOffers]":{"items":{"type":"string"},"type":"array"},"fields[subscriptions]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["subscription","prices"],"type":"string"},"type":"array"},"limit[prices]":{"maximum":50,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/subscriptionPromotionalOffers/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[subscriptionPromotionalOffers]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[subscriptions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[subscriptionPromotionalOfferPrices]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[prices]".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "subscriptionPromotionalOffers_updateInstance".to_string(),
            description: "Update attributes or relationships of one `subscriptionPromotionalOffers` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"id":{"type":"string"},"relationships":{"properties":{"prices":{"properties":{"data":{"items":{"properties":{"id":{"type":"string"},"type":{"enum":["subscriptionPromotionalOfferPrices"],"type":"string"}},"required":["id","type"],"type":"object"},"type":"array"}},"type":"object"}},"type":"object"},"type":{"enum":["subscriptionPromotionalOffers"],"type":"string"}},"required":["id","type"],"type":"object"},"id":{"type":"string"},"included":{"items":{"properties":{"id":{"type":"string"},"relationships":{"properties":{"subscriptionPricePoint":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["subscriptionPricePoints"],"type":"string"}},"required":["id","type"],"type":"object"}},"type":"object"},"territory":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["territories"],"type":"string"}},"required":["id","type"],"type":"object"}},"type":"object"}},"type":"object"},"type":{"enum":["subscriptionPromotionalOfferPrices"],"type":"string"}},"required":["type"],"type":"object"},"type":"array"}},"required":["id","data"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "PATCH".to_string(),
                path_template: "/v1/subscriptionPromotionalOffers/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                    "data".to_string(),
                    "included".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "subscriptionPromotionalOffers_deleteInstance".to_string(),
            description: "Delete one `subscriptionPromotionalOffers` resource. This cannot be undone.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"id":{"type":"string"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "DELETE".to_string(),
                path_template: "/v1/subscriptionPromotionalOffers/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(true),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "subscriptionSubmissions_createInstance".to_string(),
            description: "Create one `subscriptionSubmissions` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"relationships":{"properties":{"subscription":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["subscriptions"],"type":"string"}},"required":["id","type"],"type":"object"}},"required":["data"],"type":"object"}},"required":["subscription"],"type":"object"},"type":{"enum":["subscriptionSubmissions"],"type":"string"}},"required":["relationships","type"],"type":"object"}},"required":["data"],"title":"SubscriptionSubmissionCreateRequest","type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "POST".to_string(),
                path_template: "/v1/subscriptionSubmissions".to_string(),
                params: vec![
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "subscriptions_createInstance".to_string(),
            description: "Create one `subscriptions` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"familySharable":{"nullable":true,"type":"boolean"},"groupLevel":{"nullable":true,"type":"integer"},"name":{"type":"string"},"productId":{"type":"string"},"reviewNote":{"nullable":true,"type":"string"},"subscriptionPeriod":{"enum":["ONE_WEEK","ONE_MONTH","TWO_MONTHS","THREE_MONTHS","SIX_MONTHS","ONE_YEAR"],"nullable":true,"type":"string"}},"required":["productId","name"],"type":"object"},"relationships":{"properties":{"group":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["subscriptionGroups"],"type":"string"}},"required":["id","type"],"type":"object"}},"required":["data"],"type":"object"}},"required":["group"],"type":"object"},"type":{"enum":["subscriptions"],"type":"string"}},"required":["relationships","attributes","type"],"type":"object"}},"required":["data"],"title":"SubscriptionCreateRequest","type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "POST".to_string(),
                path_template: "/v1/subscriptions".to_string(),
                params: vec![
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "subscriptions_getInstance".to_string(),
            description: "Get one `subscriptions` resource by id.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[promotedPurchases]":{"items":{"type":"string"},"type":"array"},"fields[subscriptionAppStoreReviewScreenshots]":{"items":{"type":"string"},"type":"array"},"fields[subscriptionAvailabilities]":{"items":{"type":"string"},"type":"array"},"fields[subscriptionGroups]":{"items":{"type":"string"},"type":"array"},"fields[subscriptionImages]":{"items":{"type":"string"},"type":"array"},"fields[subscriptionIntroductoryOffers]":{"items":{"type":"string"},"type":"array"},"fields[subscriptionLocalizations]":{"items":{"type":"string"},"type":"array"},"fields[subscriptionOfferCodes]":{"items":{"type":"string"},"type":"array"},"fields[subscriptionPlanAvailabilities]":{"items":{"type":"string"},"type":"array"},"fields[subscriptionPrices]":{"items":{"type":"string"},"type":"array"},"fields[subscriptionPromotionalOffers]":{"items":{"type":"string"},"type":"array"},"fields[subscriptionVersions]":{"items":{"type":"string"},"type":"array"},"fields[subscriptions]":{"items":{"type":"string"},"type":"array"},"fields[winBackOffers]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["subscriptionLocalizations","appStoreReviewScreenshot","group","introductoryOffers","promotionalOffers","offerCodes","prices","promotedPurchase","subscriptionAvailability","winBackOffers","images","planAvailabilities","versions"],"type":"string"},"type":"array"},"limit[images]":{"maximum":50,"type":"integer"},"limit[introductoryOffers]":{"maximum":50,"type":"integer"},"limit[offerCodes]":{"maximum":50,"type":"integer"},"limit[planAvailabilities]":{"maximum":50,"type":"integer"},"limit[prices]":{"maximum":50,"type":"integer"},"limit[promotionalOffers]":{"maximum":50,"type":"integer"},"limit[subscriptionLocalizations]":{"maximum":50,"type":"integer"},"limit[versions]":{"maximum":50,"type":"integer"},"limit[winBackOffers]":{"maximum":50,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/subscriptions/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[subscriptions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[subscriptionLocalizations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[subscriptionAppStoreReviewScreenshots]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[subscriptionGroups]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[subscriptionIntroductoryOffers]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[subscriptionPromotionalOffers]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[subscriptionOfferCodes]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[subscriptionPrices]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[promotedPurchases]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[subscriptionAvailabilities]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[winBackOffers]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[subscriptionImages]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[subscriptionPlanAvailabilities]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[subscriptionVersions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[images]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[introductoryOffers]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[offerCodes]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[planAvailabilities]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[prices]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[promotionalOffers]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[subscriptionLocalizations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[versions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[winBackOffers]".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "subscriptions_updateInstance".to_string(),
            description: "Update attributes or relationships of one `subscriptions` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"familySharable":{"nullable":true,"type":"boolean"},"groupLevel":{"nullable":true,"type":"integer"},"marketSettings":{"items":{"enum":["APPLE_SCHOOL","APP_STORE","APPLE_BUSINESS"],"type":"string"},"nullable":true,"type":"array"},"multiSeatStatus":{"enum":["ENABLED","DISABLED"],"nullable":true,"type":"string"},"name":{"nullable":true,"type":"string"},"reviewNote":{"nullable":true,"type":"string"},"subscriptionPeriod":{"enum":["ONE_WEEK","ONE_MONTH","TWO_MONTHS","THREE_MONTHS","SIX_MONTHS","ONE_YEAR"],"nullable":true,"type":"string"}},"type":"object"},"id":{"type":"string"},"relationships":{"properties":{"introductoryOffers":{"properties":{"data":{"items":{"properties":{"id":{"type":"string"},"type":{"enum":["subscriptionIntroductoryOffers"],"type":"string"}},"required":["id","type"],"type":"object"},"type":"array"}},"type":"object"},"prices":{"properties":{"data":{"items":{"properties":{"id":{"type":"string"},"type":{"enum":["subscriptionPrices"],"type":"string"}},"required":["id","type"],"type":"object"},"type":"array"}},"type":"object"},"promotionalOffers":{"properties":{"data":{"items":{"properties":{"id":{"type":"string"},"type":{"enum":["subscriptionPromotionalOffers"],"type":"string"}},"required":["id","type"],"type":"object"},"type":"array"}},"type":"object"}},"type":"object"},"type":{"enum":["subscriptions"],"type":"string"}},"required":["id","type"],"type":"object"},"id":{"type":"string"},"included":{"items":{"oneOf":[{"properties":{"attributes":{"properties":{"duration":{"enum":["THREE_DAYS","ONE_WEEK","TWO_WEEKS","ONE_MONTH","TWO_MONTHS","THREE_MONTHS","SIX_MONTHS","ONE_YEAR"],"type":"string"},"name":{"type":"string"},"numberOfPeriods":{"type":"integer"},"offerCode":{"type":"string"},"offerMode":{"enum":["PAY_AS_YOU_GO","PAY_UP_FRONT","FREE_TRIAL"],"type":"string"},"targetSubscriptionPlanType":{"enum":["MONTHLY","UPFRONT"],"type":"string"}},"required":["duration","offerCode","name","numberOfPeriods","offerMode"],"type":"object"},"id":{"type":"string"},"relationships":{"properties":{"prices":{"properties":{"data":{"items":{"properties":{"id":{"type":"string"},"type":{"enum":["subscriptionPromotionalOfferPrices"],"type":"string"}},"required":["id","type"],"type":"object"},"type":"array"}},"type":"object"},"subscription":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["subscriptions"],"type":"string"}},"required":["id","type"],"type":"object"}},"type":"object"}},"type":"object"},"type":{"enum":["subscriptionPromotionalOffers"],"type":"string"}},"required":["attributes","type"],"type":"object"},{"properties":{"attributes":{"properties":{"planType":{"enum":["MONTHLY","UPFRONT"],"type":"string"},"preserveCurrentPrice":{"nullable":true,"type":"boolean"},"startDate":{"format":"date","nullable":true,"type":"string"}},"type":"object"},"id":{"type":"string"},"relationships":{"properties":{"subscription":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["subscriptions"],"type":"string"}},"required":["id","type"],"type":"object"}},"type":"object"},"subscriptionPricePoint":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["subscriptionPricePoints"],"type":"string"}},"required":["id","type"],"type":"object"}},"type":"object"},"territory":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["territories"],"type":"string"}},"required":["id","type"],"type":"object"}},"type":"object"}},"type":"object"},"type":{"enum":["subscriptionPrices"],"type":"string"}},"required":["type"],"type":"object"},{"properties":{"attributes":{"properties":{"duration":{"enum":["THREE_DAYS","ONE_WEEK","TWO_WEEKS","ONE_MONTH","TWO_MONTHS","THREE_MONTHS","SIX_MONTHS","ONE_YEAR"],"type":"string"},"endDate":{"format":"date","nullable":true,"type":"string"},"numberOfPeriods":{"type":"integer"},"offerMode":{"enum":["PAY_AS_YOU_GO","PAY_UP_FRONT","FREE_TRIAL"],"type":"string"},"startDate":{"format":"date","nullable":true,"type":"string"},"targetSubscriptionPlanType":{"enum":["MONTHLY","UPFRONT"],"type":"string"}},"required":["duration","numberOfPeriods","offerMode"],"type":"object"},"id":{"type":"string"},"relationships":{"properties":{"subscription":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["subscriptions"],"type":"string"}},"required":["id","type"],"type":"object"}},"type":"object"},"subscriptionPricePoint":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["subscriptionPricePoints"],"type":"string"}},"required":["id","type"],"type":"object"}},"type":"object"},"territory":{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["territories"],"type":"string"}},"required":["id","type"],"type":"object"}},"type":"object"}},"type":"object"},"type":{"enum":["subscriptionIntroductoryOffers"],"type":"string"}},"required":["attributes","type"],"type":"object"}]},"type":"array"}},"required":["id","data"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "PATCH".to_string(),
                path_template: "/v1/subscriptions/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                    "data".to_string(),
                    "included".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "subscriptions_deleteInstance".to_string(),
            description: "Delete one `subscriptions` resource. This cannot be undone.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"id":{"type":"string"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "DELETE".to_string(),
                path_template: "/v1/subscriptions/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(true),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "territories_getCollection".to_string(),
            description: "List App Store territories and their currencies; use these IDs instead of guessing territory codes.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[territories]":{"items":{"type":"string"},"type":"array"},"limit":{"maximum":200,"type":"integer"}},"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/territories".to_string(),
                params: vec![
                    ParamBinding {
                        name: "fields[territories]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "territoryAvailabilities_updateInstance".to_string(),
            description: "Update attributes or relationships of one `territoryAvailabilities` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"attributes":{"properties":{"available":{"nullable":true,"type":"boolean"},"preOrderEnabled":{"nullable":true,"type":"boolean"},"releaseDate":{"format":"date","nullable":true,"type":"string"}},"type":"object"},"id":{"type":"string"},"type":{"enum":["territoryAvailabilities"],"type":"string"}},"required":["id","type"],"type":"object"},"id":{"type":"string"}},"required":["id","data"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "PATCH".to_string(),
                path_template: "/v1/territoryAvailabilities/{id}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "analyticsReportInstances_segments_getToManyRelated".to_string(),
            description: "List the `segments` related to one `analyticsReportInstances` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[analyticsReportSegments]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"limit":{"maximum":200,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/analyticsReportInstances/{id}/segments".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[analyticsReportSegments]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "analyticsReportRequests_reports_getToManyRelated".to_string(),
            description: "List the `reports` related to one `analyticsReportRequests` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[analyticsReports]":{"items":{"type":"string"},"type":"array"},"filter[category]":{"items":{"enum":["APP_USAGE","APP_STORE_ENGAGEMENT","COMMERCE","FRAMEWORK_USAGE","PERFORMANCE"],"type":"string"},"type":"array"},"filter[name]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"limit":{"maximum":200,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/analyticsReportRequests/{id}/reports".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "filter[name]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[category]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[analyticsReports]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "analyticsReports_instances_getToManyRelated".to_string(),
            description: "List the `instances` related to one `analyticsReports` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[analyticsReportInstances]":{"items":{"type":"string"},"type":"array"},"filter[granularity]":{"items":{"enum":["DAILY","WEEKLY","MONTHLY"],"type":"string"},"type":"array"},"filter[processingDate]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"limit":{"maximum":200,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/analyticsReports/{id}/instances".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "filter[granularity]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[processingDate]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[analyticsReportInstances]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appAvailabilitiesV2_territoryAvailabilities_getToManyRelationship".to_string(),
            description: "Get the IDs of the `territoryAvailabilities` linked to one `appAvailabilities` resource (linkage only).".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"id":{"type":"string"},"limit":{"maximum":200,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v2/appAvailabilities/{id}/relationships/territoryAvailabilities".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appAvailabilitiesV2_territoryAvailabilities_getToManyRelated".to_string(),
            description: "List the `territoryAvailabilities` related to one `appAvailabilities` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[territories]":{"items":{"type":"string"},"type":"array"},"fields[territoryAvailabilities]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["territory"],"type":"string"},"type":"array"},"limit":{"maximum":200,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v2/appAvailabilities/{id}/territoryAvailabilities".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[territoryAvailabilities]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[territories]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appCategories_subcategories_getToManyRelated".to_string(),
            description: "List the `subcategories` related to one `appCategories` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[appCategories]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"limit":{"maximum":200,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/appCategories/{id}/subcategories".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[appCategories]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appInfos_ageRatingDeclaration_getToOneRelated".to_string(),
            description: "List the `ageRatingDeclaration` related to one `appInfos` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[ageRatingDeclarations]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/appInfos/{id}/ageRatingDeclaration".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[ageRatingDeclarations]".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appInfos_appInfoLocalizations_getToManyRelated".to_string(),
            description: "List the `appInfoLocalizations` related to one `appInfos` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[appInfoLocalizations]":{"items":{"type":"string"},"type":"array"},"fields[appInfos]":{"items":{"type":"string"},"type":"array"},"filter[locale]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["appInfo"],"type":"string"},"type":"array"},"limit":{"maximum":200,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/appInfos/{id}/appInfoLocalizations".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "filter[locale]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appInfoLocalizations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appInfos]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appPreviewSets_appPreviews_replaceToManyRelationship".to_string(),
            description: "Reorder the previews in a set: send every preview ID in the desired order.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"items":{"properties":{"id":{"type":"string"},"type":{"enum":["appPreviews"],"type":"string"}},"required":["id","type"],"type":"object"},"type":"array"},"id":{"type":"string"}},"required":["id","data"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "PATCH".to_string(),
                path_template: "/v1/appPreviewSets/{id}/relationships/appPreviews".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appPreviewSets_appPreviews_getToManyRelated".to_string(),
            description: "List the `appPreviews` related to one `appPreviewSets` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[appPreviewSets]":{"items":{"type":"string"},"type":"array"},"fields[appPreviews]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["appPreviewSet"],"type":"string"},"type":"array"},"limit":{"maximum":200,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/appPreviewSets/{id}/appPreviews".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[appPreviews]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appPreviewSets]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appPricePointsV3_equalizations_getToManyRelated".to_string(),
            description: "List the `equalizations` related to one `appPricePoints` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[appPricePoints]":{"items":{"type":"string"},"type":"array"},"fields[apps]":{"items":{"type":"string"},"type":"array"},"fields[territories]":{"items":{"type":"string"},"type":"array"},"filter[territory]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["app","territory"],"type":"string"},"type":"array"},"limit":{"maximum":200,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v3/appPricePoints/{id}/equalizations".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "filter[territory]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appPricePoints]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[apps]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[territories]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appPriceSchedules_automaticPrices_getToManyRelated".to_string(),
            description: "List the `automaticPrices` related to one `appPriceSchedules` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[appPricePoints]":{"items":{"type":"string"},"type":"array"},"fields[appPrices]":{"items":{"type":"string"},"type":"array"},"fields[territories]":{"items":{"type":"string"},"type":"array"},"filter[endDate]":{"items":{"type":"string"},"type":"array"},"filter[startDate]":{"items":{"type":"string"},"type":"array"},"filter[territory]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["appPricePoint","territory"],"type":"string"},"type":"array"},"limit":{"maximum":200,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/appPriceSchedules/{id}/automaticPrices".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "filter[startDate]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[endDate]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[territory]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appPrices]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appPricePoints]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[territories]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appPriceSchedules_baseTerritory_getToOneRelated".to_string(),
            description: "List the `baseTerritory` related to one `appPriceSchedules` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[territories]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/appPriceSchedules/{id}/baseTerritory".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[territories]".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appPriceSchedules_manualPrices_getToManyRelated".to_string(),
            description: "List the `manualPrices` related to one `appPriceSchedules` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[appPricePoints]":{"items":{"type":"string"},"type":"array"},"fields[appPrices]":{"items":{"type":"string"},"type":"array"},"fields[territories]":{"items":{"type":"string"},"type":"array"},"filter[endDate]":{"items":{"type":"string"},"type":"array"},"filter[startDate]":{"items":{"type":"string"},"type":"array"},"filter[territory]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["appPricePoint","territory"],"type":"string"},"type":"array"},"limit":{"maximum":200,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/appPriceSchedules/{id}/manualPrices".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "filter[startDate]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[endDate]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[territory]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appPrices]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appPricePoints]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[territories]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appScreenshotSets_appScreenshots_replaceToManyRelationship".to_string(),
            description: "Reorder the screenshots in a set: send every screenshot ID in the desired order.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"items":{"properties":{"id":{"type":"string"},"type":{"enum":["appScreenshots"],"type":"string"}},"required":["id","type"],"type":"object"},"type":"array"},"id":{"type":"string"}},"required":["id","data"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "PATCH".to_string(),
                path_template: "/v1/appScreenshotSets/{id}/relationships/appScreenshots".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appScreenshotSets_appScreenshots_getToManyRelated".to_string(),
            description: "List the `appScreenshots` related to one `appScreenshotSets` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[appScreenshotSets]":{"items":{"type":"string"},"type":"array"},"fields[appScreenshots]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["appScreenshotSet"],"type":"string"},"type":"array"},"limit":{"maximum":200,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/appScreenshotSets/{id}/appScreenshots".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[appScreenshots]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appScreenshotSets]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appStoreReviewDetails_appStoreReviewAttachments_getToManyRelated".to_string(),
            description: "List the `appStoreReviewAttachments` related to one `appStoreReviewDetails` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[appStoreReviewAttachments]":{"items":{"type":"string"},"type":"array"},"fields[appStoreReviewDetails]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["appStoreReviewDetail"],"type":"string"},"type":"array"},"limit":{"maximum":200,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/appStoreReviewDetails/{id}/appStoreReviewAttachments".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[appStoreReviewAttachments]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appStoreReviewDetails]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appStoreVersionLocalizations_appPreviewSets_getToManyRelated".to_string(),
            description: "List the `appPreviewSets` related to one `appStoreVersionLocalizations` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[appCustomProductPageLocalizations]":{"items":{"type":"string"},"type":"array"},"fields[appPreviewSets]":{"items":{"type":"string"},"type":"array"},"fields[appPreviews]":{"items":{"type":"string"},"type":"array"},"fields[appStoreVersionExperimentTreatmentLocalizations]":{"items":{"type":"string"},"type":"array"},"fields[appStoreVersionLocalizations]":{"items":{"type":"string"},"type":"array"},"filter[appCustomProductPageLocalization]":{"items":{"type":"string"},"type":"array"},"filter[appStoreVersionExperimentTreatmentLocalization]":{"items":{"type":"string"},"type":"array"},"filter[previewType]":{"items":{"enum":["IPHONE_67","IPHONE_61","IPHONE_65","IPHONE_58","IPHONE_55","IPHONE_47","IPHONE_40","IPHONE_35","IPAD_PRO_3GEN_129","IPAD_PRO_3GEN_11","IPAD_PRO_129","IPAD_105","IPAD_97","DESKTOP","APPLE_TV","APPLE_VISION_PRO"],"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["appStoreVersionLocalization","appCustomProductPageLocalization","appStoreVersionExperimentTreatmentLocalization","appPreviews"],"type":"string"},"type":"array"},"limit":{"maximum":200,"type":"integer"},"limit[appPreviews]":{"maximum":50,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/appStoreVersionLocalizations/{id}/appPreviewSets".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "filter[previewType]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[appCustomProductPageLocalization]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[appStoreVersionExperimentTreatmentLocalization]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appPreviewSets]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appStoreVersionLocalizations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appCustomProductPageLocalizations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appStoreVersionExperimentTreatmentLocalizations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appPreviews]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[appPreviews]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appStoreVersionLocalizations_appScreenshotSets_getToManyRelated".to_string(),
            description: "List the `appScreenshotSets` related to one `appStoreVersionLocalizations` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[appCustomProductPageLocalizations]":{"items":{"type":"string"},"type":"array"},"fields[appScreenshotSets]":{"items":{"type":"string"},"type":"array"},"fields[appScreenshots]":{"items":{"type":"string"},"type":"array"},"fields[appStoreVersionExperimentTreatmentLocalizations]":{"items":{"type":"string"},"type":"array"},"fields[appStoreVersionLocalizations]":{"items":{"type":"string"},"type":"array"},"filter[appCustomProductPageLocalization]":{"items":{"type":"string"},"type":"array"},"filter[appStoreVersionExperimentTreatmentLocalization]":{"items":{"type":"string"},"type":"array"},"filter[screenshotDisplayType]":{"items":{"enum":["APP_IPHONE_67","APP_IPHONE_61","APP_IPHONE_65","APP_IPHONE_58","APP_IPHONE_55","APP_IPHONE_47","APP_IPHONE_40","APP_IPHONE_35","APP_IPAD_PRO_3GEN_129","APP_IPAD_PRO_3GEN_11","APP_IPAD_PRO_129","APP_IPAD_105","APP_IPAD_97","APP_DESKTOP","APP_WATCH_ULTRA","APP_WATCH_SERIES_10","APP_WATCH_SERIES_7","APP_WATCH_SERIES_4","APP_WATCH_SERIES_3","APP_APPLE_TV","APP_APPLE_VISION_PRO","IMESSAGE_APP_IPHONE_67","IMESSAGE_APP_IPHONE_61","IMESSAGE_APP_IPHONE_65","IMESSAGE_APP_IPHONE_58","IMESSAGE_APP_IPHONE_55","IMESSAGE_APP_IPHONE_47","IMESSAGE_APP_IPHONE_40","IMESSAGE_APP_IPAD_PRO_3GEN_129","IMESSAGE_APP_IPAD_PRO_3GEN_11","IMESSAGE_APP_IPAD_PRO_129","IMESSAGE_APP_IPAD_105","IMESSAGE_APP_IPAD_97"],"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["appStoreVersionLocalization","appCustomProductPageLocalization","appStoreVersionExperimentTreatmentLocalization","appScreenshots"],"type":"string"},"type":"array"},"limit":{"maximum":200,"type":"integer"},"limit[appScreenshots]":{"maximum":50,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/appStoreVersionLocalizations/{id}/appScreenshotSets".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "filter[screenshotDisplayType]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[appCustomProductPageLocalization]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[appStoreVersionExperimentTreatmentLocalization]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appScreenshotSets]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appStoreVersionLocalizations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appCustomProductPageLocalizations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appStoreVersionExperimentTreatmentLocalizations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appScreenshots]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[appScreenshots]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appStoreVersions_appStoreReviewDetail_getToOneRelated".to_string(),
            description: "List the `appStoreReviewDetail` related to one `appStoreVersions` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[appStoreReviewAttachments]":{"items":{"type":"string"},"type":"array"},"fields[appStoreReviewDetails]":{"items":{"type":"string"},"type":"array"},"fields[appStoreVersions]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["appStoreVersion","appStoreReviewAttachments"],"type":"string"},"type":"array"},"limit[appStoreReviewAttachments]":{"maximum":50,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/appStoreVersions/{id}/appStoreReviewDetail".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[appStoreReviewDetails]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appStoreVersions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appStoreReviewAttachments]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[appStoreReviewAttachments]".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appStoreVersions_appStoreVersionLocalizations_getToManyRelated".to_string(),
            description: "List the `appStoreVersionLocalizations` related to one `appStoreVersions` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[appKeywords]":{"items":{"type":"string"},"type":"array"},"fields[appPreviewSets]":{"items":{"type":"string"},"type":"array"},"fields[appScreenshotSets]":{"items":{"type":"string"},"type":"array"},"fields[appStoreVersionLocalizations]":{"items":{"type":"string"},"type":"array"},"fields[appStoreVersions]":{"items":{"type":"string"},"type":"array"},"filter[locale]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["appStoreVersion","appScreenshotSets","appPreviewSets","searchKeywords"],"type":"string"},"type":"array"},"limit":{"maximum":200,"type":"integer"},"limit[appPreviewSets]":{"maximum":50,"type":"integer"},"limit[appScreenshotSets]":{"maximum":50,"type":"integer"},"limit[searchKeywords]":{"maximum":50,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/appStoreVersions/{id}/appStoreVersionLocalizations".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "filter[locale]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appStoreVersionLocalizations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appStoreVersions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appScreenshotSets]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appPreviewSets]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appKeywords]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[appScreenshotSets]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[appPreviewSets]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[searchKeywords]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appStoreVersions_appStoreVersionPhasedRelease_getToOneRelated".to_string(),
            description: "List the `appStoreVersionPhasedRelease` related to one `appStoreVersions` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[appStoreVersionPhasedReleases]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/appStoreVersions/{id}/appStoreVersionPhasedRelease".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[appStoreVersionPhasedReleases]".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appStoreVersions_build_updateToOneRelationship".to_string(),
            description: "Attach a processed build to an App Store version (body: `data: {type: builds, id}`). Use a build with `processingState=VALID`.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["builds"],"type":"string"}},"required":["id","type"],"type":"object"},"id":{"type":"string"}},"required":["id","data"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "PATCH".to_string(),
                path_template: "/v1/appStoreVersions/{id}/relationships/build".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appStoreVersions_build_getToOneRelated".to_string(),
            description: "List the `build` related to one `appStoreVersions` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[builds]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/appStoreVersions/{id}/build".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[builds]".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "appStoreVersions_customerReviews_getToManyRelated".to_string(),
            description: "List the `customerReviews` related to one `appStoreVersions` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"exists[publishedResponse]":{"type":"boolean"},"fields[customerReviewResponses]":{"items":{"type":"string"},"type":"array"},"fields[customerReviews]":{"items":{"type":"string"},"type":"array"},"fields[territories]":{"items":{"type":"string"},"type":"array"},"filter[rating]":{"items":{"type":"string"},"type":"array"},"filter[reviewTerritory]":{"items":{"type":"string"},"type":"array"},"filter[territory]":{"items":{"enum":["ABW","AFG","AGO","AIA","ALB","AND","ANT","ARE","ARG","ARM","ASM","ATG","AUS","AUT","AZE","BDI","BEL","BEN","BES","BFA","BGD","BGR","BHR","BHS","BIH","BLR","BLZ","BMU","BOL","BRA","BRB","BRN","BTN","BWA","CAF","CAN","CHE","CHL","CHN","CIV","CMR","COD","COG","COK","COL","COM","CPV","CRI","CUB","CUW","CXR","CYM","CYP","CZE","DEU","DJI","DMA","DNK","DOM","DZA","ECU","EGY","ERI","ESP","EST","ETH","FIN","FJI","FLK","FRA","FRO","FSM","GAB","GBR","GEO","GGY","GHA","GIB","GIN","GLP","GMB","GNB","GNQ","GRC","GRD","GRL","GTM","GUF","GUM","GUY","HKG","HND","HRV","HTI","HUN","IDN","IMN","IND","IRL","IRQ","ISL","ISR","ITA","JAM","JEY","JOR","JPN","KAZ","KEN","KGZ","KHM","KIR","KNA","KOR","KWT","LAO","LBN","LBR","LBY","LCA","LIE","LKA","LSO","LTU","LUX","LVA","MAC","MAR","MCO","MDA","MDG","MDV","MEX","MHL","MKD","MLI","MLT","MMR","MNE","MNG","MNP","MOZ","MRT","MSR","MTQ","MUS","MWI","MYS","MYT","NAM","NCL","NER","NFK","NGA","NIC","NIU","NLD","NOR","NPL","NRU","NZL","OMN","PAK","PAN","PER","PHL","PLW","PNG","POL","PRI","PRT","PRY","PSE","PYF","QAT","REU","ROU","RUS","RWA","SAU","SEN","SGP","SHN","SLB","SLE","SLV","SMR","SOM","SPM","SRB","SSD","STP","SUR","SVK","SVN","SWE","SWZ","SXM","SYC","TCA","TCD","TGO","THA","TJK","TKM","TLS","TON","TTO","TUN","TUR","TUV","TWN","TZA","UGA","UKR","UMI","URY","USA","UZB","VAT","VCT","VEN","VGB","VIR","VNM","VUT","WLF","WSM","XKS","YEM","ZAF","ZMB","ZWE"],"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["response","reviewTerritory"],"type":"string"},"type":"array"},"limit":{"maximum":200,"type":"integer"},"sort":{"items":{"enum":["rating","-rating","createdDate","-createdDate"],"type":"string"},"type":"array"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/appStoreVersions/{id}/customerReviews".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "filter[territory]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[rating]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[reviewTerritory]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "exists[publishedResponse]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "sort".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[customerReviews]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[customerReviewResponses]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[territories]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "apps_analyticsReportRequests_getToManyRelated".to_string(),
            description: "List the `analyticsReportRequests` related to one `apps` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[analyticsReportRequests]":{"items":{"type":"string"},"type":"array"},"fields[analyticsReports]":{"items":{"type":"string"},"type":"array"},"filter[accessType]":{"items":{"enum":["ONE_TIME_SNAPSHOT","ONGOING"],"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["reports"],"type":"string"},"type":"array"},"limit":{"maximum":200,"type":"integer"},"limit[reports]":{"maximum":50,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/apps/{id}/analyticsReportRequests".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "filter[accessType]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[analyticsReportRequests]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[analyticsReports]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[reports]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "apps_appAvailabilityV2_getToOneRelated".to_string(),
            description: "List the `appAvailabilityV2` related to one `apps` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[appAvailabilities]":{"items":{"type":"string"},"type":"array"},"fields[territoryAvailabilities]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["territoryAvailabilities"],"type":"string"},"type":"array"},"limit[territoryAvailabilities]":{"maximum":50,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/apps/{id}/appAvailabilityV2".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[appAvailabilities]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[territoryAvailabilities]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[territoryAvailabilities]".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "apps_appEncryptionDeclarations_getToManyRelated".to_string(),
            description: "List the `appEncryptionDeclarations` related to one `apps` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[appEncryptionDeclarationDocuments]":{"items":{"type":"string"},"type":"array"},"fields[appEncryptionDeclarations]":{"items":{"type":"string"},"type":"array"},"fields[apps]":{"items":{"type":"string"},"type":"array"},"fields[builds]":{"items":{"type":"string"},"type":"array"},"filter[builds]":{"items":{"type":"string"},"type":"array"},"filter[platform]":{"items":{"enum":["IOS","MAC_OS","TV_OS","VISION_OS"],"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["app","builds","appEncryptionDeclarationDocument"],"type":"string"},"type":"array"},"limit":{"maximum":200,"type":"integer"},"limit[builds]":{"maximum":50,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/apps/{id}/appEncryptionDeclarations".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "filter[platform]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[builds]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appEncryptionDeclarations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[apps]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[builds]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appEncryptionDeclarationDocuments]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[builds]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "apps_appInfos_getToManyRelated".to_string(),
            description: "List the `appInfos` related to one `apps` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[ageRatingDeclarations]":{"items":{"type":"string"},"type":"array"},"fields[appCategories]":{"items":{"type":"string"},"type":"array"},"fields[appInfoLocalizations]":{"items":{"type":"string"},"type":"array"},"fields[appInfos]":{"items":{"type":"string"},"type":"array"},"fields[apps]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["app","ageRatingDeclaration","appInfoLocalizations","primaryCategory","primarySubcategoryOne","primarySubcategoryTwo","secondaryCategory","secondarySubcategoryOne","secondarySubcategoryTwo"],"type":"string"},"type":"array"},"limit":{"maximum":200,"type":"integer"},"limit[appInfoLocalizations]":{"maximum":50,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/apps/{id}/appInfos".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[appInfos]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[apps]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[ageRatingDeclarations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appInfoLocalizations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appCategories]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[appInfoLocalizations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "apps_appPricePoints_getToManyRelated".to_string(),
            description: "List the `appPricePoints` related to one `apps` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[appPricePoints]":{"items":{"type":"string"},"type":"array"},"fields[apps]":{"items":{"type":"string"},"type":"array"},"fields[territories]":{"items":{"type":"string"},"type":"array"},"filter[territory]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["app","territory"],"type":"string"},"type":"array"},"limit":{"maximum":200,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/apps/{id}/appPricePoints".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "filter[territory]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appPricePoints]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[apps]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[territories]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "apps_appPriceSchedule_getToOneRelated".to_string(),
            description: "List the `appPriceSchedule` related to one `apps` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[appPriceSchedules]":{"items":{"type":"string"},"type":"array"},"fields[appPrices]":{"items":{"type":"string"},"type":"array"},"fields[apps]":{"items":{"type":"string"},"type":"array"},"fields[territories]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["app","baseTerritory","manualPrices","automaticPrices"],"type":"string"},"type":"array"},"limit[automaticPrices]":{"maximum":50,"type":"integer"},"limit[manualPrices]":{"maximum":50,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/apps/{id}/appPriceSchedule".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[appPriceSchedules]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[apps]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[territories]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appPrices]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[manualPrices]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[automaticPrices]".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "apps_appStoreVersions_getToManyRelated".to_string(),
            description: "List the `appStoreVersions` related to one `apps` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[alternativeDistributionPackages]":{"items":{"type":"string"},"type":"array"},"fields[appClipDefaultExperiences]":{"items":{"type":"string"},"type":"array"},"fields[appStoreReviewDetails]":{"items":{"type":"string"},"type":"array"},"fields[appStoreVersionExperiments]":{"items":{"type":"string"},"type":"array"},"fields[appStoreVersionLocalizations]":{"items":{"type":"string"},"type":"array"},"fields[appStoreVersionPhasedReleases]":{"items":{"type":"string"},"type":"array"},"fields[appStoreVersionSubmissions]":{"items":{"type":"string"},"type":"array"},"fields[appStoreVersions]":{"items":{"type":"string"},"type":"array"},"fields[apps]":{"items":{"type":"string"},"type":"array"},"fields[builds]":{"items":{"type":"string"},"type":"array"},"fields[gameCenterAppVersions]":{"items":{"type":"string"},"type":"array"},"fields[routingAppCoverages]":{"items":{"type":"string"},"type":"array"},"filter[appStoreState]":{"items":{"enum":["ACCEPTED","DEVELOPER_REMOVED_FROM_SALE","DEVELOPER_REJECTED","IN_REVIEW","INVALID_BINARY","METADATA_REJECTED","PENDING_APPLE_RELEASE","PENDING_CONTRACT","PENDING_DEVELOPER_RELEASE","PREPARE_FOR_SUBMISSION","PREORDER_READY_FOR_SALE","PROCESSING_FOR_APP_STORE","READY_FOR_REVIEW","READY_FOR_SALE","REJECTED","REMOVED_FROM_SALE","WAITING_FOR_EXPORT_COMPLIANCE","WAITING_FOR_REVIEW","REPLACED_WITH_NEW_VERSION","NOT_APPLICABLE"],"type":"string"},"type":"array"},"filter[appVersionState]":{"items":{"enum":["ACCEPTED","DEVELOPER_REJECTED","IN_REVIEW","INVALID_BINARY","METADATA_REJECTED","PENDING_APPLE_RELEASE","PENDING_DEVELOPER_RELEASE","PREPARE_FOR_SUBMISSION","PROCESSING_FOR_DISTRIBUTION","READY_FOR_DISTRIBUTION","READY_FOR_REVIEW","REJECTED","REPLACED_WITH_NEW_VERSION","WAITING_FOR_EXPORT_COMPLIANCE","WAITING_FOR_REVIEW"],"type":"string"},"type":"array"},"filter[id]":{"items":{"type":"string"},"type":"array"},"filter[platform]":{"items":{"enum":["IOS","MAC_OS","TV_OS","VISION_OS"],"type":"string"},"type":"array"},"filter[versionString]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["app","appStoreVersionLocalizations","build","appStoreVersionPhasedRelease","gameCenterAppVersion","routingAppCoverage","appStoreReviewDetail","appStoreVersionSubmission","appClipDefaultExperience","appStoreVersionExperiments","appStoreVersionExperimentsV2","alternativeDistributionPackage"],"type":"string"},"type":"array"},"limit":{"maximum":200,"type":"integer"},"limit[appStoreVersionExperimentsV2]":{"maximum":50,"type":"integer"},"limit[appStoreVersionExperiments]":{"maximum":50,"type":"integer"},"limit[appStoreVersionLocalizations]":{"maximum":50,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/apps/{id}/appStoreVersions".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "filter[platform]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[versionString]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[appStoreState]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[appVersionState]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[id]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appStoreVersions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[apps]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appStoreVersionLocalizations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[builds]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appStoreVersionPhasedReleases]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[gameCenterAppVersions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[routingAppCoverages]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appStoreReviewDetails]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appStoreVersionSubmissions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appClipDefaultExperiences]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appStoreVersionExperiments]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[alternativeDistributionPackages]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[appStoreVersionLocalizations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[appStoreVersionExperiments]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[appStoreVersionExperimentsV2]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "apps_betaAppLocalizations_getToManyRelated".to_string(),
            description: "List the `betaAppLocalizations` related to one `apps` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[betaAppLocalizations]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"limit":{"maximum":200,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/apps/{id}/betaAppLocalizations".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[betaAppLocalizations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "apps_betaAppReviewDetail_getToOneRelated".to_string(),
            description: "List the `betaAppReviewDetail` related to one `apps` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[betaAppReviewDetails]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/apps/{id}/betaAppReviewDetail".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[betaAppReviewDetails]".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "apps_betaGroups_getToManyRelated".to_string(),
            description: "List the `betaGroups` related to one `apps` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[betaGroups]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"limit":{"maximum":200,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/apps/{id}/betaGroups".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[betaGroups]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "apps_betaLicenseAgreement_getToOneRelated".to_string(),
            description: "List the `betaLicenseAgreement` related to one `apps` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[betaLicenseAgreements]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/apps/{id}/betaLicenseAgreement".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[betaLicenseAgreements]".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "apps_builds_getToManyRelated".to_string(),
            description: "List the `builds` related to one `apps` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[builds]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"limit":{"maximum":200,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/apps/{id}/builds".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[builds]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "apps_customerReviews_getToManyRelated".to_string(),
            description: "List the `customerReviews` related to one `apps` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"exists[publishedResponse]":{"type":"boolean"},"fields[customerReviewResponses]":{"items":{"type":"string"},"type":"array"},"fields[customerReviews]":{"items":{"type":"string"},"type":"array"},"fields[territories]":{"items":{"type":"string"},"type":"array"},"filter[rating]":{"items":{"type":"string"},"type":"array"},"filter[reviewTerritory]":{"items":{"type":"string"},"type":"array"},"filter[territory]":{"items":{"enum":["ABW","AFG","AGO","AIA","ALB","AND","ANT","ARE","ARG","ARM","ASM","ATG","AUS","AUT","AZE","BDI","BEL","BEN","BES","BFA","BGD","BGR","BHR","BHS","BIH","BLR","BLZ","BMU","BOL","BRA","BRB","BRN","BTN","BWA","CAF","CAN","CHE","CHL","CHN","CIV","CMR","COD","COG","COK","COL","COM","CPV","CRI","CUB","CUW","CXR","CYM","CYP","CZE","DEU","DJI","DMA","DNK","DOM","DZA","ECU","EGY","ERI","ESP","EST","ETH","FIN","FJI","FLK","FRA","FRO","FSM","GAB","GBR","GEO","GGY","GHA","GIB","GIN","GLP","GMB","GNB","GNQ","GRC","GRD","GRL","GTM","GUF","GUM","GUY","HKG","HND","HRV","HTI","HUN","IDN","IMN","IND","IRL","IRQ","ISL","ISR","ITA","JAM","JEY","JOR","JPN","KAZ","KEN","KGZ","KHM","KIR","KNA","KOR","KWT","LAO","LBN","LBR","LBY","LCA","LIE","LKA","LSO","LTU","LUX","LVA","MAC","MAR","MCO","MDA","MDG","MDV","MEX","MHL","MKD","MLI","MLT","MMR","MNE","MNG","MNP","MOZ","MRT","MSR","MTQ","MUS","MWI","MYS","MYT","NAM","NCL","NER","NFK","NGA","NIC","NIU","NLD","NOR","NPL","NRU","NZL","OMN","PAK","PAN","PER","PHL","PLW","PNG","POL","PRI","PRT","PRY","PSE","PYF","QAT","REU","ROU","RUS","RWA","SAU","SEN","SGP","SHN","SLB","SLE","SLV","SMR","SOM","SPM","SRB","SSD","STP","SUR","SVK","SVN","SWE","SWZ","SXM","SYC","TCA","TCD","TGO","THA","TJK","TKM","TLS","TON","TTO","TUN","TUR","TUV","TWN","TZA","UGA","UKR","UMI","URY","USA","UZB","VAT","VCT","VEN","VGB","VIR","VNM","VUT","WLF","WSM","XKS","YEM","ZAF","ZMB","ZWE"],"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["response","reviewTerritory"],"type":"string"},"type":"array"},"limit":{"maximum":200,"type":"integer"},"sort":{"items":{"enum":["rating","-rating","createdDate","-createdDate"],"type":"string"},"type":"array"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/apps/{id}/customerReviews".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "filter[territory]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[rating]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[reviewTerritory]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "exists[publishedResponse]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "sort".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[customerReviews]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[customerReviewResponses]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[territories]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "apps_inAppPurchasesV2_getToManyRelated".to_string(),
            description: "List the `inAppPurchasesV2` related to one `apps` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[inAppPurchaseAppStoreReviewScreenshots]":{"items":{"type":"string"},"type":"array"},"fields[inAppPurchaseAvailabilities]":{"items":{"type":"string"},"type":"array"},"fields[inAppPurchaseContents]":{"items":{"type":"string"},"type":"array"},"fields[inAppPurchaseImages]":{"items":{"type":"string"},"type":"array"},"fields[inAppPurchaseLocalizations]":{"items":{"type":"string"},"type":"array"},"fields[inAppPurchaseOfferCodes]":{"items":{"type":"string"},"type":"array"},"fields[inAppPurchasePriceSchedules]":{"items":{"type":"string"},"type":"array"},"fields[inAppPurchaseVersions]":{"items":{"type":"string"},"type":"array"},"fields[inAppPurchases]":{"items":{"type":"string"},"type":"array"},"fields[promotedPurchases]":{"items":{"type":"string"},"type":"array"},"filter[inAppPurchaseType]":{"items":{"enum":["CONSUMABLE","NON_CONSUMABLE","NON_RENEWING_SUBSCRIPTION"],"type":"string"},"type":"array"},"filter[name]":{"items":{"type":"string"},"type":"array"},"filter[productId]":{"items":{"type":"string"},"type":"array"},"filter[state]":{"items":{"enum":["MISSING_METADATA","WAITING_FOR_UPLOAD","PROCESSING_CONTENT","READY_TO_SUBMIT","WAITING_FOR_REVIEW","IN_REVIEW","DEVELOPER_ACTION_NEEDED","PENDING_BINARY_APPROVAL","APPROVED","DEVELOPER_REMOVED_FROM_SALE","REMOVED_FROM_SALE","REJECTED"],"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["inAppPurchaseLocalizations","content","appStoreReviewScreenshot","promotedPurchase","iapPriceSchedule","inAppPurchaseAvailability","images","offerCodes","versions"],"type":"string"},"type":"array"},"limit":{"maximum":200,"type":"integer"},"limit[images]":{"maximum":50,"type":"integer"},"limit[inAppPurchaseLocalizations]":{"maximum":50,"type":"integer"},"limit[offerCodes]":{"maximum":50,"type":"integer"},"limit[versions]":{"maximum":50,"type":"integer"},"sort":{"items":{"enum":["name","-name","inAppPurchaseType","-inAppPurchaseType"],"type":"string"},"type":"array"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/apps/{id}/inAppPurchasesV2".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "filter[productId]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[name]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[state]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[inAppPurchaseType]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "sort".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[inAppPurchases]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[inAppPurchaseLocalizations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[inAppPurchaseContents]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[inAppPurchaseAppStoreReviewScreenshots]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[promotedPurchases]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[inAppPurchasePriceSchedules]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[inAppPurchaseAvailabilities]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[inAppPurchaseImages]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[inAppPurchaseOfferCodes]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[inAppPurchaseVersions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[inAppPurchaseLocalizations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[images]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[offerCodes]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[versions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "apps_preReleaseVersions_getToManyRelated".to_string(),
            description: "List the `preReleaseVersions` related to one `apps` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[preReleaseVersions]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"limit":{"maximum":200,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/apps/{id}/preReleaseVersions".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[preReleaseVersions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "apps_subscriptionGroups_getToManyRelated".to_string(),
            description: "List the `subscriptionGroups` related to one `apps` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[subscriptionGroupLocalizations]":{"items":{"type":"string"},"type":"array"},"fields[subscriptionGroupVersions]":{"items":{"type":"string"},"type":"array"},"fields[subscriptionGroups]":{"items":{"type":"string"},"type":"array"},"fields[subscriptions]":{"items":{"type":"string"},"type":"array"},"filter[referenceName]":{"items":{"type":"string"},"type":"array"},"filter[subscriptions.state]":{"items":{"enum":["MISSING_METADATA","READY_TO_SUBMIT","WAITING_FOR_REVIEW","IN_REVIEW","DEVELOPER_ACTION_NEEDED","PENDING_BINARY_APPROVAL","APPROVED","DEVELOPER_REMOVED_FROM_SALE","REMOVED_FROM_SALE","REJECTED"],"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["subscriptions","subscriptionGroupLocalizations","versions"],"type":"string"},"type":"array"},"limit":{"maximum":200,"type":"integer"},"limit[subscriptionGroupLocalizations]":{"maximum":50,"type":"integer"},"limit[subscriptions]":{"maximum":50,"type":"integer"},"limit[versions]":{"maximum":50,"type":"integer"},"sort":{"items":{"enum":["referenceName","-referenceName"],"type":"string"},"type":"array"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/apps/{id}/subscriptionGroups".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "filter[referenceName]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[subscriptions.state]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "sort".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[subscriptionGroups]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[subscriptions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[subscriptionGroupLocalizations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[subscriptionGroupVersions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[subscriptions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[subscriptionGroupLocalizations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[versions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "betaGroups_betaTesters_createToManyRelationship".to_string(),
            description: "Add existing testers to a TestFlight group.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"items":{"properties":{"id":{"type":"string"},"type":{"enum":["betaTesters"],"type":"string"}},"required":["id","type"],"type":"object"},"type":"array"},"id":{"type":"string"}},"required":["id","data"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "POST".to_string(),
                path_template: "/v1/betaGroups/{id}/relationships/betaTesters".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "betaGroups_betaTesters_deleteToManyRelationship".to_string(),
            description: "Unlink `betaTesters` from one `betaGroups` resource (linkage only).".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"items":{"properties":{"id":{"type":"string"},"type":{"enum":["betaTesters"],"type":"string"}},"required":["id","type"],"type":"object"},"type":"array"},"id":{"type":"string"}},"required":["id","data"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "DELETE".to_string(),
                path_template: "/v1/betaGroups/{id}/relationships/betaTesters".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(true),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "betaGroups_betaTesters_getToManyRelated".to_string(),
            description: "List the `betaTesters` related to one `betaGroups` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[betaTesters]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"limit":{"maximum":200,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/betaGroups/{id}/betaTesters".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[betaTesters]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "betaGroups_builds_createToManyRelationship".to_string(),
            description: "Add builds to a TestFlight group (body: `data: [{type: builds, id}]`).".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"items":{"properties":{"id":{"type":"string"},"type":{"enum":["builds"],"type":"string"}},"required":["id","type"],"type":"object"},"type":"array"},"id":{"type":"string"}},"required":["id","data"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "POST".to_string(),
                path_template: "/v1/betaGroups/{id}/relationships/builds".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "betaGroups_builds_deleteToManyRelationship".to_string(),
            description: "Unlink `builds` from one `betaGroups` resource (linkage only).".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"items":{"properties":{"id":{"type":"string"},"type":{"enum":["builds"],"type":"string"}},"required":["id","type"],"type":"object"},"type":"array"},"id":{"type":"string"}},"required":["id","data"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "DELETE".to_string(),
                path_template: "/v1/betaGroups/{id}/relationships/builds".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(true),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "betaGroups_builds_getToManyRelated".to_string(),
            description: "List the `builds` related to one `betaGroups` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[builds]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"limit":{"maximum":200,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/betaGroups/{id}/builds".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[builds]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "betaTesters_apps_deleteToManyRelationship".to_string(),
            description: "Unlink `apps` from one `betaTesters` resource (linkage only).".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"items":{"properties":{"id":{"type":"string"},"type":{"enum":["apps"],"type":"string"}},"required":["id","type"],"type":"object"},"type":"array"},"id":{"type":"string"}},"required":["id","data"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "DELETE".to_string(),
                path_template: "/v1/betaTesters/{id}/relationships/apps".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(true),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "betaTesters_betaGroups_getToManyRelated".to_string(),
            description: "List the `betaGroups` related to one `betaTesters` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[betaGroups]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"limit":{"maximum":200,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/betaTesters/{id}/betaGroups".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[betaGroups]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "builds_appEncryptionDeclaration_updateToOneRelationship".to_string(),
            description: "Replace the `appEncryptionDeclaration` linkage of one `builds` resource (linkage only).".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"properties":{"id":{"type":"string"},"type":{"enum":["appEncryptionDeclarations"],"type":"string"}},"required":["id","type"],"type":"object"},"id":{"type":"string"}},"required":["id","data"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "PATCH".to_string(),
                path_template: "/v1/builds/{id}/relationships/appEncryptionDeclaration".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "builds_appEncryptionDeclaration_getToOneRelated".to_string(),
            description: "List the `appEncryptionDeclaration` related to one `builds` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[appEncryptionDeclarations]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/builds/{id}/appEncryptionDeclaration".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[appEncryptionDeclarations]".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "builds_betaBuildLocalizations_getToManyRelated".to_string(),
            description: "List the `betaBuildLocalizations` related to one `builds` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[betaBuildLocalizations]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"limit":{"maximum":200,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/builds/{id}/betaBuildLocalizations".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[betaBuildLocalizations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "builds_betaGroups_createToManyRelationship".to_string(),
            description: "Link additional `betaGroups` to one `builds` resource (linkage only).".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"items":{"properties":{"id":{"type":"string"},"type":{"enum":["betaGroups"],"type":"string"}},"required":["id","type"],"type":"object"},"type":"array"},"id":{"type":"string"}},"required":["id","data"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "POST".to_string(),
                path_template: "/v1/builds/{id}/relationships/betaGroups".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "builds_betaGroups_deleteToManyRelationship".to_string(),
            description: "Unlink `betaGroups` from one `builds` resource (linkage only).".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"items":{"properties":{"id":{"type":"string"},"type":{"enum":["betaGroups"],"type":"string"}},"required":["id","type"],"type":"object"},"type":"array"},"id":{"type":"string"}},"required":["id","data"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "DELETE".to_string(),
                path_template: "/v1/builds/{id}/relationships/betaGroups".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(true),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "builds_individualTesters_createToManyRelationship".to_string(),
            description: "Link additional `individualTesters` to one `builds` resource (linkage only).".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"items":{"properties":{"id":{"type":"string"},"type":{"enum":["betaTesters"],"type":"string"}},"required":["id","type"],"type":"object"},"type":"array"},"id":{"type":"string"}},"required":["id","data"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "POST".to_string(),
                path_template: "/v1/builds/{id}/relationships/individualTesters".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "builds_individualTesters_deleteToManyRelationship".to_string(),
            description: "Unlink `individualTesters` from one `builds` resource (linkage only).".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"items":{"properties":{"id":{"type":"string"},"type":{"enum":["betaTesters"],"type":"string"}},"required":["id","type"],"type":"object"},"type":"array"},"id":{"type":"string"}},"required":["id","data"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "DELETE".to_string(),
                path_template: "/v1/builds/{id}/relationships/individualTesters".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(true),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "builds_individualTesters_getToManyRelated".to_string(),
            description: "List the `individualTesters` related to one `builds` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[betaTesters]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"limit":{"maximum":200,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/builds/{id}/individualTesters".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[betaTesters]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "bundleIds_bundleIdCapabilities_getToManyRelated".to_string(),
            description: "List the `bundleIdCapabilities` related to one `bundleIds` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[bundleIdCapabilities]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"limit":{"maximum":200,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/bundleIds/{id}/bundleIdCapabilities".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[bundleIdCapabilities]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "bundleIds_profiles_getToManyRelated".to_string(),
            description: "List the `profiles` related to one `bundleIds` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[profiles]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"limit":{"maximum":200,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/bundleIds/{id}/profiles".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[profiles]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "customerReviews_response_getToOneRelated".to_string(),
            description: "List the `response` related to one `customerReviews` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[customerReviewResponses]":{"items":{"type":"string"},"type":"array"},"fields[customerReviews]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["review"],"type":"string"},"type":"array"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/customerReviews/{id}/response".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[customerReviewResponses]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[customerReviews]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "inAppPurchaseAvailabilities_availableTerritories_getToManyRelationship".to_string(),
            description: "Get the IDs of the `availableTerritories` linked to one `inAppPurchaseAvailabilities` resource (linkage only).".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"id":{"type":"string"},"limit":{"maximum":200,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/inAppPurchaseAvailabilities/{id}/relationships/availableTerritories".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "inAppPurchaseAvailabilities_availableTerritories_getToManyRelated".to_string(),
            description: "List the `availableTerritories` related to one `inAppPurchaseAvailabilities` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[territories]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"limit":{"maximum":200,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/inAppPurchaseAvailabilities/{id}/availableTerritories".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[territories]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "inAppPurchasePricePoints_equalizations_getToManyRelated".to_string(),
            description: "List the `equalizations` related to one `inAppPurchasePricePoints` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[inAppPurchasePricePoints]":{"items":{"type":"string"},"type":"array"},"fields[territories]":{"items":{"type":"string"},"type":"array"},"filter[inAppPurchaseV2]":{"items":{"type":"string"},"type":"array"},"filter[territory]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["territory"],"type":"string"},"type":"array"},"limit":{"maximum":8000,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/inAppPurchasePricePoints/{id}/equalizations".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "filter[territory]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[inAppPurchaseV2]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[inAppPurchasePricePoints]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[territories]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "inAppPurchasePriceSchedules_automaticPrices_getToManyRelated".to_string(),
            description: "List the `automaticPrices` related to one `inAppPurchasePriceSchedules` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[inAppPurchasePricePoints]":{"items":{"type":"string"},"type":"array"},"fields[inAppPurchasePrices]":{"items":{"type":"string"},"type":"array"},"fields[territories]":{"items":{"type":"string"},"type":"array"},"filter[territory]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["inAppPurchasePricePoint","territory"],"type":"string"},"type":"array"},"limit":{"maximum":200,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/inAppPurchasePriceSchedules/{id}/automaticPrices".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "filter[territory]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[inAppPurchasePrices]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[inAppPurchasePricePoints]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[territories]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "inAppPurchasePriceSchedules_baseTerritory_getToOneRelated".to_string(),
            description: "List the `baseTerritory` related to one `inAppPurchasePriceSchedules` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[territories]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/inAppPurchasePriceSchedules/{id}/baseTerritory".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[territories]".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "inAppPurchasePriceSchedules_manualPrices_getToManyRelated".to_string(),
            description: "List the `manualPrices` related to one `inAppPurchasePriceSchedules` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[inAppPurchasePricePoints]":{"items":{"type":"string"},"type":"array"},"fields[inAppPurchasePrices]":{"items":{"type":"string"},"type":"array"},"fields[territories]":{"items":{"type":"string"},"type":"array"},"filter[territory]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["inAppPurchasePricePoint","territory"],"type":"string"},"type":"array"},"limit":{"maximum":200,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/inAppPurchasePriceSchedules/{id}/manualPrices".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "filter[territory]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[inAppPurchasePrices]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[inAppPurchasePricePoints]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[territories]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "inAppPurchasesV2_appStoreReviewScreenshot_getToOneRelated".to_string(),
            description: "List the `appStoreReviewScreenshot` related to one `inAppPurchases` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[inAppPurchaseAppStoreReviewScreenshots]":{"items":{"type":"string"},"type":"array"},"fields[inAppPurchases]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["inAppPurchaseV2"],"type":"string"},"type":"array"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v2/inAppPurchases/{id}/appStoreReviewScreenshot".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[inAppPurchaseAppStoreReviewScreenshots]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[inAppPurchases]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "inAppPurchasesV2_iapPriceSchedule_getToOneRelated".to_string(),
            description: "List the `iapPriceSchedule` related to one `inAppPurchases` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[inAppPurchasePriceSchedules]":{"items":{"type":"string"},"type":"array"},"fields[inAppPurchasePrices]":{"items":{"type":"string"},"type":"array"},"fields[territories]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["baseTerritory","manualPrices","automaticPrices"],"type":"string"},"type":"array"},"limit[automaticPrices]":{"maximum":50,"type":"integer"},"limit[manualPrices]":{"maximum":50,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v2/inAppPurchases/{id}/iapPriceSchedule".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[inAppPurchasePriceSchedules]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[territories]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[inAppPurchasePrices]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[manualPrices]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[automaticPrices]".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "inAppPurchasesV2_inAppPurchaseAvailability_getToOneRelated".to_string(),
            description: "List the `inAppPurchaseAvailability` related to one `inAppPurchases` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[inAppPurchaseAvailabilities]":{"items":{"type":"string"},"type":"array"},"fields[territories]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["availableTerritories"],"type":"string"},"type":"array"},"limit[availableTerritories]":{"maximum":50,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v2/inAppPurchases/{id}/inAppPurchaseAvailability".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[inAppPurchaseAvailabilities]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[territories]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[availableTerritories]".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "inAppPurchasesV2_inAppPurchaseLocalizations_getToManyRelated".to_string(),
            description: "List the `inAppPurchaseLocalizations` related to one `inAppPurchases` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[inAppPurchaseLocalizations]":{"items":{"type":"string"},"type":"array"},"fields[inAppPurchases]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["inAppPurchaseV2"],"type":"string"},"type":"array"},"limit":{"maximum":200,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v2/inAppPurchases/{id}/inAppPurchaseLocalizations".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[inAppPurchaseLocalizations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[inAppPurchases]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "inAppPurchasesV2_pricePoints_getToManyRelated".to_string(),
            description: "List the `pricePoints` related to one `inAppPurchases` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[inAppPurchasePricePoints]":{"items":{"type":"string"},"type":"array"},"fields[territories]":{"items":{"type":"string"},"type":"array"},"filter[territory]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["territory"],"type":"string"},"type":"array"},"limit":{"maximum":8000,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v2/inAppPurchases/{id}/pricePoints".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "filter[territory]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[inAppPurchasePricePoints]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[territories]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "preReleaseVersions_builds_getToManyRelated".to_string(),
            description: "List the `builds` related to one `preReleaseVersions` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[builds]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"limit":{"maximum":200,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/preReleaseVersions/{id}/builds".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[builds]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "profiles_bundleId_getToOneRelated".to_string(),
            description: "List the `bundleId` related to one `profiles` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[bundleIds]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/profiles/{id}/bundleId".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[bundleIds]".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "profiles_certificates_getToManyRelated".to_string(),
            description: "List the `certificates` related to one `profiles` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[certificates]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"limit":{"maximum":200,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/profiles/{id}/certificates".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[certificates]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "profiles_devices_getToManyRelated".to_string(),
            description: "List the `devices` related to one `profiles` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[devices]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"limit":{"maximum":200,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/profiles/{id}/devices".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[devices]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "reviewSubmissions_items_getToManyRelationship".to_string(),
            description: "Get the IDs of the `items` linked to one `reviewSubmissions` resource (linkage only).".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"id":{"type":"string"},"limit":{"maximum":200,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/reviewSubmissions/{id}/relationships/items".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "reviewSubmissions_items_getToManyRelated".to_string(),
            description: "List the `items` related to one `reviewSubmissions` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[appCustomProductPageVersions]":{"items":{"type":"string"},"type":"array"},"fields[appEvents]":{"items":{"type":"string"},"type":"array"},"fields[appStoreVersionExperiments]":{"items":{"type":"string"},"type":"array"},"fields[appStoreVersions]":{"items":{"type":"string"},"type":"array"},"fields[backgroundAssetVersions]":{"items":{"type":"string"},"type":"array"},"fields[gameCenterAchievementVersions]":{"items":{"type":"string"},"type":"array"},"fields[gameCenterActivityVersions]":{"items":{"type":"string"},"type":"array"},"fields[gameCenterChallengeVersions]":{"items":{"type":"string"},"type":"array"},"fields[gameCenterLeaderboardSetVersions]":{"items":{"type":"string"},"type":"array"},"fields[gameCenterLeaderboardVersions]":{"items":{"type":"string"},"type":"array"},"fields[inAppPurchaseVersions]":{"items":{"type":"string"},"type":"array"},"fields[reviewSubmissionItems]":{"items":{"type":"string"},"type":"array"},"fields[subscriptionGroupVersions]":{"items":{"type":"string"},"type":"array"},"fields[subscriptionVersions]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["appStoreVersion","appCustomProductPageVersion","appStoreVersionExperiment","appStoreVersionExperimentV2","appEvent","backgroundAssetVersion","gameCenterAchievementVersion","gameCenterActivityVersion","gameCenterChallengeVersion","gameCenterLeaderboardSetVersion","gameCenterLeaderboardVersion","inAppPurchaseVersion","subscriptionVersion","subscriptionGroupVersion"],"type":"string"},"type":"array"},"limit":{"maximum":200,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/reviewSubmissions/{id}/items".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[reviewSubmissionItems]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appStoreVersions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appCustomProductPageVersions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appStoreVersionExperiments]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[appEvents]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[backgroundAssetVersions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[gameCenterAchievementVersions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[gameCenterActivityVersions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[gameCenterChallengeVersions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[gameCenterLeaderboardSetVersions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[gameCenterLeaderboardVersions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[inAppPurchaseVersions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[subscriptionVersions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[subscriptionGroupVersions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "subscriptionGroups_subscriptionGroupLocalizations_getToManyRelated".to_string(),
            description: "List the `subscriptionGroupLocalizations` related to one `subscriptionGroups` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[subscriptionGroupLocalizations]":{"items":{"type":"string"},"type":"array"},"fields[subscriptionGroups]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["subscriptionGroup"],"type":"string"},"type":"array"},"limit":{"maximum":200,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/subscriptionGroups/{id}/subscriptionGroupLocalizations".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[subscriptionGroupLocalizations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[subscriptionGroups]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "subscriptionGroups_subscriptions_getToManyRelated".to_string(),
            description: "List the `subscriptions` related to one `subscriptionGroups` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[promotedPurchases]":{"items":{"type":"string"},"type":"array"},"fields[subscriptionAppStoreReviewScreenshots]":{"items":{"type":"string"},"type":"array"},"fields[subscriptionAvailabilities]":{"items":{"type":"string"},"type":"array"},"fields[subscriptionGroups]":{"items":{"type":"string"},"type":"array"},"fields[subscriptionImages]":{"items":{"type":"string"},"type":"array"},"fields[subscriptionIntroductoryOffers]":{"items":{"type":"string"},"type":"array"},"fields[subscriptionLocalizations]":{"items":{"type":"string"},"type":"array"},"fields[subscriptionOfferCodes]":{"items":{"type":"string"},"type":"array"},"fields[subscriptionPlanAvailabilities]":{"items":{"type":"string"},"type":"array"},"fields[subscriptionPrices]":{"items":{"type":"string"},"type":"array"},"fields[subscriptionPromotionalOffers]":{"items":{"type":"string"},"type":"array"},"fields[subscriptionVersions]":{"items":{"type":"string"},"type":"array"},"fields[subscriptions]":{"items":{"type":"string"},"type":"array"},"fields[winBackOffers]":{"items":{"type":"string"},"type":"array"},"filter[name]":{"items":{"type":"string"},"type":"array"},"filter[productId]":{"items":{"type":"string"},"type":"array"},"filter[state]":{"items":{"enum":["MISSING_METADATA","READY_TO_SUBMIT","WAITING_FOR_REVIEW","IN_REVIEW","DEVELOPER_ACTION_NEEDED","PENDING_BINARY_APPROVAL","APPROVED","DEVELOPER_REMOVED_FROM_SALE","REMOVED_FROM_SALE","REJECTED"],"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["subscriptionLocalizations","appStoreReviewScreenshot","group","introductoryOffers","promotionalOffers","offerCodes","prices","promotedPurchase","subscriptionAvailability","winBackOffers","images","planAvailabilities","versions"],"type":"string"},"type":"array"},"limit":{"maximum":200,"type":"integer"},"limit[images]":{"maximum":50,"type":"integer"},"limit[introductoryOffers]":{"maximum":50,"type":"integer"},"limit[offerCodes]":{"maximum":50,"type":"integer"},"limit[planAvailabilities]":{"maximum":50,"type":"integer"},"limit[prices]":{"maximum":50,"type":"integer"},"limit[promotionalOffers]":{"maximum":50,"type":"integer"},"limit[subscriptionLocalizations]":{"maximum":50,"type":"integer"},"limit[versions]":{"maximum":50,"type":"integer"},"limit[winBackOffers]":{"maximum":50,"type":"integer"},"sort":{"items":{"enum":["name","-name"],"type":"string"},"type":"array"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/subscriptionGroups/{id}/subscriptions".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "filter[productId]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[name]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[state]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "sort".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[subscriptions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[subscriptionLocalizations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[subscriptionAppStoreReviewScreenshots]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[subscriptionGroups]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[subscriptionIntroductoryOffers]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[subscriptionPromotionalOffers]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[subscriptionOfferCodes]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[subscriptionPrices]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[promotedPurchases]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[subscriptionAvailabilities]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[winBackOffers]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[subscriptionImages]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[subscriptionPlanAvailabilities]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[subscriptionVersions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[subscriptionLocalizations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[introductoryOffers]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[promotionalOffers]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[offerCodes]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[prices]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[winBackOffers]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[images]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[planAvailabilities]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[versions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "subscriptionPlanAvailabilities_availableTerritories_getToManyRelationship".to_string(),
            description: "Get the IDs of the `availableTerritories` linked to one `subscriptionPlanAvailabilities` resource (linkage only).".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"id":{"type":"string"},"limit":{"maximum":200,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/subscriptionPlanAvailabilities/{id}/relationships/availableTerritories".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "subscriptionPlanAvailabilities_availableTerritories_replaceToManyRelationship".to_string(),
            description: "Replace the `availableTerritories` linkage of one `subscriptionPlanAvailabilities` resource (linkage only).".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"items":{"properties":{"id":{"type":"string"},"type":{"enum":["territories"],"type":"string"}},"required":["id","type"],"type":"object"},"type":"array"},"id":{"type":"string"}},"required":["id","data"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "PATCH".to_string(),
                path_template: "/v1/subscriptionPlanAvailabilities/{id}/relationships/availableTerritories".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(false),
                idempotent: Some(false),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "subscriptionPlanAvailabilities_availableTerritories_getToManyRelated".to_string(),
            description: "List the `availableTerritories` related to one `subscriptionPlanAvailabilities` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[territories]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"limit":{"maximum":200,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/subscriptionPlanAvailabilities/{id}/availableTerritories".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[territories]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "subscriptionPricePoints_equalizations_getToManyRelated".to_string(),
            description: "List the `equalizations` related to one `subscriptionPricePoints` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[subscriptionPricePoints]":{"items":{"type":"string"},"type":"array"},"fields[territories]":{"items":{"type":"string"},"type":"array"},"filter[planType]":{"items":{"type":"string"},"type":"array"},"filter[subscription]":{"items":{"type":"string"},"type":"array"},"filter[territory]":{"items":{"type":"string"},"type":"array"},"filter[upfrontPricePointId]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["territory"],"type":"string"},"type":"array"},"limit":{"maximum":8000,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/subscriptionPricePoints/{id}/equalizations".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "filter[territory]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[subscription]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[upfrontPricePointId]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[planType]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[subscriptionPricePoints]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[territories]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "subscriptions_appStoreReviewScreenshot_getToOneRelated".to_string(),
            description: "List the `appStoreReviewScreenshot` related to one `subscriptions` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[subscriptionAppStoreReviewScreenshots]":{"items":{"type":"string"},"type":"array"},"fields[subscriptions]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["subscription"],"type":"string"},"type":"array"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/subscriptions/{id}/appStoreReviewScreenshot".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[subscriptionAppStoreReviewScreenshots]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[subscriptions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "subscriptions_introductoryOffers_getToManyRelated".to_string(),
            description: "List the `introductoryOffers` related to one `subscriptions` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[subscriptionIntroductoryOffers]":{"items":{"type":"string"},"type":"array"},"fields[subscriptionPricePoints]":{"items":{"type":"string"},"type":"array"},"fields[subscriptions]":{"items":{"type":"string"},"type":"array"},"fields[territories]":{"items":{"type":"string"},"type":"array"},"filter[territory]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["subscription","territory","subscriptionPricePoint"],"type":"string"},"type":"array"},"limit":{"maximum":200,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/subscriptions/{id}/introductoryOffers".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "filter[territory]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[subscriptionIntroductoryOffers]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[subscriptions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[territories]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[subscriptionPricePoints]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "subscriptions_planAvailabilities_getToManyRelated".to_string(),
            description: "List the `planAvailabilities` related to one `subscriptions` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[subscriptionPlanAvailabilities]":{"items":{"type":"string"},"type":"array"},"fields[territories]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["availableTerritories"],"type":"string"},"type":"array"},"limit":{"maximum":200,"type":"integer"},"limit[availableTerritories]":{"maximum":50,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/subscriptions/{id}/planAvailabilities".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[subscriptionPlanAvailabilities]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[territories]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[availableTerritories]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "subscriptions_pricePoints_getToManyRelated".to_string(),
            description: "List the `pricePoints` related to one `subscriptions` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[subscriptionPricePoints]":{"items":{"type":"string"},"type":"array"},"fields[territories]":{"items":{"type":"string"},"type":"array"},"filter[planType]":{"items":{"type":"string"},"type":"array"},"filter[territory]":{"items":{"type":"string"},"type":"array"},"filter[upfrontPricePointId]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["territory"],"type":"string"},"type":"array"},"limit":{"maximum":8000,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/subscriptions/{id}/pricePoints".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "filter[territory]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[upfrontPricePointId]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[planType]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[subscriptionPricePoints]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[territories]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "subscriptions_prices_deleteToManyRelationship".to_string(),
            description: "Unlink `prices` from one `subscriptions` resource (linkage only).".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"data":{"items":{"properties":{"id":{"type":"string"},"type":{"enum":["subscriptionPrices"],"type":"string"}},"required":["id","type"],"type":"object"},"type":"array"},"id":{"type":"string"}},"required":["id","data"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "DELETE".to_string(),
                path_template: "/v1/subscriptions/{id}/relationships/prices".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                    "data".to_string(),
                ],
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(false),
                destructive: Some(true),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "subscriptions_prices_getToManyRelated".to_string(),
            description: "List the `prices` related to one `subscriptions` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[subscriptionPricePoints]":{"items":{"type":"string"},"type":"array"},"fields[subscriptionPrices]":{"items":{"type":"string"},"type":"array"},"fields[territories]":{"items":{"type":"string"},"type":"array"},"filter[planType]":{"items":{"enum":["MONTHLY","UPFRONT"],"type":"string"},"type":"array"},"filter[subscriptionPricePoint]":{"items":{"type":"string"},"type":"array"},"filter[territory]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["territory","subscriptionPricePoint"],"type":"string"},"type":"array"},"limit":{"maximum":200,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/subscriptions/{id}/prices".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "filter[planType]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[subscriptionPricePoint]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "filter[territory]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[subscriptionPrices]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[territories]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[subscriptionPricePoints]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "subscriptions_promotionalOffers_getToManyRelated".to_string(),
            description: "List the `promotionalOffers` related to one `subscriptions` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[subscriptionPromotionalOfferPrices]":{"items":{"type":"string"},"type":"array"},"fields[subscriptionPromotionalOffers]":{"items":{"type":"string"},"type":"array"},"fields[subscriptions]":{"items":{"type":"string"},"type":"array"},"filter[territory]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["subscription","prices"],"type":"string"},"type":"array"},"limit":{"maximum":200,"type":"integer"},"limit[prices]":{"maximum":50,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/subscriptions/{id}/promotionalOffers".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "filter[territory]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[subscriptionPromotionalOffers]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[subscriptions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[subscriptionPromotionalOfferPrices]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit[prices]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
        ToolSpec {
            name: "subscriptions_subscriptionLocalizations_getToManyRelated".to_string(),
            description: "List the `subscriptionLocalizations` related to one `subscriptions` resource.".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"cursor":{"description":"Next page: the `cursor` value from `links.next` (none = last page).","type":"string"},"fields[subscriptionLocalizations]":{"items":{"type":"string"},"type":"array"},"fields[subscriptions]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["subscription"],"type":"string"},"type":"array"},"limit":{"maximum":200,"type":"integer"}},"required":["id"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/v1/subscriptions/{id}/subscriptionLocalizations".to_string(),
                params: vec![
                    ParamBinding {
                        name: "id".to_string(),
                        location: ParamLocation::Path,
                    },
                    ParamBinding {
                        name: "fields[subscriptionLocalizations]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "fields[subscriptions]".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "limit".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "include".to_string(),
                        location: ParamLocation::Query,
                    },
                    ParamBinding {
                        name: "cursor".to_string(),
                        location: ParamLocation::Query,
                    },
                ],
                body_fields: vec![
                ],
            content_type: None,
            raw_body: false,
            media: None,
        }),
            hints: ToolHints {
                title: None,
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
    ]
}

/// Profiles exposed when neither config.toml nor MCP_FACTORY_PROFILES choose.
#[rustfmt::skip]
pub fn default_profiles() -> Vec<String> {
    vec!["core".to_string(), "metadata".to_string(), "assets".to_string(), "testflight".to_string()]
}

/// Tool name -> runtime profiles (see `--tool-config`).
#[rustfmt::skip]
pub fn build_tool_profiles() -> Vec<(&'static str, &'static [&'static str])> {
    vec![
        ("ageRatingDeclarations_updateInstance", &["metadata"]),
        ("analytics_segment_download", &["reports"]),
        ("analyticsReportInstances_getInstance", &["reports"]),
        ("analyticsReportInstances_segments_getToManyRelated", &["reports"]),
        ("analyticsReportRequests_createInstance", &["reports"]),
        ("analyticsReportRequests_deleteInstance", &["reports"]),
        ("analyticsReportRequests_getInstance", &["reports"]),
        ("analyticsReportRequests_reports_getToManyRelated", &["reports"]),
        ("analyticsReports_getInstance", &["reports"]),
        ("analyticsReports_instances_getToManyRelated", &["reports"]),
        ("analyticsReportSegments_getInstance", &["reports"]),
        ("appAvailabilitiesV2_createInstance", &["pricing"]),
        ("appAvailabilitiesV2_getInstance", &["pricing"]),
        ("appAvailabilitiesV2_territoryAvailabilities_getToManyRelated", &["pricing"]),
        ("appAvailabilitiesV2_territoryAvailabilities_getToManyRelationship", &["pricing"]),
        ("appCategories_getCollection", &["metadata"]),
        ("appCategories_getInstance", &["metadata"]),
        ("appCategories_subcategories_getToManyRelated", &["metadata"]),
        ("appEncryptionDeclarationDocuments_createInstance", &["assets"]),
        ("appEncryptionDeclarationDocuments_getInstance", &["metadata", "assets"]),
        ("appEncryptionDeclarationDocuments_updateInstance", &["assets"]),
        ("appEncryptionDeclarations_createInstance", &["metadata"]),
        ("appEncryptionDeclarations_getCollection", &["metadata"]),
        ("appEncryptionDeclarations_getInstance", &["metadata"]),
        ("appInfoLocalizations_createInstance", &["metadata"]),
        ("appInfoLocalizations_deleteInstance", &["metadata"]),
        ("appInfoLocalizations_getInstance", &["metadata"]),
        ("appInfoLocalizations_updateInstance", &["metadata"]),
        ("appInfos_ageRatingDeclaration_getToOneRelated", &["metadata"]),
        ("appInfos_appInfoLocalizations_getToManyRelated", &["metadata"]),
        ("appInfos_getInstance", &["metadata"]),
        ("appInfos_updateInstance", &["metadata"]),
        ("appPreviews_createInstance", &["assets"]),
        ("appPreviews_deleteInstance", &["assets"]),
        ("appPreviews_getInstance", &["assets"]),
        ("appPreviews_updateInstance", &["assets"]),
        ("appPreviewSets_appPreviews_getToManyRelated", &["assets"]),
        ("appPreviewSets_appPreviews_replaceToManyRelationship", &["assets"]),
        ("appPreviewSets_createInstance", &["assets"]),
        ("appPreviewSets_deleteInstance", &["assets"]),
        ("appPreviewSets_getInstance", &["assets"]),
        ("appPricePointsV3_equalizations_getToManyRelated", &["pricing"]),
        ("appPricePointsV3_getInstance", &["pricing"]),
        ("appPriceSchedules_automaticPrices_getToManyRelated", &["pricing"]),
        ("appPriceSchedules_baseTerritory_getToOneRelated", &["pricing"]),
        ("appPriceSchedules_createInstance", &["pricing"]),
        ("appPriceSchedules_getInstance", &["pricing"]),
        ("appPriceSchedules_manualPrices_getToManyRelated", &["pricing"]),
        ("apps_analyticsReportRequests_getToManyRelated", &["reports"]),
        ("apps_appAvailabilityV2_getToOneRelated", &["pricing"]),
        ("apps_appEncryptionDeclarations_getToManyRelated", &["metadata"]),
        ("apps_appInfos_getToManyRelated", &["metadata"]),
        ("apps_appPricePoints_getToManyRelated", &["pricing"]),
        ("apps_appPriceSchedule_getToOneRelated", &["pricing"]),
        ("apps_appStoreVersions_getToManyRelated", &["core"]),
        ("apps_betaAppLocalizations_getToManyRelated", &["testflight"]),
        ("apps_betaAppReviewDetail_getToOneRelated", &["testflight"]),
        ("apps_betaGroups_getToManyRelated", &["testflight"]),
        ("apps_betaLicenseAgreement_getToOneRelated", &["testflight"]),
        ("apps_builds_getToManyRelated", &["core"]),
        ("apps_customerReviews_getToManyRelated", &["reviews"]),
        ("apps_getCollection", &["core"]),
        ("apps_getInstance", &["core"]),
        ("apps_inAppPurchasesV2_getToManyRelated", &["monetization"]),
        ("apps_preReleaseVersions_getToManyRelated", &["testflight"]),
        ("apps_subscriptionGroups_getToManyRelated", &["monetization"]),
        ("apps_updateInstance", &["metadata"]),
        ("appScreenshots_createInstance", &["assets"]),
        ("appScreenshots_deleteInstance", &["assets"]),
        ("appScreenshots_getInstance", &["assets"]),
        ("appScreenshots_updateInstance", &["assets"]),
        ("appScreenshotSets_appScreenshots_getToManyRelated", &["assets"]),
        ("appScreenshotSets_appScreenshots_replaceToManyRelationship", &["assets"]),
        ("appScreenshotSets_createInstance", &["assets"]),
        ("appScreenshotSets_deleteInstance", &["assets"]),
        ("appScreenshotSets_getInstance", &["assets"]),
        ("appStoreReviewAttachments_createInstance", &["assets"]),
        ("appStoreReviewAttachments_deleteInstance", &["assets"]),
        ("appStoreReviewAttachments_getInstance", &["assets"]),
        ("appStoreReviewAttachments_updateInstance", &["assets"]),
        ("appStoreReviewDetails_appStoreReviewAttachments_getToManyRelated", &["assets"]),
        ("appStoreReviewDetails_createInstance", &["core"]),
        ("appStoreReviewDetails_getInstance", &["core"]),
        ("appStoreReviewDetails_updateInstance", &["core"]),
        ("appStoreVersionLocalizations_appPreviewSets_getToManyRelated", &["assets"]),
        ("appStoreVersionLocalizations_appScreenshotSets_getToManyRelated", &["assets"]),
        ("appStoreVersionLocalizations_createInstance", &["metadata"]),
        ("appStoreVersionLocalizations_deleteInstance", &["metadata"]),
        ("appStoreVersionLocalizations_getInstance", &["metadata"]),
        ("appStoreVersionLocalizations_updateInstance", &["metadata"]),
        ("appStoreVersionPhasedReleases_createInstance", &["core"]),
        ("appStoreVersionPhasedReleases_deleteInstance", &["core"]),
        ("appStoreVersionPhasedReleases_updateInstance", &["core"]),
        ("appStoreVersionReleaseRequests_createInstance", &["core"]),
        ("appStoreVersions_appStoreReviewDetail_getToOneRelated", &["core"]),
        ("appStoreVersions_appStoreVersionLocalizations_getToManyRelated", &["metadata"]),
        ("appStoreVersions_appStoreVersionPhasedRelease_getToOneRelated", &["core"]),
        ("appStoreVersions_build_getToOneRelated", &["core"]),
        ("appStoreVersions_build_updateToOneRelationship", &["core"]),
        ("appStoreVersions_createInstance", &["core"]),
        ("appStoreVersions_customerReviews_getToManyRelated", &["reviews"]),
        ("appStoreVersions_deleteInstance", &["core"]),
        ("appStoreVersions_getInstance", &["core"]),
        ("appStoreVersions_updateInstance", &["core"]),
        ("asset_upload", &["assets", "monetization"]),
        ("betaAppLocalizations_createInstance", &["testflight"]),
        ("betaAppLocalizations_deleteInstance", &["testflight"]),
        ("betaAppLocalizations_getCollection", &["testflight"]),
        ("betaAppLocalizations_getInstance", &["testflight"]),
        ("betaAppLocalizations_updateInstance", &["testflight"]),
        ("betaAppReviewDetails_getCollection", &["testflight"]),
        ("betaAppReviewDetails_getInstance", &["testflight"]),
        ("betaAppReviewDetails_updateInstance", &["testflight"]),
        ("betaAppReviewSubmissions_createInstance", &["testflight"]),
        ("betaAppReviewSubmissions_getCollection", &["testflight"]),
        ("betaAppReviewSubmissions_getInstance", &["testflight"]),
        ("betaBuildLocalizations_createInstance", &["testflight"]),
        ("betaBuildLocalizations_deleteInstance", &["testflight"]),
        ("betaBuildLocalizations_getCollection", &["testflight"]),
        ("betaBuildLocalizations_getInstance", &["testflight"]),
        ("betaBuildLocalizations_updateInstance", &["testflight"]),
        ("betaGroups_betaTesters_createToManyRelationship", &["testflight"]),
        ("betaGroups_betaTesters_deleteToManyRelationship", &["testflight"]),
        ("betaGroups_betaTesters_getToManyRelated", &["testflight"]),
        ("betaGroups_builds_createToManyRelationship", &["testflight"]),
        ("betaGroups_builds_deleteToManyRelationship", &["testflight"]),
        ("betaGroups_builds_getToManyRelated", &["testflight"]),
        ("betaGroups_createInstance", &["testflight"]),
        ("betaGroups_deleteInstance", &["testflight"]),
        ("betaGroups_getCollection", &["testflight"]),
        ("betaGroups_getInstance", &["testflight"]),
        ("betaGroups_updateInstance", &["testflight"]),
        ("betaLicenseAgreements_getCollection", &["testflight"]),
        ("betaLicenseAgreements_getInstance", &["testflight"]),
        ("betaLicenseAgreements_updateInstance", &["testflight"]),
        ("betaTesters_apps_deleteToManyRelationship", &["testflight"]),
        ("betaTesters_betaGroups_getToManyRelated", &["testflight"]),
        ("betaTesters_createInstance", &["testflight"]),
        ("betaTesters_deleteInstance", &["testflight"]),
        ("betaTesters_getCollection", &["testflight"]),
        ("betaTesters_getInstance", &["testflight"]),
        ("buildBetaDetails_getCollection", &["testflight"]),
        ("buildBetaDetails_getInstance", &["testflight"]),
        ("buildBetaDetails_updateInstance", &["testflight"]),
        ("builds_appEncryptionDeclaration_getToOneRelated", &["metadata"]),
        ("builds_appEncryptionDeclaration_updateToOneRelationship", &["metadata"]),
        ("builds_betaBuildLocalizations_getToManyRelated", &["testflight"]),
        ("builds_betaGroups_createToManyRelationship", &["testflight"]),
        ("builds_betaGroups_deleteToManyRelationship", &["testflight"]),
        ("builds_getCollection", &["core"]),
        ("builds_getInstance", &["core"]),
        ("builds_individualTesters_createToManyRelationship", &["testflight"]),
        ("builds_individualTesters_deleteToManyRelationship", &["testflight"]),
        ("builds_individualTesters_getToManyRelated", &["testflight"]),
        ("builds_updateInstance", &["core"]),
        ("bundleIdCapabilities_createInstance", &["signing"]),
        ("bundleIdCapabilities_deleteInstance", &["signing"]),
        ("bundleIdCapabilities_updateInstance", &["signing"]),
        ("bundleIds_bundleIdCapabilities_getToManyRelated", &["signing"]),
        ("bundleIds_createInstance", &["signing"]),
        ("bundleIds_deleteInstance", &["signing"]),
        ("bundleIds_getCollection", &["signing"]),
        ("bundleIds_getInstance", &["signing"]),
        ("bundleIds_profiles_getToManyRelated", &["signing"]),
        ("bundleIds_updateInstance", &["signing"]),
        ("certificates_createInstance", &["signing"]),
        ("certificates_deleteInstance", &["signing"]),
        ("certificates_getCollection", &["signing"]),
        ("certificates_getInstance", &["signing"]),
        ("certificates_updateInstance", &["signing"]),
        ("customerReviewResponses_createInstance", &["reviews"]),
        ("customerReviewResponses_deleteInstance", &["reviews"]),
        ("customerReviewResponses_getInstance", &["reviews"]),
        ("customerReviews_getInstance", &["reviews"]),
        ("customerReviews_response_getToOneRelated", &["reviews"]),
        ("devices_createInstance", &["signing"]),
        ("devices_getCollection", &["signing"]),
        ("devices_getInstance", &["signing"]),
        ("devices_updateInstance", &["signing"]),
        ("financeReports_getCollection", &["reports"]),
        ("inAppPurchaseAppStoreReviewScreenshots_createInstance", &["monetization"]),
        ("inAppPurchaseAppStoreReviewScreenshots_deleteInstance", &["monetization"]),
        ("inAppPurchaseAppStoreReviewScreenshots_getInstance", &["monetization"]),
        ("inAppPurchaseAppStoreReviewScreenshots_updateInstance", &["monetization"]),
        ("inAppPurchaseAvailabilities_availableTerritories_getToManyRelated", &["monetization"]),
        ("inAppPurchaseAvailabilities_availableTerritories_getToManyRelationship", &["monetization"]),
        ("inAppPurchaseAvailabilities_createInstance", &["monetization"]),
        ("inAppPurchaseAvailabilities_getInstance", &["monetization"]),
        ("inAppPurchaseLocalizations_createInstance", &["monetization"]),
        ("inAppPurchaseLocalizations_deleteInstance", &["monetization"]),
        ("inAppPurchaseLocalizations_getInstance", &["monetization"]),
        ("inAppPurchaseLocalizations_updateInstance", &["monetization"]),
        ("inAppPurchasePricePoints_equalizations_getToManyRelated", &["pricing"]),
        ("inAppPurchasePriceSchedules_automaticPrices_getToManyRelated", &["pricing"]),
        ("inAppPurchasePriceSchedules_baseTerritory_getToOneRelated", &["pricing"]),
        ("inAppPurchasePriceSchedules_createInstance", &["pricing"]),
        ("inAppPurchasePriceSchedules_getInstance", &["pricing"]),
        ("inAppPurchasePriceSchedules_manualPrices_getToManyRelated", &["pricing"]),
        ("inAppPurchaseSubmissions_createInstance", &["monetization"]),
        ("inAppPurchasesV2_appStoreReviewScreenshot_getToOneRelated", &["monetization"]),
        ("inAppPurchasesV2_createInstance", &["monetization"]),
        ("inAppPurchasesV2_deleteInstance", &["monetization"]),
        ("inAppPurchasesV2_getInstance", &["monetization"]),
        ("inAppPurchasesV2_iapPriceSchedule_getToOneRelated", &["pricing"]),
        ("inAppPurchasesV2_inAppPurchaseAvailability_getToOneRelated", &["monetization"]),
        ("inAppPurchasesV2_inAppPurchaseLocalizations_getToManyRelated", &["monetization"]),
        ("inAppPurchasesV2_pricePoints_getToManyRelated", &["pricing"]),
        ("inAppPurchasesV2_updateInstance", &["monetization"]),
        ("preReleaseVersions_builds_getToManyRelated", &["testflight"]),
        ("preReleaseVersions_getCollection", &["testflight"]),
        ("profiles_bundleId_getToOneRelated", &["signing"]),
        ("profiles_certificates_getToManyRelated", &["signing"]),
        ("profiles_createInstance", &["signing"]),
        ("profiles_deleteInstance", &["signing"]),
        ("profiles_devices_getToManyRelated", &["signing"]),
        ("profiles_getCollection", &["signing"]),
        ("profiles_getInstance", &["signing"]),
        ("reviewSubmissionItems_createInstance", &["core"]),
        ("reviewSubmissionItems_deleteInstance", &["core"]),
        ("reviewSubmissionItems_updateInstance", &["core"]),
        ("reviewSubmissions_createInstance", &["core"]),
        ("reviewSubmissions_getCollection", &["core"]),
        ("reviewSubmissions_getInstance", &["core"]),
        ("reviewSubmissions_items_getToManyRelated", &["core"]),
        ("reviewSubmissions_items_getToManyRelationship", &["core"]),
        ("reviewSubmissions_updateInstance", &["core"]),
        ("salesReports_getCollection", &["reports"]),
        ("subscriptionAppStoreReviewScreenshots_createInstance", &["monetization"]),
        ("subscriptionAppStoreReviewScreenshots_deleteInstance", &["monetization"]),
        ("subscriptionAppStoreReviewScreenshots_getInstance", &["monetization"]),
        ("subscriptionAppStoreReviewScreenshots_updateInstance", &["monetization"]),
        ("subscriptionGroupLocalizations_createInstance", &["monetization"]),
        ("subscriptionGroupLocalizations_deleteInstance", &["monetization"]),
        ("subscriptionGroupLocalizations_getInstance", &["monetization"]),
        ("subscriptionGroupLocalizations_updateInstance", &["monetization"]),
        ("subscriptionGroups_createInstance", &["monetization"]),
        ("subscriptionGroups_deleteInstance", &["monetization"]),
        ("subscriptionGroups_getInstance", &["monetization"]),
        ("subscriptionGroups_subscriptionGroupLocalizations_getToManyRelated", &["monetization"]),
        ("subscriptionGroups_subscriptions_getToManyRelated", &["monetization"]),
        ("subscriptionGroups_updateInstance", &["monetization"]),
        ("subscriptionIntroductoryOffers_createInstance", &["monetization"]),
        ("subscriptionIntroductoryOffers_deleteInstance", &["monetization"]),
        ("subscriptionIntroductoryOffers_updateInstance", &["monetization"]),
        ("subscriptionLocalizations_createInstance", &["monetization"]),
        ("subscriptionLocalizations_deleteInstance", &["monetization"]),
        ("subscriptionLocalizations_getInstance", &["monetization"]),
        ("subscriptionLocalizations_updateInstance", &["monetization"]),
        ("subscriptionPlanAvailabilities_availableTerritories_getToManyRelated", &["monetization"]),
        ("subscriptionPlanAvailabilities_availableTerritories_getToManyRelationship", &["monetization"]),
        ("subscriptionPlanAvailabilities_availableTerritories_replaceToManyRelationship", &["monetization"]),
        ("subscriptionPlanAvailabilities_createInstance", &["monetization"]),
        ("subscriptionPlanAvailabilities_getInstance", &["monetization"]),
        ("subscriptionPlanAvailabilities_updateInstance", &["monetization"]),
        ("subscriptionPricePoints_equalizations_getToManyRelated", &["pricing"]),
        ("subscriptionPricePoints_getInstance", &["pricing"]),
        ("subscriptionPrices_createInstance", &["pricing"]),
        ("subscriptionPrices_deleteInstance", &["pricing"]),
        ("subscriptionPromotionalOffers_createInstance", &["monetization"]),
        ("subscriptionPromotionalOffers_deleteInstance", &["monetization"]),
        ("subscriptionPromotionalOffers_getInstance", &["monetization"]),
        ("subscriptionPromotionalOffers_updateInstance", &["monetization"]),
        ("subscriptions_appStoreReviewScreenshot_getToOneRelated", &["monetization"]),
        ("subscriptions_createInstance", &["monetization"]),
        ("subscriptions_deleteInstance", &["monetization"]),
        ("subscriptions_getInstance", &["monetization"]),
        ("subscriptions_introductoryOffers_getToManyRelated", &["monetization"]),
        ("subscriptions_planAvailabilities_getToManyRelated", &["monetization"]),
        ("subscriptions_pricePoints_getToManyRelated", &["pricing"]),
        ("subscriptions_prices_deleteToManyRelationship", &["pricing"]),
        ("subscriptions_prices_getToManyRelated", &["pricing"]),
        ("subscriptions_promotionalOffers_getToManyRelated", &["monetization"]),
        ("subscriptions_subscriptionLocalizations_getToManyRelated", &["monetization"]),
        ("subscriptions_updateInstance", &["monetization"]),
        ("subscriptionSubmissions_createInstance", &["monetization"]),
        ("territories_getCollection", &["pricing"]),
        ("territoryAvailabilities_updateInstance", &["pricing"]),
    ]
}
