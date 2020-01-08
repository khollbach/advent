from collections import namedtuple
from typing import List, Tuple
import fileinput, re, doctest

Point = namedtuple("Point", "x y z")

def _add_points(self: Point, other: Point) -> Point:
    x1, y1, z1 = self
    x2, y2, z2 = other
    return Point(x1 + x2, y1 + y2, z1 + z2)
Point.__add__ = _add_points

Point.ORIGIN = Point(0, 0, 0)

class Moon:
    def __init__(self, position: Point, velocity: Point=Point.ORIGIN):
        self.pos = position
        self.vel = velocity

    def __hash__(self) -> int:
        return hash((self.pos, self.vel))

    def __eq__(self, other: "Moon") -> bool:
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

def part2(moons: List[Moon]) -> None:
    print("TODO")

def part2_naive(moons: List[Moon]) -> None:
    """Just checking that we do indeed have to think of something clever..."""
    seen = set()

    i = 0
    while True:
        if i % 100000 == 0:  # Can do 2M pretty quick, but it's not enough.
            print(i)

        # Check if this is a repeated state, and otherwise record this state.
        a, b, c, d = moons
        t = (a, b, c, d)
        if t in seen:
            print(i)
            return
        else:
            seen.add(t)

        moons = time_step(moons)
        i += 1

def time_step(moons: List[Moon]) -> List[Moon]:
    """Update velocities and positions according to the problem statement. Do
    not mutate the original list, but rather return an updated copy."""
    new_moons = []
    for m in moons:
        # Copy into a new moon object.
        moon = Moon(m.pos, m.vel)

        # Compute new velocity, based on the old moon objects.
        for other in moons:
            if other is not m:
                moon.vel += Point(sign(other.pos.x - moon.pos.x), 0, 0)
                moon.vel += Point(0, sign(other.pos.y - moon.pos.y), 0)
                moon.vel += Point(0, 0, sign(other.pos.z - moon.pos.z))

        # Compute new position, based on the new velocity.
        moon.pos += moon.vel

        new_moons.append(moon)
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

if __name__ == "__main__":
    doctest.testmod()
    main()
