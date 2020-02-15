from cpu.intcode_computer import IntcodeComputer
from day7 import permutations

from typing import List
from queue import Queue
from threading import Thread
import fileinput, queue

def get_thruster_signal(program: List[int], settings: List[int]) -> int:
    num_amps = len(settings)

    # queues[i] is a message buffer for the inputs to amplifier i, which come
    # from the outputs of amplifier i - 1 (mod num_amps).
    queues: List[Queue] = [Queue() for i in range(num_amps)]
    for i, q in enumerate(queues):
        q.put(settings[i])

    # Initial value: 0 into amp #0.
    queues[0].put(0)

    # Function to be run by each thread (one thread per amp).
    def run_amp(amp_idx: int) -> None:
        i = amp_idx
        n = num_amps
        IntcodeComputer(program).run(
            get_input_fn=(lambda: queues[i].get()),
            send_output_fn=(lambda val: queues[(i + 1) % n].put(val)))

    # Run all the threads concurrently.
    threads = []
    for i in range(num_amps):
        t = Thread(target=run_amp, args=(i,))
        t.start()
        threads.append(t)
    for t in threads:
        t.join()

    # The input to amp #0 is the last amp's final output. Return this.
    final_output = queues[0].get()
    for q in queues:
        assert q.qsize() == 0
    return final_output

def best_signal(program: List[int]) -> int:
    return max(
            get_thruster_signal(program, settings)
            for settings in permutations(list(range(5, 10))))

if __name__ == "__main__":
    program = list(map(int, fileinput.input().readline().split(",")))

    print(best_signal(program))
