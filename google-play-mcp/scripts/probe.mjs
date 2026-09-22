import { spawn } from "node:child_process";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const binary = resolve(
  process.env.GOOGLE_PLAY_MCP_BINARY ?? resolve(root, "target/debug/google-play-mcp"),
);
const child = spawn(binary, [], {
  cwd: root,
  env: process.env,
  stdio: ["pipe", "pipe", "inherit"],
});

let nextId = 1;
const pending = new Map();
let buffer = "";
child.stdout.setEncoding("utf8");
child.stdout.on("data", (chunk) => {
  buffer += chunk;
  for (;;) {
    const newline = buffer.indexOf("\n");
    if (newline < 0) break;
    const line = buffer.slice(0, newline).trim();
    buffer = buffer.slice(newline + 1);
    if (!line) continue;
    const message = JSON.parse(line);
    if (message.id !== undefined && pending.has(message.id)) {
      const { resolve: finish, reject } = pending.get(message.id);
      pending.delete(message.id);
      if (message.error) reject(new Error(JSON.stringify(message.error)));
      else finish(message.result);
    }
  }
});

function request(method, params = {}) {
  const id = nextId++;
  child.stdin.write(`${JSON.stringify({ jsonrpc: "2.0", id, method, params })}\n`);
  return new Promise((finish, reject) => {
    pending.set(id, { resolve: finish, reject });
  });
}

function notify(method, params = {}) {
  child.stdin.write(`${JSON.stringify({ jsonrpc: "2.0", method, params })}\n`);
}

try {
  const initialized = await request("initialize", {
    protocolVersion: "2025-03-26",
    capabilities: {},
    clientInfo: { name: "google-play-mcp-probe", version: "0.1.0" },
  });
  notify("notifications/initialized");
  const toolResult = await request("tools/list");
  const resourceResult = await request("resources/list");
  const tools = toolResult.tools ?? [];
  const resources = resourceResult.resources ?? [];
  const summary = {
    protocolVersion: initialized.protocolVersion,
    server: initialized.serverInfo,
    toolCount: tools.length,
    resourceCount: resources.length,
    firstTools: tools.slice(0, 5).map((tool) => tool.name),
    highLevelTools: tools
      .map((tool) => tool.name)
      .filter((name) => name.startsWith("report_"))
      .sort(),
    resources: resources.map((resource) => resource.uri),
  };
  if (process.argv.includes("--live")) {
    const packageName = process.env.GOOGLE_PLAY_PROBE_PACKAGE;
    if (!packageName) {
      throw new Error("GOOGLE_PLAY_PROBE_PACKAGE is required with --live");
    }
    const publisher = await request("tools/call", {
      name: "reviews_list",
      arguments: { packageName, maxResults: 1 },
    });
    const reporting = await request("tools/call", {
      name: "apps_search",
      arguments: { pageSize: 1 },
    });
    const capabilities = await request("tools/call", {
      name: "report_capabilities",
      arguments: { packageName, probe: true },
    });
    summary.live = {
      publisherRead: !publisher.isError,
      reportingRead: !reporting.isError,
      highLevelRead: !capabilities.isError,
      highLevelStatus: capabilities.structuredContent?.status,
      publisherError: publisher.isError
        ? publisher.content?.[0]?.text?.slice(0, 300)
        : undefined,
      reportingError: reporting.isError
        ? reporting.content?.[0]?.text?.slice(0, 300)
        : undefined,
      highLevelError: capabilities.isError
        ? capabilities.content?.[0]?.text?.slice(0, 300)
        : undefined,
    };
  }
  console.log(JSON.stringify(summary));
} finally {
  child.stdin.end();
  child.kill();
}
