from typing import List, Iterable

import sys

def module_fuel_required(module_mass: int) -> int:
    fuel = module_mass // 3 - 2
    if fuel <= 0:
        return 0
    else:
        return fuel + module_fuel_required(fuel)

def total_fuel_required(module_masses: Iterable[int]) -> int:
    return sum(module_fuel_required(mass) for mass in module_masses)

if __name__ == "__main__":
    print(total_fuel_required(map(int, sys.stdin.readlines())))
