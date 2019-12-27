from typing import List, Set, Tuple
import fileinput
import math

def main():
    grid = read_grid()
    asteroids = get_asteroids(grid)

    print(max(count_visible_asteroids(asteroids, i, j) for i, j in asteroids))

def read_grid() -> List[List[bool]]:
    grid = []
    for line in fileinput.input():
        line = line.rstrip("\n")
        row = []
        for char in line:
            if char == "#":
                row.append(True)
            else:
                assert char == "."
                row.append(False)
        grid.append(row)
    return grid

def print_grid(grid: List[List[bool]]) -> None:
    for row in grid:
        for cell in row:
            if cell:
                print("#", end="")
            else:
                print(".", end="")
        print()

def get_asteroids(grid: List[List[bool]]) -> Set[Tuple[int, int]]:
    """Find asteroids and store as a set of coordinates."""
    asteroids = set()
    for i in range(len(grid)):
        for j in range(len(grid[i])):
            if grid[i][j]:
                asteroids.add((i, j))
    return asteroids

def count_visible_asteroids(
    asteroids: Set[Tuple[int, int]],
    i: int,
    j: int
) -> int:
    """How many asteroids are visible from (i, j)?"""
    slopes: Set[Tuple[int, int]] = set()
    for x, y in asteroids:
        if (x, y) != (i, j):
            slopes.add(reduce_slope(x - i, y - j))
    return len(slopes)

def reduce_slope(dx: int, dy: int) -> Tuple[int, int]:
    divisor = math.gcd(abs(dx), abs(dy))
    return (dx // divisor, dy // divisor)

main()
