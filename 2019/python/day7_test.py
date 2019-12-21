from day7 import IntcodeComputer
import day7, day7_2

import unittest, io, sys

class TestIntcodeComputer(unittest.TestCase):
    MEM_EXAMPLES = [
        # (program, expected-memory-afterwords)
        ([1,0,0,3,99], [1,0,0,2,99]),
        ([1,9,10,3,2,3,11,0,99,30,40,50], [3500,9,10,70,2,3,11,0,99,30,40,50]),
        ([1,0,0,0,99], [2,0,0,0,99]),
        ([2,3,0,3,99], [2,3,0,6,99]),
        ([2,4,4,5,99,0], [2,4,4,5,99,9801]),
        ([1,1,1,4,99,5,6,0,99], [30,1,1,4,2,5,6,0,99]),
        ([1002,4,3,4,33], [1002,4,3,4,99]),
    ]

    IO_EXAMPLES = [
        # Produce same output as input (a.k.a. "echo")
        ([3,0,4,0,99], [
            ([1], [1]),
            ([1337], [1337]),
            ([0], [0]),
            ([-1], [-1]),
            ([-1001], [-1001]),
        ]),
        # Check if input equals 8 (output 1 or 0 accordingly)
        ([3,9,8,9,10,9,4,9,99,-1,8], [
            ([8], [1]),
            ([7], [0]),
            ([0], [0]),
            ([-3], [0]),
            ([100], [0]),
        ]),
        # Same as previous
        ([3,3,1108,-1,8,3,4,3,99], [
            ([8], [1]),
            ([7], [0]),
            ([0], [0]),
            ([-3], [0]),
            ([100], [0]),
        ]),
        # Check if input less than 8 (output 1 or 0 accordingly)
        ([3,9,7,9,10,9,4,9,99,-1,8], [
            ([8], [0]),
            ([7], [1]),
            ([0], [1]),
            ([-3], [1]),
            ([100], [0]),
        ]),
        # Same as previous
        ([3,3,1107,-1,8,3,4,3,99], [
            ([8], [0]),
            ([7], [1]),
            ([0], [1]),
            ([-3], [1]),
            ([100], [0]),
        ]),
        # Check if input is nonzero.
        ([3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], [
            ([8], [1]),
            ([7], [1]),
            ([0], [0]),
            ([-3], [1]),
            ([100], [1]),
        ]),
        # Same as previous
        ([3,3,1105,-1,9,1101,0,0,12,4,12,99,1], [
            ([8], [1]),
            ([7], [1]),
            ([0], [0]),
            ([-3], [1]),
            ([100], [1]),
        ]),
        # Output 999/1000/1001 if input is lt/eq/gt the value 8.
        ([
            3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
            1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
            999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99
        ], [
            ([8], [1000]),
            ([7], [999]),
            ([0], [999]),
            ([-3], [999]),
            ([100], [1001]),
        ]),
    ]

    def test_mem_examples(self):
        for program, expected_mem in self.MEM_EXAMPLES:
            cpu = IntcodeComputer(program)
            cpu.run()
            self.assertEqual(expected_mem, cpu._mem)

    def test_io_examples(self):
        for program, testcases in self.IO_EXAMPLES:
            cpu = IntcodeComputer(program)
            for inputs, expected_outputs in testcases:
                inputs_iter = iter(inputs)
                outputs = []
                def get_input() -> int:
                    return next(inputs_iter)
                def send_output(val: int) -> None:
                    outputs.append(val)
                cpu.run(get_input_fn=get_input, send_output_fn=send_output)
                self.assertEqual(outputs, expected_outputs)

class TestDay7(unittest.TestCase):
    TESTS = [
        # (program, best-settings, expected-signal)
        ([3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0],
            [4,3,2,1,0],
            43210),
        ([3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,
          0,0],
            [0,1,2,3,4],
            54321),
        ([3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,
          31,31,1,32,31,31,4,31,99,0,0,0],
            [1,0,4,3,2],
            65210),
    ]

    def test_all(self):
        for program, best_settings, expected_signal in self.TESTS:
            self.assertEqual(
                day7.get_thruster_signal(program, best_settings),
                expected_signal)
            self.assertEqual(
                day7.best_signal(program),
                expected_signal)

class TestDay7_2(unittest.TestCase):
    TESTS = [
        # (program, best-settings, expected-signal)
        ([3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,
          1005,28,6,99,0,0,5],
            [9,8,7,6,5],
            139629729),
        ([3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,
          -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,
          1001,56,-1,56,1005,56,6,99,0,0,0,0,10],
            [9,7,8,5,6],
            18216),
    ]

    def test_all(self):
        for program, best_settings, expected_signal in self.TESTS:
            self.assertEqual(
                day7_2.get_thruster_signal(program, best_settings),
                expected_signal)
            self.assertEqual(
                day7_2.best_signal(program),
                expected_signal)

if __name__ == "__main__":
    unittest.main()
