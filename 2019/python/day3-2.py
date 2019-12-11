from io import TextIOBase
from typing import List, Tuple, Generator, Set, Dict
from collections import defaultdict, namedtuple
from itertools import islice

import sys, re, doctest

Point = namedtuple("Point", "x y")
def _add_points(self: Point, other: Point) -> Point:
    return Point(self.x + other.x, self.y + other.y)
Point.__add__ = _add_points
ORIGIN = Point(0, 0)

Wire = List[Point]  # Ordered list of deltas from the origin.

def read_wire(input_stream: TextIOBase) -> Wire:
    """Read and parse a wire, which is described by a list of directions."""
    line = input_stream.readline()
    directions = re.findall(r"([UDLR]\d+),?", line)
    def direction_to_delta(direction: str) -> Point:
        d, l = direction[0], int(direction[1:])
        if d == "U":
            return Point(0, l)
        elif d == "D":
            return Point(0, -l)
        elif d == "L":
            return Point(-l, 0)
        else:
            assert d == "R"
            return Point(l, 0)
    deltas = list(map(direction_to_delta, directions))
    return deltas

def wire_to_points(wire: Wire) -> Dict[Point, int]:
    """
    Return the points this wire passes through, *EXCEPT* the origin.
    Those points are the keys of a dictionary whose values are the distance the
    wire travelled to get to that point.
    """
    points = {}
    distance_travelled = 0
    pos = ORIGIN
    for delta in wire:
        new_pos = pos + delta
        for i, p in enumerate(line_segment(pos, new_pos)):
            if i == 0:  # Skip `pos` itself.
                continue
            distance_travelled += 1
            if p not in points:
                points[p] = distance_travelled
        pos = new_pos
    return points

def intersections(wire1: Wire, wire2: Wire) -> Dict[Point, int]:
    """
    Return the intersections of the two wires, excluding the origin.
    Each intersection point is the key of a dictionary, where the value is the
    sum of the two distances travelled by the two wires to get to that point.
    """
    points1 = wire_to_points(wire1)
    points2 = wire_to_points(wire2)
    result = {}
    for p in points1.keys() & points2.keys():  # Set-intersection.
        result[p] = points1[p] + points2[p]
    return result

def closest_intersection(wire1: Wire, wire2: Wire) -> Tuple[Point, int]:
    """
    Return the "closest" intersection point (as defined in the problem),
    together with its combined distance-travelled along the two wires.
    """
    def compare_by(key_val_pair: Tuple[Point, int]) -> int:
        point, dist = key_val_pair
        return dist

    # Return minimum distance point, together with that distance.
    return min(intersections(wire1, wire2).items(), key=compare_by)

def line_segment(p1: Point, p2: Point) -> Generator[Point, None, None]:
    """
    Generate the sequence of points between p1 and p2, inclusive.
    The two points must lie on a horizontal or vertical line.

    >>> [(x, y) for x, y in line_segment(ORIGIN, Point(5, 0))]
    [(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0)]
    >>> [(x, y) for x, y in line_segment(ORIGIN, Point(-5, 0))]
    [(0, 0), (-1, 0), (-2, 0), (-3, 0), (-4, 0), (-5, 0)]
    >>> [(x, y) for x, y in line_segment(ORIGIN, Point(0, 1))]
    [(0, 0), (0, 1)]
    >>> [(x, y) for x, y in line_segment(ORIGIN, Point(0, -1))]
    [(0, 0), (0, -1)]
    """
    dx = p2.x - p1.x
    dy = p2.y - p1.y
    assert not (dx != 0 and dy != 0)

    if dx != 0:
        step = 1 if dx > 0 else -1
        for x in range(p1.x, p2.x + step, step):  # Inclusive
            yield Point(x, p1.y)
    elif dy != 0:
        step = 1 if dy > 0 else -1
        for y in range(p1.y, p2.y + step, step):  # Inclusive
            yield Point(p1.x, y)
    else:
        yield p1

def distance(p1: Point, p2: Point = ORIGIN) -> int:
    """
    Return the manhattan distance between two points.
    If just given one point, return the distance from the origin.

    >>> distance(ORIGIN, Point(5, 2))
    7
    >>> distance(Point(5, 2))
    7
    """
    return abs(p1.x - p2.x) + abs(p1.y - p2.y)

if __name__ == "__main__":
    doctest.testmod()

    wire1 = read_wire(sys.stdin)
    wire2 = read_wire(sys.stdin)

    print(closest_intersection(wire1, wire2))
