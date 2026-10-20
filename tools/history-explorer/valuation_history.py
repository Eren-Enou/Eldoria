"""Explicit024 valuation source adapter. Observer consumer, no scientific-reader imports.
Native022 primitives are a shared presentation vocabulary, never a schema coercion.
Original024 envelope remains explicit; primitive presentation is shared, never coerced.
"""
import collections,copy,gzip,hashlib,json
from pathlib import Path
import importlib.util
_native_spec=importlib.util.spec_from_file_location('viewer024_primitives',Path(__file__).with_name('concurrent_history.py'))
native=importlib.util.module_from_spec(_native_spec);_native_spec.loader.exec_module(native)
VERSION='experiment-024-trial/v1'
PHYSICAL='experiment-024-physical-history/v1'
EXTRACTOR=8

def validate(original):
    if set(original)!={'schema','config','history','checkpoints','valuations'} or original['schema']!=VERSION:
        raise ValueError('Unsupported024original envelope')
    native.validate(original['history'],version=PHYSICAL)
    cfg=original['config']['world'];h=original['history']
    if not 1<=cfg['horizon']<=1024 or not 1<=cfg['period']<=128 or cfg['capacity']!=2 or cfg['duration'] not in (2,4,8) or cfg['cadence']!=8:
        raise ValueError('Unsupported bounded024configuration')
    if cfg['mode'] != 'Continuous' or cfg['phase'] not in ('Epoch','Absolute'):
        raise ValueError('Invalid024mode/phase')
    if h['end']!=h['start']+cfg['horizon']+33:
        raise ValueError('Invalid024nominal/drainedendpoint')
    if not original['checkpoints'] or original['checkpoints'][-1]['tick']!=h['end']:
        raise ValueError('Missing024finalcheckpoint')
    known={x['commitment']['id'] for x in h['transitions'] if not x['completed']}
    if any(not set(c['active']).issubset(known) for c in original['checkpoints']):
        raise ValueError('Unknown024checkpoint commitment')
    mode=original['config']['valuation']
    if mode not in ('Baseline','MealRelief'):
        raise ValueError('Unknown024valuation')
    decisions={(bi,di):d for bi,b in enumerate(h['boundaries']) for di,d in enumerate(b['choices'])}
    seen=set()
    for audit in original['valuations']:
        key=(audit['boundary'],audit['choice'])
        if key in seen or key not in decisions or audit['policy']!=mode:
            raise ValueError('Invalid024valuation decision reference')
        seen.add(key);decision=decisions[key];terms=audit['selection']['terms']
        if len(terms)!=4 or len(decision['candidates'])!=4 or [t['action'] for t in terms]!=['Wait','Consume','Invite','Acquire']:
            raise ValueError('Invalid024candidate order')
        for term,candidate in zip(terms,decision['candidates']):
            bonus=min(decision['input']['legacy']['own']['hunger'],40) if mode=='MealRelief' and term['action']=='Consume' and candidate['eligible'] else 0
            if (term['eligible'],term['meal_relief'],term['final_score'])!=(candidate['eligible'],bonus,candidate['score']) or term['base_score']+bonus!=term['final_score']:
                raise ValueError('Invalid024captured score terms')
        selected=max(terms,key=lambda t:t['final_score'])['action']
        if selected!=decision['selected'] or selected!=audit['selection']['selected']:
            raise ValueError('Invalid024stable selected action')
    if seen!=set(decisions):
        raise ValueError('Missing024valuation audit')

class Source:
    experiment='024'
    def __init__(self,root,path):
        self.root=Path(root).resolve();self.path=Path(path).resolve()
        if self.path.is_dir():self.path=self.path/'trials.jsonl.gz'
        self.registered=self.path==self.root/'experiments/024/trials.jsonl.gz'
        self.entries={};self.loaded=collections.OrderedDict();self.expected=None
    def guard(self):
        freeze=self.root/'experiments/024/source-freeze.json'
        for name,expected in json.loads(freeze.read_bytes()).items():
            if hashlib.sha256((self.root/name).read_bytes().replace(b'\r\n',b'\n')).hexdigest()!=expected:
                raise ValueError('Original024runtime source lock changed: '+name)
        value=hashlib.sha256(self.path.read_bytes()).hexdigest()
        if self.expected is not None and value!=self.expected:raise ValueError('024source changed after catalog')
        if self.registered:
            manifest=json.loads((self.path.parent/'archive-manifest.json').read_bytes())['archives']['trials.jsonl.gz']
            if value!=manifest['sha256']:raise ValueError('024registeredarchive changed')
        return hashlib.sha256(freeze.read_bytes().replace(b'\r\n',b'\n')).hexdigest(),value
    def lines(self):
        with (gzip.open(self.path,'rb') if self.path.suffix=='.gz' else self.path.open('rb')) as f:
            for line in f:
                if len(line)>64*1024*1024:raise ValueError('024member exceeds64MiB input bound')
                yield line
    def catalog(self,status):
        freeze,archive=self.guard();self.expected=archive;decoded=hashlib.sha256();entries={}
        matrix=json.loads((self.root/'experiments/024/registered-matrix.json').read_bytes()) if self.registered else None
        for ordinal,line in enumerate(self.lines()):
            decoded.update(line);original=json.loads(line);validate(original);cfg=original['config']['world'];h=original['history']
            if matrix is not None and (ordinal>=len(matrix) or original['config']!=matrix[ordinal]):raise ValueError('024registration mismatch')
            identity=dict(experiment='024',archive=self.path.name,record=ordinal,seed=str(cfg['seed']),case=cfg['case'],mode=original['config']['valuation'],config=original['config'])
            key=native.digest(dict(identity=identity,archive_sha256=archive,schema=VERSION))[:24]
            entries[key]=dict(id=key,**identity,source_schema=VERSION,physical_schema=PHYSICAL,archive_sha256=archive,record_sha256=hashlib.sha256(line.rstrip(b'\r\n')).hexdigest(),trial_fingerprint=native.digest(original),policy_freeze_sha256=freeze,epoch=h['start'],end=h['end'],nominal_stop=h['start']+cfg['horizon'],raw_bytes=len(line),individual_ids=[0,1,2,3],original_reference_root='history',registered=self.registered,scientific_status=status if self.registered else 'Custom deterministic prototype run; not registered research evidence; no protection/adoption',integrity='Original024runtime source locks and actual source/member hashes verified')
        if not entries:raise ValueError('Empty024source')
        if matrix is not None:
            expected=json.loads((self.path.parent/'archive-manifest.json').read_bytes())['archives']['trials.jsonl.gz']
            if len(entries)!=len(matrix) or decoded.hexdigest()!=expected['decoded_sha256']:raise ValueError('Incomplete024archive')
        for entry in entries.values():entry['decoded_archive_sha256']=decoded.hexdigest()
        self.entries=entries;return entries
    def get(self,entry):
        self.guard();key=entry['id']
        if key not in self.entries or entry['trial_fingerprint']!=self.entries[key]['trial_fingerprint']:raise ValueError('Unregistered024member')
        if key in self.loaded:self.loaded.move_to_end(key);return self.loaded[key]
        for n,line in enumerate(self.lines()):
            if n!=entry['record']:continue
            if hashlib.sha256(line.rstrip(b'\r\n')).hexdigest()!=entry['record_sha256']:raise ValueError('024member changed')
            original=json.loads(line);validate(original)
            if native.digest(original)!=entry['trial_fingerprint']:raise ValueError('024original changed')
            self.loaded[key]=original
            while len(self.loaded)>2:self.loaded.popitem(last=False)
            return original
        raise ValueError('Missing024member')

def rebuild(data):
    header=data['continuous']
    return dict(schema=data['valuation']['schema'],config=data['valuation']['config'],history=data['trial'],checkpoints=header['checkpoints'],valuations=data['valuation']['valuations'])

def export(entry,original):
    validate(original);data=native.export(entry,original['history']);data['extractor_version']=EXTRACTOR
    data['continuous']=dict(schema=original['schema'],config=copy.deepcopy(original['config']['world']),checkpoints=copy.deepcopy(original['checkpoints']))
    data['valuation']=dict(schema=original['schema'],config=copy.deepcopy(original['config']),valuations=copy.deepcopy(original['valuations']))
    moments=data['timeline'];nodes=data['causal']['nodes']
    history=original['history'];compact=history['final_state']['base']['base']['base']
    queries=compact['base']['inquiry']['records'];provenance=compact['provenance']
    for interval in data['intervals']:
        if interval['kind']!='Query':continue
        proposal=history['boundaries'][interval['source']]
        choice=next(d for d in proposal['choices'] if d['input']['legacy']['own']['id']==interval['owners'][0])
        matches=[(n,q) for n,q in enumerate(queries) if q['decision']['input']['owner']==interval['owners'][0] and q['tick']==interval['end'] and q['tick']-q['time_spent']==interval['start'] and q['decision']['selected']==choice['input']['legacy']['query']['decision']['selected']]
        if len(matches)!=1:raise ValueError('Ambiguous024query execution identity')
        n,q=matches[0];query_ref=f'final_state.base.base.base.base.inquiry.records[{n}]';joined=[query_ref]
        data['causal']['edges'].append(dict(source=interval['start_ref'],target=query_ref,kind='actor_action_start_completion_duration_query_identity'))
        for pn,pq in enumerate(provenance['queries']):
            if pq['inquiry']!=q['id']:continue
            pref=f'final_state.base.base.base.provenance.queries[{pn}]'
            nodes[pref]=dict(ref=pref,anchor=query_ref,tick=q['tick'],kind='provenance_query',owners=interval['owners'],label=f"Provenance query{pq['id']} refers to inquiry{q['id']}")
            joined.append(pref);data['causal']['edges'].append(dict(source=query_ref,target=pref,kind='explicit_inquiry_id'))
            if pq['receipt'] is not None:
                rid=pq['receipt'];receipt=provenance['receipts'][rid]
                rref=f'final_state.base.base.base.provenance.receipts[{rid}]'
                nodes[rref]=dict(ref=rref,anchor=query_ref,tick=receipt['tick'],kind='provenance_receipt',owners=[receipt['listener']],label=f"Actual provenance receipt{rid}: communicator{receipt['communicator']} to listener{receipt['listener']}")
                joined.append(rref);data['causal']['edges'].append(dict(source=pref,target=rref,kind='explicit_receipt_id'))
                if receipt['cognition_receipt'] is not None:
                    cref=f"final_state.base.base.base.base.base.cognition.information[{receipt['cognition_receipt']}]"
                    joined.append(cref);data['causal']['edges'].append(dict(source=rref,target=cref,kind='explicit_cognition_receipt_id'))
        for chain in data['causal']['chains']:
            if interval['start_ref'] in chain['nodes']:
                index=chain['nodes'].index(interval['start_ref'])+1;chain['nodes'][index:index]=joined
    for ref in nodes:native.resolve(history,ref)
    data['causal']['note']+=' Query execution joins use actor, selected action and exact start/completion duration; delivery then follows explicit inquiry/receipt IDs. Null native response does not imply no provenance receipt.'
    for index,checkpoint in enumerate(original['checkpoints']):
        ref=f'checkpoints[{index}]';label=' / '.join(checkpoint['labels'])+' checkpoint (observer only)'
        moment=dict(tick=checkpoint['tick'],kind='checkpoint',ref=ref,label=label,owners=[],phase=15)
        moments.append(moment);nodes[ref]=dict(moment,anchor=ref)
    moments.sort(key=lambda x:(x['tick'],x.get('phase',0),x['ref']))
    # Main navigation indexes the final timeline. Inherited IDs predate inserted
    # checkpoints and therefore cannot be retained after sorting this projection.
    for index,moment in enumerate(moments):moment['id']=index
    for index,audit in enumerate(original['valuations']):
        bi,di=audit['boundary'],audit['choice'];boundary=history['boundaries'][bi]
        owner=boundary['choices'][di]['input']['legacy']['own']['id']
        ref=f'valuations[{index}]';anchor=f'boundaries[{bi}]'
        nodes[ref]=dict(ref=ref,anchor=anchor,tick=boundary['tick'],owners=[owner],kind='local_valuation',label=f'Captured024 {audit["policy"]} score terms for individual {owner}')
        data['causal']['edges'].append(dict(source=f'{anchor}.choices[{di}]',target=ref,kind='exact_boundary_choice_valuation'))
    deferrals=[]
    for bi,boundary in enumerate(history['boundaries']):
        for di,decision in enumerate(boundary['choices']):
            own=decision['input']['legacy']['own']
            meal=next(c for c in decision['candidates'] if c['action']=='Consume')
            if not own['food'] or not meal['eligible'] or decision['selected']=='Consume':continue
            work=next((c for c in data['intervals'] if c['source']==bi and own['id'] in c['owners'] and c['start']==boundary['tick']),None)
            if work is None:continue
            after=next(p for p in data['observer_physical'] if p['tick']==work['end'])
            deferrals.append(dict(ref=f'boundaries[{bi}].choices[{di}]',owner=own['id'],tick=boundary['tick'],food=own['food'],hunger=own['hunger'],selected=decision['selected'],execution_ref=work['start_ref'],release_ref=work['release_ref'],end=work['end'],hunger_after_completion_tick=after['hunger'][own['id']]))
    data['valuation_deferrals']=deferrals
    return data


def comparison(left,right):
    result=native.comparison(left,right)
    ca,cb=dict(left['valuation']['config']),dict(right['valuation']['config'])
    ma,mb=ca.pop('valuation'),cb.pop('valuation')
    choices=lambda data:{(b['tick'],d['input']['legacy']['own']['id']):(bi,di,d) for bi,b in enumerate(data['trial']['boundaries']) for di,d in enumerate(b['choices'])}
    aa,bb=choices(left),choices(right);first=None
    for key in sorted(aa.keys() & bb.keys()):
        if aa[key][2]['candidates']==bb[key][2]['candidates']:continue
        first=dict(tick=key[0],owner=key[1],same_local_input=aa[key][2]['input']==bb[key][2]['input'])
        for side,data,mapping in [('left',left,aa),('right',right,bb)]:
            bi,di,d=mapping[key]
            index,audit=next((n,v) for n,v in enumerate(data['valuation']['valuations']) if (v['boundary'],v['choice'])==(bi,di))
            first[side]=dict(ref=f'boundaries[{bi}].choices[{di}]',audit_ref=f'valuations[{index}]',audit=audit,selected=d['selected'])
        break
    result.update(valuation_treatment=True,matched_except_valuation=ca==cb,
                  valuation_contrast={ma,mb}=={'Baseline','MealRelief'},left_mode=ma,right_mode=mb,
                  full_state_trace_null=left['trial']==right['trial'],first_score_difference=first,
                  note='Observer-only024comparison. Only ordinary eligible Consume valuation changes; public opportunities, independent consent, finite supply and physical laws are unchanged. Later outcomes were unavailable to selection. Counts and hunger area are distinct dimensions, not a welfare score.')
    result['left_metrics']['queries']=sum(m['kind']=='inquiry' for m in left['timeline'])
    result['right_metrics']['queries']=sum(m['kind']=='inquiry' for m in right['timeline'])
    return result
