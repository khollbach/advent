from collections import namedtuple
from typing import Iterable

import sys, re, time

Point = namedtuple("Point", "x y")
def _add_points(self: "Point", other: "Point") -> "Point":
    return Point(self.x + other.x, self.y + other.y)
Point.__add__ = _add_points

class Star:
    """
    A point in space, and a velocity. The point corresponds to the star's
    location at time t=0. The current position at any time can be computed from
    the starting point and velocity.
    """

    # For parsing input
    _pattern = re.compile(
        "position=< *(-?\d+), *(-?\d+)> velocity=< *(-?\d+), *(-?\d+)>")

    def __init__(self, pos_x: int, pos_y: int, vel_x: int, vel_y: int) -> None:
        self._initial_position = Point(pos_x, pos_y)
        self._velocity = Point(vel_x, vel_y)

    def position(self, time: int) -> int:
        """Get this star's position at the given time."""
        dx, dy = self._velocity
        delta = Point(dx * time, dy * time)
        return self._initial_position + delta

    @classmethod
    def parse(cls, s: str) -> "Star":
        return cls(*map(int, re.match(cls._pattern, s).groups()))

    def __str__(self) -> str:
        return f"initial_position={self._initial_position} velocity={self._velocity}"

class Sky:
    """
    A collection of stars and a current time. The star's positions are always
    dependent on the time.
    """

    def __init__(self, stars: Iterable[Star]) -> None:
        self.stars = [s for s in stars]
        self.time = 0

    def bounding_box(self) -> (int, int, int, int):
        """Return the min_x, min_y, max_x+1, max_y+1 of all the stars."""
        assert self.stars
        min_x, min_y = self.stars[0].position(self.time)
        max_x, max_y = self.stars[0].position(self.time)
        for star in self.stars:
            x, y = star.position(self.time)
            min_x = min(min_x, x)
            min_y = min(min_y, y)
            max_x = max(max_x, x)
            max_y = max(max_y, y)
        return min_x, min_y, max_x + 1, max_y + 1

    def area(self) -> int:
        """Return the area taken up by the bounding box."""
        assert self.stars
        x, y, X, Y = self.bounding_box()
        dx, dy = X - x, Y - y
        return dx * dy

    def print_secret_message(self) -> None:
        """
        Find the time at which the stars are closest together (when their
        bounding box has the smallest area), and print the secret message that
        they form at that time.
        """
        assert self.stars
        original_area = self.area()

        min_area = original_area
        min_time = self.time

        # Find the min bounding-box area.
        # We stop iterating when the stars eventually get far away again.
        area = original_area
        while area < 2 * original_area:
            if area < min_area:
                min_area = area
                min_time = self.time

            self.time += 1
            area = self.area()

        self.time = min_time
        print(self.time)
        print(self)

    def __str__(self) -> str:
        """Return a view of the sky, showing all the stars."""
        min_x, min_y, X, Y = self.bounding_box()
        dx, dy = X - min_x, Y - min_y

        # grid[y][x] is the location (min_x + x, min_y + y)
        grid = [[' '] * dx for _ in range(dy)]

        for star in self.stars:
            x, y = star.position(self.time)
            grid[y - min_y][x - min_x] = '*'

        return '\n'.join(''.join(row) for row in grid)

if __name__ == "__main__":
    stars = map(Star.parse, sys.stdin.readlines())
    sky = Sky(stars)
    sky.print_secret_message()
