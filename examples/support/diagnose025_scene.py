"""Read-only diagnosis of a captured025snapshot; never run a simulation.

The local scene calculation is independent of the Rust projection. Observer-only
translation is returned as a copy for a separate full typed validation, never as
policy input or a replacement trial. Does not edit archived evidence.
"""
import argparse
import copy
import hashlib
import json
from pathlib import Path


def inspect(snapshot):
    translated = copy.deepcopy(snapshot)
    compact = translated['base']['base']['base']
    events = compact['base']['base']['events']
    scenes = compact['base']['base']['scenes']
    rows = []
    for query in compact['base']['inquiry']['records']:
        if not query['valid']:
            continue
        own = query['decision']['input']
        observed = query['tick'] - query['time_spent']
        assert observed >= 0
        completed = [scene for scene in scenes
                     if max(e['tick'] for e in events if e['scene'] == scene['id']) <= observed]
        local_now = sum(own['owner'] in scene['participants'] for scene in completed)
        assert own['scene'] == local_now, ('wrong actor-local current scene', query['id'])
        original_scene = own['scene']
        for concern in own['concerns']:
            event = events[concern['event']]
            assert event['id'] == concern['event']
            # An owner's resource encounters cannot overlap; lower scene IDs
            # involving that same actor precede this concern's scene. Foreign
            # scenes do not advance the local counter.
            local = sum(scene['id'] < event['scene']
                        and own['owner'] in scene['participants']
                        for scene in scenes)
            checks = dict(owner=concern['owner'] == own['owner'],
                          target=concern['target'] == event['decision']['actor'],
                          time=event['tick'] <= observed,
                          participant=own['owner'] in event['participants'],
                          local_scene=concern['created_scene'] == local)
            assert all(checks.values()), (query['id'], concern['id'], checks)
            assert (concern['created_scene'] < original_scene) == (event['scene'] < len(completed))
            rows.append(dict(inquiry=query['id'], owner=own['owner'],
                             tick=query['tick'], observed=observed,
                             concern=concern['id'], event=event['id'],
                             event_tick=event['tick'],
                             event_global_scene=event['scene'],
                             input_local_scene=concern['created_scene'],
                             independent_local_scene=local, checks=checks,
                             input_local_current_scene=original_scene,
                             independent_local_current_scene=local_now,
                             observer_global_current_scene=len(completed),
                             legacy_global_comparison=(
                                 concern['created_scene'] == event['scene'])))
            concern['created_scene'] = event['scene']
        # Both coordinates must use the same namespace: the inherited policy
        # replay tests created_scene < input.scene. Changing only one coordinate
        # falsely invalidates that replay. This copy never enters a policy run.
        own['scene'] = len(completed)
    return dict(valid_inquiries=sum(q['valid'] for q in compact['base']['inquiry']['records']),
                references=rows,
                scope='Actor-local scene diagnosis only; no trial outcome or physical claim'), translated


def controls(snapshot):
    cases = ('owner', 'target', 'time', 'local_scene', 'current_scene', 'participation')
    rejected = []
    for case in cases:
        changed = copy.deepcopy(snapshot)
        compact = changed['base']['base']['base']
        q = next(q for q in compact['base']['inquiry']['records'] if q['valid'])
        own = q['decision']['input']
        c = own['concerns'][0]
        event = compact['base']['base']['events'][c['event']]
        if case == 'owner':
            c['owner'] = (own['owner'] + 1) % 4
        elif case == 'target':
            c['target'] = (event['decision']['actor'] + 1) % 4
        elif case == 'time':
            event['tick'] = q['tick'] + 1
        elif case == 'local_scene':
            c['created_scene'] += 1
        elif case == 'current_scene':
            own['scene'] += 1
        else:
            event['participants'] = [p for p in event['participants'] if p != own['owner']]
        try:
            inspect(changed)
        except AssertionError:
            rejected.append(case)
        else:
            raise AssertionError(('diagnostic accepted corruption', case))
    return rejected


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('snapshot')
    parser.add_argument('--output')
    parser.add_argument('--translated-copy')
    args = parser.parse_args()
    blob = Path(args.snapshot).read_bytes()
    snapshot = json.loads(blob)
    result, translated = inspect(snapshot)
    result['snapshot_sha256'] = hashlib.sha256(blob).hexdigest()
    result['corruptions_rejected'] = controls(snapshot)
    if args.output:
        with Path(args.output).open('x', encoding='utf-8') as f:
            json.dump(result, f, indent=2)
            f.write('\n')
    if args.translated_copy:
        with Path(args.translated_copy).open('x', encoding='utf-8') as f:
            json.dump(translated, f, separators=(',', ':'))
            f.write('\n')
    print(json.dumps(result, indent=2))


if __name__ == '__main__':
    main()
