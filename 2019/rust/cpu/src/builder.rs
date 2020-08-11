use super::CPU;
use std::cell::RefCell;
use std::rc::Rc;

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
    mem: Vec<i32>,
    args: Option<(i32, i32)>,
    get_input: Option<Box<dyn FnMut() -> i32>>,
    send_output: Option<Box<dyn FnMut(i32)>>,
}

impl CPUBuilder {
    /// Create a new CPUBuilder. `memory` must be non-empty.
    pub fn new(memory: Vec<i32>) -> CPUBuilder {
        assert!(!memory.is_empty());
        CPUBuilder {
            mem: memory,
            args: None,
            get_input: None,
            send_output: None,
        }
    }

    /// Set the arguments to execute with. (Optional!)
    /// Arguments and return values are described in Day 2.
    pub fn args(self, noun: i32, verb: i32) -> Self {
        assert!(self.args.is_none());
        Self {
            args: Some((noun, verb)),
            ..self
        }
    }

    /// Set input function. (Optional.) I/O is described in Day 5.
    pub fn input_fn<F: 'static>(self, get_input: F) -> Self
    where
        F: FnMut() -> i32,
    {
        assert!(self.get_input.is_none());
        Self {
            get_input: Some(Box::new(get_input)),
            ..self
        }
    }

    /// Use an iterator as the input function.
    pub fn input_iter<I: 'static>(self, mut input_iter: I) -> Self
    where
        I: Iterator<Item = i32>,
    {
        self.input_fn(move || input_iter.next().unwrap())
    }

    /// Set output function. (Optional.) I/O is described in Day 5.
    pub fn output_fn<F: 'static>(self, send_output: F) -> Self
    where
        F: FnMut(i32),
    {
        assert!(self.send_output.is_none());
        Self {
            send_output: Some(Box::new(send_output)),
            ..self
        }
    }

    /// Use a vector as the output stream. Output values will be appended to the existing vector.
    pub fn output_vec(self, output_vec: &Rc<RefCell<Vec<i32>>>) -> Self {
        let clone = Rc::clone(output_vec);
        self.output_fn(move |x| clone.borrow_mut().push(x))
    }

    /// Finish building; returns a CPU, ready to execute.
    pub fn finish(self) -> CPU {
        let mut mem = self.mem;

        if let Some((noun, verb)) = self.args {
            assert!(mem.len() >= 3);
            mem[1] = noun;
            mem[2] = verb;
        }

        let get_input = match self.get_input {
            Some(f) => f,
            // Throw at runtime if input is requested, but no closure was given.
            None => Box::new(|| panic!("No input mechanism specified.")),
        };

        let send_output = match self.send_output {
            Some(f) => f,
            // Default to stdout if no closure was given.
            None => Box::new(|x| println!("{}", x)),
        };

        CPU {
            mem,
            pc: 0,
            get_input,
            send_output,
        }
    }

    /// Finish building and execute the program.
    pub fn run(self) -> i32 {
        let cpu = self.finish();

        cpu.run()
    }
}
