from cpu.intcode_computer import IntcodeComputer

from typing import List, Tuple, Dict
from enum import Enum

import sys

def main():
    prog = list(map(int, next(sys.stdin).split(',')))
    part1(prog)
    part2(prog)

def part1(program: List[int]) -> None:
    screen = ArcadeScreen()

    cpu = IntcodeComputer(program)
    cpu.run(send_output_fn=screen.get_input)

    screen.draw()

def part2(program: List[int]) -> None:
    screen = ArcadeScreen()

    # Feed in two quarters.
    program[0] = 2
    cpu = IntcodeComputer(program)

    # TODO
    #cpu.run(send_output_fn=screen.get_input, get_input_fn=???)

class Tile(Enum):
    empty = 0
    wall = 1
    block = 2
    paddle = 3  # Horizontal
    ball = 4

    def __str__(self) -> str:
        if self is Tile.empty:
            return ' '
        elif self is Tile.wall:
            return '#'
        elif self is Tile.block:
            return '-'
        elif self is Tile.paddle:
            return '='
        elif self is Tile.ball:
            return '*'
        else:
            assert False

class ArcadeScreen:
    def __init__(self):
        self.inputs = []
        self.screen: Dict[Tuple[int, int], Tile] = {}

    def get_input(self, val: int) -> None:
        self.inputs.append(val)

        if len(self.inputs) >= 3:
            assert len(self.inputs) == 3
            x, y, tile_id = self.inputs
            self.screen[(x, y)] = Tile(tile_id)
            self.inputs = []

    def draw(self) -> None:
        max_x = -1
        max_y = -1
        for x, y in self.screen:
            max_x = max(x, max_x)
            max_y = max(y, max_y)
        assert max_x == 44
        assert max_y == 23

        num_blocks = 0
        for y in range(max_y + 1):
            for x in range(max_x + 1):
                t = self.screen[(x, y)]
                print(t, end='')
                if t is Tile.block:
                    num_blocks += 1
            print()
        print('num blocks:', num_blocks)

if __name__ == "__main__":
    main()
