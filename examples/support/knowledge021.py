"""Independent 021 primitives. No Rust scorer/evaluator or prepared outcomes.

Only unchanged historical contact/activity arithmetic is reused. New memory,
receipt ownership,expiry,own-result updates and optional-knowledge gating are
reconstructed here from actual channels,not from archived slot assertions.
"""
import copy
import review020 as legacy


class Memory:
    def __init__(self, mode, epoch, cadence):
        self.mode=mode;self.epoch=epoch;self.cadence=cadence
        self.slots=[None]*3;self.receipts=[0]*3;self.records=[]

    def wants(self,now):
        assert now>=self.epoch
        return self.mode=='FreshReference' or self.mode!='Unobserved' and (now-self.epoch)%self.cadence==0

    def change(self,owner,now,kind,after,physical=None,point=None):
        self.records.append(dict(id=len(self.records),owner=owner,tick=now,kind=kind,
                                 before=copy.deepcopy(self.slots[owner]),after=copy.deepcopy(after),physical=physical,point=point))
        self.slots[owner]=copy.deepcopy(after)

    def expire(self,owner,now):
        c=self.slots[owner]
        if c is not None and (self.mode=='Expire12' and now-c['observation']['observed_at']>=12 or self.mode=='CurrentOnly' and now!=c['observation']['observed_at'] or self.mode=='Unobserved'):
            self.change(owner,now,'Expired',None)

    def receive(self,owner,obs,kind,physical=None,point=None):
        c=dict(owner=owner,receipt=self.receipts[owner],observation=copy.deepcopy(obs),own_takes=0)
        self.receipts[owner]+=1
        self.change(owner,obs['observed_at'],kind,c,physical,point)

    def prepare(self,owner,now,cache,duration,point):
        self.expire(owner,now)
        if self.wants(now):
            self.receive(owner,dict(id=cache['id'],observed_at=now,stock=cache['stock'],accessible=cache['accessible'],duration=duration),'Sense',point=point)
        return copy.deepcopy(self.slots[owner])

    def discard(self,owner,now):
        if self.mode=='CurrentOnly' and self.slots[owner] is not None:self.change(owner,now,'Discarded',None)

    def result(self,owner,cache,now,outcome,physical,duration):
        self.expire(owner,now)
        if self.mode=='Unobserved':return
        if outcome=='Granted' and self.slots[owner] is not None:
            c=copy.deepcopy(self.slots[owner]);assert c['observation']['id']==cache
            c['own_takes']=min(2**32-1,c['own_takes']+1)
            self.change(owner,now,'Taken',c,physical=physical)
        elif outcome=='Depleted':
            self.receive(owner,dict(id=cache,observed_at=now,stock=0,accessible=True,duration=duration),'Depleted',physical=physical)

    def finish(self,now):
        for owner in range(3):self.expire(owner,now)
        return copy.deepcopy(self.slots)


def choose(i,mode):
    actor=i['legacy'];cs,selected=legacy.old.initiate(actor,'History')
    if not i['inquiry_lane_free']:
        for c in cs:
            if c['intent']!='Wait' and c['intent']['Invite']['kind']=='Inquiry':c.update(eligible=False,score=-1)
        selected=max(cs,key=lambda c:c['score'])['intent']
    contact=dict(input=actor,mode='History',candidates=cs,selected=selected)
    candidates,action=legacy.parent.activity(actor,'Baseline',cs,selected)
    base=dict(contact=contact,mode='Active',candidates=candidates,selected=action)
    c=i['retained']
    can=(c is not None and c['owner']==actor['own']['id'] and c['observation']['observed_at']<=actor['now']
         and c['observation']['stock']>c['own_takes'] and c['observation']['accessible']
         and next(p['available'] for p in actor['people'] if p['id']==actor['own']['id'])
         and actor['own']['food']==0 and actor['own']['hunger']>0 and actor['now']+i['duration']<=actor['end'])
    candidates=candidates+[dict(action='Acquire',eligible=bool(can),score=actor['own']['hunger'] if can else -1)]
    selected=action if mode=='Ignored' else 'Acquire' if mode=='Acquire' and can else max(candidates,key=lambda x:x['score'])['action']
    return dict(input=i,policy=mode,base=base,candidates=candidates,selected=selected)
