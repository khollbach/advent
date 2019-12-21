from day7 import IntcodeComputer, permutations

from typing import List
import fileinput

def get_thruster_signal(program: List[int], settings: List[int]) -> int:
    num_amps = len(settings)

    # todo: Init a number of queue-like objects to act as message buffers.
    # Add a copy of `settings[i]` to each queue, 0 <= i < num_amps.
    # Then append a copy of the value 0 to the 0th queue.
    #?

    for i in range(num_amps):
        # todo: Spawn a thread to do the following:
        IntcodeComputer(program).run(
                            # todo: make this popleft block waiting
            get_input_fn=(lambda: queues[i].popleft()),
            send_output_fn=(lambda val: queues[(i + 1) % n].append(val)))

    # todo: Join all the threads
    # dequeue from the last queue
    # assert that all queues are empty

    return the_dequeued_value

def best_signal(program: List[int]) -> int:
    return max(
            get_thruster_signal(program, settings)
            for settings in permutations(list(range(5, 10))))

if __name__ == "__main__":
    program = list(map(int, fileinput.input().readline().split(",")))

    print(best_signal(program))
