"""Create NEW 022 originals from independently checked isolated runner output.

Exclusive creation only. This helper cannot regenerate any historical directory.
It streams JSONL compression and records raw as well as compressed hashes.
"""
import argparse
import ast
import gzip
import hashlib
import json
import subprocess
from pathlib import Path

ROOT=Path(__file__).resolve().parents[2]
OUTPUT=ROOT/'experiments/022'


def text_hash(path):
    return hashlib.sha256(path.read_bytes().replace(b'\r\n',b'\n')).hexdigest()


def write_new(path,value):
    with path.open('x',encoding='utf-8',newline='\n') as stream:
        json.dump(value,stream,indent=2);stream.write('\n')


def compressed(source,target):
    digest=hashlib.sha256();size=0
    with source.open('rb') as incoming,target.open('xb') as raw:
        with gzip.GzipFile(filename='',mode='wb',fileobj=raw,mtime=0) as out:
            while chunk:=incoming.read(1024*1024):
                out.write(chunk);digest.update(chunk);size+=len(chunk)
    data=target.read_bytes()
    return dict(gzip_bytes=len(data),gzip_sha256=hashlib.sha256(data).hexdigest(),json_bytes=size,json_sha256=digest.hexdigest())


def main():
    parser=argparse.ArgumentParser();parser.add_argument('jsonl');parser.add_argument('analysis');args=parser.parse_args()
    assert not (OUTPUT/'archive-manifest.json').exists() and not (OUTPUT/'trials.jsonl.gz').exists(), 'Original022 already exists; do not regenerate'
    matrix=json.loads((OUTPUT/'registered-matrix.json').read_text())
    count=0
    with Path(args.jsonl).open(encoding='utf-8') as stream:
        for count,line in enumerate(stream,1):
            trial=json.loads(line);assert trial['schema']=='experiment-022-trial/v1' and trial['config']==matrix[count-1]
    assert count==len(matrix)==100
    manifest={'trials.jsonl.gz':dict(compressed(Path(args.jsonl),OUTPUT/'trials.jsonl.gz'),records=count)}
    for name in ('analysis','reports','comparisons','causal-slices'):
        filename=name+'.json.gz';manifest[filename]=compressed(Path(args.analysis)/(name+'.json'),OUTPUT/filename)
    paths=set('src/experiment.rs src/experiment022.rs src/simulation/concurrent.rs src/simulation/adaptive.rs src/simulation/provenance.rs src/simulation/self_evaluation.rs src/acquisition.rs src/environmental_knowledge.rs src/priorities.rs src/encounters.rs src/contact_learning.rs src/opportunities.rs src/behavior.rs src/model.rs src/simulation.rs src/lib.rs src/foresight.rs src/intentional.rs src/inquiry.rs src/provenance.rs src/concerns.rs src/cognition.rs src/assessment.rs src/retention.rs src/self_evaluation.rs src/audit.rs src/history.rs src/experiment009.rs src/experiment008.rs src/experiment007.rs src/experiment006.rs src/experiment005.rs src/experiment013.rs src/simulation/recurring.rs src/recurring.rs Cargo.toml Cargo.lock examples/experiment022.rs examples/measure022.rs examples/validate022.rs tests/concurrent.rs'.split())
    support=ROOT/'examples/support'
    pending=['review022','corrupt022','validate022','analyze022','archive022'];seen=set()
    while pending:
        name=pending.pop()
        if name in seen:continue
        seen.add(name);path=support/(name+'.py');paths.add(path.relative_to(ROOT).as_posix())
        for node in ast.walk(ast.parse(path.read_text(encoding='utf-8'))):
            modules=[a.name for a in node.names] if isinstance(node,ast.Import) else [node.module] if isinstance(node,ast.ImportFrom) and node.module else []
            for module in modules:
                if (support/(module+'.py')).exists():pending.append(module)
    registered=['docs/development/concurrent-world-architecture.md','experiments/022/specification.md','experiments/022/registered-matrix.json','experiments/022/design-gate.md']
    paths.update(registered)
    write_new(OUTPUT/'source-freeze.json',{name:text_hash(ROOT/name) for name in sorted(paths)})
    commit=subprocess.check_output(['git','rev-parse','1f81324'],cwd=ROOT,text=True).strip()
    prereg={}
    for name in registered:
        before=subprocess.check_output(['git','show',commit+':'+name],cwd=ROOT).replace(b'\r\n',b'\n')
        expected=hashlib.sha256(before).hexdigest();assert expected==text_hash(ROOT/name)
        prereg[name]=expected
    write_new(OUTPUT/'preregistration-source-freeze.json',dict(commit=commit,artifacts=prereg))
    write_new(OUTPUT/'archive-manifest.json',manifest)
    print(json.dumps(dict(worlds=count,source_locks=len(paths),python_dependencies=len(seen),archives=manifest)))


if __name__=='__main__':main()
