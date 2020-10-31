use crate::{Dir, Point, Tile, ORIGIN};
use cpu::CPUBuilder;
use std::sync::mpsc::{self, Receiver, RecvError, SendError, Sender};

/// The robot.
pub struct Robot {
    /// Where is the robot currently?
    pub pos: Point,

    /// What is the robot currently "standing on"?
    pub tile: Tile,

    /// Movement instructions and their sensor responses are handled via channels. The robot sends
    /// instructions to its CPU, which executes them and responds with the sensor's reading.
    /// These channels remain open until the robot is dropped.
    instructions: Sender<Dir>,
    sensor: Receiver<Tile>,
}

impl Robot {
    /// Create a new Robot. Spawns a new thread running the robot's cpu.
    pub fn new(mem: Vec<i64>) -> Self {
        let (instructions, instr_rx) = mpsc::channel();
        let (sensor_sx, sensor) = mpsc::channel();

        Self::spawn_cpu(mem, instr_rx, sensor_sx);

        Self {
            pos: ORIGIN,
            tile: Tile::Floor,
            instructions,
            sensor,
        }
    }

    /// Start the CPU. The spawned thread is cleaned up when the IO channels are closed, which
    /// happens automatically when the Robot is dropped.
    fn spawn_cpu(mem: Vec<i64>, instr_rx: Receiver<Dir>, sensor_sx: Sender<Tile>) {
        CPUBuilder::new(mem)
            .input_or_halt(move || {
                use Dir::*;
                match instr_rx.recv() {
                    Ok(d) => Some(match d {
                        // Note the unconventional encoding.
                        N => 1,
                        S => 2,
                        W => 3,
                        E => 4,
                    }),
                    // Halt the CPU.
                    Err(RecvError) => None,
                }
            })
            .output_or_halt(move |x| {
                use Tile::*;
                let tile = match x {
                    0 => Wall,
                    1 => Floor,
                    2 => Target,
                    _ => panic!("Invalid sensor output: {}", x),
                };
                match sensor_sx.send(tile) {
                    Ok(()) => Some(()),
                    // Halt the CPU.
                    Err(SendError(_)) => None,
                }
            })
            .finish()
            .run_async();
    }

    /// Try to move onto an adjacent tile. Return true if successful.
    ///
    /// Only fails if the destination tile is a wall, in which case the robot does not move.
    pub fn move_(&mut self, d: Dir) -> bool {
        self.instructions.send(d).unwrap();
        match self.sensor.recv().unwrap() {
            Tile::Wall => false,
            tile => {
                self.pos = Dir::step(self.pos, d);
                self.tile = tile;
                true
            }
        }
    }
}
