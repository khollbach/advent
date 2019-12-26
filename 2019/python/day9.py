from cpu.intcode_computer import IntcodeComputer

import fileinput

def main():
    program = list(map(int, next(fileinput.input()).split(",")))

    inputs = iter([1])
    IntcodeComputer(program).run(get_input_fn=lambda: next(inputs))

    inputs = iter([2])
    IntcodeComputer(program).run(get_input_fn=lambda: next(inputs))

if __name__ == "__main__":
    main()
