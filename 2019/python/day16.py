from typing import List, Iterator
import fileinput

class Repeater(Iterator):
    """Iterator that outputs each list element n times consecutively, and then
    rinse-and-repeats *FOREVER*."""
    def __init__(self, n: int, l: List[int]):
        assert n > 0
        assert l
        self.n = n
        self.l = l
        self.i = 0  # index into l
        self.reps = 0  # counts num reps done at the current index

    def __next__(self):
        val = self.l[self.i]

        self.reps += 1

        if self.reps == self.n:  # Reset reps and increment i
            self.reps = 0
            self.i += 1

            if self.i == len(self.l):  # Reset i
                self.i = 0

        return val

def main():
    input_str = "".join(fileinput.input()).rstrip("\n")
    nums = list(map(int, input_str))
    print(f"len(nums): {len(nums)}")
    print(f"nums[:8]: {nums[:8]}")

    for _ in range(100):
        nums = fft_phase(nums)

    print("".join(map(str, nums[:8])))

_base_pattern = [0, 1, 0, -1]
def fft_phase(l: List[int]) -> List[int]:
    new_l = []

    for i in range(len(l)):
        pattern = Repeater(i + 1, _base_pattern)
        next(pattern)

        val = sum(a * b for a, b in zip(l, pattern))
        val = abs(val) % 10
        new_l.append(val)

    return new_l

if __name__ == "__main__":
    main()
