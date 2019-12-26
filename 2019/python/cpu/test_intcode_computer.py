from intcode_computer import IntcodeComputer

import unittest

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

    OUTPUT_EXAMPLES = [
        ([109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99],
         [109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99]),
        ([1102,34915192,34915192,7,4,7,99,0],
         [1219070632396864]),
        ([104,1125899906842624,99],
         [1125899906842624]),
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
                self.assertEqual(expected_outputs, outputs)

    def test_output_examples(self):
        for program, expected_outputs in self.OUTPUT_EXAMPLES:
            cpu = IntcodeComputer(program)
            outputs = []
            def get_input() -> int:
                assert False
            def send_output(val: int) -> None:
                outputs.append(val)
            cpu.run(get_input_fn=get_input, send_output_fn=send_output)
            self.assertEqual(expected_outputs, outputs)

if __name__ == "__main__":
    unittest.main()
