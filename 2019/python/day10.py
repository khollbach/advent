from typing import List, Set, Tuple, Optional
import fileinput, math, copy, doctest

Point = Tuple[int, int]

def main():
    grid = read_grid()
    asteroids = get_asteroids(grid)

    max_, home_base = max(
        (count_visible_asteroids(asteroids, base), base) for base in asteroids)
    #print_grid(grid, home_base)
    print(f"base: {home_base}, count: {max_}")

    n = 200
    last_zap = nth_asteroid_vaporized(grid, asteroids, home_base, n)
    print(f"{n}th zap (x, y): {last_zap}")

    # Un-convert coordinates. (weird airplane mode)
    num_rows = len(grid[0])
    x, y = last_zap
    xx = x
    yy = num_rows - y - 1
    print(f"(xx, yy): {(xx, yy)}")

def read_grid() -> List[List[bool]]:
    """Read in the grid in such a way that grid[x][y] is x from the left, and
    y from the bottom."""
    # Read in standard array/matrix indexing order.
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

    # Change orientation to match standard (x, y) plane notation.
    n_rows = len(grid)
    n_cols = len(grid[0])
    new_grid = []
    for j in range(n_cols):
        col = []
        for i in reversed(range(n_rows)):
            col.append(grid[i][j])
        new_grid.append(col)

    return new_grid

def print_grid(grid: List[List[bool]], base: Optional[Point]=None) -> None:
    n_cols = len(grid)
    n_rows = len(grid[0])
    for y in reversed(range(n_rows)):
        for x in range(n_cols):
            if base and (x, y) == base:
                print('O', end="")
            elif grid[x][y]:
                print("#", end="")
            else:
                print(".", end="")
        print()

def get_asteroids(grid: List[List[bool]]) -> Set[Point]:
    """Find asteroids and store as a set of coordinates."""
    asteroids = set()
    for x in range(len(grid)):
        for y in range(len(grid[x])):
            if grid[x][y]:
                asteroids.add((x, y))
    return asteroids

def count_visible_asteroids(asteroids: Set[Point], p: Point) -> int:
    """How many asteroids are visible from p?"""
    x, y = p
    return len(get_visible_slopes(asteroids, p))

def get_visible_slopes(asteroids: Set[Point], p: Point) -> Set[Point]:
    """What are the visible slopes from p?"""
    x, y = p
    slopes = set()
    for x2, y2 in asteroids:
        if (x2, y2) != (x, y):
            slopes.add(reduce_slope(x2 - x, y2 - y))
    return slopes

def reduce_slope(di: int, dj: int) -> (int, int):
    """Reduce a fraction to its lowest terms, without cancelling signs.

    >>> reduce_slope(0, 2)
    (0, 1)
    """
    assert di != 0 or dj != 0
    divisor = math.gcd(abs(di), abs(dj))
    return (di // divisor, dj // divisor)

def nth_asteroid_vaporized(
    grid: List[List[bool]],
    asteroids: Set[Point],
    home_base: Point,
    n: int,
) -> Optional[Point]:
    """Lazer starts pointing up, and rotates until nth asteroid is zapped."""
    # We will mutate grid!
    grid = copy.deepcopy(grid)

    slopes: List[Point] = \
        sorted(get_visible_slopes(asteroids, home_base), key=slope_sort_key)

    num_zapped = 0
    while slopes:
        slopes_to_remove = set()
        for slope in slopes:
            # Find the first visible asteroid (if any); "zap" it.
            p = first_visible_asteroid(grid, home_base, slope)
            if p:
                x, y = p
                grid[x][y] = False
                num_zapped += 1
                if num_zapped == n:
                    #print_grid(grid, home_base)
                    return p
            else:
                slopes_to_remove.add(slope)
        slopes = list(filter((lambda s: s not in slopes_to_remove), slopes))

    return None

def first_visible_asteroid(
    grid: List[List[bool]],
    start: Point,
    slope: Point,
) -> Optional[Point]:
    """Find the first visible asteroid in the given direction, else None."""
    dx, dy = slope

    x, y = start
    while 0 <= x < len(grid) and 0 <= y < len(grid[x]):
        if (x, y) != start and grid[x][y]:
            return (x, y)
        x += dx
        y += dy

def slope_sort_key(slope: Point) -> bool:
    """Sort order: slopes of +\infty down to (but not including) -\infty on the
    right half of the plane, then the same on the left half."""
    dx, dy = slope
    assert dx != 0 or dy != 0

    is_infty: bool = dx == 0
    left_half: bool = dx < 0 or is_infty and dy < 0
    slope: float = dy / dx if not is_infty else 0.0

    # right before left; infty before finite; positive slopes before negative.
    return (left_half, not is_infty, -1 * slope)

if __name__ == "__main__":
    doctest.testmod()
    main()
