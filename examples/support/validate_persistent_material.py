"""Real separate-process025 continuation, no archive writes or simulated responses.

Run with --binary ABSOLUTE-EXE --output NEW-ABSOLUTE-DIRECTORY. All registered
worlds retain the same fixed future; baseline state hashes select save boundaries
for observation only. They never enter actor input or alter opportunities.
"""
from pathlib import Path
import argparse, ctypes, hashlib, json, os, shutil, subprocess, time

ROOT = Path(__file__).resolve().parents[2]

def sha(data):
    return hashlib.sha256(data).hexdigest()

def storage_faults(binary, root):
    """Scratch-only binding faults, retaining the valid source directory exactly."""
    root=Path(root).resolve(strict=True)
    source=root/'world-0'
    before={p.relative_to(source).as_posix():sha(p.read_bytes()) for p in source.rglob('*') if p.is_file()}
    runtime=json.loads(subprocess.check_output([str(binary),'identity']))['runtime'].encode()
    fault_root=root/'storage-faults';fault_root.mkdir()
    rows=[]
    for kind in ('checkpoint_bytes','runtime_bytes','schema_bytes','journal_prefix'):
        parent=fault_root/kind;parent.mkdir()
        target=parent/source.name;shutil.copytree(source,target)
        checkpoint=sorted(target.glob('checkpoint-*.cbor'))[-1]
        if kind=='journal_prefix':
            path=target/'journal.jsonl';data=bytearray(path.read_bytes());data[len(data)//2]^=1;path.write_bytes(data)
        else:
            data=checkpoint.read_bytes()
            if kind=='checkpoint_bytes':
                data=bytearray(data);data[len(data)//2]^=1;data=bytes(data)
            elif kind=='runtime_bytes':
                assert runtime in data,'fault requires a raw commit with actual runtime text'
                data=data.replace(runtime,b'0'*64)
            else:
                assert b'woi-world-commit/v1' in data
                data=data.replace(b'woi-world-commit/v1',b'woi-world-commit/v9')
            checkpoint.write_bytes(data)
        result=subprocess.run([str(binary),'status',str(target)],capture_output=True)
        assert result.returncode,('fault unexpectedly accepted',kind)
        rows.append({'fault':kind,'exit':result.returncode,'error':result.stderr.decode(errors='replace').strip()})
    assert before=={p.relative_to(source).as_posix():sha(p.read_bytes()) for p in source.rglob('*') if p.is_file()}
    (root/'storage-faults.json').write_text(json.dumps({'checks':rows,'parent_byte_identical':True,
        'scope':'Outer storage/binding corruptions; the separate Rust controls rehash inner state and binding to test material semantics.'},indent=2),encoding='utf8')

def reporting_splits(binary, root):
    """Immediate before/after report11 while original world23 is Preparing."""
    root=Path(root).resolve(strict=True)
    baseline=json.loads((root/'baseline-23.json').read_bytes())
    parent=root/'report-boundary';parent.mkdir()
    directory=parent/'world-23'
    def run(*args):
        return subprocess.check_output([str(binary),*map(str,args)])
    run('create',directory,root/'config-23.json')
    previous=0
    comparisons=[]
    for generation,tick in [(2,10),(3,11)]:
        expected=next(s for s in baseline['states'] if s['tick']==tick)
        run('advance',directory,expected['step']-previous)
        output=parent/f'state-{tick}.cbor'
        run('state',directory,output)
        assert sha(output.read_bytes())==expected['state_sha256']
        run('verify',directory,'--generation',generation)
        assert any(a['phase']=='Preparing' for a in expected['material_work'])
        comparisons.append({'tick':tick,'step':expected['step'],'state_sha256':expected['state_sha256'],'material_work':expected['material_work']})
        previous=expected['step']
    run('advance',directory,2048);run('finalize',directory)
    assert sha((directory/'history.jsonl').read_bytes().rstrip(b'\n'))==baseline['history_sha256']
    (root/'report-boundary-validation.json').write_text(json.dumps({'index':23,'report_tick':11,'comparisons':comparisons,'original_terminal_exact':True,'cli_processes':9},indent=2),encoding='utf8')

def main():
    parser=argparse.ArgumentParser()
    parser.add_argument('--binary',type=Path,required=True)
    parser.add_argument('--output',type=Path,required=True)
    parser.add_argument('--worlds',default='all')
    args=parser.parse_args()
    binary=args.binary.resolve(strict=True)
    out=args.output
    assert out.is_absolute() and not out.exists()
    assert not any(p.lower() in ('experiments','reinforcement','.git') for p in out.parts)
    assert not any(p.is_symlink() for p in out.parents)
    out.mkdir()
    matrix=json.loads((ROOT/'experiments/025/registered-matrix.json').read_bytes())
    calls=[]
    def run(*arguments,fail=False):
        start=time.perf_counter()
        process=subprocess.Popen([str(binary),*map(str,arguments)],stdout=subprocess.PIPE,stderr=subprocess.PIPE)
        stdout,stderr=process.communicate()
        elapsed=time.perf_counter()-start
        peak=None
        if os.name=='nt':
            class Counters(ctypes.Structure):
                _fields_=[('cb',ctypes.c_ulong),('faults',ctypes.c_ulong),*[(n,ctypes.c_size_t) for n in ('peak_working','working','quota_peak_paged','quota_paged','quota_peak_nonpaged','quota_nonpaged','pagefile','peak_pagefile')]]
            counters=Counters();counters.cb=ctypes.sizeof(counters)
            psapi=ctypes.WinDLL('psapi')
            psapi.GetProcessMemoryInfo.argtypes=[ctypes.c_void_p,ctypes.c_void_p,ctypes.c_ulong]
            if psapi.GetProcessMemoryInfo(int(process._handle),ctypes.byref(counters),counters.cb):peak=counters.peak_working
        calls.append({'command':str(arguments[0]),'seconds':elapsed,'peak_working_set_bytes':peak,'exit':process.returncode})
        if fail:
            assert process.returncode,arguments
        elif process.returncode:
            raise RuntimeError((arguments,process.returncode,stderr.decode(errors='replace')[:4000]))
        return stdout
    identity=json.loads(run('identity'))
    rows=[]
    flags=set()
    indices=range(len(matrix)) if args.worlds=='all' else [int(n) for n in args.worlds.split(',')]
    for index in indices:
        config=matrix[index]
        config_path=out/f'config-{index}.json'
        config_path.write_text(json.dumps(config),encoding='utf8')
        baseline=out/f'baseline-{index}.json'
        run('baseline',config_path,baseline)
        reference=json.loads(baseline.read_bytes())
        states=reference['states']
        selected={0,1,len(states)-1}
        cutoff=states[0]['tick']+config['world']['horizon']
        for i,s in enumerate(states[1:],1):
            previous=states[i-1]
            if s['material_work']!=previous['material_work']:
                selected.update((i-1,i))
            if previous['tick']<cutoff<=s['tick']:
                selected.update((i-1,i));flags.add('cutoff')
            if previous['tick']<cutoff+config['world']['period']<=s['tick']:
                selected.update((i-1,i));flags.add('drain')
            if i==1 or previous['tick']//config['world']['period']<s['tick']//config['world']['period'] and 'reporting' not in flags:
                selected.update((i-1,i));flags.add('reporting')
            if s['material_work'] and any(c['work']!='Prepare' for c in s['active']):
                flags.add('concurrent_work')
            for active in s['material_work']:
                flags.add(active['phase'])
        directory=out/f'world-{index}'
        run('create',directory,config_path)
        generation=1
        prior=0
        for step in sorted(selected):
            if step:
                options=['--codec','zlib'] if generation%2 else []
                run('advance',directory,step-prior,*options)
                generation+=1
            state_path=out/f'state-{index}-{step}.cbor'
            run('state',directory,state_path)
            assert sha(state_path.read_bytes())==states[step]['state_sha256'],('full state',index,step)
            proof=json.loads(run('verify',directory,'--generation',generation))
            prior=step
        run('finalize',directory)
        history=(directory/'history.jsonl').read_bytes().rstrip(b'\n')
        assert sha(history)==reference['history_sha256'],('original terminal history',index)
        final=json.loads(history)
        operations=final['material']['operations']
        row={'index':index,'family':config['family'],'pair':config['pair'],'splits':len(selected),
             'conserved_material':proof['conserved_material'],
             'raw_initial':config['preparation']['raw_initial'],'raw_final':final['material']['final_raw'],
             'waste_final':final['material']['final_waste'],
             'outcomes':{name:sum(o['outcome']==name for o in operations) for name in ('Extracted','Insufficient','Prepared','Failed')},
             'history_sha256':sha(history),'checkpoint_bytes':[p.stat().st_size for p in sorted(directory.glob('*.cbor'))],
             'journal_bytes':(directory/'journal.jsonl').stat().st_size,'retained_bytes':sum(p.stat().st_size for p in directory.rglob('*') if p.is_file())}
        rows.append(row)
        receipt={'schema':'woi-material-process-validation/v1','runtime':identity['runtime'],
            'worlds':rows,'flags':sorted(flags),'processes':len(calls),
            'scope':'Full saved-state bytes at selected phase/cutoff/concurrency splits and original corrected025 terminal-history hash; real new process for every operation.'}
        (out/'receipt.json').write_text(json.dumps(receipt,indent=2),encoding='utf8')
        (out/'process-timings.json').write_text(json.dumps(calls,indent=2),encoding='utf8')
        print(json.dumps({k:row[k] for k in ('index','splits','outcomes')}),flush=True)
    assert {'Extracting','Preparing','Failed','cutoff','drain','reporting','concurrent_work'}<=flags,flags
    # A real partial material branch with both deliberately excluded committed
    # progress and an additional unverified process-exit suffix.
    config_path=out/'config-recovery.json'
    config_path.write_text(json.dumps(matrix[1]),encoding='utf8')
    parent=out/'world-recovery'
    run('create',parent,config_path)
    saved_status=json.loads(run('advance',parent,3))
    selected_state=out/'recovery-selected.cbor'
    run('state',parent,selected_state)
    run('advance',parent,4,'--codec','zlib')
    run('interrupt',parent,2)
    run('status',parent,fail=True)
    before={p.relative_to(parent).as_posix():sha(p.read_bytes()) for p in parent.rglob('*') if p.is_file()}
    branch_root=out/'recovery-branch';branch_root.mkdir()
    branch=branch_root/parent.name
    recovery=json.loads(run('recover',parent,branch,'--generation',2))
    assert recovery['later_committed_bytes']>0 and recovery['tail_bytes']>0
    assert before=={p.relative_to(parent).as_posix():sha(p.read_bytes()) for p in parent.rglob('*') if p.is_file()}
    restored=out/'recovery-restored.cbor'
    run('state',branch,restored)
    assert restored.read_bytes()==selected_state.read_bytes()
    run('advance',branch,2048)
    run('finalize',branch)
    reference=out/'recovery-reference.json'
    run('reference',config_path,reference)
    assert reference.read_bytes()==(branch/'history.jsonl').read_bytes().rstrip(b'\n')
    (out/'recovery-validation.json').write_text(json.dumps({'saved_status':saved_status,'recovery':recovery,
        'parent_files_byte_identical':True,'selected_full_state_exact':True,'original_terminal_exact':True},indent=2),encoding='utf8')
    # Rejection before world directory creation; these are malformed commands,
    # never replacement experimental worlds or new behavioral outcomes.
    for key,change in [('unknown',lambda c:c.update(unknown=True)),('stock',lambda c:c['preparation'].update(raw_initial=17)),('source',lambda c:c['preparation'].update(source_id=1)),('horizon',lambda c:c['world'].update(horizon=1025))]:
        broken=json.loads(json.dumps(matrix[1]));change(broken)
        path=out/f'invalid-{key}.json';path.write_text(json.dumps(broken),encoding='utf8')
        directory=out/f'invalid-world-{key}'
        run('create',directory,path,fail=True)
        assert not directory.exists()
    run('create',out/'experiments'/'blocked',config_path,fail=True)
    assert not (out/'experiments').exists()
    timings=[]
    for index in (1,17,43):
        if args.worlds!='all' and index not in indices:continue
        configuration=out/f'config-{index}.json'
        measured=out/f'measurement-{index}.json'
        run('measure',configuration,measured)
        pair=[]
        for codec in ('raw','zlib'):
            root=out/f'performance-{index}-{codec}'
            root.mkdir()
            directory=root/f'world-{index}'
            created=json.loads(run('create',directory,configuration))
            samples=[]
            for steps in (4,8,16,32,2048):
                before=time.perf_counter()
                status=json.loads(run('advance',directory,steps,*(['--codec','zlib'] if codec=='zlib' else [])))
                generation=status['latest_committed_generation']
                samples.append({'tick':status['tick'],'generation':generation,
                    'advance_save_process_seconds':time.perf_counter()-before,
                    'checkpoint_bytes':(directory/f'checkpoint-{generation:08}.cbor').stat().st_size,
                    'journal_bytes':status['journal_bytes']})
            run('finalize',directory)
            storage=json.loads(run('storage',directory))
            proof=json.loads(run('verify',directory,'--generation',generation))
            state=out/f'performance-state-{index}-{codec}.cbor'
            run('state',directory,state)
            pair.append({'codec':codec,'samples':samples,'storage':storage,'proof':proof,
                'state_sha256':sha(state.read_bytes()),'history_sha256':sha((directory/'history.jsonl').read_bytes())})
        assert pair[0]['state_sha256']==pair[1]['state_sha256']
        assert pair[0]['history_sha256']==pair[1]['history_sha256']
        timings.append({'index':index,'uninterrupted':json.loads(measured.read_bytes()),'storage_pairs':pair})
    (out/'performance.json').write_text(json.dumps({'runtime':identity['runtime'],'cases':timings,
        'processes':calls,'scope':'Wall-clock process and internal samples; separate from deterministic evidence. Peak working set is this CLI child only, or null if unavailable.'},indent=2),encoding='utf8')
    if args.worlds=='all':
        storage_faults(binary,out)
        reporting_splits(binary,out)

if __name__=='__main__':main()
