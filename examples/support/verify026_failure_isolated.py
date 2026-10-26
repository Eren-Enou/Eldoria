"""Post-run path-only projection of immutable v1 failure verification."""
from pathlib import Path
from identity026 import ROOT
def main():
    path=ROOT/'examples/support/verify026_failure.py';original=path.read_text(encoding='utf-8')
    before="ROOT/'target/delayed-work-valuation/release/delayed-work-valuation.exe'"
    after="ROOT/'target/026-v1-isolated/release/delayed-work-valuation.exe'"
    assert original.count(before)==1
    source=original.replace(before,after);assert source.replace(after,before)==original
    namespace={'__file__':str(path),'__name__':'frozen026_failure_isolated_transport'}
    exec(compile(source,str(path)+'::isolated-build-path', 'exec'),namespace)
    namespace['main']()
if __name__=='__main__':main()
