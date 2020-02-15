from collections import namedtuple
from typing import List, Tuple, Set, Any, Callable

import fileinput, re, doctest, math, functools

class Point(namedtuple("Point", "x y z")):
    ORIGIN: "Point"

    def __add__(self, other) -> "Point":
        x1, y1, z1 = self
        x2, y2, z2 = other
        return Point(x1 + x2, y1 + y2, z1 + z2)

Point.ORIGIN = Point(0, 0, 0)

class Moon:
    def __init__(self, position: Point, velocity: Point=Point.ORIGIN) -> None:
        self.pos = position
        self.vel = velocity

    def __hash__(self) -> int:
        return hash((self.pos, self.vel))

    def __eq__(self, other) -> bool:
        return (self.pos, self.vel) == (other.pos, other.vel)

def main():
    # Read the list of initial positions.
    moons = []
    for line in fileinput.input():
        x, y, z = map(int, re.findall(r"-?\d+", line))
        moons.append(Moon(Point(x, y, z)))

    part1(moons)
    part2(moons)

def part1(moons: List[Moon]) -> None:
    for _ in range(1000):
        moons = time_step(moons)

    print(f"total energy: {system_energy(moons)}")

def time_step(moons: List[Moon]) -> List[Moon]:
    """Update velocities and positions according to the problem statement, and
    return as a new list. Leaves the original list unchanged."""
    # Note that the moons update their state atomically together, so we can't
    # modify `moons` in-place.
    new_moons = []

    for moon in moons:
        vel = moon.vel
        for other in moons:
            if other is not moon:
                vel += Point(sign(other.pos.x - moon.pos.x), 0, 0)
                vel += Point(0, sign(other.pos.y - moon.pos.y), 0)
                vel += Point(0, 0, sign(other.pos.z - moon.pos.z))

        pos = moon.pos + vel

        new_moons.append(Moon(pos, vel))

    return new_moons

def sign(x: int) -> int:
    if x < 0:
        return -1
    elif x > 0:
        return 1
    else:
        return 0

def system_energy(moons: List[Moon]) -> int:
    return sum(map(total_energy, moons))

def total_energy(moon: Moon) -> int:
    return energy(moon.pos) * energy(moon.vel)

def energy(p: Point) -> int:
    """
    >>> energy(Point.ORIGIN)
    0
    >>> energy(Point(-5, 40, 0))
    45
    """
    return sum(map(abs, p))

def part2_naive(moons: List[Moon]) -> None:
    """Just checking that we do indeed have to think of something clever..."""
    seen: Set[Tuple[Moon, ...]] = set()

    min_pos = 0
    max_pos = 0
    min_vel = 0
    max_vel = 0

    i = 0
    while True:
        if i % 100000 == 0:  # Can do 2M pretty quick, but it's not enough.
            # These reach -2290 2183 -111 111 within the first 100,000 steps.
            print(i, min_pos, max_pos, min_vel, max_vel)
        # Update mins and maxes.
        for m in moons:
            min_pos = min(min_pos, min(m.pos))
            max_pos = max(max_pos, max(m.pos))
            min_vel = min(min_vel, min(m.vel))
            max_vel = max(max_vel, max(m.vel))

        # Check if this is a repeated state, and otherwise record this state.
        t = tuple(moons)
        if t in seen:
            print(i)
            return
        else:
            seen.add(t)

        moons = time_step(moons)
        i += 1

def part2(moons: List[Moon]) -> None:
    '''
    - find period of x axis:
      - simulate the system until the x pos&vel values repeat;
        return the # of steps taken
    - repeat for y, z axes
    - return the LCM of these three values.
    '''
    x_reps = find_repeated_state(moons, (lambda m: (m.pos.x, m.vel.x)))
    y_reps = find_repeated_state(moons, (lambda m: (m.pos.y, m.vel.y)))
    z_reps = find_repeated_state(moons, (lambda m: (m.pos.z, m.vel.z)))
    print(lcm_many(x_reps, y_reps, z_reps))

def gcd(a: int, b: int) -> int:
    '''
    >>> gcd(15, 10)
    5
    >>> gcd(10, 14)
    2
    '''
    return math.gcd(a, b)

def lcm(a: int, b: int) -> int:
    '''
    >>> lcm(15, 10)
    30
    >>> lcm(10, 14)
    70
    '''
    return abs(a * b) // math.gcd(a, b)

def lcm_many(*args: int) -> int:
    '''
    >>> lcm_many(15, 10, 14)
    210
    >>> lcm_many()
    1
    '''
    return functools.reduce(lcm, args, 1)

def find_repeated_state(
    moons: List[Moon],
    moon_key: Callable[[Moon], Any]
) -> int:
    '''
    Return the number of steps taken to reach a repeat state.

    Whether a state is a repeat is measured according to mapping the moon_key
    over each moon in the system.
    '''
    seen: Set[Tuple[Moon, ...]] = set()

    i = 0
    while True:
        # Check if this is a repeated state, and otherwise record this state.
        t = tuple(map(moon_key, moons))
        if t in seen:
            return i
        else:
            seen.add(t)

        moons = time_step(moons)
        i += 1

if __name__ == "__main__":
    doctest.testmod()
    main()
