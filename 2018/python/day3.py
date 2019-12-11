import re
import sys

from collections import namedtuple

# Part 1

Rect = namedtuple('Rect', 'id x y w h')

pattern = re.compile(r'^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$')

rects = []
for line in open('../inputs/3').readlines():
    line = line.strip()

    r = Rect(*map(int, pattern.match(line).groups()))
    rects.append(r)

square_width = max(map(lambda r: r.x + r.w, rects))
square_height = max(map(lambda r: r.y + r.h, rects))
size = max(square_width, square_height)

square = [[0] * size for i in range(size)]

for r in rects:
    for x in range(r.x, r.x + r.w):
        for y in range(r.y, r.y + r.h):
            square[x][y] += 1

num_collisions = 0
for x in range(size):
    for y in range(size):
        if square[x][y] > 1:
            num_collisions += 1

print(num_collisions)

# Part 2

def is_successful(r):
    for x in range(r.x, r.x + r.w):
        for y in range(r.y, r.y + r.h):
            if square[x][y] > 1:
                return False
    return True

for r in rects:
    if is_successful(r):
        print(r.id)
        break
