from typing import List, Set, Tuple
import fileinput
import math

def read_grid() -> List[List[bool]]:
    grid = []
    for line in fileinput.input():
        row = []
        for char in line:
            if char == "#":
                row.append(True)
            elif char == ".":
                row.append(False)
        grid.append(row)
    return grid

def print_grid(grid: List[List[bool]]):
    for line in grid:
        for char in line:
            if char:
                print("#", end="")
            else:
                print(".", end="")
        print() # newline

def reduce_fraction(x, y) -> Tuple[int, int]:
    d = math.gcd(abs(x), abs(y))
    x_reduced = x // d
    y_reduced = y // d
    return x_reduced, y_reduced # retain signs

def count_visible_asteroids(i: int, j: int, asteroids: Set[Tuple[int, int]]) -> int:
    # find distinct slopes
    slopes = set()

    for asteroid in asteroids:
        # skip this asteroid but compute for all others
        if asteroid != (i, j):
            # compute distance
            x, y = asteroid
            x_diff = x - i
            y_diff = y - j
            # compute slope and store distinct slopes
            slope = reduce_fraction(x_diff, y_diff)
            if slope not in slopes: # unnecessary guard but helps readability
                slopes.add(slope)

    return len(slopes)

# Find asteroids and store as a set of coordinates
def get_asteroids(grid: List[List[bool]]) -> Set[Tuple[int, int]]:
    asteroids = set() # empty set

    for i in range(len(grid)):
        for j in range(len(grid[i])):
            if grid[i][j]:
                asteroids.add((i, j))

    return asteroids

def main():
    grid = read_grid()
    print_grid(grid)

    asteroids = get_asteroids(grid)

    max_count = 0
    best_location = None

    for asteroid in asteroids:
        x, y = asteroid
        count = count_visible_asteroids(x, y, asteroids)
        if count > max_count:
            max_count = count
            best_location = (x, y)
            print("New best is", best_location, "within sight of", max_count)

main()
