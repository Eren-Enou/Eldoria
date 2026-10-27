"""Real separate-process026 C1 continuation, no archive writes or simulated responses.

Run with --binary ABSOLUTE-EXE --output NEW-ABSOLUTE-DIRECTORY. All registered
worlds retain the same fixed future; baseline state hashes select save boundaries
for observation only. They never enter actor input or alter opportunities.
"""
from pathlib import Path
import argparse, ctypes, gzip, hashlib, json, os, shutil, subprocess, sys, time

ROOT = Path(__file__).resolve().parents[2]

def sha(data):
    return hashlib.sha256(data).hexdigest()

def storage_faults(binary, root, run):
    """Disposable raw template; no assumption about the campaign's last codec."""
    root=Path(root).resolve(strict=True)
    reference=root/'raw-storage-reference';reference.mkdir();source=reference/'world-0'
    run('create',source,root/'config-0.json');run('advance',source,2048);run('finalize',source)
    before={p.relative_to(source).as_posix():sha(p.read_bytes())for p in source.rglob('*')if p.is_file()}
    runtime=json.loads(run('identity'))['runtime'].encode();fault_root=root/'storage-faults';fault_root.mkdir();rows=[]
    for kind in ('checkpoint_bytes','runtime_bytes','schema_bytes','journal_prefix'):
        parent=fault_root/kind;parent.mkdir();target=parent/source.name;shutil.copytree(source,target)
        checkpoint=sorted(target.glob('checkpoint-*.cbor'))[-1]
        if kind=='journal_prefix':
            path=target/'journal.jsonl';data=bytearray(path.read_bytes());data[len(data)//2]^=1;path.write_bytes(data)
        else:
            data=checkpoint.read_bytes()
            if kind=='checkpoint_bytes':data=bytearray(data);data[len(data)//2]^=1;data=bytes(data)
            elif kind=='runtime_bytes':assert runtime in data;data=data.replace(runtime,b'0'*64)
            else:assert b'woi-world-commit/v1'in data;data=data.replace(b'woi-world-commit/v1',b'woi-world-commit/v9')
            checkpoint.write_bytes(data)
        run('status',target,fail=True);rows.append({'fault':kind,'rejected':True})
    incomplete=root/'incomplete-save';incomplete.mkdir();copy=incomplete/source.name;shutil.copytree(source,copy)
    (copy/'checkpoint-99999999.pending').write_bytes(b'incomplete scratch publication')
    inventory=json.loads(run('storage',copy));assert inventory['pending']
    assert not any(g.get('generation')==99999999 and g.get('valid')for g in inventory['generations'])
    rows.append({'fault':'incomplete_pending_publication','detected_not_promoted':True})
    corrupt=root/'corrupt-newest';corrupt.mkdir();copy=corrupt/source.name;shutil.copytree(source,copy)
    newest=sorted(copy.glob('checkpoint-*.cbor'))[-1];data=bytearray(newest.read_bytes());data[len(data)//2]^=1;newest.write_bytes(data)
    run('status',copy,fail=True)
    snapshot={p.relative_to(copy).as_posix():sha(p.read_bytes())for p in copy.rglob('*')if p.is_file()}
    branch_root=root/'corrupt-recovery-branch';branch_root.mkdir();branch=branch_root/source.name
    run('recover',copy,branch,'--generation',1)
    assert snapshot=={p.relative_to(copy).as_posix():sha(p.read_bytes())for p in copy.rglob('*')if p.is_file()}
    run('advance',branch,2048);run('finalize',branch)
    assert (branch/'history.jsonl').read_bytes()==(source/'history.jsonl').read_bytes()
    rows.append({'fault':'corrupt_newest_explicit_older_branch','normal_open_rejected':True,'parent_byte_identical':True,'exact_original_terminal':True})
    assert before=={p.relative_to(source).as_posix():sha(p.read_bytes())for p in source.rglob('*')if p.is_file()}
    (root/'storage-faults.json').write_text(json.dumps({'checks':rows,'parent_byte_identical':True,'scope':'Four outer faults, incomplete publication detection and explicit corrupt-newest recovery. Separate Rust controls rehash semantic corruptions.'},indent=2),encoding='utf8')


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
    matrix=json.loads((ROOT/'experiments/026/registered-matrix.json').read_bytes())
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
        calls.append({'command':str(arguments[0]),'seconds':elapsed,'peak_working_set_bytes':peak,'exit':process.returncode,'pid':process.pid})
        if fail:
            assert process.returncode,arguments
        elif process.returncode:
            raise RuntimeError((arguments,process.returncode,stderr.decode(errors='replace')[:4000]))
        return stdout
    identity=json.loads(run('identity'))
    parity=json.loads(run('parity',ROOT/'experiments/026/continuation-c1/trials.jsonl.gz'))
    assert parity['exact_original_histories']==100
    (out/'uninterrupted-parity.json').write_text(json.dumps(parity,indent=2),encoding='utf8')
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
        report_saved=False
        cutoff=states[0]['tick']+config['world']['world']['horizon']
        for i,s in enumerate(states[1:],1):
            previous=states[i-1]
            if s['material_work']!=previous['material_work']:
                selected.update((i-1,i))
            if previous['tick']<cutoff<=s['tick']:
                selected.update((i-1,i));flags.add('cutoff')
            if previous['tick']<cutoff+config['world']['world']['period']<=s['tick']:
                selected.update((i-1,i));flags.add('drain')
            if not report_saved and previous['tick']//config['world']['world']['period']<s['tick']//config['world']['world']['period']:
                selected.update((i-1,i));flags.add('reporting');report_saved=True
            if s['material_work'] and any(c['work']!='Prepare' for c in s['active']):
                flags.add('concurrent_work')
            if s['pending']:
                flags.add('pending_social')
                if not any(states[j]['pending'] for j in range(i)):
                    selected.update((i-1,i))
                if any(p['response'] is not None for p in s['pending']):
                    flags.add('pending_social_response')
                    if not any(any(p['response'] is not None for p in states[j]['pending'])for j in range(i)):
                        selected.update((i-1,i))
            if previous['raw']>0 and s['raw']==0:flags.add('actual_exhaustion')
            if config['source025']==21 and config['mode']=='ProjectedNeed' and s['tick']in(8,9,10,17,18):selected.update((i-1,i))
            for active in s['material_work']:
                flags.add(active['phase'])
                if active['escrow']>0:flags.add('owned_escrow')
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
        envelope=json.loads(history)
        assert envelope['runtime']==identity['runtime']
        final=envelope['c1_projection']['trial']['body']
        operations=final['material']['operations']
        row={'index':index,'source025':config['source025'],'mode':config['mode'],'family':config['world']['family'],'pair':config['world']['pair'],'splits':len(selected),
             'conserved_material':proof['conserved_material'],
             'raw_initial':config['world']['preparation']['raw_initial'],'raw_final':final['material']['final_raw'],
             'waste_final':final['material']['final_waste'],
             'outcomes':{name:sum(o['outcome']==name for o in operations) for name in ('Extracted','Insufficient','Prepared','Failed')},
             'history_sha256':sha(history),'canonical_C1_sha256':sha(json.dumps(envelope['c1_projection'],sort_keys=True,separators=(',',':')).encode()),
             'final_actors':[{k:a[k]for k in ('id','food','hunger','trust')}for a in final['history']['final_agents']],
             'split_witnesses':[{'step':step,'tick':states[step]['tick'],'state_sha256':states[step]['state_sha256'],'material_work':states[step]['material_work'],'pending_social':states[step]['pending'],'raw':states[step]['raw'],'waste':states[step]['waste']}for step in sorted(selected)],
             'checkpoint_bytes':[p.stat().st_size for p in sorted(directory.glob('*.cbor'))],
             'journal_bytes':(directory/'journal.jsonl').stat().st_size,'retained_bytes':sum(p.stat().st_size for p in directory.rglob('*') if p.is_file())}
        rows.append(row)
        receipt={'schema':'woi-delayed-work-process-validation/v1','runtime':identity['runtime'],
            'worlds':rows,'flags':sorted(flags),'processes':len(calls),
            'scope':'Full saved-state bytes at selected phase/cutoff/concurrency splits and original accepted026 C1 canonical projection and complete terminal engineering-history hash; real new process for every operation.'}
        (out/'receipt.json').write_text(json.dumps(receipt,indent=2),encoding='utf8')
        (out/'process-timings.json').write_text(json.dumps(calls,indent=2),encoding='utf8')
        print(json.dumps({k:row[k] for k in ('index','splits','outcomes')}),flush=True)
    if args.worlds=='all':
        projected=out/'canonical-c1-projections.jsonl.gz'
        with projected.open('xb')as raw:
            with gzip.GzipFile(filename='',fileobj=raw,mode='wb',mtime=0)as stream:
                for index in range(100):
                    envelope=json.loads((out/f'world-{index}'/'history.jsonl').read_bytes())
                    stream.write(json.dumps(envelope['c1_projection'],sort_keys=True,separators=(',',':')).encode()+b'\n')
        with (out/'independent-resumed-reader.log').open('wb')as log:
            result=subprocess.run([sys.executable,'-B','examples/support/review026_c1.py',str(projected),'--expected','experiments/026/continuation-c1/findings.json'],cwd=ROOT,stdout=log,stderr=subprocess.STDOUT)
        assert result.returncode==0,'independent resumed C1 reconstruction failed;see log'
        (out/'independent-resumed-validation.json').write_text(json.dumps({'worlds':100,'exit':0,'runtime':identity['runtime'],'scope':'Canonical projections of actual resumed engineering histories,not new or replacement scientific archives.'},indent=2),encoding='utf8')
    assert {'Extracting','Preparing','Failed','cutoff','drain','reporting','concurrent_work','pending_social','pending_social_response','actual_exhaustion','owned_escrow'}<=flags,flags
    # A real partial material branch with both deliberately excluded committed
    # progress and an additional unverified process-exit suffix.
    config_path=out/'config-recovery.json'
    config_path.write_text(json.dumps(matrix[3]),encoding='utf8')
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
    assert json.loads(reference.read_bytes())==json.loads((branch/'history.jsonl').read_bytes())['c1_projection']
    (out/'recovery-validation.json').write_text(json.dumps({'saved_status':saved_status,'recovery':recovery,
        'parent_files_byte_identical':True,'selected_full_state_exact':True,'original_terminal_exact':True},indent=2),encoding='utf8')
    # Rejection before world directory creation; these are malformed commands,
    # never replacement experimental worlds or new behavioral outcomes.
    for key,change in [('unknown',lambda c:c.update(unknown=True)),('stock',lambda c:c['world']['preparation'].update(raw_initial=17)),('source',lambda c:c['world']['preparation'].update(source_id=1)),('horizon',lambda c:c['world']['world'].update(horizon=1025))]:
        broken=json.loads(json.dumps(matrix[3]));change(broken)
        path=out/f'invalid-{key}.json';path.write_text(json.dumps(broken),encoding='utf8')
        directory=out/f'invalid-world-{key}'
        run('create',directory,path,fail=True)
        assert not directory.exists()
    run('create',out/'experiments'/'blocked',config_path,fail=True)
    assert not (out/'experiments').exists()
    timings=[]
    for index in (3,35,47,83,87):
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
        storage_faults(binary,out,run)
        # Reporting crossings are covered in all100 baseline-selected splits above.
    # The old025-specific helper is not used for this typed026 host.
    receipt['processes']=len(calls)
    (out/'receipt.json').write_text(json.dumps(receipt,indent=2),encoding='utf8')
    (out/'process-timings.json').write_text(json.dumps(calls,indent=2),encoding='utf8')
    (out/'performance.json').write_text(json.dumps({'runtime':identity['runtime'],'cases':timings,'processes':calls,'scope':'Wall-clock process/internal samples;separate from deterministic evidence. Own CLI process working set or null.'},indent=2),encoding='utf8')

if __name__=='__main__':main()
