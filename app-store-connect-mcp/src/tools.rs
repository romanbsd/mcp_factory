use mcp_factory_core::ExecutionKind;
use mcp_factory_core::ParamBinding;
use mcp_factory_core::ParamLocation;
use mcp_factory_core::RestOperation;
use mcp_factory_core::ToolHints;
use mcp_factory_core::ToolSpec;
pub fn build_tools() -> Vec<ToolSpec> {
    vec![
        ToolSpec {
            name: "appStoreReviewDetails_createInstance".to_string(),
            description: "Generated from OpenAPI operation".to_string(),
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
            name: "appStoreReviewDetails_updateInstance".to_string(),
            description: "Generated from OpenAPI operation".to_string(),
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
            description: "Generated from OpenAPI operation".to_string(),
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
            name: "appStoreVersionLocalizations_updateInstance".to_string(),
            description: "Generated from OpenAPI operation".to_string(),
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
            name: "appStoreVersionPhasedReleases_createInstance".to_string(),
            description: "Generated from OpenAPI operation".to_string(),
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
            description: "Generated from OpenAPI operation".to_string(),
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
            name: "appStoreVersionReleaseRequests_createInstance".to_string(),
            description: "Generated from OpenAPI operation".to_string(),
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
            description: "Generated from OpenAPI operation".to_string(),
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
            description: "Generated from OpenAPI operation".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[alternativeDistributionPackages]":{"items":{"enum":["sourceFileChecksum","versions"],"type":"string"},"type":"array"},"fields[appClipDefaultExperiences]":{"items":{"enum":["action","appClip","releaseWithAppStoreVersion","appClipDefaultExperienceLocalizations","appClipAppStoreReviewDetail"],"type":"string"},"type":"array"},"fields[appStoreReviewDetails]":{"items":{"enum":["contactFirstName","contactLastName","contactPhone","contactEmail","demoAccountName","demoAccountPassword","demoAccountRequired","notes","appStoreVersion","appStoreReviewAttachments"],"type":"string"},"type":"array"},"fields[appStoreVersionExperiments]":{"items":{"enum":["name","trafficProportion","state","reviewRequired","startDate","endDate","appStoreVersion","appStoreVersionExperimentTreatments","platform","app","latestControlVersion","controlVersions"],"type":"string"},"type":"array"},"fields[appStoreVersionLocalizations]":{"items":{"enum":["description","locale","keywords","marketingUrl","promotionalText","supportUrl","whatsNew","appStoreVersion","appScreenshotSets","appPreviewSets","searchKeywords"],"type":"string"},"type":"array"},"fields[appStoreVersionPhasedReleases]":{"items":{"enum":["phasedReleaseState","startDate","totalPauseDuration","currentDayNumber"],"type":"string"},"type":"array"},"fields[appStoreVersionSubmissions]":{"items":{"enum":["appStoreVersion"],"type":"string"},"type":"array"},"fields[appStoreVersions]":{"items":{"enum":["platform","versionString","appStoreState","appVersionState","copyright","reviewType","releaseType","earliestReleaseDate","usesIdfa","downloadable","createdDate","app","appStoreVersionLocalizations","build","appStoreVersionPhasedRelease","gameCenterAppVersion","routingAppCoverage","appStoreReviewDetail","appStoreVersionSubmission","appClipDefaultExperience","appStoreVersionExperiments","appStoreVersionExperimentsV2","customerReviews","alternativeDistributionPackage"],"type":"string"},"type":"array"},"fields[apps]":{"items":{"enum":["accessibilityUrl","name","bundleId","sku","primaryLocale","isOrEverWasMadeForKids","subscriptionStatusUrl","subscriptionStatusUrlVersion","subscriptionStatusUrlForSandbox","subscriptionStatusUrlVersionForSandbox","contentRightsDeclaration","streamlinedPurchasingEnabled","accessibilityDeclarations","appEncryptionDeclarations","appStoreIcon","ciProduct","betaTesters","betaGroups","appStoreVersions","appTags","preReleaseVersions","betaAppLocalizations","builds","betaLicenseAgreement","betaAppReviewDetail","appInfos","appClips","appPricePoints","endUserLicenseAgreement","appPriceSchedule","appAvailabilityV2","inAppPurchases","subscriptionGroups","gameCenterEnabledVersions","performanceOverviews","perfPowerMetrics","appCustomProductPages","inAppPurchasesV2","promotedPurchases","appEvents","reviewSubmissions","subscriptionGracePeriod","customerReviews","customerReviewSummarizations","gameCenterDetail","appStoreVersionExperimentsV2","alternativeDistributionKey","analyticsReportRequests","marketplaceSearchDetail","buildUploads","backgroundAssets","betaFeedbackScreenshotSubmissions","betaFeedbackCrashSubmissions","searchKeywords","webhooks","androidToIosAppMappingDetails"],"type":"string"},"type":"array"},"fields[builds]":{"items":{"enum":["version","uploadedDate","expirationDate","expired","minOsVersion","lsMinimumSystemVersion","computedMinMacOsVersion","computedMinVisionOsVersion","iconAssetToken","processingState","buildAudienceType","usesNonExemptEncryption","preReleaseVersion","individualTesters","betaGroups","betaBuildLocalizations","appEncryptionDeclaration","betaAppReviewSubmission","app","buildBetaDetail","appStoreVersion","icons","buildBundles","buildUpload","perfPowerMetrics","diagnosticSignatures"],"type":"string"},"type":"array"},"fields[gameCenterAppVersions]":{"items":{"enum":["enabled","compatibilityVersions","appStoreVersion"],"type":"string"},"type":"array"},"fields[routingAppCoverages]":{"items":{"enum":["fileSize","fileName","sourceFileChecksum","uploadOperations","assetDeliveryState","appStoreVersion"],"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["app","appStoreVersionLocalizations","build","appStoreVersionPhasedRelease","gameCenterAppVersion","routingAppCoverage","appStoreReviewDetail","appStoreVersionSubmission","appClipDefaultExperience","appStoreVersionExperiments","appStoreVersionExperimentsV2","alternativeDistributionPackage"],"type":"string"},"type":"array"},"limit[appStoreVersionExperimentsV2]":{"maximum":50,"type":"integer"},"limit[appStoreVersionExperiments]":{"maximum":50,"type":"integer"},"limit[appStoreVersionLocalizations]":{"maximum":50,"type":"integer"}},"required":["id"],"type":"object"}"#)
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
            description: "Generated from OpenAPI operation".to_string(),
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
            name: "apps_getCollection".to_string(),
            description: "Generated from OpenAPI operation".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"exists[gameCenterEnabledVersions]":{"type":"boolean"},"fields[androidToIosAppMappingDetails]":{"items":{"enum":["packageName","appSigningKeyPublicCertificateSha256Fingerprints"],"type":"string"},"type":"array"},"fields[appClips]":{"items":{"enum":["bundleId","app","appClipDefaultExperiences","appClipAdvancedExperiences"],"type":"string"},"type":"array"},"fields[appCustomProductPages]":{"items":{"enum":["name","url","visible","app","appCustomProductPageVersions"],"type":"string"},"type":"array"},"fields[appEncryptionDeclarations]":{"items":{"enum":["appDescription","createdDate","usesEncryption","exempt","containsProprietaryCryptography","containsThirdPartyCryptography","availableOnFrenchStore","platform","uploadedDate","documentUrl","documentName","documentType","appEncryptionDeclarationState","codeValue","app","builds","appEncryptionDeclarationDocument"],"type":"string"},"type":"array"},"fields[appEvents]":{"items":{"enum":["referenceName","badge","eventState","deepLink","purchaseRequirement","primaryLocale","priority","purpose","territorySchedules","archivedTerritorySchedules","localizations"],"type":"string"},"type":"array"},"fields[appInfos]":{"items":{"enum":["appStoreState","state","appStoreAgeRating","australiaAgeRating","brazilAgeRating","brazilAgeRatingV2","franceAgeRating","koreaAgeRating","app","ageRatingDeclaration","appInfoLocalizations","primaryCategory","primarySubcategoryOne","primarySubcategoryTwo","secondaryCategory","secondarySubcategoryOne","secondarySubcategoryTwo","territoryAgeRatings"],"type":"string"},"type":"array"},"fields[appStoreVersionExperiments]":{"items":{"enum":["name","platform","trafficProportion","state","reviewRequired","startDate","endDate","app","latestControlVersion","controlVersions","appStoreVersionExperimentTreatments"],"type":"string"},"type":"array"},"fields[appStoreVersions]":{"items":{"enum":["platform","versionString","appStoreState","appVersionState","copyright","reviewType","releaseType","earliestReleaseDate","usesIdfa","downloadable","createdDate","app","appStoreVersionLocalizations","build","appStoreVersionPhasedRelease","gameCenterAppVersion","routingAppCoverage","appStoreReviewDetail","appStoreVersionSubmission","appClipDefaultExperience","appStoreVersionExperiments","appStoreVersionExperimentsV2","customerReviews","alternativeDistributionPackage"],"type":"string"},"type":"array"},"fields[apps]":{"items":{"enum":["accessibilityUrl","name","bundleId","sku","primaryLocale","isOrEverWasMadeForKids","subscriptionStatusUrl","subscriptionStatusUrlVersion","subscriptionStatusUrlForSandbox","subscriptionStatusUrlVersionForSandbox","contentRightsDeclaration","streamlinedPurchasingEnabled","accessibilityDeclarations","appEncryptionDeclarations","appStoreIcon","ciProduct","betaTesters","betaGroups","appStoreVersions","appTags","preReleaseVersions","betaAppLocalizations","builds","betaLicenseAgreement","betaAppReviewDetail","appInfos","appClips","appPricePoints","endUserLicenseAgreement","appPriceSchedule","appAvailabilityV2","inAppPurchases","subscriptionGroups","gameCenterEnabledVersions","performanceOverviews","perfPowerMetrics","appCustomProductPages","inAppPurchasesV2","promotedPurchases","appEvents","reviewSubmissions","subscriptionGracePeriod","customerReviews","customerReviewSummarizations","gameCenterDetail","appStoreVersionExperimentsV2","alternativeDistributionKey","analyticsReportRequests","marketplaceSearchDetail","buildUploads","backgroundAssets","betaFeedbackScreenshotSubmissions","betaFeedbackCrashSubmissions","searchKeywords","webhooks","androidToIosAppMappingDetails"],"type":"string"},"type":"array"},"fields[betaAppLocalizations]":{"items":{"enum":["feedbackEmail","marketingUrl","privacyPolicyUrl","tvOsPrivacyPolicy","description","locale","app"],"type":"string"},"type":"array"},"fields[betaAppReviewDetails]":{"items":{"enum":["contactFirstName","contactLastName","contactPhone","contactEmail","demoAccountName","demoAccountPassword","demoAccountRequired","notes","app"],"type":"string"},"type":"array"},"fields[betaGroups]":{"items":{"enum":["name","createdDate","isInternalGroup","hasAccessToAllBuilds","publicLinkEnabled","publicLinkId","publicLinkLimitEnabled","publicLinkLimit","publicLink","feedbackEnabled","iosBuildsAvailableForAppleSiliconMac","iosBuildsAvailableForAppleVision","app","builds","betaTesters","betaRecruitmentCriteria","betaRecruitmentCriterionCompatibleBuildCheck"],"type":"string"},"type":"array"},"fields[betaLicenseAgreements]":{"items":{"enum":["agreementText","app"],"type":"string"},"type":"array"},"fields[buildIcons]":{"items":{"enum":["iconAsset","iconType","masked","name"],"type":"string"},"type":"array"},"fields[builds]":{"items":{"enum":["version","uploadedDate","expirationDate","expired","minOsVersion","lsMinimumSystemVersion","computedMinMacOsVersion","computedMinVisionOsVersion","iconAssetToken","processingState","buildAudienceType","usesNonExemptEncryption","preReleaseVersion","individualTesters","betaGroups","betaBuildLocalizations","appEncryptionDeclaration","betaAppReviewSubmission","app","buildBetaDetail","appStoreVersion","icons","buildBundles","buildUpload","perfPowerMetrics","diagnosticSignatures"],"type":"string"},"type":"array"},"fields[ciProducts]":{"items":{"enum":["name","createdDate","productType","app","bundleId","workflows","primaryRepositories","additionalRepositories","buildRuns"],"type":"string"},"type":"array"},"fields[endUserLicenseAgreements]":{"items":{"enum":["agreementText","app","territories"],"type":"string"},"type":"array"},"fields[gameCenterDetails]":{"items":{"enum":["arcadeEnabled","challengeEnabled","app","gameCenterAppVersions","gameCenterGroup","gameCenterLeaderboards","gameCenterLeaderboardsV2","gameCenterLeaderboardSets","gameCenterLeaderboardSetsV2","gameCenterAchievements","gameCenterAchievementsV2","gameCenterActivities","gameCenterChallenges","defaultLeaderboard","defaultLeaderboardV2","defaultGroupLeaderboard","defaultGroupLeaderboardV2","achievementReleases","activityReleases","challengeReleases","leaderboardReleases","leaderboardSetReleases","blockedPlayers","challengesMinimumPlatformVersions"],"type":"string"},"type":"array"},"fields[gameCenterEnabledVersions]":{"items":{"enum":["platform","versionString","iconAsset","compatibleVersions","app"],"type":"string"},"type":"array"},"fields[inAppPurchases]":{"items":{"enum":["referenceName","productId","inAppPurchaseType","state","apps","name","reviewNote","familySharable","contentHosting","inAppPurchaseLocalizations","pricePoints","content","appStoreReviewScreenshot","promotedPurchase","iapPriceSchedule","inAppPurchaseAvailability","images","offerCodes","versions"],"type":"string"},"type":"array"},"fields[preReleaseVersions]":{"items":{"enum":["version","platform","builds","app"],"type":"string"},"type":"array"},"fields[promotedPurchases]":{"items":{"enum":["visibleForAllUsers","enabled","state","inAppPurchaseV2","subscription"],"type":"string"},"type":"array"},"fields[reviewSubmissions]":{"items":{"enum":["platform","submittedDate","state","app","items","appStoreVersionForReview","submittedByActor","lastUpdatedByActor"],"type":"string"},"type":"array"},"fields[subscriptionGracePeriods]":{"items":{"enum":["optIn","sandboxOptIn","duration","renewalType"],"type":"string"},"type":"array"},"fields[subscriptionGroups]":{"items":{"enum":["referenceName","subscriptions","subscriptionGroupLocalizations","versions"],"type":"string"},"type":"array"},"filter[appStoreVersions.appStoreState]":{"items":{"enum":["ACCEPTED","DEVELOPER_REMOVED_FROM_SALE","DEVELOPER_REJECTED","IN_REVIEW","INVALID_BINARY","METADATA_REJECTED","PENDING_APPLE_RELEASE","PENDING_CONTRACT","PENDING_DEVELOPER_RELEASE","PREPARE_FOR_SUBMISSION","PREORDER_READY_FOR_SALE","PROCESSING_FOR_APP_STORE","READY_FOR_REVIEW","READY_FOR_SALE","REJECTED","REMOVED_FROM_SALE","WAITING_FOR_EXPORT_COMPLIANCE","WAITING_FOR_REVIEW","REPLACED_WITH_NEW_VERSION","NOT_APPLICABLE"],"type":"string"},"type":"array"},"filter[appStoreVersions.appVersionState]":{"items":{"enum":["ACCEPTED","DEVELOPER_REJECTED","IN_REVIEW","INVALID_BINARY","METADATA_REJECTED","PENDING_APPLE_RELEASE","PENDING_DEVELOPER_RELEASE","PREPARE_FOR_SUBMISSION","PROCESSING_FOR_DISTRIBUTION","READY_FOR_DISTRIBUTION","READY_FOR_REVIEW","REJECTED","REPLACED_WITH_NEW_VERSION","WAITING_FOR_EXPORT_COMPLIANCE","WAITING_FOR_REVIEW"],"type":"string"},"type":"array"},"filter[appStoreVersions.platform]":{"items":{"enum":["IOS","MAC_OS","TV_OS","VISION_OS"],"type":"string"},"type":"array"},"filter[appStoreVersions]":{"items":{"type":"string"},"type":"array"},"filter[bundleId]":{"items":{"type":"string"},"type":"array"},"filter[id]":{"items":{"type":"string"},"type":"array"},"filter[name]":{"items":{"type":"string"},"type":"array"},"filter[reviewSubmissions.platform]":{"items":{"enum":["IOS","MAC_OS","TV_OS","VISION_OS"],"type":"string"},"type":"array"},"filter[reviewSubmissions.state]":{"items":{"enum":["READY_FOR_REVIEW","WAITING_FOR_REVIEW","IN_REVIEW","UNRESOLVED_ISSUES","CANCELING","COMPLETING","COMPLETE"],"type":"string"},"type":"array"},"filter[sku]":{"items":{"type":"string"},"type":"array"},"include":{"items":{"enum":["appEncryptionDeclarations","appStoreIcon","ciProduct","betaGroups","appStoreVersions","preReleaseVersions","betaAppLocalizations","builds","betaLicenseAgreement","betaAppReviewDetail","appInfos","appClips","endUserLicenseAgreement","inAppPurchases","subscriptionGroups","gameCenterEnabledVersions","appCustomProductPages","inAppPurchasesV2","promotedPurchases","appEvents","reviewSubmissions","subscriptionGracePeriod","gameCenterDetail","appStoreVersionExperimentsV2","androidToIosAppMappingDetails"],"type":"string"},"type":"array"},"limit":{"maximum":200,"type":"integer"},"limit[androidToIosAppMappingDetails]":{"maximum":50,"type":"integer"},"limit[appClips]":{"maximum":50,"type":"integer"},"limit[appCustomProductPages]":{"maximum":50,"type":"integer"},"limit[appEncryptionDeclarations]":{"maximum":50,"type":"integer"},"limit[appEvents]":{"maximum":50,"type":"integer"},"limit[appInfos]":{"maximum":50,"type":"integer"},"limit[appStoreVersionExperimentsV2]":{"maximum":50,"type":"integer"},"limit[appStoreVersions]":{"maximum":50,"type":"integer"},"limit[betaAppLocalizations]":{"maximum":50,"type":"integer"},"limit[betaGroups]":{"maximum":50,"type":"integer"},"limit[builds]":{"maximum":50,"type":"integer"},"limit[gameCenterEnabledVersions]":{"maximum":50,"type":"integer"},"limit[inAppPurchasesV2]":{"maximum":50,"type":"integer"},"limit[inAppPurchases]":{"maximum":50,"type":"integer"},"limit[preReleaseVersions]":{"maximum":50,"type":"integer"},"limit[promotedPurchases]":{"maximum":50,"type":"integer"},"limit[reviewSubmissions]":{"maximum":50,"type":"integer"},"limit[subscriptionGroups]":{"maximum":50,"type":"integer"},"sort":{"items":{"enum":["name","-name","bundleId","-bundleId","sku","-sku"],"type":"string"},"type":"array"}},"type":"object"}"#)
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
            description: "Generated from OpenAPI operation".to_string(),
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
            name: "betaBuildLocalizations_createInstance".to_string(),
            description: "Generated from OpenAPI operation".to_string(),
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
            name: "betaBuildLocalizations_updateInstance".to_string(),
            description: "Generated from OpenAPI operation".to_string(),
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
            name: "betaGroups_getCollection".to_string(),
            description: "Generated from OpenAPI operation".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[apps]":{"items":{"enum":["accessibilityUrl","name","bundleId","sku","primaryLocale","isOrEverWasMadeForKids","subscriptionStatusUrl","subscriptionStatusUrlVersion","subscriptionStatusUrlForSandbox","subscriptionStatusUrlVersionForSandbox","contentRightsDeclaration","streamlinedPurchasingEnabled","accessibilityDeclarations","appEncryptionDeclarations","appStoreIcon","ciProduct","betaTesters","betaGroups","appStoreVersions","appTags","preReleaseVersions","betaAppLocalizations","builds","betaLicenseAgreement","betaAppReviewDetail","appInfos","appClips","appPricePoints","endUserLicenseAgreement","appPriceSchedule","appAvailabilityV2","inAppPurchases","subscriptionGroups","gameCenterEnabledVersions","performanceOverviews","perfPowerMetrics","appCustomProductPages","inAppPurchasesV2","promotedPurchases","appEvents","reviewSubmissions","subscriptionGracePeriod","customerReviews","customerReviewSummarizations","gameCenterDetail","appStoreVersionExperimentsV2","alternativeDistributionKey","analyticsReportRequests","marketplaceSearchDetail","buildUploads","backgroundAssets","betaFeedbackScreenshotSubmissions","betaFeedbackCrashSubmissions","searchKeywords","webhooks","androidToIosAppMappingDetails"],"type":"string"},"type":"array"},"fields[betaGroups]":{"items":{"enum":["name","createdDate","isInternalGroup","hasAccessToAllBuilds","publicLinkEnabled","publicLinkId","publicLinkLimitEnabled","publicLinkLimit","publicLink","feedbackEnabled","iosBuildsAvailableForAppleSiliconMac","iosBuildsAvailableForAppleVision","app","builds","betaTesters","betaRecruitmentCriteria","betaRecruitmentCriterionCompatibleBuildCheck"],"type":"string"},"type":"array"},"fields[betaRecruitmentCriteria]":{"items":{"enum":["lastModifiedDate","deviceFamilyOsVersionFilters"],"type":"string"},"type":"array"},"fields[betaTesters]":{"items":{"enum":["firstName","lastName","email","inviteType","state","appDevices","apps","betaGroups","builds"],"type":"string"},"type":"array"},"fields[builds]":{"items":{"enum":["version","uploadedDate","expirationDate","expired","minOsVersion","lsMinimumSystemVersion","computedMinMacOsVersion","computedMinVisionOsVersion","iconAssetToken","processingState","buildAudienceType","usesNonExemptEncryption","preReleaseVersion","individualTesters","betaGroups","betaBuildLocalizations","appEncryptionDeclaration","betaAppReviewSubmission","app","buildBetaDetail","appStoreVersion","icons","buildBundles","buildUpload","perfPowerMetrics","diagnosticSignatures"],"type":"string"},"type":"array"},"filter[app]":{"items":{"type":"string"},"type":"array"},"filter[builds]":{"items":{"type":"string"},"type":"array"},"filter[id]":{"items":{"type":"string"},"type":"array"},"filter[isInternalGroup]":{"items":{"type":"string"},"type":"array"},"filter[name]":{"items":{"type":"string"},"type":"array"},"filter[publicLinkEnabled]":{"items":{"type":"string"},"type":"array"},"filter[publicLinkLimitEnabled]":{"items":{"type":"string"},"type":"array"},"filter[publicLink]":{"items":{"type":"string"},"type":"array"},"include":{"items":{"enum":["app","builds","betaTesters","betaRecruitmentCriteria"],"type":"string"},"type":"array"},"limit":{"maximum":200,"type":"integer"},"limit[betaTesters]":{"maximum":50,"type":"integer"},"limit[builds]":{"maximum":1000,"type":"integer"},"sort":{"items":{"enum":["name","-name","createdDate","-createdDate","publicLinkEnabled","-publicLinkEnabled","publicLinkLimit","-publicLinkLimit"],"type":"string"},"type":"array"}},"type":"object"}"#)
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
            name: "builds_getCollection".to_string(),
            description: "Generated from OpenAPI operation".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"exists[usesNonExemptEncryption]":{"type":"boolean"},"fields[appEncryptionDeclarations]":{"items":{"enum":["appDescription","createdDate","usesEncryption","exempt","containsProprietaryCryptography","containsThirdPartyCryptography","availableOnFrenchStore","platform","uploadedDate","documentUrl","documentName","documentType","appEncryptionDeclarationState","codeValue","app","builds","appEncryptionDeclarationDocument"],"type":"string"},"type":"array"},"fields[appStoreVersions]":{"items":{"enum":["platform","versionString","appStoreState","appVersionState","copyright","reviewType","releaseType","earliestReleaseDate","usesIdfa","downloadable","createdDate","app","appStoreVersionLocalizations","build","appStoreVersionPhasedRelease","gameCenterAppVersion","routingAppCoverage","appStoreReviewDetail","appStoreVersionSubmission","appClipDefaultExperience","appStoreVersionExperiments","appStoreVersionExperimentsV2","customerReviews","alternativeDistributionPackage"],"type":"string"},"type":"array"},"fields[apps]":{"items":{"enum":["accessibilityUrl","name","bundleId","sku","primaryLocale","isOrEverWasMadeForKids","subscriptionStatusUrl","subscriptionStatusUrlVersion","subscriptionStatusUrlForSandbox","subscriptionStatusUrlVersionForSandbox","contentRightsDeclaration","streamlinedPurchasingEnabled","accessibilityDeclarations","appEncryptionDeclarations","appStoreIcon","ciProduct","betaTesters","betaGroups","appStoreVersions","appTags","preReleaseVersions","betaAppLocalizations","builds","betaLicenseAgreement","betaAppReviewDetail","appInfos","appClips","appPricePoints","endUserLicenseAgreement","appPriceSchedule","appAvailabilityV2","inAppPurchases","subscriptionGroups","gameCenterEnabledVersions","performanceOverviews","perfPowerMetrics","appCustomProductPages","inAppPurchasesV2","promotedPurchases","appEvents","reviewSubmissions","subscriptionGracePeriod","customerReviews","customerReviewSummarizations","gameCenterDetail","appStoreVersionExperimentsV2","alternativeDistributionKey","analyticsReportRequests","marketplaceSearchDetail","buildUploads","backgroundAssets","betaFeedbackScreenshotSubmissions","betaFeedbackCrashSubmissions","searchKeywords","webhooks","androidToIosAppMappingDetails"],"type":"string"},"type":"array"},"fields[betaAppReviewSubmissions]":{"items":{"enum":["betaReviewState","submittedDate","build"],"type":"string"},"type":"array"},"fields[betaBuildLocalizations]":{"items":{"enum":["whatsNew","locale","build"],"type":"string"},"type":"array"},"fields[betaGroups]":{"items":{"enum":["name","createdDate","isInternalGroup","hasAccessToAllBuilds","publicLinkEnabled","publicLinkId","publicLinkLimitEnabled","publicLinkLimit","publicLink","feedbackEnabled","iosBuildsAvailableForAppleSiliconMac","iosBuildsAvailableForAppleVision","app","builds","betaTesters","betaRecruitmentCriteria","betaRecruitmentCriterionCompatibleBuildCheck"],"type":"string"},"type":"array"},"fields[betaTesters]":{"items":{"enum":["firstName","lastName","email","inviteType","state","appDevices","apps","betaGroups","builds"],"type":"string"},"type":"array"},"fields[buildBetaDetails]":{"items":{"enum":["autoNotifyEnabled","internalBuildState","externalBuildState","build"],"type":"string"},"type":"array"},"fields[buildBundles]":{"items":{"enum":["bundleId","bundleType","sdkBuild","platformBuild","fileName","hasSirikit","hasOnDemandResources","hasPrerenderedIcon","usesLocationServices","isIosBuildMacAppStoreCompatible","includesSymbols","dSYMUrl","supportedArchitectures","requiredCapabilities","deviceProtocols","locales","entitlements","baDownloadAllowance","baMaxInstallSize","minimumOsVersion","appClipDomainCacheStatus","appClipDomainDebugStatus","betaAppClipInvocations","buildBundleFileSizes"],"type":"string"},"type":"array"},"fields[buildIcons]":{"items":{"enum":["iconAsset","iconType","masked","name"],"type":"string"},"type":"array"},"fields[buildUploads]":{"items":{"enum":["cfBundleShortVersionString","cfBundleVersion","createdDate","state","platform","uploadedDate","build","assetFile","assetDescriptionFile","assetSpiFile","buildUploadFiles"],"type":"string"},"type":"array"},"fields[builds]":{"items":{"enum":["version","uploadedDate","expirationDate","expired","minOsVersion","lsMinimumSystemVersion","computedMinMacOsVersion","computedMinVisionOsVersion","iconAssetToken","processingState","buildAudienceType","usesNonExemptEncryption","preReleaseVersion","individualTesters","betaGroups","betaBuildLocalizations","appEncryptionDeclaration","betaAppReviewSubmission","app","buildBetaDetail","appStoreVersion","icons","buildBundles","buildUpload","perfPowerMetrics","diagnosticSignatures"],"type":"string"},"type":"array"},"fields[preReleaseVersions]":{"items":{"enum":["version","platform","builds","app"],"type":"string"},"type":"array"},"filter[appStoreVersion]":{"items":{"type":"string"},"type":"array"},"filter[app]":{"items":{"type":"string"},"type":"array"},"filter[betaAppReviewSubmission.betaReviewState]":{"items":{"enum":["WAITING_FOR_REVIEW","IN_REVIEW","REJECTED","APPROVED"],"type":"string"},"type":"array"},"filter[betaGroups]":{"items":{"type":"string"},"type":"array"},"filter[buildAudienceType]":{"items":{"enum":["INTERNAL_ONLY","APP_STORE_ELIGIBLE"],"type":"string"},"type":"array"},"filter[expired]":{"items":{"type":"string"},"type":"array"},"filter[id]":{"items":{"type":"string"},"type":"array"},"filter[preReleaseVersion.platform]":{"items":{"enum":["IOS","MAC_OS","TV_OS","VISION_OS"],"type":"string"},"type":"array"},"filter[preReleaseVersion.version]":{"items":{"type":"string"},"type":"array"},"filter[preReleaseVersion]":{"items":{"type":"string"},"type":"array"},"filter[processingState]":{"items":{"enum":["PROCESSING","FAILED","INVALID","VALID"],"type":"string"},"type":"array"},"filter[usesNonExemptEncryption]":{"items":{"type":"string"},"type":"array"},"filter[version]":{"items":{"type":"string"},"type":"array"},"include":{"items":{"enum":["preReleaseVersion","individualTesters","betaGroups","betaBuildLocalizations","appEncryptionDeclaration","betaAppReviewSubmission","app","buildBetaDetail","appStoreVersion","icons","buildBundles","buildUpload"],"type":"string"},"type":"array"},"limit":{"maximum":200,"type":"integer"},"limit[betaBuildLocalizations]":{"maximum":50,"type":"integer"},"limit[betaGroups]":{"maximum":50,"type":"integer"},"limit[buildBundles]":{"maximum":50,"type":"integer"},"limit[icons]":{"maximum":50,"type":"integer"},"limit[individualTesters]":{"maximum":50,"type":"integer"},"sort":{"items":{"enum":["version","-version","uploadedDate","-uploadedDate","preReleaseVersion","-preReleaseVersion"],"type":"string"},"type":"array"}},"type":"object"}"#)
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
            description: "Generated from OpenAPI operation".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[appEncryptionDeclarations]":{"items":{"enum":["appDescription","createdDate","usesEncryption","exempt","containsProprietaryCryptography","containsThirdPartyCryptography","availableOnFrenchStore","platform","uploadedDate","documentUrl","documentName","documentType","appEncryptionDeclarationState","codeValue","app","builds","appEncryptionDeclarationDocument"],"type":"string"},"type":"array"},"fields[appStoreVersions]":{"items":{"enum":["platform","versionString","appStoreState","appVersionState","copyright","reviewType","releaseType","earliestReleaseDate","usesIdfa","downloadable","createdDate","app","appStoreVersionLocalizations","build","appStoreVersionPhasedRelease","gameCenterAppVersion","routingAppCoverage","appStoreReviewDetail","appStoreVersionSubmission","appClipDefaultExperience","appStoreVersionExperiments","appStoreVersionExperimentsV2","customerReviews","alternativeDistributionPackage"],"type":"string"},"type":"array"},"fields[apps]":{"items":{"enum":["accessibilityUrl","name","bundleId","sku","primaryLocale","isOrEverWasMadeForKids","subscriptionStatusUrl","subscriptionStatusUrlVersion","subscriptionStatusUrlForSandbox","subscriptionStatusUrlVersionForSandbox","contentRightsDeclaration","streamlinedPurchasingEnabled","accessibilityDeclarations","appEncryptionDeclarations","appStoreIcon","ciProduct","betaTesters","betaGroups","appStoreVersions","appTags","preReleaseVersions","betaAppLocalizations","builds","betaLicenseAgreement","betaAppReviewDetail","appInfos","appClips","appPricePoints","endUserLicenseAgreement","appPriceSchedule","appAvailabilityV2","inAppPurchases","subscriptionGroups","gameCenterEnabledVersions","performanceOverviews","perfPowerMetrics","appCustomProductPages","inAppPurchasesV2","promotedPurchases","appEvents","reviewSubmissions","subscriptionGracePeriod","customerReviews","customerReviewSummarizations","gameCenterDetail","appStoreVersionExperimentsV2","alternativeDistributionKey","analyticsReportRequests","marketplaceSearchDetail","buildUploads","backgroundAssets","betaFeedbackScreenshotSubmissions","betaFeedbackCrashSubmissions","searchKeywords","webhooks","androidToIosAppMappingDetails"],"type":"string"},"type":"array"},"fields[betaAppReviewSubmissions]":{"items":{"enum":["betaReviewState","submittedDate","build"],"type":"string"},"type":"array"},"fields[betaBuildLocalizations]":{"items":{"enum":["whatsNew","locale","build"],"type":"string"},"type":"array"},"fields[betaGroups]":{"items":{"enum":["name","createdDate","isInternalGroup","hasAccessToAllBuilds","publicLinkEnabled","publicLinkId","publicLinkLimitEnabled","publicLinkLimit","publicLink","feedbackEnabled","iosBuildsAvailableForAppleSiliconMac","iosBuildsAvailableForAppleVision","app","builds","betaTesters","betaRecruitmentCriteria","betaRecruitmentCriterionCompatibleBuildCheck"],"type":"string"},"type":"array"},"fields[betaTesters]":{"items":{"enum":["firstName","lastName","email","inviteType","state","appDevices","apps","betaGroups","builds"],"type":"string"},"type":"array"},"fields[buildBetaDetails]":{"items":{"enum":["autoNotifyEnabled","internalBuildState","externalBuildState","build"],"type":"string"},"type":"array"},"fields[buildBundles]":{"items":{"enum":["bundleId","bundleType","sdkBuild","platformBuild","fileName","hasSirikit","hasOnDemandResources","hasPrerenderedIcon","usesLocationServices","isIosBuildMacAppStoreCompatible","includesSymbols","dSYMUrl","supportedArchitectures","requiredCapabilities","deviceProtocols","locales","entitlements","baDownloadAllowance","baMaxInstallSize","minimumOsVersion","appClipDomainCacheStatus","appClipDomainDebugStatus","betaAppClipInvocations","buildBundleFileSizes"],"type":"string"},"type":"array"},"fields[buildIcons]":{"items":{"enum":["iconAsset","iconType","masked","name"],"type":"string"},"type":"array"},"fields[buildUploads]":{"items":{"enum":["cfBundleShortVersionString","cfBundleVersion","createdDate","state","platform","uploadedDate","build","assetFile","assetDescriptionFile","assetSpiFile","buildUploadFiles"],"type":"string"},"type":"array"},"fields[builds]":{"items":{"enum":["version","uploadedDate","expirationDate","expired","minOsVersion","lsMinimumSystemVersion","computedMinMacOsVersion","computedMinVisionOsVersion","iconAssetToken","processingState","buildAudienceType","usesNonExemptEncryption","preReleaseVersion","individualTesters","betaGroups","betaBuildLocalizations","appEncryptionDeclaration","betaAppReviewSubmission","app","buildBetaDetail","appStoreVersion","icons","buildBundles","buildUpload","perfPowerMetrics","diagnosticSignatures"],"type":"string"},"type":"array"},"fields[preReleaseVersions]":{"items":{"enum":["version","platform","builds","app"],"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["preReleaseVersion","individualTesters","betaGroups","betaBuildLocalizations","appEncryptionDeclaration","betaAppReviewSubmission","app","buildBetaDetail","appStoreVersion","icons","buildBundles","buildUpload"],"type":"string"},"type":"array"},"limit[betaBuildLocalizations]":{"maximum":50,"type":"integer"},"limit[betaGroups]":{"maximum":50,"type":"integer"},"limit[buildBundles]":{"maximum":50,"type":"integer"},"limit[icons]":{"maximum":50,"type":"integer"},"limit[individualTesters]":{"maximum":50,"type":"integer"}},"required":["id"],"type":"object"}"#)
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
            description: "Generated from OpenAPI operation".to_string(),
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
            name: "customerReviewResponses_createInstance".to_string(),
            description: "Generated from OpenAPI operation".to_string(),
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
            name: "reviewSubmissionItems_createInstance".to_string(),
            description: "Generated from OpenAPI operation".to_string(),
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
            name: "reviewSubmissions_getCollection".to_string(),
            description: "Generated from OpenAPI operation".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[actors]":{"items":{"enum":["actorType","userFirstName","userLastName","userEmail","apiKeyId"],"type":"string"},"type":"array"},"fields[appStoreVersions]":{"items":{"enum":["platform","versionString","appStoreState","appVersionState","copyright","reviewType","releaseType","earliestReleaseDate","usesIdfa","downloadable","createdDate","app","appStoreVersionLocalizations","build","appStoreVersionPhasedRelease","gameCenterAppVersion","routingAppCoverage","appStoreReviewDetail","appStoreVersionSubmission","appClipDefaultExperience","appStoreVersionExperiments","appStoreVersionExperimentsV2","customerReviews","alternativeDistributionPackage"],"type":"string"},"type":"array"},"fields[apps]":{"items":{"enum":["accessibilityUrl","name","bundleId","sku","primaryLocale","isOrEverWasMadeForKids","subscriptionStatusUrl","subscriptionStatusUrlVersion","subscriptionStatusUrlForSandbox","subscriptionStatusUrlVersionForSandbox","contentRightsDeclaration","streamlinedPurchasingEnabled","accessibilityDeclarations","appEncryptionDeclarations","appStoreIcon","ciProduct","betaTesters","betaGroups","appStoreVersions","appTags","preReleaseVersions","betaAppLocalizations","builds","betaLicenseAgreement","betaAppReviewDetail","appInfos","appClips","appPricePoints","endUserLicenseAgreement","appPriceSchedule","appAvailabilityV2","inAppPurchases","subscriptionGroups","gameCenterEnabledVersions","performanceOverviews","perfPowerMetrics","appCustomProductPages","inAppPurchasesV2","promotedPurchases","appEvents","reviewSubmissions","subscriptionGracePeriod","customerReviews","customerReviewSummarizations","gameCenterDetail","appStoreVersionExperimentsV2","alternativeDistributionKey","analyticsReportRequests","marketplaceSearchDetail","buildUploads","backgroundAssets","betaFeedbackScreenshotSubmissions","betaFeedbackCrashSubmissions","searchKeywords","webhooks","androidToIosAppMappingDetails"],"type":"string"},"type":"array"},"fields[reviewSubmissionItems]":{"items":{"enum":["state","appStoreVersion","appCustomProductPageVersion","appStoreVersionExperiment","appStoreVersionExperimentV2","appEvent","backgroundAssetVersion","gameCenterAchievementVersion","gameCenterActivityVersion","gameCenterChallengeVersion","gameCenterLeaderboardSetVersion","gameCenterLeaderboardVersion","inAppPurchaseVersion","subscriptionVersion","subscriptionGroupVersion"],"type":"string"},"type":"array"},"fields[reviewSubmissions]":{"items":{"enum":["platform","submittedDate","state","app","items","appStoreVersionForReview","submittedByActor","lastUpdatedByActor"],"type":"string"},"type":"array"},"filter[app]":{"items":{"type":"string"},"type":"array"},"filter[platform]":{"items":{"enum":["IOS","MAC_OS","TV_OS","VISION_OS"],"type":"string"},"type":"array"},"filter[state]":{"items":{"enum":["READY_FOR_REVIEW","WAITING_FOR_REVIEW","IN_REVIEW","UNRESOLVED_ISSUES","CANCELING","COMPLETING","COMPLETE"],"type":"string"},"type":"array"},"include":{"items":{"enum":["app","items","appStoreVersionForReview","submittedByActor","lastUpdatedByActor"],"type":"string"},"type":"array"},"limit":{"maximum":200,"type":"integer"},"limit[items]":{"maximum":50,"type":"integer"}},"required":["filter[app]"],"type":"object"}"#)
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
            description: "Generated from OpenAPI operation".to_string(),
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
            name: "reviewSubmissions_updateInstance".to_string(),
            description: "Generated from OpenAPI operation".to_string(),
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
            name: "appStoreVersions_build_updateToOneRelationship".to_string(),
            description: "Generated from OpenAPI operation".to_string(),
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
            name: "apps_appStoreVersions_getToManyRelated".to_string(),
            description: "Generated from OpenAPI operation".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"fields[alternativeDistributionPackages]":{"items":{"enum":["sourceFileChecksum","versions"],"type":"string"},"type":"array"},"fields[appClipDefaultExperiences]":{"items":{"enum":["action","appClip","releaseWithAppStoreVersion","appClipDefaultExperienceLocalizations","appClipAppStoreReviewDetail"],"type":"string"},"type":"array"},"fields[appStoreReviewDetails]":{"items":{"enum":["contactFirstName","contactLastName","contactPhone","contactEmail","demoAccountName","demoAccountPassword","demoAccountRequired","notes","appStoreVersion","appStoreReviewAttachments"],"type":"string"},"type":"array"},"fields[appStoreVersionExperiments]":{"items":{"enum":["name","trafficProportion","state","reviewRequired","startDate","endDate","appStoreVersion","appStoreVersionExperimentTreatments","platform","app","latestControlVersion","controlVersions"],"type":"string"},"type":"array"},"fields[appStoreVersionLocalizations]":{"items":{"enum":["description","locale","keywords","marketingUrl","promotionalText","supportUrl","whatsNew","appStoreVersion","appScreenshotSets","appPreviewSets","searchKeywords"],"type":"string"},"type":"array"},"fields[appStoreVersionPhasedReleases]":{"items":{"enum":["phasedReleaseState","startDate","totalPauseDuration","currentDayNumber"],"type":"string"},"type":"array"},"fields[appStoreVersionSubmissions]":{"items":{"enum":["appStoreVersion"],"type":"string"},"type":"array"},"fields[appStoreVersions]":{"items":{"enum":["platform","versionString","appStoreState","appVersionState","copyright","reviewType","releaseType","earliestReleaseDate","usesIdfa","downloadable","createdDate","app","appStoreVersionLocalizations","build","appStoreVersionPhasedRelease","gameCenterAppVersion","routingAppCoverage","appStoreReviewDetail","appStoreVersionSubmission","appClipDefaultExperience","appStoreVersionExperiments","appStoreVersionExperimentsV2","customerReviews","alternativeDistributionPackage"],"type":"string"},"type":"array"},"fields[apps]":{"items":{"enum":["accessibilityUrl","name","bundleId","sku","primaryLocale","isOrEverWasMadeForKids","subscriptionStatusUrl","subscriptionStatusUrlVersion","subscriptionStatusUrlForSandbox","subscriptionStatusUrlVersionForSandbox","contentRightsDeclaration","streamlinedPurchasingEnabled","accessibilityDeclarations","appEncryptionDeclarations","appStoreIcon","ciProduct","betaTesters","betaGroups","appStoreVersions","appTags","preReleaseVersions","betaAppLocalizations","builds","betaLicenseAgreement","betaAppReviewDetail","appInfos","appClips","appPricePoints","endUserLicenseAgreement","appPriceSchedule","appAvailabilityV2","inAppPurchases","subscriptionGroups","gameCenterEnabledVersions","performanceOverviews","perfPowerMetrics","appCustomProductPages","inAppPurchasesV2","promotedPurchases","appEvents","reviewSubmissions","subscriptionGracePeriod","customerReviews","customerReviewSummarizations","gameCenterDetail","appStoreVersionExperimentsV2","alternativeDistributionKey","analyticsReportRequests","marketplaceSearchDetail","buildUploads","backgroundAssets","betaFeedbackScreenshotSubmissions","betaFeedbackCrashSubmissions","searchKeywords","webhooks","androidToIosAppMappingDetails"],"type":"string"},"type":"array"},"fields[builds]":{"items":{"enum":["version","uploadedDate","expirationDate","expired","minOsVersion","lsMinimumSystemVersion","computedMinMacOsVersion","computedMinVisionOsVersion","iconAssetToken","processingState","buildAudienceType","usesNonExemptEncryption","preReleaseVersion","individualTesters","betaGroups","betaBuildLocalizations","appEncryptionDeclaration","betaAppReviewSubmission","app","buildBetaDetail","appStoreVersion","icons","buildBundles","buildUpload","perfPowerMetrics","diagnosticSignatures"],"type":"string"},"type":"array"},"fields[gameCenterAppVersions]":{"items":{"enum":["enabled","compatibilityVersions","appStoreVersion"],"type":"string"},"type":"array"},"fields[routingAppCoverages]":{"items":{"enum":["fileSize","fileName","sourceFileChecksum","uploadOperations","assetDeliveryState","appStoreVersion"],"type":"string"},"type":"array"},"filter[appStoreState]":{"items":{"enum":["ACCEPTED","DEVELOPER_REMOVED_FROM_SALE","DEVELOPER_REJECTED","IN_REVIEW","INVALID_BINARY","METADATA_REJECTED","PENDING_APPLE_RELEASE","PENDING_CONTRACT","PENDING_DEVELOPER_RELEASE","PREPARE_FOR_SUBMISSION","PREORDER_READY_FOR_SALE","PROCESSING_FOR_APP_STORE","READY_FOR_REVIEW","READY_FOR_SALE","REJECTED","REMOVED_FROM_SALE","WAITING_FOR_EXPORT_COMPLIANCE","WAITING_FOR_REVIEW","REPLACED_WITH_NEW_VERSION","NOT_APPLICABLE"],"type":"string"},"type":"array"},"filter[appVersionState]":{"items":{"enum":["ACCEPTED","DEVELOPER_REJECTED","IN_REVIEW","INVALID_BINARY","METADATA_REJECTED","PENDING_APPLE_RELEASE","PENDING_DEVELOPER_RELEASE","PREPARE_FOR_SUBMISSION","PROCESSING_FOR_DISTRIBUTION","READY_FOR_DISTRIBUTION","READY_FOR_REVIEW","REJECTED","REPLACED_WITH_NEW_VERSION","WAITING_FOR_EXPORT_COMPLIANCE","WAITING_FOR_REVIEW"],"type":"string"},"type":"array"},"filter[id]":{"items":{"type":"string"},"type":"array"},"filter[platform]":{"items":{"enum":["IOS","MAC_OS","TV_OS","VISION_OS"],"type":"string"},"type":"array"},"filter[versionString]":{"items":{"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["app","appStoreVersionLocalizations","build","appStoreVersionPhasedRelease","gameCenterAppVersion","routingAppCoverage","appStoreReviewDetail","appStoreVersionSubmission","appClipDefaultExperience","appStoreVersionExperiments","appStoreVersionExperimentsV2","alternativeDistributionPackage"],"type":"string"},"type":"array"},"limit":{"maximum":200,"type":"integer"},"limit[appStoreVersionExperimentsV2]":{"maximum":50,"type":"integer"},"limit[appStoreVersionExperiments]":{"maximum":50,"type":"integer"},"limit[appStoreVersionLocalizations]":{"maximum":50,"type":"integer"}},"required":["id"],"type":"object"}"#)
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
            description: "Generated from OpenAPI operation".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"exists[publishedResponse]":{"type":"boolean"},"fields[customerReviewResponses]":{"items":{"enum":["responseBody","lastModifiedDate","state","review"],"type":"string"},"type":"array"},"fields[customerReviews]":{"items":{"enum":["rating","title","body","reviewerNickname","createdDate","territory","response","reviewTerritory"],"type":"string"},"type":"array"},"fields[territories]":{"items":{"enum":["currency"],"type":"string"},"type":"array"},"filter[rating]":{"items":{"type":"string"},"type":"array"},"filter[reviewTerritory]":{"items":{"type":"string"},"type":"array"},"filter[territory]":{"items":{"enum":["ABW","AFG","AGO","AIA","ALB","AND","ANT","ARE","ARG","ARM","ASM","ATG","AUS","AUT","AZE","BDI","BEL","BEN","BES","BFA","BGD","BGR","BHR","BHS","BIH","BLR","BLZ","BMU","BOL","BRA","BRB","BRN","BTN","BWA","CAF","CAN","CHE","CHL","CHN","CIV","CMR","COD","COG","COK","COL","COM","CPV","CRI","CUB","CUW","CXR","CYM","CYP","CZE","DEU","DJI","DMA","DNK","DOM","DZA","ECU","EGY","ERI","ESP","EST","ETH","FIN","FJI","FLK","FRA","FRO","FSM","GAB","GBR","GEO","GGY","GHA","GIB","GIN","GLP","GMB","GNB","GNQ","GRC","GRD","GRL","GTM","GUF","GUM","GUY","HKG","HND","HRV","HTI","HUN","IDN","IMN","IND","IRL","IRQ","ISL","ISR","ITA","JAM","JEY","JOR","JPN","KAZ","KEN","KGZ","KHM","KIR","KNA","KOR","KWT","LAO","LBN","LBR","LBY","LCA","LIE","LKA","LSO","LTU","LUX","LVA","MAC","MAR","MCO","MDA","MDG","MDV","MEX","MHL","MKD","MLI","MLT","MMR","MNE","MNG","MNP","MOZ","MRT","MSR","MTQ","MUS","MWI","MYS","MYT","NAM","NCL","NER","NFK","NGA","NIC","NIU","NLD","NOR","NPL","NRU","NZL","OMN","PAK","PAN","PER","PHL","PLW","PNG","POL","PRI","PRT","PRY","PSE","PYF","QAT","REU","ROU","RUS","RWA","SAU","SEN","SGP","SHN","SLB","SLE","SLV","SMR","SOM","SPM","SRB","SSD","STP","SUR","SVK","SVN","SWE","SWZ","SXM","SYC","TCA","TCD","TGO","THA","TJK","TKM","TLS","TON","TTO","TUN","TUR","TUV","TWN","TZA","UGA","UKR","UMI","URY","USA","UZB","VAT","VCT","VEN","VGB","VIR","VNM","VUT","WLF","WSM","XKS","YEM","ZAF","ZMB","ZWE"],"type":"string"},"type":"array"},"id":{"type":"string"},"include":{"items":{"enum":["response","reviewTerritory"],"type":"string"},"type":"array"},"limit":{"maximum":200,"type":"integer"},"sort":{"items":{"enum":["rating","-rating","createdDate","-createdDate"],"type":"string"},"type":"array"}},"required":["id"],"type":"object"}"#)
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
            description: "Generated from OpenAPI operation".to_string(),
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
    ]
}
