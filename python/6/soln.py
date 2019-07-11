#!/usr/bin/python3

def getAndParseInput():
    pairs = []
    for line in open('input'):
        pairs.append([int(x) for x in line.strip().split(', ')])
    return pairs

def part1():
    coords = getAndParseInput()
    n = len(coords)

    xs = list(map(lambda x: x[0], coords))
    ys = list(map(lambda x: x[1], coords))
    min_x = min(xs)
    max_x = max(xs)
    min_y = min(ys)
    max_y = max(ys)

    num_xs = max_x - min_x + 1
    num_ys = max_y - min_y + 1

    # Contains winners: the index of the closest point (or None, if tied)
    grid = [[None] * num_ys for _ in range(num_xs)]

    for x in range(min_x, max_x+1):
        for y in range(min_y, max_y+1):
            best = None
            bestval = None
            for i, p in enumerate(coords):
                px, py = p
                val = dist(x, y, px, py)
                if val == bestval:
                    # Tie
                    best = None
                    bestval = val
                elif bestval is None or val < bestval:
                    best = i
                    bestval = val
            grid[x % num_xs][y % num_ys] = best

    areas = [0] * n
    for x in range(min_x, max_x+1):
        for y in range(min_y, max_y+1):
            if grid[x % num_xs][y % num_ys] is None:
                continue
            if x == min_x or x == max_x or \
                    y == min_y or y == max_y:
                areas[grid[x % num_xs][y % num_ys]] = None
            elif areas[grid[x % num_xs][y % num_ys]] is not None:
                areas[grid[x % num_xs][y % num_ys]] += 1

    best = None
    bestval = None
    for i, a in enumerate(areas):
        if a is not None and (bestval is None or a > bestval):
            best = i
            bestval = a

    printGrid(grid)
    print(bestval)

def dist(x1, y1, x2, y2):
    return abs(x1 - x2) + abs(y1 - y2)

def printGrid(grid):
    for row in grid[:40]:
        for val in row[:80]:
            print(val % 10 if val is not None else '-', end='')
        print()

def part2():
    coords = getAndParseInput()
    n = len(coords)

    xs = list(map(lambda x: x[0], coords))
    ys = list(map(lambda x: x[1], coords))
    min_x = min(xs)
    max_x = max(xs)
    min_y = min(ys)
    max_y = max(ys)

    THRESH = 10000

    count = 0
    for x in range(min_x, max_x+1):
        for y in range(min_y, max_y+1):
            if sum(dist(x, y, p[0], p[1]) for p in coords) < THRESH:
                count += 1

    print(count)

#part1()
part2()
