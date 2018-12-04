import re
from collections import Counter
pattern = re.compile('[[](.+)[]]\s(.+)')
rows = []
with open('input.txt') as f:
    for line in f:
        rows.append(pattern.findall(line)[0])

rows = sorted(rows, key=lambda x: x[0])

minutes = 0
guard_stats = {}
guard_rows = []

def grouped_events(itrbl):
    for row in itrbl:
        if 'Guard' in row[1]:
            guard_id = re.findall('\d+', row[1])[0]
            yield (0, row[0], guard_id)
        else:
            yield row

guards_stats = {}
gurads_summed = {}

for event in grouped_events(rows):
    if len(event)==3:
        guard_id = event[2]
        if guard_id not in guards_stats:
            guards_stats[guard_id] = []
    elif event[1] == 'falls asleep':
        start_time = int(event[0][-2:])
    elif event[1] == 'wakes up':
        end_time = int(event[0][-2:])
        guards_stats[guard_id] += list(range(start_time, end_time))

guards_summed = dict((k, len(guards_stats[k])) for k in guards_stats)

top_guard = sorted(guards_summed.items(), key=lambda x: x[1], reverse=True)[0][0]
top_guard_stats = guards_stats[top_guard]

print(Counter(top_guard_stats).most_common()[0])
print(guards_summed[top_guard])

minutes_guards = [(minute, guard) for guard in guards_stats for minute in guards_stats[guard]]
minutes_guards = sorted(minutes_guards, key=operator.itemgetter(0))

print(Counter(minutes_guards).most_common()[0])

