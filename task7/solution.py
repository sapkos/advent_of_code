import re
from operator import itemgetter
from collections import defaultdict
from collections import Counter

job_out = defaultdict(list)
job_in = defaultdict(list)

r = re.compile('\s(\w)\s')

nodes = set()

with open('input.txt') as f:
    for line in f:
        _from, _to = tuple(r.findall(line))
        nodes.add(_from)
        nodes.add(_to)
        job_out[_to].append(_from)
 
tasks = nodes.copy()

done_tasks = []

while tasks:
    # looking for tasks without dependencies
    # taking first one alphabeticaly

    available_tasks = sorted([k for k in tasks if len(job_out[k])==0])
    next_task = available_tasks[0]
    print(available_tasks)
    print(next_task)
    # task is done so we drop it from tasks
    tasks.remove(next_task)
    
    # update dependencies
    for k in job_out:
        if next_task in job_out[k]:
            job_out[k].remove(next_task)
    # saving what task we did
    done_tasks.append(next_task)

print(''.join(done_tasks))

tasks = nodes.copy()
done_tasks = []
workers = dict((i, dict(task=None, time_left=None)) for i in range(6))

def task_size(task):
    return 60 + ord(task) - 64

total_time = 0

job_out = defaultdict(list)

with open('input.txt') as f:
    for line in f:
        _from, _to = tuple(r.findall(line))
        nodes.add(_from)
        nodes.add(_to)
        job_out[_to].append(_from)

active_tasks = []

while tasks:
    # looking for tasks without dependencies
    # sorting
    available_tasks = sorted([k for k in tasks if len(job_out[k])==0 and k not in active_tasks], reverse=True)
    # if there are available workers assign tasks
    for _, worker in workers.items():
        if available_tasks:
            if worker['task'] is None:
                worker['task'] = available_tasks.pop()
                active_tasks.append(worker['task'])
                worker['time_left'] = task_size(worker['task'])
        else:
            break
    
    # second passes
    
    for _, worker in workers.items():
        if worker['task'] is not None:
            worker['time_left'] -= 1
            # check if task is done
            if worker['time_left'] == 0:
                # task is done
                tasks.remove(worker['task'])
                
                # update dependencies
                for k in job_out:
                    if worker['task'] in job_out[k]:
                        job_out[k].remove(worker['task'])
                
                # task no longer active

                active_tasks.remove(worker['task'])
                worker['task'] = None

    # time goes by
    total_time += 1

print(total_time)


















