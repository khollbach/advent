from enum import Enum
from typing import List, Dict, Tuple, Set
import doctest, unittest, sys

input_str = """\
         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z       
"""

input_str2 = """\
                   A               
                   A               
  #################.#############  
  #.#...#...................#.#.#  
  #.#.#.###.###.###.#########.#.#  
  #.#.#.......#...#.....#.#.#...#  
  #.#########.###.#####.#.#.###.#  
  #.............#.#.....#.......#  
  ###.###########.###.#####.#.#.#  
  #.....#        A   C    #.#.#.#  
  #######        S   P    #####.#  
  #.#...#                 #......VT
  #.#.#.#                 #.#####  
  #...#.#               YN....#.#  
  #.###.#                 #####.#  
DI....#.#                 #.....#  
  #####.#                 #.###.#  
ZZ......#               QG....#..AS
  ###.###                 #######  
JO..#.#.#                 #.....#  
  #.#.#.#                 ###.#.#  
  #...#..DI             BU....#..LF
  #####.#                 #.#####  
YN......#               VT..#....QG
  #.###.#                 #.###.#  
  #.#...#                 #.....#  
  ###.###    J L     J    #.#.###  
  #.....#    O F     P    #.#...#  
  #.###.#####.#.#####.#####.###.#  
  #...#.#.#...#.....#.....#.#...#  
  #.#####.###.###.#.#.#########.#  
  #...#.#.....#...#.#.#.#.....#.#  
  #.###.#####.###.###.#.#.#######  
  #.#.........#...#.............#  
  #########.###.###.#############  
           B   J   C               
           U   P   P               
"""

class Tile(Enum):
    EMPTY = 0
    WALL = 1
    FLOOR = 2
    #PORTAL = 3

    def __str__(self) -> str:
        if self is Tile.EMPTY:
            return " "
        elif self is Tile.WALL:
            return "#"
        elif self is Tile.FLOOR:
            return "."
        #elif self is Tile.PORTAL:
        #    return "O"
        else:
            assert False

Point = Tuple[int, int]

def main():
    #input_ = input_str
    #input_ = input_str2
    input_ = sys.stdin.read()
    grid, portals, start, end = read_grid(input_)

    # Sanity checks
    #print_grid(grid, portals)
    #print(portals)
    print(f"len(portals): {len(portals)}")
    print(f"start: {start}, end: {end}")

    dist = shortest_path_length(grid, portals, start, end)
    print(f"shortest path length: {dist}")

    doctest.testmod()
    unittest.main()

class TestMain(unittest.TestCase):
    def test_process_label(self):
        lines = input_str.split("\n")
        self.assertEqual(process_label(lines, 0, 9), ("AA", 2, 9))
        self.assertEqual(process_label(lines, 1, 9), ("AA", 2, 9))

def shortest_path_length(
    grid: List[List[Tile]],
    portals: Dict[Point, Point],
    start: Point,
    end: Point,
) -> int:
    queue: List[Tuple[Point, int]] = [(start, 0)]
    visited: Set[Point] = set()

    while queue:
        current, dist = queue.pop(0)
        if current in visited:
            continue
        visited.add(current)

        if current == end:
            return dist

        if current in portals:
            queue.append((portals[current], dist + 1))

        i, j = current
        if grid[i - 1][j] is Tile.FLOOR:
            queue.append(((i - 1, j), dist + 1))
        if grid[i + 1][j] is Tile.FLOOR:
            queue.append(((i + 1, j), dist + 1))
        if grid[i][j - 1] is Tile.FLOOR:
            queue.append(((i, j - 1), dist + 1))
        if grid[i][j + 1] is Tile.FLOOR:
            queue.append(((i, j + 1), dist + 1))

    assert False

def read_grid(
    input_str: str
) -> Tuple[
    List[List[Tile]],  # grid
    Dict[Point, Point],  # portals
    Point,  # start
    Point,  # end
]:
    lines = input_str.split("\n")

    grid = []
    labels: Dict[str, Point] = {}  # E.g., {"AB": (x1,y1)}
    portals: Dict[Point, Point] = {}  # E.g., {(x1,y1): (x2,y2), (x2,y2): (x1,y1)}

    for i, line in enumerate(lines):
        row = []
        for j, char in enumerate(line):
            if char == "#":
                row.append(Tile.WALL)
            elif char == ".":
                row.append(Tile.FLOOR)
            elif char.isalpha():
                label, x, y = process_label(lines, i, j)
                if label == "AA":
                    start = (x, y)
                elif label == "ZZ":
                    end = (x, y)
                elif label in labels and (x, y) not in portals:
                    x2, y2 = labels[label]
                    portals[(x, y)] = (x2, y2)
                    portals[(x2, y2)] = (x, y)
                else:
                    labels[label] = (x, y)
                row.append(Tile.EMPTY)
            else:
                row.append(Tile.EMPTY)
        grid.append(row)

    #for i, j in portals:
    #    grid[i][j] = Tile.PORTAL

    return grid, portals, start, end

def process_label(
    lines: List[str], i: int, j: int
) -> Tuple[
    str,  # label
    int,  # portal x
    int,  # portal y
]:
    r"""
    Example:
      A<-- this one is (i, j)
      A
    ##.<-- the portal goes here (x, y)
    ##....
    ##.### etc...

    >>> lines = input_str.split("\n")
    >>> process_label(lines, 0, 9)
    ('AA', 2, 9)
    >>> process_label(lines, 1, 9)
    ('AA', 2, 9)
    """
    if i > 0 and lines[i - 1][j].isalpha():
        label = lines[i - 1][j] + lines[i][j]
        if i - 1 > 0 and lines[i - 2][j] == ".":  # .BA
            x = i - 2
        else:  # A is current location: BA.
            assert lines[i + 1][j] == "."
            x = i + 1
        y = j
    elif i < len(lines) - 1 and lines[i + 1][j].isalpha():
        label = lines[i][j] + lines[i + 1][j]
        if i > 0 and lines[i - 1][j] == ".":
            x = i - 1
        else:
            assert lines[i + 2][j] == "."
            x = i + 2
        y = j
    elif j > 0 and lines[i][j - 1].isalpha():
        label = lines[i][j - 1] + lines[i][j]
        if j - 1 > 0 and lines[i][j - 2] == ".":
            y = j - 2
        else:
            assert lines[i][j + 1] == "."
            y = j + 1
        x = i
    elif j < len(lines[i]) - 1 and lines[i][j + 1].isalpha():
        label = lines[i][j] + lines[i][j + 1]
        if j > 0 and lines[i][j - 1] == ".":
            y = j - 1
        else:
            assert lines[i][j + 2] == "."
            y = j + 2
        x = i
    else:
        assert False

    return label, x, y

def print_grid(grid: List[List[Tile]], portals: Dict[Point, Point]):
    for i, row in enumerate(grid):
        for j, tile in enumerate(row):
            if (i, j) in portals:
                print("O", end="")
            else:
                print(tile, end="")
        print()

if __name__ == "__main__":
    main()
    doctest.testmod()

"""
[ctrl-C]
:imap jk <Esc>
"""
