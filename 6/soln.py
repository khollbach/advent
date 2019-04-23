#!/usr/bin/python3

def get_and_parse_input():
    pairs = []
    for line in open('input'):
        pairs.append([int(x) for x in line.strip().split(', ')])
    return pairs

def part1():
    coords = get_and_parse_input()

    xs = list(map(lambda x: x[0], coords))
    ys = list(map(lambda x: x[1], coords))
    min_x = min(xs)
    max_x = max(xs)
    min_y = min(ys)
    max_y = max(ys)

    x_dist = max_x - min_x
    y_dist = max_y - min_y

    # Run the simulation for this many rounds
    max_dist = max(x_dist, y_dist)

    x_upper = (x_dist + 1) + 2 * max_dist
    y_upper = (y_dist + 1) + 2 * max_dist

    grid = [[None] * x_upper for y in range(y_upper)]

    print(len(grid))
    print(len(grid[0]))

part1()
