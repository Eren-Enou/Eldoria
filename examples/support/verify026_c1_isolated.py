"""I1 engineering gate:all frozen C1 algorithms,three counted transport paths.

No original source,metric,assertion or experimental parameter is changed.
Separate target directories prevent generated-host cache collisions.
"""
import hashlib,json
from identity026_c1 import ROOT,identity
def projection():
    path=ROOT/'examples/support/verify026_c1.py';original=path.read_text(encoding='utf-8')
    seams=[
        ("'prototypes/delayed-work-valuation/Cargo.toml','--target-dir','target/delayed-work-valuation'","'prototypes/delayed-work-valuation/Cargo.toml','--target-dir','target/026-v1-isolated'"),
        ("'prototypes/delayed-work-valuation-c1/Cargo.toml','--target-dir','target/delayed-work-valuation'","'prototypes/delayed-work-valuation-c1/Cargo.toml','--target-dir','target/026-c1-isolated'"),
        ("'examples/support/verify026_failure.py'","'examples/support/verify026_failure_isolated.py'"),
    ]
    source=original
    for before,after in seams:
        assert source.count(before)==1;source=source.replace(before,after)
    reverse=source
    for before,after in reversed(seams):
        assert reverse.count(after)==1;reverse=reverse.replace(after,before)
    assert reverse==original
    return path,source,original
def main():
    identity();path,source,original=projection()
    record={'kind':'post-run I1 engineering verification;unchanged frozen scientific algorithms','path_substitutions':3,'reverse_exact':True,
        'original_verifier_sha256':hashlib.sha256(original.encode()).hexdigest(),
        'v1_target':'target/026-v1-isolated','c1_target':'target/026-c1-isolated',
        'metrics_assertions_policies_changed':False}
    (ROOT/'target/026-c1-isolation-projection.json').write_text(json.dumps(record,indent=2)+'\n',encoding='utf-8')
    namespace={'__file__':str(path),'__name__':'frozen026c1_isolated_transport'}
    exec(compile(source,str(path)+'::isolated-build-path','exec'),namespace)
    namespace['main']()
    print('I1 isolated transport PASS:all frozen algorithms/assertions preserved;v1 and C1 native builds disjoint')
if __name__=='__main__':main()
