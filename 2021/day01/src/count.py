import sys
import itertools
print(dir(itertools))
from itertools import tee

def window(iterable, size):
    iters = tee(iterable, size)
    for i in range(1, size):
        for each in iters[i:]:
            next(each, None)
    return zip(*iters)

input = sys.argv[1]
print(input)

measurements = []

for line in open(input):
    measurements.append(int(line))

print(f"{len(measurements)} measurements")
print(measurements[0:6])

windows = window(measurements, 3)
window_sums = [sum(w) for w in windows]

print(f"{len(window_sums)} window_sums")
print(window_sums[:4])


count = 0
for prev, current in itertools.pairwise(window_sums):
    if prev < current:
        count += 1

print(count)
