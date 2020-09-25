use super::memory::Memory;
use super::CPU;
use std::sync::{Arc, Mutex};

/// This is the 'builder' struct for a CPU, created by CPU::new(). You can supply optional
/// parameters via their corresponding methods, and then finish building and run the CPU with the
/// run() method.
///
/// ```
/// use cpu::CPU;
///
/// let result = CPU::new(vec![1101, 0, 0, 0, 99]).args(12, 2).run();
/// assert_eq!(result, 14);
/// ```
pub struct CPUBuilder {
    mem: Memory,
    args: Option<(i64, i64)>,
    get_input: Option<Box<dyn Send + FnMut() -> Option<i64>>>,
    send_output: Option<Box<dyn Send + FnMut(i64) -> Option<()>>>,
}

impl CPUBuilder {
    /// Create a new CPUBuilder.
    pub fn new(initial_memory: Vec<i64>) -> CPUBuilder {
        CPUBuilder {
            mem: Memory::new(initial_memory),
            args: None,
            get_input: None,
            send_output: None,
        }
    }

    /// Set the arguments to execute with. (Optional!)
    /// Arguments and return values are described in Day 2.
    pub fn args(self, noun: i64, verb: i64) -> Self {
        assert!(self.args.is_none());
        Self {
            args: Some((noun, verb)),
            ..self
        }
    }

    /// See `input`.
    pub fn input_or_halt<F: 'static>(self, get_input: F) -> Self
    where
        F: Send + FnMut() -> Option<i64>,
    {
        assert!(self.get_input.is_none());
        Self {
            get_input: Some(Box::new(get_input)),
            ..self
        }
    }

    /// Set input function (optional).
    ///
    /// See `CPU.get_input` for more.
    pub fn input<F: 'static>(self, mut get_input: F) -> Self
    where
        F: Send + FnMut() -> i64,
    {
        self.input_or_halt(move || Some(get_input()))
    }

    /// Use an iterator as the input function.
    pub fn input_iter<I: 'static>(self, mut input_iter: I) -> Self
    where
        I: Send + Iterator<Item = i64>,
    {
        self.input(move || input_iter.next().unwrap())
    }

    /// See `output`.
    pub fn output_or_halt<F: 'static>(self, send_output: F) -> Self
    where
        F: Send + FnMut(i64) -> Option<()>,
    {
        assert!(self.send_output.is_none());
        Self {
            send_output: Some(Box::new(send_output)),
            ..self
        }
    }

    /// Set ouptut function (optional).
    ///
    /// See `CPU.send_output` for more.
    pub fn output<F: 'static>(self, mut send_output: F) -> Self
    where
        F: Send + FnMut(i64),
    {
        self.output_or_halt(move |x| Some(send_output(x)))
    }

    /// Use a vector to collect the output stream.
    ///
    /// Output values are appended to the existing vector.
    pub fn output_vec(self, output_vec: &Arc<Mutex<Vec<i64>>>) -> Self {
        let clone = Arc::clone(output_vec);
        self.output(move |x| clone.lock().unwrap().push(x))
    }

    /// Finish building; returns a CPU, ready to execute.
    pub fn finish(self) -> CPU {
        let mut mem = self.mem;

        if let Some((noun, verb)) = self.args {
            mem.set(1, noun);
            mem.set(2, verb);
        }

        let get_input = match self.get_input {
            Some(f) => f,
            // Throw at runtime if input is requested, since no closure was given.
            None => Box::new(|| panic!("No input mechanism specified.")),
        };

        let send_output = match self.send_output {
            Some(f) => f,
            // Default to stdout if no closure was given.
            None => Box::new(|x| {
                println!("{}", x);
                Some(())
            }),
        };

        CPU {
            mem,
            pc: 0,
            relative_base: 0,
            get_input,
            send_output,
        }
    }

    /// Convenience method to finish building and run the CPU.
    pub fn run(self) -> i64 {
        let cpu = self.finish();

        cpu.run()
    }

    /// See `run`, and `CPU.run_async`.
    pub fn run_async(self) {
        let cpu = self.finish();

        cpu.run_async()
    }
}
