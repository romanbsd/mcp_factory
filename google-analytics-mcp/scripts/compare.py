"""Compare installed MCP stdio servers without credentials or Analytics API calls."""
import argparse, json, os, pathlib, selectors, statistics, subprocess, tempfile, time


def measure(command, config, profile, output, label, repetition):
    env = dict(os.environ, MCP_FACTORY_CONFIG=config, MCP_FACTORY_PROFILES=profile)
    with tempfile.TemporaryFile() as errors:
        start = time.perf_counter()
        process = subprocess.Popen(command, stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=errors, env=env)
        selector = selectors.DefaultSelector()
        selector.register(process.stdout, selectors.EVENT_READ)
        def send(message):
            process.stdin.write(json.dumps(message).encode()+b'\n'); process.stdin.flush()
        def receive(identifier):
            deadline=time.perf_counter()+60
            while time.perf_counter()<deadline:
                if not selector.select(max(0,deadline-time.perf_counter())): break
                line=process.stdout.readline()
                if not line: break
                message=json.loads(line)
                if message.get('id')==identifier: return message,len(line)
            errors.seek(0)
            raise RuntimeError(f'{label} did not return response {identifier}; exit={process.poll()}; stderr={errors.read().decode()[:2000]}')
        try:
            send({'jsonrpc':'2.0','id':1,'method':'initialize','params':{'protocolVersion':'2025-11-25','capabilities':{},'clientInfo':{'name':'comparison','version':'1'}}})
            init,_=receive(1)
            initialized=time.perf_counter()
            send({'jsonrpc':'2.0','method':'notifications/initialized'})
            before=time.perf_counter()
            send({'jsonrpc':'2.0','id':2,'method':'tools/list','params':{}})
            response,wire=receive(2)
            finished=time.perf_counter()
            if 'error' in response: raise RuntimeError(response['error'])
            tools=response['result']['tools']
            time.sleep(.25)
            rss=int(subprocess.check_output(['/bin/ps','-o','rss=','-p',str(process.pid)],text=True).strip())
            # Same serialization and payload boundary for both servers.
            payload=json.dumps({'tools':tools},separators=(',',':'),ensure_ascii=False).encode()
            if repetition==0:
                (output/f'{label}-tools.json').write_bytes(payload+b'\n')
            return {'initialize_ms':round((initialized-start)*1000,2),'ready_ms':round((finished-start)*1000,2),'tools_list_ms':round((finished-before)*1000,2),'rss_kib':rss,'tool_count':len(tools),'tool_list_bytes':len(payload),'tool_list_wire_bytes':wire,'server_info':init.get('result',{}).get('serverInfo')}
        finally:
            selector.close()
            process.terminate()
            try:process.wait(timeout=5)
            except subprocess.TimeoutExpired:process.kill();process.wait()


def main():
    parser=argparse.ArgumentParser()
    parser.add_argument('--python-server',default=os.path.expanduser('~/.local/bin/analytics-mcp'))
    parser.add_argument('--rust-server',default='target/release/google-analytics-mcp')
    parser.add_argument('--repetitions',type=int,default=5)
    parser.add_argument('--output',type=pathlib.Path,default=pathlib.Path('google-analytics-mcp/benchmarks'))
    args=parser.parse_args()
    if args.repetitions<1: parser.error("--repetitions must be positive")
    args.output.mkdir(parents=True,exist_ok=True)
    with tempfile.TemporaryDirectory() as temp:
        config=pathlib.Path(temp)/'config.toml';config.write_text('base_url="https://analyticsdata.googleapis.com"\nserver_name="google-analytics-mcp"\n[auth]\ntype="none"\n')
        samples={'google_python':[],'factory_workflow':[],'factory_raw':[]}
        for i in range(args.repetitions):
            for label,command,profile in [('google_python',[args.python_server],'workflow'),('factory_workflow',[str(pathlib.Path(args.rust_server).resolve())],'workflow')]:
                sample=measure(command,str(config),profile,args.output,label,i);samples[label].append(sample)
                print(label,i+1,json.dumps(sample),flush=True)
        samples['factory_raw']=[measure([str(pathlib.Path(args.rust_server).resolve())],str(config),'raw',args.output,'factory_raw',0)]
    result={'method':'macOS RSS after initialize and tools/list, 250ms settled; direct installed executables; no Analytics API calls; warm filesystem/cache runs; normalized tools payload includes descriptions and schemas','samples':samples,'medians':{label:{key:statistics.median(sample[key] for sample in rows) for key in ['ready_ms','initialize_ms','tools_list_ms','rss_kib','tool_count','tool_list_bytes']} for label,rows in samples.items()}}
    (args.output/'comparison.json').write_text(json.dumps(result,indent=2)+'\n')

if __name__=='__main__':main()
