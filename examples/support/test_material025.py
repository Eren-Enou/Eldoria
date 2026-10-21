"""Synthetic component controls only; no registered025outcomes are generated."""
import copy
import types
import unittest

import material025 as m


def cell(receipt, tick, raw=True):
    observation = dict(id=0, observed_at=tick, stock=2 if raw else 1, accessible=True)
    if raw:
        observation.update(extraction_ticks=2, preparation_ticks=4, input_units=2, edible_units=1, waste_units=1)
    else:
        observation['duration'] = 4
    return dict(owner=0, receipt=receipt, observation=observation, own_takes=0)


def fixture():
    starts = {n: dict(id=n, work='Prepare', participants=[n], start=0, deadline=6, source=0) for n in range(2)}
    processes = [dict(commitment=n, owner=n, source=0, start=0, extract_at=2, finish_at=6) for n in range(2)]
    def active(n, phase, escrow):
        return dict(process=processes[n], phase=phase, escrow=escrow)
    def point(tick, raw, waste, active):
        return dict(tick=tick, raw=raw, waste=waste, active=active)
    rows = [(2,0,'Extracted',2,0,0,2,0,0,0,0),
            (2,1,'Insufficient',0,0,0,0,0,0,0,0),
            (6,0,'Prepared',0,0,2,0,0,1,0,1),
            (6,1,'Failed',0,0,0,0,0,0,1,1)]
    operations = [dict(id=n,tick=tick,commitment=owner,owner=owner,source=0,outcome=outcome,
                       raw_before=rb,raw_after=ra,escrow_before=eb,escrow_after=ea,
                       food_before=fb,food_after=fa,waste_before=wb,waste_after=wa)
                  for n,(tick,owner,outcome,rb,ra,eb,ea,fb,fa,wb,wa) in enumerate(rows)]
    points = [point(0,2,0,[]),point(0,2,0,[active(0,'Extracting',0)]),
              point(0,2,0,[active(0,'Extracting',0),active(1,'Extracting',0)]),
              point(2,0,0,[active(0,'Preparing',2),active(1,'Extracting',0)]),
              point(2,0,0,[active(0,'Preparing',2),active(1,'Failed',0)]),
              point(6,0,1,[active(1,'Failed',0)]),point(6,0,1,[])]
    settings = dict(feature_enabled=True,access_enabled=True,source_id=0,raw_initial=2,
                    extraction_ticks=2,preparation_ticks=4,
                    observation=dict(period=8,raw_offsets=[0]*4,ready_offsets=[None]*4))
    trial = dict(config=dict(preparation=settings),history=dict(start=0,end=6,config=dict(order='Ascending'),boundaries=[]),
                 material=dict(initial_raw=2,processes=processes,operations=operations,points=points,
                               final_raw=0,final_waste=1,final_active=[]))
    return trial, starts, {0:6,1:6}


def physical(trial, starts, finishes):
    p = m.Physical(trial, starts, finishes)
    agents = [dict(food=0) for _ in range(4)]
    p.finish_tick(0)
    for tick in range(1,7):
        for _,_,_,op in p.by_tick[tick]:
            p.apply(op,agents)
            assert sum(a['food'] for a in agents)+p.remaining() == 2
        p.finish_tick(tick)
    p.finish()
    return p, agents


class Controls(unittest.TestCase):
    def test_explicit_conserved_batch_and_full_failure_deadline(self):
        trial, starts, finishes = fixture()
        p, agents = physical(trial, starts, finishes)
        self.assertEqual([a['food'] for a in agents],[1,0,0,0])
        self.assertEqual((p.raw,p.waste),(0,1))
        self.assertEqual(p.points[4]['active'][1]['phase'],'Failed')
        self.assertEqual(p.points[4]['tick'],2)
        self.assertEqual(p.points[-1]['tick'],6)

    def test_material_corruptions_are_rejected(self):
        for kind in ('missing_waste','early_food','lost_escrow','duplicate_extraction','early_completion','failure_refund'):
            with self.subTest(kind=kind):
                trial, starts, finishes = fixture()
                if kind == 'missing_waste':trial['material']['operations'][2]['waste_after']=0
                elif kind == 'early_food':trial['material']['operations'][0]['food_after']=1
                elif kind == 'lost_escrow':trial['material']['operations'][0]['escrow_after']=0
                elif kind == 'duplicate_extraction':trial['material']['operations'][1]['outcome']='Extracted'
                elif kind == 'early_completion':trial['material']['operations'][2]['tick']=3
                else:finishes[1]=2
                with self.assertRaises(AssertionError):physical(trial,starts,finishes)

    def test_local_selection_and_true_stable_tie(self):
        i = dict(legacy=dict(own=dict(id=0,food=0,hunger=80),now=0,end=33,people=[dict(id=0,available=True)]),
                 retained=None,duration=4,inquiry_lane_free=True,raw_retained=cell(0,0))
        original = [dict(action=a,eligible=eligible,score=score) for a,eligible,score in
                    [('Wait',True,0),('Consume',False,-1),('Invite',True,58),('Acquire',False,-1)]]
        selected = m.selection(i,original,'Baseline')
        self.assertEqual(selected['selected'],'Prepare')
        self.assertEqual(selected['terms'][4]['final_score'],74)
        tied = copy.deepcopy(original);tied[2]['score']=74
        self.assertEqual(m.selection(i,tied,'Baseline')['selected'],'Invite')
        i['raw_retained']=None
        self.assertFalse(m.selection(i,original,'Baseline')['terms'][4]['eligible'])

    def test_overwrite_and_redelivery_are_not_archive_recovery(self):
        settings = dict(feature_enabled=True,access_enabled=True,source_id=0,raw_initial=2,
                        extraction_ticks=2,preparation_ticks=4,
                        observation=dict(period=16,raw_offsets=[0]*4,ready_offsets=[8]*4))
        raw0, ready1, raw2 = cell(0,0), cell(1,8,False), cell(2,16)
        old = [dict(id=0,owner=0,tick=8,kind='Sense',before=None,after=ready1,physical=None,point=None),
               dict(id=1,owner=0,tick=16,kind='Discarded',before=ready1,after=None,physical=None,point=None)]
        raw = [dict(id=0,owner=0,tick=0,kind='Sense',before=None,after=raw0,physical=None,cache_record=None),
               dict(id=1,owner=0,tick=8,kind='OverwrittenByReady',before=raw0,after=None,physical=None,cache_record=0),
               dict(id=2,owner=0,tick=16,kind='Sense',before=None,after=raw2,physical=None,cache_record=1)]
        boundaries = [dict(tick=tick,choices=[{'input':{'legacy':{'own':{'id':0}}}}]) for tick in (0,8,16)]
        trial = dict(config=dict(world=dict(phase='Epoch',horizon=17),preparation=settings),
                     history=dict(start=0,end=16,config=dict(duration=4),grants=[],boundaries=boundaries,memories=old,final_memory=[None]*4),
                     material=dict(operations=[],memories=raw,final_slots=[raw2,None,None,None]))
        p = types.SimpleNamespace(raw_at_tick={tick:2 for tick in range(17)})
        slots = m.Slots(trial,p,{tick:1 for tick in range(17)},lambda tick:[dict(id=n,available=True) for n in range(4)])
        self.assertEqual(slots.inputs[0,0][1]['receipt'],0)
        self.assertIsNone(slots.inputs[1,0][1])
        self.assertEqual(slots.inputs[2,0][1]['receipt'],2)
        self.assertEqual(slots.inputs[2,0][1]['observation']['observed_at'],16)


if __name__ == '__main__':
    unittest.main()
