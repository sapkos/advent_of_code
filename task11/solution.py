from collections import defaultdict
from operator import itemgetter

power_levels = defaultdict(int)
total_power_levels = defaultdict(int)

with open('input.txt') as f:
    grid_serial_number = int(f.read())


def get_power_level(p):
    rack_id = p[0]+10
    power_level = rack_id * p[1]
    power_level += grid_serial_number
    power_level *= rack_id
    power_level = power_level // 100 % 10
    power_level -= 5
    return power_level

for i in range(300):
    for j in range(300):
        power_levels[(i+1, j+1)] = get_power_level((i+1, j+1))

def get_total_power_levels(p, n, power_levels=power_levels):
    return sum(power_levels[p[0]+i, p[1]+j] for i in range(0, n) for j in range(0, n))

for n in range(1, 300):
    for i in range(1, 300-n+1):
        for j in range(1, 300-n+1):
            total_power_levels[(i, j, n)] = (total_power_levels[(i, j, n-1)]
                + sum(power_levels[(i+n-1, j+k)] for k in range(0, n))
                + sum(power_levels[(i+k, j+n-1)] for k in range(0, n-1))
                )


print(sorted([(k, v) for k, v in total_power_levels.items() if k[2] == 3], key=itemgetter(1), reverse=True)[0])
print(sorted([(k, v) for k, v in total_power_levels.items()], key=itemgetter(1), reverse=True)[0])
