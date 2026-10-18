"""Read-only021 viewer cost sample. Wall clock is not simulation evidence."""
import ctypes
import json
import os
from pathlib import Path
import tempfile
import time
import explorer


def memory():
    if os.name != 'nt': return None
    from ctypes import wintypes
    class Counters(ctypes.Structure):
        _fields_=[('cb',wintypes.DWORD),('faults',wintypes.DWORD)]+[(n,ctypes.c_size_t) for n in
            ('peak_working_set','working_set','peak_paged','paged','peak_nonpaged','nonpaged','pagefile','peak_pagefile')]
    kernel=ctypes.WinDLL('kernel32',use_last_error=True)
    kernel.GetCurrentProcess.restype=wintypes.HANDLE
    psapi=ctypes.WinDLL('psapi',use_last_error=True)
    psapi.GetProcessMemoryInfo.argtypes=[wintypes.HANDLE,ctypes.POINTER(Counters),wintypes.DWORD]
    c=Counters();c.cb=ctypes.sizeof(c)
    if not psapi.GetProcessMemoryInfo(kernel.GetCurrentProcess(),ctypes.byref(c),c.cb):
        raise ctypes.WinError(ctypes.get_last_error())
    return dict(working_set_bytes=c.working_set,peak_working_set_bytes=c.peak_working_set)


def main():
    result=dict(note='One engineering run under concurrent validation load. Independent fresh temporary cache. Source-cold means first selection in this repository instance; OS disk cache is uncontrolled. Process working set is not browser heap or isolated adapter memory.',samples=[])
    with tempfile.TemporaryDirectory(prefix='woi021-load-') as tmp:
        start=time.perf_counter();repo=explorer.Repository(cache=Path(tmp),experiments=('021',)).catalog()
        result['catalog_seconds']=time.perf_counter()-start;result['catalog_entries']=len(repo.entries)
        result['after_index_memory']=memory()
        largest=max(repo.entries.values(),key=lambda e:e['raw_bytes'])
        result['largest_original_record']=largest['record']
        for ordinal in dict.fromkeys([largest['record'],82,32,163]):
            entry=next(e for e in repo.entries.values() if e['record']==ordinal)
            for run in range(2):
                start=time.perf_counter();data=repo.get(entry['id']);elapsed=time.perf_counter()-start
                payload=explorer.encoded(data)
                result['samples'].append(dict(record=ordinal,run=run+1,source_verification='first source/member verification' if run==0 else 'already verified; presentation rebuilt',
                    seconds=elapsed,original_bytes=entry['raw_bytes'],export_bytes=len(payload),
                    timeline_moments=len(data['timeline']),memory_changes=len(data['trial']['memory_records']),process_memory=memory()))
                del data,payload
        result['final_process_memory']=memory()
    print(json.dumps(result,indent=2))


if __name__=='__main__':main()
