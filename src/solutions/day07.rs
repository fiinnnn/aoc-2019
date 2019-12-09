use crate::solver::Solver;
use crate::intcode_computer::{IntcodeComputer, IO, QueueIO, AsyncIO, read_program};
use std::io::Read;
use itertools::Itertools;
use std::thread;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<i64>;
    type Output1 = i64;
    type Output2 = i64;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        read_program(r)
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        (0..5).permutations(5)
            .map(|seq| test_sequence(input, seq))
            .max()
            .expect("Can't find max value")
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        (5..10).permutations(5)
            .map(|seq| test_sequence_2(input, seq))
            .max()
            .expect("Can't find max value")
    }
}

fn test_sequence(program: &Vec<i64>, sequence: Vec<i64>) -> i64 {
    let mut input = 0;
    for n in sequence {
        let mut computer = IntcodeComputer::new(&mut program.clone(), QueueIO::new_init(vec![n, input]));
        computer.run();
        input = computer.io.pop_output();
    }
    input
}

fn test_sequence_2(program: &Vec<i64>, sequence: Vec<i64>) -> i64 {
    let mut io_a = AsyncIO::new_init(vec![0, sequence[0]]);
    let mut io_b = AsyncIO::new_init(vec![sequence[1]]);
    let mut io_c = AsyncIO::new_init(vec![sequence[2]]);
    let mut io_d = AsyncIO::new_init(vec![sequence[3]]);
    let mut io_e = AsyncIO::new_init(vec![sequence[4]]);

    io_a.set_receiver(io_e.get_receiver());
    io_b.set_receiver(io_a.get_receiver());
    io_c.set_receiver(io_b.get_receiver());
    io_d.set_receiver(io_c.get_receiver());
    io_e.set_receiver(io_d.get_receiver());
    let output = io_e.get_receiver();

    let mut cpu_a = IntcodeComputer::new(&mut program.clone(), io_a);
    let mut cpu_b = IntcodeComputer::new(&mut program.clone(), io_b);
    let mut cpu_c = IntcodeComputer::new(&mut program.clone(), io_c);
    let mut cpu_d = IntcodeComputer::new(&mut program.clone(), io_d);
    let mut cpu_e = IntcodeComputer::new(&mut program.clone(), io_e);

    let output_thread = thread::spawn(move || {
        let mut out = 0;
        while let Ok(val) = output.recv() {
            out = val;
        }
        out
    });

    let threads = vec![
        thread::spawn(move || cpu_a.run()),
        thread::spawn(move || cpu_b.run()),
        thread::spawn(move || cpu_c.run()),
        thread::spawn(move || cpu_d.run()),
        thread::spawn(move || cpu_e.run()),
    ];

    for t in threads {
        t.join().unwrap();
    }

    output_thread.join().unwrap()
}