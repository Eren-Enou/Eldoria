"""Narrow observer-only protocol links over original018/019 references.

No policies, truth classification, historical cognition reconstruction or simulator.
Only explicit invitations, scene/query IDs, receipt namespaces and cause IDs link.
"""
import re

BASE = 'final_state.base.base.base.base.base'
INQUIRY = 'final_state.base.base.base.base.inquiry'
PROV = 'final_state.base.base.base.provenance'
ASSESS = 'final_state.base.base.assessment'


def resolve(trial, ref):
    if not isinstance(ref, str) or not re.fullmatch(r'[a-z_]+(?:\[\d+\]|\.[a-z_]+)*', ref):
        raise ValueError('Invalid original reference')
    value = trial
    try:
        for word, index in re.findall(r'([a-z_]+)|\[(\d+)\]', ref):
            value = value[word] if word else value[int(index)]
    except (KeyError, IndexError, TypeError) as error:
        raise ValueError('Missing original reference: ' + ref) from error
    return value


def build(trial, identity):
    nodes, chains, encounters = {}, [], []
    base, inquiry = resolve(trial, BASE), resolve(trial, INQUIRY)
    provenance, assessment = resolve(trial, PROV), resolve(trial, ASSESS)
    moments = []
    for pn, p in enumerate(trial['periods']):
        for sn, wrapped in enumerate(p['steps']):
            s = wrapped['activity']
            moments.append((f'periods[{pn}].steps[{sn}].activity', s))

    def tick(point):
        if point is None:
            return None
        return resolve(trial, f'points[{point}].state.tick')

    def node(ref, kind, now, actor=None, recipient=None, anchor=None, before=None, after=None, outcome=None):
        value = resolve(trial, ref)  # Fail closed; IDs are never substituted for ordinals.
        if before is not None:
            resolve(trial, before)
        if after is not None:
            resolve(trial, after)
        item = dict(id=ref, trial=identity, ref=ref, source_kind=kind, tick=now,
                    actor=actor, recipient=recipient, anchor=anchor or ref,
                    before=before, after=after, outcome=outcome,
                    original_id=value.get('id') if isinstance(value, dict) else None,
                    ordinals=[int(i) for i in re.findall(r'\[(\d+)\]', ref)])
        if ref not in nodes:
            nodes[ref] = item
        return ref

    def by_id(records, identifier, label):
        found = [(i, r) for i, r in enumerate(records) if r['id'] == identifier]
        if len(found) != 1:
            raise ValueError('Missing or duplicate ' + label + ' ID')
        return found[0]

    for root, s in moments:
        now = tick(s['observed'])
        group = dict(id=root, anchor=root, tick=now, nodes=[], edges=[], scenes=[], query=s['query'])

        def add(ref, kind, when=now, **kwargs):
            ident = node(ref, kind, when, **kwargs)
            if ident not in group['nodes']:
                group['nodes'].append(ident)
            return ident

        def edge(left, right, relation):
            group['edges'].append(dict(source=left, target=right, relation=relation))

        selected = {}
        for owner, d in enumerate(s['decisions']):
            if d is not None:
                ref = root + f'.decisions[{owner}]'
                selected[owner] = add(ref, 'Selected activity', actor=owner, anchor=root,
                                      before=f'points[{s["observed"]}]', outcome=d['selected'])
        for mi, meal in enumerate(s['consumption']):
            ref = add(root + f'.consumption[{mi}]', 'Executed meal', meal['tick'], actor=meal['actor'],
                      anchor=root + f'.consumption[{mi}]', outcome=f'food {meal["food_before"]}→{meal["food_after"]}; hunger {meal["hunger_before"]}→{meal["hunger_after"]}')
            if meal['actor'] in selected and s['decisions'][meal['actor']]['selected'] == 'Consume':
                edge(selected[meal['actor']], ref, 'Recorded Consume and same-step meal')
        for ai, allocation in enumerate(s['allocations']):
            inv = allocation['invitation']; owner, recipient = inv['proposer'], inv['recipient']
            d = s['decisions'][owner]
            if d is None or d['selected'] != 'Invite' or d['contact']['selected'] != {'Invite': dict(recipient=recipient, kind=inv['kind'])}:
                raise ValueError('Invitation is not an actual selected outgoing activity')
            invref = add(root + f'.allocations[{ai}].invitation', 'Outgoing invitation', tick(s['sent']), actor=owner,
                         recipient=recipient, anchor=root + '.responses', outcome=inv['kind'])
            edge(selected[owner], invref, 'Selected Invite matches recorded proposal')
            ar = add(root + f'.allocations[{ai}]', 'Arbitration', tick(s['admitted']), actor=owner,
                     recipient=recipient, anchor=root + '.allocations', outcome=allocation['disposition'])
            edge(invref, ar, 'Exact recorded invitation')
            for ri, response in enumerate(s['responses']):
                if inv in response['input']['incoming']:
                    if response['input']['own']['own']['id'] != recipient:
                        raise ValueError('Recipient response owner mismatch')
                    accepted = response['selected'].get('Accept') if isinstance(response['selected'], dict) else None
                    rr = add(root + f'.responses[{ri}]', 'Recipient response', response['input']['own']['now'], actor=recipient,
                             recipient=accepted, anchor=root + '.responses', outcome=response['selected'])
                    edge(invref, rr, 'Recipient incoming list contains exact invitation')
                    edge(rr, ar, 'Provisional reply precedes disposition; not a grant')
                    own = response['input']['own']['own']
                    for mi, meal in enumerate(s['consumption']):
                        if meal['actor'] == recipient and meal['tick'] == response['input']['own']['now'] and (meal['food_after'], meal['hunger_after']) == (own['food'], own['hunger']):
                            edge(root + f'.consumption[{mi}]', rr, 'Own same-step meal values match captured recipient input; reply has independent scoring')
            if allocation['disposition'] != 'Execute':
                continue
            if inv['kind'] == 'Resource':
                commit = s['started']
                if commit is None or commit['invitation'] != inv:
                    raise ValueError('Executed resource allocation lacks matching commitment')
                scene_id = commit['scene']; ordinal, scene = by_id(base['scenes'], scene_id, 'scene')
                if scene['participants'] != [owner, recipient]:
                    raise ValueError('Scene participants mismatch')
                cr = add(root + '.started', 'Actual resource execution', commit['start'], actor=owner,
                         recipient=recipient, anchor=root + '.started', before=f'points[{s["admitted"]}]', outcome=f'Scene {scene_id}')
                edge(ar, cr, 'Execute allocation names exact commitment')
                group['scenes'].append(scene_id)
                events = [(i, e) for i, e in enumerate(base['events']) if e['scene'] == scene_id]
                if not events or events[0][1]['id'] != commit['first_event']:
                    raise ValueError('Missing scene first event')
                completed = next(((r, w) for r, w in moments if w['finished'] is not None and w['finished']['scene'] == scene_id), None)
                previous = cr
                for i, event in events:
                    if event['participants'] != scene['participants']:
                        raise ValueError('Resource event participants mismatch')
                    er = add(BASE + f'.events[{i}]', 'Resource action', event['tick'], actor=event['decision']['actor'],
                             recipient=event['decision']['observation']['partner'], outcome=dict(action=event['decision']['selected'], transferred=event['transferred'], before=event['balances_before'], after=event['balances_after'], termination=event['outcome']))
                    edge(cr, er, 'Recorded scene ID')
                    if previous != cr:
                        edge(previous, er, 'Recorded order within same scene')
                    previous = er
                    for ii, interpretation in enumerate(event['interpretations']):
                        ir = add(BASE + f'.events[{i}].interpretations[{ii}]', 'Recorded interpretation', event['tick'],
                                 actor=scene['participants'][ii], recipient=interpretation['partner'], anchor=er, outcome=interpretation)
                        edge(er, ir, 'Interpretation recorded for this event')
                    for cause in event['decision']['memory_causes']:
                        ei, earlier = by_id(base['events'], cause, 'memory cause event')
                        if earlier['tick'] > event['tick']:
                            raise ValueError('Future memory cause')
                        origin = node(BASE + f'.events[{ei}]', 'Resource action', earlier['tick'], actor=earlier['decision']['actor'])
                        edge(origin, er, 'Recorded memory-cause ID; episode content is not restored')
                after = None
                if completed:
                    after = f'points[{completed[1]["after"]}]'
                    sr = add(after, 'Recorded after-state', tick(completed[1]['after']), before=f'points[{s["admitted"]}]', after=after,
                             outcome='Captured whole state; trust/episodes may have other contributions')
                    edge(previous, sr, 'Explicit finished commitment and after-point')
                terminal = events[-1][1]
                if completed and (scene['outcome'] is None or terminal['outcome'] != scene['outcome']):
                    raise ValueError('Completed scene lacks matching terminal outcome')
                actions = [e['decision']['selected'] for _, e in events]
                end_anchor = BASE + f'.events[{events[-1][0]}]'
                if completed:
                    scene_ref = add(BASE + f'.scenes[{ordinal}]', 'Completed resource encounter', terminal['tick'], actor=owner, recipient=recipient,
                                    anchor=end_anchor, before=f'points[{s["admitted"]}]', after=after,
                                    outcome=dict(scene=scene_id, termination=scene['outcome'], transferred=sum(e['transferred'] for _, e in events)))
                    edge(previous, scene_ref, 'Explicit scene ID, terminal outcome and finished commitment')
                encounters.append(dict(id=scene_id, chain=root, ref=BASE + f'.scenes[{ordinal}]', anchor=end_anchor,
                                       start=commit['start'], end=terminal['tick'], completed=completed is not None,
                                       participants=scene['participants'], transferred=sum(e['transferred'] for _, e in events),
                                       actions=actions, outcome=scene['outcome'], before=f'points[{s["admitted"]}]', after=after))
                scene_events = {e['id'] for _, e in events}
                later = next(((i, e) for i, e in enumerate(base['events']) if e['tick'] > terminal['tick'] and scene_events.intersection(e['decision']['memory_causes'])), None)
                if later:
                    i, e = later
                    target = add(BASE + f'.events[{i}]', 'Resource action', e['tick'], actor=e['decision']['actor'],
                                 recipient=e['decision']['observation']['partner'], outcome=dict(action=e['decision']['selected'], transferred=e['transferred'], before=e['balances_before'], after=e['balances_after'], termination=e['outcome']))
                    for cause in scene_events.intersection(e['decision']['memory_causes']):
                        ei, _ = by_id(base['events'], cause, 'later memory cause')
                        edge(BASE + f'.events[{ei}]', target, 'First later resource decision explicitly names this memory cause; not an isolated treatment effect')
            else:
                if s['query'] is None:
                    raise ValueError('Executed inquiry allocation lacks actual query')
                qi, q = by_id(inquiry['records'], s['query'], 'inquiry')
                ask = q['decision']['selected']['Ask']
                if q['decision']['input']['owner'] != owner or ask['source'] != recipient:
                    raise ValueError('Query owner/source mismatch')
                qr = add(INQUIRY + f'.records[{qi}]', 'Executed inquiry', tick(s['after']), actor=owner, recipient=recipient,
                         before=f'points[{s["admitted"]}]', after=f'points[{s["after"]}]',
                         outcome=dict(selected=ask, actual_time=q['time_spent'], native_response=q['response'], novelty=q['novelty'], status_before=q['concern_before']['status'], status_after=q['concern_after']['status']))
                edge(ar, qr, 'Execute allocation and recorded native query ID')
                if q['response'] is not None:
                    nr = add(INQUIRY + f'.records[{qi}].response', 'Native response', q['tick'], actor=owner, recipient=recipient, anchor=qr,
                             outcome=dict(public_action=q['response']['public_action'], valid=q['response']['valid'], receipts=q['response']['receipts']))
                    edge(qr, nr, 'Native response object, separate from provenance queries')
                    for receipt_id in q['response']['receipts']:
                        ri, receipt = by_id(base['cognition']['information'], receipt_id, 'native receipt')
                        rr = add(BASE + f'.cognition.information[{ri}]', 'Native information receipt', receipt['tick'], actor=receipt['listener'], recipient=receipt['speaker'], anchor=qr, outcome=dict(receipt=receipt_id))
                        edge(nr, rr, 'Explicit native receipt ID; separate namespace')
                for pi, pq in enumerate(provenance['queries']):
                    if pq['inquiry'] != q['id']:
                        continue
                    pr = add(PROV + f'.queries[{pi}]', 'Provenance query', pq['completed_at'], actor=pq['owner'], recipient=recipient, anchor=qr, outcome=dict(observed_at=pq['observed_at'], completed_at=pq['completed_at'], receipt=pq['receipt']))
                    edge(qr, pr, 'Explicit inquiry ID; native null does not exclude delivery')
                    if pq['receipt'] is not None:
                        ri, receipt = by_id(provenance['receipts'], pq['receipt'], 'provenance receipt')
                        if receipt['listener'] != owner:
                            raise ValueError('Receipt listener mismatch')
                        rr = add(PROV + f'.receipts[{ri}]', 'Provenance delivery', receipt['tick'], actor=owner, recipient=receipt['communicator'], anchor=qr,
                                 outcome=dict(receipt=receipt['id'], received_claim=receipt['knowledge']['claim'], received_quality=receipt['knowledge']['quality'], evaluation=receipt['evaluation']))
                        edge(pr, rr, 'Explicit provenance receipt ID')
                        for ai, arx in enumerate(assessment['records']):
                            if arx['owner'] == owner and arx['incoming']['receipt'] == {'Provenance': receipt['id']}:
                                ac = add(ASSESS + f'.records[{ai}]', 'Retained assessment update', receipt['tick'], actor=owner, anchor=qr,
                                         outcome=dict(before=arx['basis_before'], support=arx['support'], retained=arx['retained'], evicted=arx['evicted']))
                                edge(rr, ac, 'Exact owner and typed provenance receipt namespace')
                for li, lr in enumerate(trial['final_state']['learning']['records']):
                    if lr['inquiry'] == q['id'] and lr['owner'] == owner:
                        lrref = add(f'final_state.learning.records[{li}]', 'Observed learning consequence', q['tick'], actor=owner, anchor=qr, outcome=lr['outcome'])
                        edge(qr, lrref, 'Explicit inquiry ID; progress is not objective truth')
        chains.append(group)
    return dict(version=1, observer_only=True, nodes=nodes, chains=chains, encounters=encounters)
