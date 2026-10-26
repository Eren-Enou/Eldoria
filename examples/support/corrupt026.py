"""Read-only archive; independent scratch mutations, no simulator or tuning."""
import argparse,copy,json
import review026 as r

def core(t):return r.review025_food_fate.core(t['body']['history'])
def first_choice(t):return t['body']['history']['boundaries'][0]['choices'][0]
def corruptions(anchor):
    checked=r.verify(anchor)
    changes=[
        ('future_stock_input',lambda t:first_choice(t)['input'].update(actual_future_stock=8)),
        ('modified_hunger',lambda t:first_choice(t)['input']['legacy']['own'].update(hunger=79)),
        ('extra_score',lambda t:first_choice(t)['candidates'][4].update(final_score=75)),
        ('duration',lambda t:first_choice(t)['input']['raw_retained']['observation'].update(preparation_ticks=5)),
        ('stable_selection',lambda t:first_choice(t).update(selected='Wait')),
        ('eligibility',lambda t:first_choice(t)['candidates'][4].update(eligible=False)),
        ('duplicate_output',lambda t:t['body']['material']['operations'].append(copy.deepcopy(t['body']['material']['operations'][-1]))),
        ('missing_waste',lambda t:next(o for o in t['body']['material']['operations'] if o['outcome']=='Prepared').update(waste_after=0)),
        ('forged_meal',lambda t:core(t)['cognition']['consumption'][0].update(consumed=2)),
        ('social_admission',lambda t:t['body']['history']['admissions'][0].update(reason='Expired')),
        ('wrong_source',lambda t:first_choice(t)['input']['raw_retained']['observation'].update(id=1)),
        ('registered_parameter',lambda t:t['config']['world']['preparation'].update(raw_initial=9)),
        ('audit_claim',lambda t:t['projection_audit'][0].update(projected_need=100)),
    ]
    failures=[]
    for name,mutate in changes:
        trial=copy.deepcopy(anchor);mutate(trial)
        try:r.verify(trial)
        except (AssertionError,KeyError,ValueError,TypeError):failures.append(name)
        else:raise AssertionError(('corruption accepted',name))
    fabricated=copy.deepcopy(checked);fabricated['food_fate']['ever_used']=[999,999]
    try:r.verify(anchor,findings=fabricated)
    except AssertionError:failures.append('fabricated_later_use_report')
    else:raise AssertionError('fabricated use accepted')
    assert len(failures)==14
    return failures
def main():
    parser=argparse.ArgumentParser();parser.add_argument('archive');parser.add_argument('--output');args=parser.parse_args()
    anchor=next(t for t in r.read(args.archive) if t['config']['source025']==1 and t['config']['mode']=='Baseline')
    failures=corruptions(anchor)
    if args.output:
        with open(args.output,'x',encoding='utf-8')as out:json.dump(dict(anchor='source0251/Baseline',rejected=failures),out,indent=2);out.write('\n')
    print('026 independent scratch controls:',len(failures),'rejected',','.join(failures))
if __name__=='__main__':main()
