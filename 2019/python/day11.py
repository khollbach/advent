from cpu.intcode_computer import IntcodeComputer

from collections import defaultdict
from typing import Dict, Tuple, List
import fileinput

def main():
    program = list(map(int, next(iter(fileinput.input())).split(",")))
    run_simulation(program, part2=False)
    run_simulation(program, part2=True)

def run_simulation(program: List[int], part2: bool=False):
    # Infinite grid; initially all black. 0 is black, 1 is white.
    grid: Dict[Tuple[int, int], int] = defaultdict(lambda: 0)

    x, y = 0, 0  # Robot starts at position (0, 0).
    direction = 0  # 0 is up, 1 is right, 2 is down, 3 is left

    # Part2: the robot should start on a white tile.
    if part2:
        grid[(x, y)] = 1

    def get_input() -> int:
        """Read the color of the tile directly below the robot."""
        return grid[(x, y)]

    num_outputs = 0
    def send_output(val: int) -> None:
        """Alternately paint the current tile, or rotate and move one step."""
        nonlocal num_outputs, x, y, direction
        assert val in (0, 1)

        if num_outputs % 2 == 0:
            grid[(x, y)] = val
        else:
            rotation = -1 if val == 0 else +1  # 0 is left, 1 is right
            direction = (direction + rotation) % 4
            if direction == 0:
                y += 1
            elif direction == 1:
                x += 1
            elif direction == 2:
                y -= 1
            else:
                x -= 1

        num_outputs += 1

    IntcodeComputer(program).run(
        get_input_fn=get_input,
        send_output_fn=send_output)

    if not part2:  # Part 1
        print(f"len(grid): {len(grid)}")  # Number of painted tiles
    else:  # Part 2
        print_grid(grid)

def print_grid(grid: Dict[Tuple[int, int], int]) -> None:
    min_x = min(x for x, y in grid)
    max_x = max(x for x, y in grid)
    min_y = min(y for x, y in grid)
    max_y = max(y for x, y in grid)

    for y in reversed(range(min_y, max_y + 1)):  # Rows
        for x in range(min_x, max_x + 1):  # Tiles within a row
            color = grid[(x, y)]
            if color == 1:
                print("#", end="")
            else:
                assert color == 0
                print(".", end="")

        # End of the row
        print()

if __name__ == "__main__":
    main()
