import tkinter as tk

from collections import namedtuple
from typing import Iterable

import sys, re, time

Point = namedtuple("Point", "x y")
def _add_points(self: "Point", other: "Point") -> "Point":
    return Point(self.x + other.x, self.y + other.y)
Point.__add__ = _add_points

class Star:
    # For parsing input
    _pattern = re.compile(
        "position=< *(-?\d+), *(-?\d+)> velocity=< *(-?\d+), *(-?\d+)>")

    def __init__(self, pos_x: int, pos_y: int, vel_x: int, vel_y: int) -> None:
        self.position = Point(pos_x, pos_y)
        self.velocity = Point(vel_x, vel_y)

    def update_position(self) -> None:
        self.position += self.velocity

    @classmethod
    def parse(cls, s: str) -> "Star":
        return cls(*map(int, re.match(cls._pattern, s).groups()))

    def __str__(self) -> str:
        return f"position={self.position} velocity={self.velocity}"

class VisibleStar(Star):
    def __init__(self, star: Star) -> None:
        super().__init__(*star.position, *star.velocity)

        # todo: store tk handle info or w/e to move the star around
        #self.

class Sky:
    def __init__(self, stars: Iterable[Star]) -> None:
        self.stars = [s for s in stars]
        self.time = 0

    def run(self) -> None:
        while True:
            self.draw()
            self.update_positions()
            time.sleep(1)

    def draw(self) -> None:
        print(f"time: {self.time}  stars[0]: {self.stars[0]}")

    def update_positions(self) -> None:
        for star in self.stars:
            star.update_position()
        self.time += 1

class App(tk.Frame):
    WIDTH = 1000
    HEIGHT = 1000

    def __init__(self, master: tk.Tk, stars: Iterable[Star]) -> None:
        super().__init__(master)
        self.pack()
        self.create_canvas()
        self.create_sky(stars)

        self.after(0, self.run)

    def create_canvas(self) -> None:
        self.canvas = tk.Canvas(self, width=self.WIDTH, height=self.HEIGHT)
        self.canvas.pack()

    def run(self) -> None:
        self.draw()
        self.sky.update_positions()
        self.after(1000, self.run)

    def draw(self) -> None:
        self.canvas.create_rectangle(
                0, 0, self.WIDTH, self.HEIGHT, fill="light blue")
        for star in self.sky.stars:
            ((x, y), (dx, dy)) = star.position, star.velocity
            self.canvas.create_oval(x-1, y-1, x+1, y+1, fill="yellow")
        self.canvas.create_oval(100, 100, 200, 200, fill="yellow")

# TODO
'''
Left off trying to figure out this whole tk thingy.

I need to keep handles to the ovals so I can move them.
I think I should probably subclass Star with "VisibleStar"/etc which also
moves the sprite when updated.

This is starting to feel big enough to put into a folder.
I'll create ./10/ and make 10.py be a wrapper which just calls ./10/10.py:main


I guess it's really bugging me that the "business logic" of stars and skies is
at risk of getting tangled up with the display logic. This is probably a case
of me overthinking/overengineering things, as usual.


META: I think my process is way too cautious. I can refactor later! Just move!
'''

if __name__ == "__main__":
    stars = map(Star.parse, sys.stdin.readlines())

    root = tk.Tk()
    app = App(root, stars)
    app.mainloop()
