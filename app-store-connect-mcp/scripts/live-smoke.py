#!/usr/bin/env python3
"""Opt-in, read-only App Store Connect smoke checks over stdio MCP.

Build the binary first. Opt in with --credentials PATH, or --live to read
~/.appstoreconnect/api_key.json. Credential JSON requires key_id, issuer_id,
and inline PEM key. Only normalized counts/statuses are printed or saved.
No upload, report creation, or non-dry-run mutation is permitted.
"""
from __future__ import annotations

import argparse
import collections
import datetime
import hashlib
import json
import os
from pathlib import Path
import queue
import struct
import subprocess
import tempfile
import threading
import time
import zlib

READ_TOOLS = frozenset({
    "apps_getCollection", "apps_appStoreVersions_getToManyRelated", "builds_getCollection",
    "apps_betaGroups_getToManyRelated", "apps_analyticsReportRequests_getToManyRelated",
    "analyticsReportRequests_reports_getToManyRelated", "analyticsReports_instances_getToManyRelated",
    "analyticsReportInstances_segments_getToManyRelated", "app_get", "version_get",
    "release_status", "release_prepare", "metadata_get", "screenshots_get", "testflight_status",
    "report_query",
})
DRY_TOOLS = frozenset({"metadata_apply", "screenshots_sync", "testflight_distribute",
                       "testflight_remove", "review_submit", "release"})
DEFAULT_TOOLS = (READ_TOOLS & {"app_get", "version_get", "release_status", "release_prepare",
                             "metadata_get", "screenshots_get", "testflight_status", "report_query"}) | DRY_TOOLS | {"asset_upload"}
MAX_RESPONSE_BYTES = 1024 * 1024
MISSING_BUNDLE = "com.example.mcp.smoke.deliberately.nonexistent"



class SmokeError(Exception):
    """Only fixed reason codes may cross the logging boundary."""
    def __init__(self, code):
        self.code = code
        super().__init__(code)


def validate_call(name, arguments):
    if not isinstance(arguments, dict):
        raise SmokeError("invalid_arguments")
    if name in DRY_TOOLS:
        if arguments.get("dryRun") is not True:
            raise SmokeError("live_writes_disabled")
    elif name not in READ_TOOLS:
        raise SmokeError("tool_not_allowlisted")
    if name == "report_query" and arguments.get("output", "summary") not in {"summary", "rows"}:
        raise SmokeError("artifact_export_disabled")


def count(value):
    if isinstance(value, (list, dict)):
        return len(value)
    if type(value) is int and value >= 0:
        return value
    return 0


def valid_identity(value):
    return isinstance(value, dict) and isinstance(value.get("id"), str) and bool(value["id"])


def valid_success(name, body):
    if name in DRY_TOOLS:
        return body.get("dryRun") is True and isinstance(body.get("changes"), list)
    if name == "app_get":
        return valid_identity(body) and isinstance(body.get("bundleId"), str) and bool(body["bundleId"])
    if name in {"version_get", "release_status"}:
        return valid_identity(body.get("app")) and valid_identity(body.get("version"))
    if name == "release_prepare":
        return type(body.get("readyForSubmission")) is bool and isinstance(body.get("blockingIssues"), list)
    if name == "metadata_get":
        return isinstance(body.get("localizations"), list)
    if name == "screenshots_get":
        return isinstance(body.get("screenshots"), list)
    if name == "testflight_status":
        return valid_identity(body.get("build")) and isinstance(body.get("groups"), list)
    if name == "report_query":
        return (body.get("ok") is True and type(body.get("rowCount")) is int and body["rowCount"] >= 0
                and isinstance(body.get("columns"), list) and isinstance(body.get("sums"), dict))
    return name in READ_TOOLS and isinstance(body.get("data"), list)


def normalize(name, profile, body, is_error, seconds, negative=False, expected_changes=None):
    """Never copy IDs, arguments, error strings, provider cells, URLs or credentials."""
    record = {"tool": name, "profile": profile, "seconds": round(seconds, 3), "status": "pass",
              "summaryBytes": len(json.dumps(body, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))}
    if negative:
        expected = (is_error and name == "app_get" and body.get("stage") == "lookup"
                    and "matched 0 resources" in str(body.get("message", "")))
        record["status"] = "expected_negative" if expected else "unexpected_error"
        record["reason"] = "missing_app_rejected" if expected else "negative_check_unexpected_result"
    elif is_error:
        blocker = ((name in {"review_submit", "release"} and body.get("stage") == "preflight")
                   or (name == "review_submit" and body.get("stage") == "reconcile"))
        record["status"] = "expected_blocker" if blocker else "unexpected_error"
        record["reason"] = "workflow_precondition" if blocker else "tool_error"
    elif not valid_success(name, body) or (expected_changes is not None and count(body.get("changes")) != expected_changes):
        record["status"] = "unexpected_error"
        record["reason"] = "semantic_failure"
    elif name == "release_prepare" and body.get("readyForSubmission") is False:
        record["status"] = "expected_blocker"
        record["reason"] = "known_submission_blockers"
    if expected_changes is not None:
        record["expectedChanges"] = expected_changes
    record["counts"] = {key: count(body[key]) for key in
                        ("changes", "unchanged", "warnings", "blockingIssues", "pending", "rowCount", "data") if key in body}
    return record


class Budget:
    def __init__(self, maximum):
        self.maximum, self.used = maximum, 0

    def take(self):
        if self.used >= self.maximum:
            return False
        self.used += 1
        return True


class Session:
    def __init__(self, binary, credentials, media_root, profile, records, budget, timeout=45,
                 popen=subprocess.Popen):
        self.binary, self.credentials, self.media_root = Path(binary), Path(credentials), Path(media_root)
        self.profile, self.records, self.budget, self.timeout = profile, records, budget, timeout
        self.popen, self.process, self.key = popen, None, None
        self.work = None
        self.responses = queue.Queue(maxsize=16)
        self.stop = threading.Event()
        self.reader = None
        self.identifier = 0
        self.tools = set()

    def __enter__(self):
        try:
            try:
                credentials = json.loads(self.credentials.read_text(encoding="utf-8"))
                if not isinstance(credentials, dict) or not all(
                    isinstance(credentials.get(k), str) and credentials[k].strip()
                    for k in ("key", "key_id", "issuer_id")
                ):
                    raise ValueError()
            except (OSError, ValueError, TypeError):
                raise SmokeError("credentials_invalid") from None
            self.key = tempfile.NamedTemporaryFile(mode="w", prefix="asc-smoke-key-", suffix=".p8", encoding="utf-8")
            os.chmod(self.key.name, 0o600)
            self.key.write(credentials["key"])
            self.key.flush()
            self.work = tempfile.TemporaryDirectory(prefix="asc-smoke-config-")
            config = Path(self.work.name) / "config.toml"
            config.write_text('base_url = "https://api.appstoreconnect.apple.com/"\ntransport = "stdio"\n'
                              'compact_jsonapi = true\n[auth]\ntype = "app_store_connect"\n', encoding="utf-8")
            env = {k: v for k, v in os.environ.items() if not k.startswith(("ASC_", "MCP_FACTORY_"))}
            env.update(ASC_KEY_ID=credentials["key_id"], ASC_ISSUER_ID=credentials["issuer_id"],
                       ASC_PRIVATE_KEY_PATH=self.key.name, MCP_FACTORY_PROFILES=self.profile,
                       MCP_FACTORY_MEDIA_ROOT=str(self.media_root), MCP_FACTORY_CONFIG=str(config),
                       MCP_FACTORY_BASE_URL="https://api.appstoreconnect.apple.com/", RUST_LOG="off")
            self.process = self.popen([str(self.binary.resolve())], cwd=self.work.name, env=env,
                                      stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                                      stderr=subprocess.DEVNULL, text=True, encoding="utf-8")
            self.reader = threading.Thread(target=self._read, daemon=True)
            self.reader.start()
            self._rpc("initialize", {"protocolVersion": "2025-11-25", "capabilities": {},
                                     "clientInfo": {"name": "asc-opt-in-smoke", "version": "1"}})
            self._send({"jsonrpc": "2.0", "method": "notifications/initialized"})
            listing = self._rpc("tools/list", {})
            self.tools = {tool["name"] for tool in listing.get("tools", []) if isinstance(tool, dict) and isinstance(tool.get("name"), str)}
            if self.profile == "workflow" and self.tools != DEFAULT_TOOLS:
                raise SmokeError("default_tool_surface_mismatch")
            return self
        except BaseException:
            self.close()
            raise

    def _read(self):
        try:
            while not self.stop.is_set():
                line = self.process.stdout.readline(MAX_RESPONSE_BYTES + 1)
                if not line:
                    self._enqueue(SmokeError("server_exited"))
                    return
                if len(line.encode("utf-8")) > MAX_RESPONSE_BYTES:
                    self._enqueue(SmokeError("response_limit"))
                    return
                value = json.loads(line)
                if not isinstance(value, dict):
                    raise ValueError()
                self._enqueue(value)
        except (ValueError, OSError, UnicodeError):
            self._enqueue(SmokeError("protocol_invalid"))

    def _enqueue(self, value):
        while not self.stop.is_set():
            try:
                self.responses.put(value, timeout=0.1)
                return
            except queue.Full:
                pass

    def _send(self, value):
        method = value.get("method")
        if method not in {"initialize", "tools/list", "tools/call", "notifications/initialized"}:
            raise SmokeError("method_not_allowlisted")
        if method == "tools/call":
            params = value.get("params", {})
            validate_call(params.get("name"), params.get("arguments"))
        try:
            self.process.stdin.write(json.dumps(value, separators=(",", ":")) + "\n")
            self.process.stdin.flush()
        except (OSError, ValueError):
            raise SmokeError("server_input_closed") from None

    def _rpc(self, method, params):
        if method not in {"initialize", "tools/list", "tools/call"}:
            raise SmokeError("method_not_allowlisted")
        if method == "tools/call":
            validate_call(params.get("name"), params.get("arguments"))
        self.identifier += 1
        identifier = self.identifier
        self._send({"jsonrpc": "2.0", "id": identifier, "method": method, "params": params})
        deadline = time.monotonic() + self.timeout
        while True:
            remaining = deadline - time.monotonic()
            if remaining <= 0:
                raise SmokeError("request_timeout")
            try:
                value = self.responses.get(timeout=remaining)
            except queue.Empty:
                raise SmokeError("request_timeout") from None
            if isinstance(value, SmokeError):
                raise value
            if value.get("id") == identifier:
                if "error" in value:
                    raise SmokeError("rpc_error")
                if not isinstance(value.get("result"), dict):
                    raise SmokeError("protocol_invalid")
                return value["result"]

    def call(self, name, arguments, negative=False, expected_changes=None):
        validate_call(name, arguments)
        if name not in self.tools:
            self.skip(name, "tool_unavailable")
            return {}
        if not self.budget.take():
            self.skip(name, "request_budget_exhausted")
            return {}
        start = time.monotonic()
        result = self._rpc("tools/call", {"name": name, "arguments": arguments})
        body = result.get("structuredContent")
        if body is None:
            try:
                body = json.loads("\n".join(c.get("text", "") for c in result.get("content", []) if c.get("type") == "text"))
            except (ValueError, TypeError):
                raise SmokeError("tool_result_invalid") from None
        if not isinstance(body, dict):
            raise SmokeError("tool_result_invalid")
        error = result.get("isError") is True or body.get("ok") is False
        self.records.append(normalize(name, self.profile, body, error, time.monotonic() - start, negative, expected_changes))
        return {} if error or self.records[-1]["status"] == "unexpected_error" else body

    def skip(self, name, reason):
        self.records.append({"tool": name, "profile": self.profile, "status": "skipped", "reason": reason})

    def close(self):
        self.stop.set()
        try:
            if self.process:
                try:
                    self.process.terminate()
                    self.process.wait(timeout=3)
                except subprocess.TimeoutExpired:
                    self.process.kill()
                    self.process.wait(timeout=3)
                except OSError:
                    pass
                finally:
                    for pipe in (self.process.stdin, self.process.stdout):
                        if pipe:
                            try:
                                pipe.close()
                            except OSError:
                                pass
        finally:
            try:
                if self.key:
                    self.key.close()
            finally:
                if self.work:
                    self.work.cleanup()
                if self.reader and self.reader is not threading.current_thread():
                    self.reader.join(timeout=1)

    def __exit__(self, *_):
        self.close()


def rows(body):
    data = body.get("data", [])
    return [row for row in data if isinstance(row, dict)] if isinstance(data, list) else []


def png_fixture(path):
    def chunk(kind, data):
        return struct.pack(">I", len(data)) + kind + data + struct.pack(">I", zlib.crc32(kind + data))
    path.write_bytes(b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", struct.pack(">IIBBBBB", 1, 1, 8, 2, 0, 0, 0))
                     + chunk(b"IDAT", zlib.compress(b"\0\0\0\0")) + chunk(b"IEND", b""))


def discover(session, limits):
    apps = rows(session.call("apps_getCollection", {"limit": limits.apps,
                "fields[apps]": ["bundleId", "primaryLocale"]}))[:limits.apps]
    plans, segments = [], []
    if not apps:
        session.skip("app_get", "no_apps_discovered")
    for app in apps:
        attrs = app.get("attributes", {})
        bundle = attrs.get("bundleId")
        if not isinstance(bundle, str) or not bundle:
            session.skip("app_get", "app_identity_unavailable")
            continue
        versions = rows(session.call("apps_appStoreVersions_getToManyRelated", {"id": app.get("id"),
                        "limit": limits.versions, "fields[appStoreVersions]": ["versionString", "platform"]}))[:limits.versions]
        builds_body = session.call("builds_getCollection", {"filter[app]": [app.get("id")], "limit": limits.builds,
                    "fields[builds]": ["version", "preReleaseVersion"], "include": ["preReleaseVersion"],
                    "fields[preReleaseVersions]": ["version", "platform"]})
        groups = rows(session.call("apps_betaGroups_getToManyRelated", {"id": app.get("id"), "limit": limits.groups,
                      "fields[betaGroups]": ["name"]}))[:limits.groups]
        build_plans = []
        for build in rows(builds_body)[:limits.builds]:
            linked = build.get("relationships", {}).get("preReleaseVersion", {}).get("data", {}).get("id")
            candidates = [r for r in builds_body.get("included", []) if r.get("type") == "preReleaseVersions" and r.get("id") == linked]
            if len(candidates) != 1:
                session.skip("testflight_status", "build_train_unavailable")
                continue
            train = candidates[0].get("attributes", {})
            build_plans.append({"bundleId": bundle, "buildNumber": build.get("attributes", {}).get("version"),
                                "version": train.get("version"), "platform": train.get("platform", "IOS")})
        plans.append({"bundle": bundle, "locale": attrs.get("primaryLocale"), "versions": versions,
                      "builds": build_plans, "groups": [g.get("attributes", {}).get("name") for g in groups
                                                        if isinstance(g.get("attributes", {}).get("name"), str)]})
        if len(segments) >= limits.reports:
            continue
        requests = rows(session.call("apps_analyticsReportRequests_getToManyRelated", {"id": app.get("id"), "limit": 1}))
        for request in requests[:1]:
            reports = rows(session.call("analyticsReportRequests_reports_getToManyRelated", {"id": request.get("id"), "limit": 1}))
            for report in reports[:1]:
                instances = rows(session.call("analyticsReports_instances_getToManyRelated", {"id": report.get("id"), "limit": 1}))
                for instance in instances[:1]:
                    found = rows(session.call("analyticsReportInstances_segments_getToManyRelated", {"id": instance.get("id"), "limit": 1}))
                    segments.extend(r["id"] for r in found[:1] if isinstance(r.get("id"), str))
    return plans, segments[:limits.reports]


def check_workflows(session, plans, segments, screenshot):
    session.call("app_get", {"bundleId": MISSING_BUNDLE}, negative=True)
    for plan in plans:
        session.call("app_get", {"bundleId": plan["bundle"]})
        if not plan["versions"]:
            session.skip("version_get", "no_versions_discovered")
        for version in plan["versions"]:
            attrs = version.get("attributes", {})
            if not all(isinstance(attrs.get(k), str) for k in ("versionString", "platform")):
                session.skip("version_get", "version_identity_unavailable")
                continue
            args = {"bundleId": plan["bundle"], "version": attrs["versionString"], "platform": attrs["platform"]}
            for tool in ("version_get", "release_status", "release_prepare", "screenshots_get"):
                session.call(tool, args)
            metadata = session.call("metadata_get", {**args, "fields": ["promotionalText"]})
            localizations = [row for row in metadata.get("localizations", []) if isinstance(row, dict)]
            existing = next((row for row in localizations if isinstance(row.get("locale"), str) and row["locale"]), None)
            locale = existing["locale"] if existing else plan["locale"]
            if existing:
                no_op = {"promotionalText": existing["promotionalText"]} if "promotionalText" in existing else {}
                session.call("metadata_apply", {**args, "localizations": {locale: no_op}, "dryRun": True}, expected_changes=0)
            else:
                session.skip("metadata_apply", "no_localization_for_noop")
            if isinstance(locale, str) and locale:
                proposal = "Opt-in live smoke proposal (dry run only)."
                if existing and existing.get("promotionalText") == proposal:
                    proposal += " Revised."
                session.call("metadata_apply", {**args, "localizations": {locale: {"promotionalText": proposal}}, "dryRun": True}, expected_changes=1)
                display = {"IOS": "APP_IPHONE_67", "MAC_OS": "APP_DESKTOP"}.get(attrs["platform"])
                if not existing:
                    session.skip("screenshots_sync", "no_localization_discovered")
                elif not display:
                    session.skip("screenshots_sync", "screenshot_platform_unsupported")
                else:
                    session.call("screenshots_sync", {**args, "locale": locale, "displayType": display,
                                 "files": ["smoke.png"] if screenshot else [], "dryRun": True})
            else:
                session.skip("metadata_apply", "locale_unavailable")
            for tool in ("review_submit", "release"):
                session.call(tool, {**args, "dryRun": True})
        if not plan["builds"]:
            session.skip("testflight_status", "no_builds_discovered")
        for build in plan["builds"]:
            session.call("testflight_status", build)
            if plan["groups"]:
                for tool in ("testflight_distribute", "testflight_remove"):
                    session.call(tool, {**build, "groupNames": plan["groups"], "dryRun": True})
            else:
                session.skip("testflight_distribute", "no_groups_discovered")
                session.skip("testflight_remove", "no_groups_discovered")
    if not segments:
        session.skip("report_query", "no_analytics_segments_discovered")
    for segment in segments:
        session.call("report_query", {"segmentId": segment, "output": "summary", "limit": 1})


def bounded_int(maximum):
    def parse(value):
        try:
            result = int(value)
        except ValueError:
            raise argparse.ArgumentTypeError("must be an integer") from None
        if not 1 <= result <= maximum:
            raise argparse.ArgumentTypeError(f"must be between 1 and {maximum}")
        return result
    return parse


def parser():
    root = Path(__file__).resolve().parents[2]
    cli = argparse.ArgumentParser(description=__doc__)
    cli.add_argument("--credentials", type=Path, help="Explicit opt-in; JSON with key_id, issuer_id and inline key")
    cli.add_argument("--live", action="store_true", help="Opt in using ~/.appstoreconnect/api_key.json if no path supplied")
    cli.add_argument("--binary", type=Path, default=root / "target/debug/app-store-connect-mcp")
    cli.add_argument("--output", type=Path, help="Write only the normalized JSON summary (mode 0600)")
    cli.add_argument("--timeout", type=bounded_int(60), default=45, help="Per-RPC deadline in seconds, at most 60")
    cli.add_argument("--max-requests", type=bounded_int(100), default=60, help="Shared tool-call cap across both sessions")
    for name, maximum, default in (("apps", 3, 2), ("versions", 3, 2), ("builds", 2, 1), ("groups", 2, 1), ("reports", 2, 1)):
        cli.add_argument("--" + name, type=bounded_int(maximum), default=default)
    cli.add_argument("--screenshots", action="store_true", help="Generate a temporary local PNG for screenshot dry runs")
    return cli


def file_sha256(path):
    try:
        if not Path(path).is_file():
            return None
        digest = hashlib.sha256()
        with Path(path).open("rb") as file:
            for chunk in iter(lambda: file.read(64 * 1024), b""):
                digest.update(chunk)
        return digest.hexdigest()
    except OSError:
        return None


def main(argv=None, session_factory=Session):
    cli = parser()
    args = cli.parse_args(argv)
    if args.credentials is None and not args.live:
        cli.error("live access requires --credentials PATH or --live")
    credentials = args.credentials or Path.home() / ".appstoreconnect/api_key.json"
    if args.output:
        try:
            same = args.output.resolve() == credentials.resolve()
            if args.output.exists() and credentials.exists():
                same = same or os.path.samefile(args.output, credentials)
        except (OSError, RuntimeError):
            cli.error("summary output path is invalid")
        if same:
            cli.error("summary output must differ from credentials input")
    records = []
    started_at = datetime.datetime.now(datetime.timezone.utc).isoformat()
    binary_hash = file_sha256(args.binary)
    runner_hash = file_sha256(__file__)
    budget = Budget(args.max_requests)
    fatal = None
    try:
        with tempfile.TemporaryDirectory(prefix="asc-smoke-media-") as directory:
            media = Path(directory)
            if args.screenshots:
                png_fixture(media / "smoke.png")
            with session_factory(args.binary, credentials, media, "all", records, budget, args.timeout) as session:
                plans, segments = discover(session, args)
            with session_factory(args.binary, credentials, media, "workflow", records, budget, args.timeout) as session:
                check_workflows(session, plans, segments, args.screenshots)
    except SmokeError as error:
        fatal = error.code
    except KeyboardInterrupt:
        fatal = "interrupted"
    except Exception:
        fatal = "runner_failure"
    statuses = collections.Counter(record["status"] for record in records)
    summary = {"live": True, "policy": "reads_and_strict_dry_run_only", "toolCalls": budget.used,
               "statuses": dict(sorted(statuses.items())), "checks": records,
               "discovery": "bounded_first_page_only", "bounds": {name: getattr(args, name) for name in
                   ("apps", "versions", "builds", "groups", "reports", "max_requests")}}
    summary["startedAtUtc"] = started_at
    summary["finishedAtUtc"] = datetime.datetime.now(datetime.timezone.utc).isoformat()
    summary["runnerSha256"] = runner_hash
    summary["binarySha256"] = binary_hash
    summary["summaryBytesTotal"] = sum(record.get("summaryBytes", 0) for record in records)
    maxima = {}
    for record in records:
        if "summaryBytes" in record:
            maxima[record["tool"]] = max(maxima.get(record["tool"], 0), record["summaryBytes"])
    summary["maxSummaryBytesByTool"] = dict(sorted(maxima.items()))
    if fatal:
        summary["error"] = fatal
    text = json.dumps(summary, sort_keys=True, separators=(",", ":")) + "\n"
    if args.output:
        try:
            descriptor = os.open(args.output, os.O_WRONLY | os.O_CREAT | getattr(os, "O_NOFOLLOW", 0), 0o600)
            with os.fdopen(descriptor, "w", encoding="utf-8") as output:
                if credentials.exists():
                    source = credentials.stat()
                    target = os.fstat(output.fileno())
                    if (source.st_dev, source.st_ino) == (target.st_dev, target.st_ino):
                        raise OSError("protected_output")
                os.fchmod(output.fileno(), 0o600)
                os.ftruncate(output.fileno(), 0)
                output.write(text)
        except OSError:
            summary["error"] = "summary_write_failed"
            fatal = "summary_write_failed"
            text = json.dumps(summary, sort_keys=True, separators=(",", ":")) + "\n"
    print(text, end="", flush=True)
    return 1 if fatal or statuses["unexpected_error"] else 0


if __name__ == "__main__":
    raise SystemExit(main())
