from typing import List
import fileinput, math

def main():
    input_str = "".join(fileinput.input()).rstrip("\n")
    pattern = list(map(int, input_str))
    reps = 10000
    print(f"reps: {reps}")
    print(f"len(pattern): {len(pattern)}")  # 650
    print(f"pattern[:7]: {pattern[:7]}")
    offset = int("".join(map(str, pattern[:7])))
    print(f"offset: {offset}")  # ~6,000,000

    # Compute initial suffix.
    suffix_len = reps * len(pattern) - offset
    actual_reps = math.ceil(suffix_len / len(pattern)) + 1  # sloppy upper-bound (hopefully)
    long_pattern = pattern * actual_reps
    skip = offset % len(pattern)
    suffix = long_pattern[skip:]
    suffix = suffix[:suffix_len]

    for _ in range(100):
        suffix_sums(suffix)

    print("".join(map(str, suffix[:8])))

def suffix_sums(l: List[int]) -> None:
    """
    Compute (in-place) all non-empty suffix sums of l, mod 10, in this order:
    map(lambda x: x % 10, [sum(l), sum(l[1:]), ..., sum(l[n-1:])]).
    """
    # Before iteration i of the for-loop, s is the sum of l[i:]
    s = sum(l)
    for i, a in enumerate(l):
        l[i] = s % 10
        s = s - a

if __name__ == "__main__":
    main()
