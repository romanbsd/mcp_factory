"""Regenerate from pinned official Discovery files; preserves handwritten extensions."""
from pathlib import Path
import json
from mcp_gen.google_discovery.parser import parse_google_discovery
from mcp_gen.composition import compose_generation_results
from mcp_gen.render import render_crate
root = Path(__file__).resolve().parents[1]
results = []
profiles = {}
for api in ['analyticsdata', 'analyticsadmin']:
    for version in ['v1beta', 'v1alpha']:
        path = root / 'schemas' / f'{api}-{version}.json'
        # Reviewed Analytics policy belongs to this server, not the shared parser.
        actions = ({'runReport','runRealtimeReport','runPivotReport','runFunnelReport','batchRunReports','batchRunPivotReports','checkCompatibility'} if api == 'analyticsdata' else {'runAccessReport','searchChangeHistoryEvents'})
        from mcp_gen.google_discovery.parser import walk_google_discovery_methods
        reads = {m['id'] for _, name, m in walk_google_discovery_methods(json.loads(path.read_text())) if name in actions}
        result = parse_google_discovery(path, read_only_operations=reads)
        for tool in result.tools:
            tool.name = f'{api}_{version}_{tool.name}'
            tool.output_schema = None
            profiles[tool.name] = [f'{api}_{"read" if tool.read_only else "write"}', 'raw']
        # Original Discovery docs remain resources; stale unprefixed indexes do not.
        result.resources = [r for r in result.resources if r.uri.startswith('schema://')]
        results.append(result)
result = compose_generation_results(results)
result.default_profiles = ['workflow']
result.tool_profiles = profiles
# Read names without importing a Rust implementation.
names = json.loads((root / 'scripts' / 'workflows.json').read_text())
result.tool_profiles.update({name:['workflow'] for name in names})
render_crate(result, output_dir=root, crate_name='google-analytics-mcp', base_url='https://analyticsdata.googleapis.com', core_path='../crates/mcp-factory-core', transport='stdio', config_text=(root / 'config.toml').read_text())
print(f'{len(result.tools)} generated operations; {len(names)} default workflows')
