"""Separate wall-clock/peak-process evidence. Output directory must be NEW."""
import argparse
import csv
import ctypes
from ctypes import wintypes
from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path
import platform
import subprocess
import time

ROOT=Path(__file__).resolve().parents[2]


def peak_counter(process):
    if os.name!='nt':return None
    class Counters(ctypes.Structure):
        _fields_=[('cb',wintypes.DWORD),('PageFaultCount',wintypes.DWORD)]+[(name,ctypes.c_size_t) for name in ('PeakWorkingSetSize','WorkingSetSize','QuotaPeakPagedPoolUsage','QuotaPagedPoolUsage','QuotaPeakNonPagedPoolUsage','QuotaNonPagedPoolUsage','PagefileUsage','PeakPagefileUsage')]
    info=Counters();info.cb=ctypes.sizeof(info)
    api=ctypes.WinDLL('psapi').GetProcessMemoryInfo
    api.argtypes=[wintypes.HANDLE,ctypes.POINTER(Counters),wintypes.DWORD];api.restype=wintypes.BOOL
    if api(wintypes.HANDLE(int(process._handle)),ctypes.byref(info),info.cb):return info.PeakWorkingSetSize
    return None


def main():
    parser=argparse.ArgumentParser();parser.add_argument('output');args=parser.parse_args()
    output=Path(args.output).resolve();output.mkdir(parents=True,exist_ok=False)
    checks=[]
    def cargo(name,arguments,data=None):
        begin=time.perf_counter()
        with (output/(data or name+'.log')).open('w',encoding='utf-8') as stdout,(output/(name+'-stderr.log')).open('w',encoding='utf-8') as stderr:
            result=subprocess.run(['cargo',*arguments],cwd=ROOT,stdout=stdout,stderr=stderr,check=False)
        checks.append(dict(name=name,exit=result.returncode,wall_seconds=time.perf_counter()-begin))
        assert result.returncode==0,name
    cargo('throughput',['bench','--locked','-j','1','--bench','throughput'],'throughput.csv')
    cargo('measure-build',['build','--release','--locked','-j','1','--example','measure022'])
    exe=ROOT/'target/release/examples'/('measure022.exe' if os.name=='nt' else 'measure022')
    begin=time.perf_counter();peak=None;samples=0
    with (output/'samples.jsonl').open('w',encoding='utf-8') as stdout,(output/'measure-stderr.log').open('w',encoding='utf-8') as stderr:
        process=subprocess.Popen([str(exe)],cwd=ROOT,stdout=stdout,stderr=stderr)
        while process.poll() is None:
            value=peak_counter(process);samples+=1
            if value is not None:peak=max(peak or 0,value)
            time.sleep(.002)
        code=process.wait()
    checks.append(dict(name='measure022',exit=code,wall_seconds=time.perf_counter()-begin));assert code==0
    rows=[json.loads(line) for line in (output/'samples.jsonl').read_text(encoding='utf-8').splitlines()]
    throughput=list(csv.DictReader((output/'throughput.csv').read_text(encoding='utf-8').splitlines()))
    assert len(rows)==24 and len(throughput)==20
    result=dict(kind='wall-clock evidence; excluded from deterministic histories',utc=datetime.now(timezone.utc).isoformat(),
                platform=platform.platform(),logical_cpus=os.cpu_count(),rustc=subprocess.check_output(['rustc','--version'],text=True).strip(),
                scope='simulation includes final history/cognition validation; serialization timed separately; process peak spans24 sequential samples; no matched pre-change performance baseline',
                source_freeze_sha256=hashlib.sha256((ROOT/'experiments/022/source-freeze.json').read_bytes()).hexdigest(),
                peak_working_set_bytes=peak,peak_poll_samples=samples,checks=checks,samples=rows,legacy_throughput=throughput)
    (output/'performance.json').write_text(json.dumps(result,indent=2)+'\n',encoding='utf-8')
    print(json.dumps(dict(checks=checks,native_samples=len(rows),legacy_rows=len(throughput),peak_working_set_bytes=peak)))


if __name__=='__main__':main()
