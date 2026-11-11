# Property Resolution Design

## Context

The Google Analytics MCP server needs to accept flexible property identifiers to improve usability. Users often know their website domain or property display name but not the numeric property ID. Google Analytics Admin API provides account summaries and data-stream listings that can be searched to resolve human-friendly identifiers to numeric IDs.

The challenge is balancing usability (accepting domains/names) against performance (API call count, latency) and correctness (complete search, ambiguity detection) for accounts of varying sizes.

## Decision

Implement a **two-phase resolution strategy** with partial coverage:

### Phase 1: Display Name Search (Complete, Paginated)

Search account summaries for exact display-name matches, paginating up to 10 pages (2000 properties maximum). This phase is complete within its pagination limit and correctly detects ambiguity across all fetched pages.

**Parameters:**
- Page size: 200 properties per page
- Max pages: 10 (safety limit)
- Total coverage: up to 2000 properties
- Ambiguity detection: across all fetched pages

**API calls:** 1–10 `accountSummaries.list` calls

### Phase 2: Hostname Fallback (Partial, Non-Paginated)

If no display-name match is found and the identifier looks like a hostname/URL:
1. Extract the hostname (strip scheme, port, path, query)
2. Re-fetch the **first page only** of account summaries (200 properties)
3. For each property, list its data streams with `pageSize=50` (**no pagination**)
4. Compare each stream's `defaultUri` hostname against the target
5. Return the first unique match, or fail on ambiguity within the scanned subset

**Parameters:**
- Account summary pagination: **none** (first page only, 200 properties)
- Data streams per property: **none** (first page only, 50 streams)
- Ambiguity detection: only within the scanned 200 properties

**API calls:** 1 `accountSummaries.list` + up to 200 `properties.dataStreams.list` calls

### Tradeoffs

This design prioritizes:
- ✅ **Common case coverage**: Small-to-medium accounts (<200 properties, <50 streams/property) resolve correctly
- ✅ **Latency**: Hostname fallback completes in seconds, not minutes
- ✅ **Quota preservation**: Avoids thousands of API calls for large accounts
- ✅ **Simplicity**: No caching, no state, no coordination across requests

At the cost of:
- ❌ **Incomplete hostname coverage**: Properties/streams beyond the scanned pages are invisible
- ❌ **Silent ambiguity**: Two properties with the same hostname (one scanned, one not) are not detected
- ❌ **False negatives**: Hostname resolution can report "not found" when a match exists beyond the scanned range

## Alternatives Considered

### Alternative 1: Full Pagination of All Properties and Streams

**Approach:** Paginate account summaries completely, then paginate data streams for every property.

**Pros:**
- Complete coverage: no false negatives
- Correct ambiguity detection across entire account

**Cons:**
- **Latency:** For a 1000-property account, 1000+ `dataStreams.list` calls executed serially could take 10–60+ seconds per resolution
- **Quota:** Google Analytics Admin API has rate limits; aggressive pagination on every tool call would quickly exhaust daily quotas
- **Scalability:** Enterprise accounts with thousands of properties would time out or hit quota limits

**Rejected because:** The latency and quota cost are unacceptable for an interactive tool. Most users don't have 1000+ properties, and those who do can use numeric IDs.

### Alternative 2: Refuse Resolution When Search is Incomplete

**Approach:** If account summaries contain `nextPageToken` after 10 pages, or any property has >50 data streams, refuse hostname resolution and require the numeric ID.

**Pros:**
- Honest: never silently returns an incomplete match
- Fails safely: users know when resolution is unavailable

**Cons:**
- **Usability regression:** Hostname resolution works for accounts with few properties but fails categorically for accounts with many properties, even when the target property is in the first page
- **Error messages:** "Your account is too large for hostname resolution" is a poor UX when the property might be scanned successfully

**Rejected because:** Partial coverage is better than categorical failure. The workaround (use numeric ID) is the same either way, but partial coverage succeeds for accounts within the scanned range.

### Alternative 3: Cache Account Summaries Across Requests

**Approach:** On first resolution, fetch all account summaries and streams, cache them in memory, and reuse for subsequent resolutions.

**Pros:**
- Amortizes the cost of full pagination across many tool calls
- Subsequent resolutions are instant

**Cons:**
- **Stale data:** Cache invalidation is hard; properties added/renamed/removed after caching are invisible or wrong
- **Memory cost:** Large accounts with thousands of properties/streams could consume significant memory
- **Concurrency:** Cache requires locking or per-session storage, adding complexity
- **Session lifetime:** Stdio MCP sessions are short-lived; caching mainly benefits long-lived HTTP sessions

**Rejected because:** The complexity and staleness risks outweigh the benefits for a tool primarily used in short sessions. Users who need repeated lookups can use numeric IDs after the first resolution.

### Alternative 4: Parallel Stream Lookups

**Approach:** Keep the partial scan (first 200 properties), but fetch data streams in parallel instead of serially.

**Pros:**
- Reduces latency for the hostname fallback (from serial N seconds to parallel ~1 second)

**Cons:**
- **Rate limits:** Google APIs have per-user/per-project QPS limits; 200 concurrent requests could trigger rate-limit errors
- **Complexity:** Parallel fetching requires spawning tasks, collecting results, and handling partial failures
- **Quota burst:** Concurrent calls consume quota faster, potentially affecting other operations

**Considered but deferred:** This is a viable optimization if latency becomes a real issue. The current serial approach is simpler and stays within rate limits. Can be revisited if users report slow hostname resolution.

## Consequences

### Positive

- **Works for most users:** Accounts with <200 properties and <50 streams/property resolve correctly
- **Fast:** Display-name search completes in <1 second; hostname fallback in 1–5 seconds
- **Quota-efficient:** Bounded API calls per resolution (max 210 calls for hostname fallback)
- **Simple implementation:** No caching, no parallel coordination, no state management

### Negative

- **Silent failures:** Hostname resolution can miss properties outside the scanned range without warning
- **Ambiguity gaps:** Two properties with the same hostname (one in first 200, one after) are not detected
- **Documentation burden:** Limitations must be clearly explained to avoid user confusion
- **Support cost:** Users with large accounts may report "property not found" when it exists

### Mitigation

- **Clear documentation:** README, tool schema, and code comments explicitly describe the limitations and recommend numeric IDs
- **Error messages:** When display-name search exceeds 10 pages, the error explicitly says "use the numeric property ID directly"
- **Workaround is simple:** Finding the numeric ID in Google Analytics Admin takes <1 minute
- **Most users unaffected:** Small-to-medium accounts represent the majority of GA4 users

## Criteria for Revisiting

Revisit this design if:

1. **User reports indicate frequent failures:** If real-world hostname resolutions frequently fail due to incomplete coverage, full pagination may be justified
2. **Google adds a search API:** If the Admin API gains a `properties.search` endpoint that accepts hostname queries, we can delegate to that instead
3. **Rate limits are relaxed:** If per-user QPS limits increase significantly, parallel stream lookups become safer
4. **Caching is standardized:** If the MCP server gains a standard per-session cache mechanism, cached summaries become viable
5. **Latency benchmarks show issues:** If serial stream lookups take >10 seconds in practice, parallel fetching becomes necessary

## Current Status

Implemented in `google-analytics-mcp/src/extensions.rs`:
- `resolve_property_id()` — main resolution function with 2-phase search
- `extract_hostname()` — hostname extraction from URLs/domains

Documented in:
- Code: thorough doc comment at `resolve_property_id()`
- README: "Property resolution" section with limitations and workarounds
- Tool schema: `propertyId` description mentions limits and recommends numeric IDs
- This design doc: rationale, alternatives, consequences

Last reviewed: 2026-11-11
