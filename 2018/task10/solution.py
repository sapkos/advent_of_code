import re
import time
import numpy as np
from operator import itemgetter

from matplotlib import pyplot as plt
from celluloid import Camera

points = []

with open('input.txt') as f:
    for line in f:
        points.append(tuple(map(int, re.findall('-?\d+', line))))


def update_points(points):
    while True:
        points = [(x+vx, y+vy, vx, vy) for x, y, vx, vy in points]
        yield points

new_points = update_points(points)

for i in range(500000):
    new = next(new_points)
    if i > 100*500-1000:
        plt.figure()
        x = list(map(itemgetter(0), new))
        y = list(map(itemgetter(1), new))
        plt.scatter(x=x, y=y)
        plt.show()
        time.sleep(0.5)
        plt.close()

