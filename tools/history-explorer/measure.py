"""Non-deterministic viewer engineering measurements; no simulation execution."""
import argparse
import ctypes
import gc
import hashlib
import importlib.util
import json
from pathlib import Path
import tempfile
import time

HOME=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('explorer',HOME/'explorer.py')
v=importlib.util.module_from_spec(spec);spec.loader.exec_module(v)


def peak_memory():
    if hasattr(ctypes,'windll'):
        from ctypes import wintypes
        class Counters(ctypes.Structure):
            _fields_=[('cb',wintypes.DWORD),('PageFaultCount',wintypes.DWORD)]+[(n,ctypes.c_size_t) for n in ('PeakWorkingSetSize','WorkingSetSize','QuotaPeakPagedPoolUsage','QuotaPagedPoolUsage','QuotaPeakNonPagedPoolUsage','QuotaNonPagedPoolUsage','PagefileUsage','PeakPagefileUsage')]
        counters=Counters();counters.cb=ctypes.sizeof(counters)
        kernel=ctypes.windll.kernel32;kernel.GetCurrentProcess.restype=wintypes.HANDLE
        measure=ctypes.windll.psapi.GetProcessMemoryInfo
        measure.argtypes=[wintypes.HANDLE,ctypes.POINTER(Counters),wintypes.DWORD];measure.restype=wintypes.BOOL
        if measure(kernel.GetCurrentProcess(),ctypes.byref(counters),counters.cb):
            return dict(peak_working_set_bytes=counters.PeakWorkingSetSize,working_set_bytes=counters.WorkingSetSize)
    return dict(note='Working-set API unavailable on this platform')


def main():
    ap=argparse.ArgumentParser();ap.add_argument('--output',type=Path,default=v.ROOT/'target/history-explorer/performance.json');args=ap.parse_args()
    resolved=args.output.resolve()
    if (v.ROOT/'experiments') in (resolved,*resolved.parents):raise ValueError('Measurements must stay outside original evidence')
    rows=[]
    def timed(label,fn):
        start=time.perf_counter();result=fn();rows.append(dict(operation=label,seconds=time.perf_counter()-start,**peak_memory()));return result
    with tempfile.TemporaryDirectory(prefix='woi-viewer-measure-') as tmp:
        repo=timed('discovery_catalog',lambda:v.Repository(cache=Path(tmp)).catalog())
        source=lambda mode:next(e for e in repo.entries.values() if e['archive']=='seed-42.jsonl.gz' and e['record']==(103 if mode=='Baseline' else 107))
        a=timed('cold_verified_default_trial',lambda:repo.get(source('Baseline')['id']))
        rows[-1].update(raw_bytes=a['source']['raw_bytes'],export_bytes=len(v.encoded(a)),moments=len(a['timeline']))
        timed('cached_verified_default_trial',lambda:repo.get(a['source']['id']))
        raw=v.encoded(a)
        timed('parse_and_verify_local_export',lambda:repo.imported(json.loads(raw)))
        b=timed('cold_verified_default_partner',lambda:repo.get(source('Scale110')['id']))
        timed('matched_comparison',lambda:v.comparison(a['trial'],b['trial']))
        del a,b,raw;gc.collect()
        largest=max(repo.entries.values(),key=lambda e:e['raw_bytes'])
        large=timed('largest_registered_trial',lambda:repo.get(largest['id']))
        rows[-1].update(case=large['source']['case'],mode=large['source']['mode'],archive=large['source']['archive'],record=large['source']['record'],raw_bytes=largest['raw_bytes'],export_bytes=len(v.encoded(large)),moments=len(large['timeline']))
    output=dict(schema='viewer-performance-v1',wall_clock_only=True,application_sources={name:hashlib.sha256((HOME/name).read_bytes().replace(b'\r\n',b'\n')).hexdigest() for name in ('explorer.py','app.js','perspective.js','index.html','style.css')},notes='One engineering run, not simulation evidence or a scalability comparison. Peak working set is for this Python process, includes export/projection/comparison work; no browser heap claim. Other repository validators may run concurrently.',rows=rows)
    args.output.parent.mkdir(parents=True,exist_ok=True);args.output.write_text(json.dumps(output,indent=2)+'\n',encoding='utf-8');print(json.dumps(output))


if __name__=='__main__':main()
