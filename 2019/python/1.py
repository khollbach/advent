from typing import List, Iterable

import sys

def module_fuel_required(module_mass: int) -> int:
    fuel_req = module_mass // 3 - 2
    assert fuel_req >= 0
    return fuel_req

def total_fuel_required(module_masses: Iterable[int]) -> int:
    return sum(module_fuel_required(mass) for mass in module_masses)

if __name__ == "__main__":
    print(total_fuel_required(map(int, sys.stdin.readlines())))
