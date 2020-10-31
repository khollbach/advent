use super::memory::Memory;
use super::CPU;
use std::cell::RefCell;
use std::rc::Rc;

/// This is the 'builder' struct for a CPU.
///
/// Supply optional parameters via their corresponding methods. Finish building the CPU with the
/// finish() method.
///
/// ```
/// use cpu::CPUBuilder;
///
/// let result = CPUBuilder::new(vec![1101, 0, 0, 0, 99]).args(12, 2).default_io().run();
/// assert_eq!(result, 14);
/// ```
pub struct CPUBuilder<I, O> {
    mem: Memory,
    args: Option<(i64, i64)>,
    get_input: Option<I>,
    send_output: Option<O>,
}

impl<I, O> CPUBuilder<I, O> {
    /// Start building.
    pub fn new(initial_memory: Vec<i64>) -> Self {
        CPUBuilder {
            mem: Memory::new(initial_memory),
            args: None,
            get_input: None,
            send_output: None,
        }
    }

    /// Set the arguments to execute with. (Optional.)
    /// Arguments and return values are described in Day 2.
    pub fn args(self, noun: i64, verb: i64) -> Self {
        assert!(self.args.is_none());
        Self {
            args: Some((noun, verb)),
            ..self
        }
    }

    /// See `input`.
    pub fn input_or_halt(self, get_input: I) -> Self {
        assert!(self.get_input.is_none());
        Self {
            get_input: Some(get_input),
            ..self
        }
    }

    /// See `output`.
    pub fn output_or_halt(self, send_output: O) -> Self {
        assert!(self.send_output.is_none());
        Self {
            send_output: Some(send_output),
            ..self
        }
    }

    /// Finish building.
    pub fn finish(mut self) -> CPU<I, O> {
        // Write arguments to memory, if given.
        if let Some((noun, verb)) = self.args {
            self.mem.set(1, noun);
            self.mem.set(2, verb);
        }

        CPU {
            mem: self.mem,
            pc: 0,
            relative_base: 0,
            get_input: self.get_input.unwrap(),
            send_output: self.send_output.unwrap(),
        }
    }
}

impl<O> CPUBuilder<(), O> {
    /// Set input function (optional).
    ///
    /// See `CPU.get_input` for details.
    pub fn input(
        self,
        mut get_input: impl FnMut() -> i64,
    ) -> CPUBuilder<impl FnMut() -> Option<i64>, O> {
        assert!(self.get_input.is_none());
        CPUBuilder {
            mem: self.mem,
            args: self.args,
            get_input: Some(move || Some(get_input())),
            send_output: self.send_output,
        }
    }

    /// Use an iterator as the input function.
    ///
    /// Panics at runtime if the iterator is empty and the CPU requests input.
    pub fn input_iter(
        self,
        mut input_iter: impl Iterator<Item = i64>,
    ) -> CPUBuilder<impl FnMut() -> Option<i64>, O> {
        self.input(move || input_iter.next().unwrap())
    }
}

impl<I> CPUBuilder<I, ()> {
    /// Set output function (optional).
    ///
    /// See `CPU.send_output` for details.
    pub fn output(
        self,
        mut send_output: impl FnMut(i64),
    ) -> CPUBuilder<I, impl FnMut(i64) -> Option<()>> {
        assert!(self.send_output.is_none());
        CPUBuilder {
            mem: self.mem,
            args: self.args,
            get_input: self.get_input,
            send_output: Some(move |x| Some(send_output(x))),
        }
    }

    /// Use a vector to collect the output stream.
    ///
    /// Output values are appended to the existing vector.
    pub fn output_vec(
        self,
        output_vec: Rc<RefCell<Vec<i64>>>,
    ) -> CPUBuilder<I, impl FnMut(i64) -> Option<()>> {
        self.output(move |x| output_vec.borrow_mut().push(x))
    }
}

type In = fn() -> Option<i64>;
type Out = fn(i64) -> Option<()>;

impl CPUBuilder<In, Out> {
    /// Supply default input/ouput functions, and finish building.
    pub fn default_io(self) -> CPU<In, Out> {
        self.default_in().default_out().finish()
    }
}

impl<O> CPUBuilder<In, O> {
    /// Supply a default input function, which panics if called.
    pub fn default_in(self) -> Self {
        assert!(self.get_input.is_none());

        fn default_in() -> Option<i64> {
            panic!("No input mechanism specified.")
        }

        Self {
            get_input: Some(default_in),
            ..self
        }
    }
}

impl<I> CPUBuilder<I, Out> {
    /// Supply a default output function, which prints to stdout.
    pub fn default_out(self) -> Self {
        assert!(self.send_output.is_none());

        fn default_out(x: i64) -> Option<()> {
            println!("{}", x);
            Some(())
        }

        Self {
            send_output: Some(default_out),
            ..self
        }
    }
}
