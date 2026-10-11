#!/usr/bin/env python3
"""Observe a debug server processing two simultaneous 16MB gzip TSV reports.
Mock HTTP runs in a separate process; time -l measures only the MCP server.
This is a reproducible observation, not a before/after performance comparison.
"""
import argparse
import gzip
import hashlib
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
import json
import os
from pathlib import Path
import re
import subprocess
import tempfile
import threading

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("--binary", type=Path, default=Path(__file__).resolve().parents[2] / "target/debug/app-store-connect-mcp")
args = parser.parse_args()
with tempfile.TemporaryDirectory() as directory:
    artifact = Path(directory) / "report.gz"
    with gzip.open(artifact, "wb") as out:
        out.write(b"Country,Installs\n")
        for _ in range(16):
            out.write(b"US,1\n" * 200_000)
    digest = hashlib.md5(artifact.read_bytes()).hexdigest()
    class Handler(BaseHTTPRequestHandler):
        def log_message(self, *args):
            pass
        def do_GET(self):
            if self.path.startswith("/v1/analyticsReportSegments/"):
                payload = json.dumps({"data":{"id":self.path.rsplit("/",1)[-1],"type":"analyticsReportSegments","attributes":{"url":f"http://127.0.0.1:{http.server_port}/download","checksum":digest}}}).encode()
                self.send_response(200)
                self.send_header("Content-Type","application/json")
                self.send_header("Content-Length",str(len(payload)))
                self.end_headers()
                self.wfile.write(payload)
            elif self.path == "/download":
                self.send_response(200)
                self.send_header("Content-Length",str(artifact.stat().st_size))
                self.end_headers()
                with artifact.open("rb") as source:
                    while chunk := source.read(65536):
                        self.wfile.write(chunk)
            else:
                self.send_error(404)
    http = ThreadingHTTPServer(("127.0.0.1",0),Handler)
    thread = threading.Thread(target=http.serve_forever,daemon=True)
    thread.start()
    env = {k:v for k,v in os.environ.items() if not k.startswith(("ASC_","MCP_FACTORY_"))}
    env.update(MCP_FACTORY_PROFILES="workflow",MCP_FACTORY_BASE_URL=f"http://127.0.0.1:{http.server_port}/")
    process = subprocess.Popen(["/usr/bin/time","-l",str(args.binary.resolve())],cwd=Path(__file__).resolve().parents[1],env=env,stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True)
    def send(value):
        process.stdin.write(json.dumps(value)+"\n")
        process.stdin.flush()
    def receive(identifier):
        while True:
            line = process.stdout.readline()
            if not line:
                raise RuntimeError("Server exited before response")
            value = json.loads(line)
            if value.get("id") == identifier:
                return value
    try:
        send({"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-11-25","capabilities":{},"clientInfo":{"name":"bounded-transfer-benchmark","version":"1"}}})
        receive(1)
        send({"jsonrpc":"2.0","method":"notifications/initialized"})
        for identifier in (2,3):
            send({"jsonrpc":"2.0","id":identifier,"method":"tools/call","params":{"name":"report_query","arguments":{"segmentId":f"segment-{identifier}","metrics":["Installs"]}}})
        results = []
        while len(results)<2:
            value = json.loads(process.stdout.readline())
            if value.get("id") in (2,3):
                assert not value["result"].get("isError"),value
                result = value["result"]["structuredContent"]
                assert result["rowCount"] == 3_200_000 and result["sums"]["Installs"] == 3_200_000,result
                results.append(result)
        process.stdin.close()
        process.wait(timeout=30)
        stderr = process.stderr.read()
        rss = re.search(r"(\d+)\s+maximum resident set size",stderr)
        cpu = re.search(r"([\d.]+) real\s+([\d.]+) user\s+([\d.]+) sys",stderr)
        print(json.dumps({"build":"debug","concurrent_queries":2,"decoded_bytes_per_query":16_000_016,"peak_rss_bytes":int(rss[1]) if rss else None,"real_seconds":float(cpu[1]) if cpu else None,"user_cpu_seconds":float(cpu[2]) if cpu else None,"system_cpu_seconds":float(cpu[3]) if cpu else None,"output_bytes_per_query":[len(json.dumps(r,separators=(",",":"))) for r in results]}))
    finally:
        if process.poll() is None:
            process.terminate()
            process.wait(timeout=10)
        http.shutdown()
        http.server_close()
