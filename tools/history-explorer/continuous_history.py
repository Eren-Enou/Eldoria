"""Explicit023 source adapter. Observer consumer, no scientific-reader imports.
Native022 primitives are a shared presentation vocabulary, never a schema coercion.
Original023envelope is losslessly represented by history plus continuous header.
"""
import collections,copy,gzip,hashlib,json
from pathlib import Path
import importlib.util
_native_spec=importlib.util.spec_from_file_location('viewer023_primitives',Path(__file__).with_name('concurrent_history.py'))
native=importlib.util.module_from_spec(_native_spec);_native_spec.loader.exec_module(native)
VERSION='experiment-023-trial/v1'
PHYSICAL='experiment-023-physical-history/v1'
EXTRACTOR=7

def validate(original):
    if set(original)!={'schema','config','history','checkpoints'} or original['schema']!=VERSION:
        raise ValueError('Unsupported023original envelope')
    native.validate(original['history'],version=PHYSICAL)
    cfg=original['config'];h=original['history']
    if not 1<=cfg['horizon']<=1024 or not 1<=cfg['period']<=128 or cfg['capacity']!=2 or cfg['duration'] not in (2,4,8) or cfg['cadence']!=8:
        raise ValueError('Unsupported bounded023configuration')
    if cfg['mode'] not in ('Windowed','Continuous') or cfg['phase'] not in ('Epoch','Absolute'):
        raise ValueError('Invalid023mode/phase')
    if h['end']!=h['start']+cfg['horizon']+33:
        raise ValueError('Invalid023nominal/drainedendpoint')
    if not original['checkpoints'] or original['checkpoints'][-1]['tick']!=h['end']:
        raise ValueError('Missing023finalcheckpoint')
    known={x['commitment']['id'] for x in h['transitions'] if not x['completed']}
    if any(not set(c['active']).issubset(known) for c in original['checkpoints']):
        raise ValueError('Unknown023checkpoint commitment')

class Source:
    def __init__(self,root,path):
        self.root=Path(root).resolve();self.path=Path(path).resolve()
        if self.path.is_dir():self.path=self.path/'trials.jsonl.gz'
        self.registered=self.path==self.root/'experiments/023/trials.jsonl.gz'
        self.entries={};self.loaded=collections.OrderedDict();self.expected=None
    def guard(self):
        freeze=self.root/'experiments/023/source-freeze.json'
        for name,expected in json.loads(freeze.read_bytes()).items():
            if hashlib.sha256((self.root/name).read_bytes().replace(b'\r\n',b'\n')).hexdigest()!=expected:
                raise ValueError('Original023runtime source lock changed: '+name)
        value=hashlib.sha256(self.path.read_bytes()).hexdigest()
        if self.expected is not None and value!=self.expected:raise ValueError('023source changed after catalog')
        if self.registered:
            manifest=json.loads((self.path.parent/'archive-manifest.json').read_bytes())['archives']['trials.jsonl.gz']
            if value!=manifest['sha256']:raise ValueError('023registeredarchive changed')
        return hashlib.sha256(freeze.read_bytes().replace(b'\r\n',b'\n')).hexdigest(),value
    def lines(self):
        with (gzip.open(self.path,'rb') if self.path.suffix=='.gz' else self.path.open('rb')) as f:
            for line in f:
                if len(line)>64*1024*1024:raise ValueError('023member exceeds64MiB input bound')
                yield line
    def catalog(self,status):
        freeze,archive=self.guard();self.expected=archive;decoded=hashlib.sha256();entries={}
        matrix=json.loads((self.root/'experiments/023/registered-matrix.json').read_bytes()) if self.registered else None
        for ordinal,line in enumerate(self.lines()):
            decoded.update(line);original=json.loads(line);validate(original);cfg=original['config'];h=original['history']
            if matrix is not None and (ordinal>=len(matrix) or cfg!=matrix[ordinal]):raise ValueError('023registration mismatch')
            identity=dict(experiment='023',archive=self.path.name,record=ordinal,seed=str(cfg['seed']),case=cfg['case'],mode=cfg['mode'],config=cfg)
            key=native.digest(dict(identity=identity,archive_sha256=archive,schema=VERSION))[:24]
            entries[key]=dict(id=key,**identity,source_schema=VERSION,physical_schema=PHYSICAL,archive_sha256=archive,record_sha256=hashlib.sha256(line.rstrip(b'\r\n')).hexdigest(),trial_fingerprint=native.digest(original),policy_freeze_sha256=freeze,epoch=h['start'],end=h['end'],nominal_stop=h['start']+cfg['horizon'],raw_bytes=len(line),individual_ids=[0,1,2,3],original_reference_root='history',registered=self.registered,scientific_status=status if self.registered else 'Custom deterministic prototype run; not registered research evidence; no protection/adoption',integrity='Original023runtime source locks and actual source/member hashes verified')
        if not entries:raise ValueError('Empty023source')
        if matrix is not None:
            expected=json.loads((self.path.parent/'archive-manifest.json').read_bytes())['archives']['trials.jsonl.gz']
            if len(entries)!=len(matrix) or decoded.hexdigest()!=expected['decoded_sha256']:raise ValueError('Incomplete023archive')
        for entry in entries.values():entry['decoded_archive_sha256']=decoded.hexdigest()
        self.entries=entries;return entries
    def get(self,entry):
        self.guard();key=entry['id']
        if key not in self.entries or entry['trial_fingerprint']!=self.entries[key]['trial_fingerprint']:raise ValueError('Unregistered023member')
        if key in self.loaded:self.loaded.move_to_end(key);return self.loaded[key]
        for n,line in enumerate(self.lines()):
            if n!=entry['record']:continue
            if hashlib.sha256(line.rstrip(b'\r\n')).hexdigest()!=entry['record_sha256']:raise ValueError('023member changed')
            original=json.loads(line);validate(original)
            if native.digest(original)!=entry['trial_fingerprint']:raise ValueError('023original changed')
            self.loaded[key]=original
            while len(self.loaded)>2:self.loaded.popitem(last=False)
            return original
        raise ValueError('Missing023member')

def rebuild(data):
    header=data['continuous']
    return dict(schema=header['schema'],config=header['config'],history=data['trial'],checkpoints=header['checkpoints'])

def export(entry,original):
    validate(original);data=native.export(entry,original['history']);data['extractor_version']=EXTRACTOR
    data['continuous']=dict(schema=original['schema'],config=copy.deepcopy(original['config']),checkpoints=copy.deepcopy(original['checkpoints']))
    moments=data['timeline'];nodes=data['causal']['nodes']
    history=original['history'];compact=history['final_state']['base']['base']['base']
    queries=compact['base']['inquiry']['records'];provenance=compact['provenance']
    for interval in data['intervals']:
        if interval['kind']!='Query':continue
        proposal=history['boundaries'][interval['source']]
        choice=next(d for d in proposal['choices'] if d['input']['legacy']['own']['id']==interval['owners'][0])
        matches=[(n,q) for n,q in enumerate(queries) if q['decision']['input']['owner']==interval['owners'][0] and q['tick']==interval['end'] and q['tick']-q['time_spent']==interval['start'] and q['decision']['selected']==choice['input']['legacy']['query']['decision']['selected']]
        if len(matches)!=1:raise ValueError('Ambiguous023query execution identity')
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
    return data

def without_public_end(value):
    if isinstance(value,dict):
        return {k:('<projected public end>' if k=='end' and set(value)=={'own','profile','now','end','people','query'} else without_public_end(v)) for k,v in value.items()}
    if isinstance(value,list):return [without_public_end(v) for v in value]
    return value

def comparison(left,right):
    result=native.comparison(left,right);a,b=left['continuous'],right['continuous'];ca,cb=dict(a['config']),dict(b['config'])
    ca.pop('mode');cb.pop('mode');ca.pop('role',None);cb.pop('role',None)
    result.update(full_state_trace_null=without_public_end(left['trial'])==without_public_end(right['trial']),opportunity_treatment=True,matched_except_mode=ca==cb,mode_contrast=a['config']['mode']!=b['config']['mode'],left_mode=a['config']['mode'],right_mode=b['config']['mode'],note='Observer-only023comparison. Reporting checkpoints do not reset the world. Mode changes public eligibility, not scores or supply. Final endpoint includes charged33tickdrain. More contact or lower hunger area is not universal benefit. Raw input-budget differences remain distinct from a behavioral null.')
    result['left_metrics']['queries']=sum(m['kind']=='inquiry' for m in left['timeline']);result['right_metrics']['queries']=sum(m['kind']=='inquiry' for m in right['timeline'])
    return result
