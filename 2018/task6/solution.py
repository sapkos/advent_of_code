import re
from operator import itemgetter
from collections import Counter

def d(p, q):
    return abs(p[0]-q[0]) + abs(p[1]-q[1])

points = []
with open('input.txt') as f:
    for line in f:
        points.append(tuple(map(int, re.findall('\d+', line))))

left_to_right = sorted(points, key=itemgetter(0))
top_to_bottom = sorted(points, key=itemgetter(1))

most_left = left_to_right[0][0]
most_top = top_to_bottom[0][1]
most_right = left_to_right[-1][0]
most_bottom = top_to_bottom[-1][1]

x_size = most_right - most_left
y_size = most_bottom - most_top

def find_owner(p):
    neighbours = sorted([(d(p, q), q) for q in points], key=itemgetter(0))
    if neighbours[0][0] == neighbours[1][0]:
        return (-1, -1)
    else:
        return neighbours[0][1]

def find_sum_distances(p):
    return sum([d(p, q) for q in points])

infinite_points = set()
# left edge
for y in range(most_top, most_bottom+1):
    infinite_points.add(find_owner((most_left, y)))
#right edge
for y in range(most_top, most_bottom+1):
    infinite_points.add(find_owner((most_right, y)))
#top edge
for x in range(most_left, most_right+1):
    infinite_points.add(find_owner((x, most_top)))
#bottom edge
for x in range(most_left, most_right+1):
    infinite_points.add(find_owner((x, most_bottom)))

owners = []

for x in range(most_left, most_right+1):
    for y in range(most_top, most_bottom+1):
        owners.append(find_owner((x, y)))

c = Counter(owners)
print([(k, v) for k, v in c.most_common() if k not in infinite_points][0][1])

D = 1e4

size = 0 

for x in range(0, most_right+1):
    for y in range(0, most_bottom+1):
        size += 1 if find_sum_distances((x, y)) < D else 0

print(size)


