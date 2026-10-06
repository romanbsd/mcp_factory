#!/usr/bin/env bash
# Regenerate the crate from openapi.oas.json, keeping only release-workflow
# operations and omitting response schemas (~700 KB of tools/list otherwise).
# Related reads go through `include` (e.g. appStoreVersions_getInstance with
# include=build,appStoreVersionLocalizations,appStoreReviewDetail).
set -euo pipefail

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
server_root=$(CDPATH= cd -- "$script_dir/.." && pwd)
factory_root=$(CDPATH= cd -- "$server_root/.." && pwd)
generator="$factory_root/generator/.venv/bin/mcp-gen"

if [[ ! -x "$generator" ]]; then
  echo "mcp-gen is not installed at $generator" >&2
  echo "Run the setup steps in $factory_root/SKILL.md first." >&2
  exit 1
fi

operations=(
  # Read and status
  apps_getCollection
  apps_appStoreVersions_getToManyRelated
  appStoreVersions_getInstance
  builds_getCollection
  builds_getInstance
  # Version preparation
  appStoreVersions_createInstance
  appStoreVersions_updateInstance
  appStoreVersions_build_updateToOneRelationship
  appStoreVersionLocalizations_createInstance
  appStoreVersionLocalizations_updateInstance
  appStoreReviewDetails_createInstance
  appStoreReviewDetails_updateInstance
  builds_updateInstance
  # Review submission and release
  reviewSubmissions_getCollection
  reviewSubmissions_createInstance
  reviewSubmissionItems_createInstance
  reviewSubmissions_updateInstance
  appStoreVersionPhasedReleases_createInstance
  appStoreVersionPhasedReleases_updateInstance
  appStoreVersionReleaseRequests_createInstance
  # TestFlight
  betaGroups_getCollection
  betaGroups_builds_createToManyRelationship
  betaBuildLocalizations_createInstance
  betaBuildLocalizations_updateInstance
  betaAppReviewSubmissions_createInstance
  # Customer reviews
  apps_customerReviews_getToManyRelated
  customerReviewResponses_createInstance
)

"$generator" generate \
  --input "$server_root/openapi.oas.json" \
  --output "$server_root" \
  --name app-store-connect-mcp \
  --no-output-schema \
  --compact-jsonapi \
  --operations "$(IFS=,; echo "${operations[*]}")"
