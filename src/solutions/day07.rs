use crate::solver::Solver;
use crate::intcode_computer::{IntcodeComputer, AsyncIO, Pipe, read_program};
use std::{
    io::Read,
    thread,
    iter::from_fn,
    sync::mpsc::channel
};
use itertools::Itertools;

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
        let (io, tx, rx) = AsyncIO::new();
        let _ = tx.send(n);
        let _ = tx.send(input);

        let mut computer = IntcodeComputer::new(&mut program.clone(), io);
        computer.run();
        drop(computer);

        input = from_fn(|| rx.recv().ok()).last().unwrap();
    }
    input
}

fn test_sequence_2(program: &Vec<i64>, sequence: Vec<i64>) -> i64 {
    // setup io
    let (io_a, tx_a, rx_a) = AsyncIO::new();
    let (io_b, tx_b, rx_b) = AsyncIO::new();
    let (io_c, tx_c, rx_c) = AsyncIO::new();
    let (io_d, tx_d, rx_d) = AsyncIO::new();
    let (io_e, tx_e, rx_e) = AsyncIO::new();
    let (tx_out, rx_out) = channel();

    // send sequence
    let _ = tx_a.send(sequence[0]);
    let _ = tx_b.send(sequence[1]);
    let _ = tx_c.send(sequence[2]);
    let _ = tx_d.send(sequence[3]);
    let _ = tx_e.send(sequence[4]);

    // send initial value for a
    let _ = tx_a.send(0);

    // setup connections
    let pipe_ab = Pipe::new(rx_a, vec![tx_b]);
    let pipe_bc = Pipe::new(rx_b, vec![tx_c]);
    let pipe_cd = Pipe::new(rx_c, vec![tx_d]);
    let pipe_de = Pipe::new(rx_d, vec![tx_e]);
    let pipe_eao = Pipe::new(rx_e, vec![tx_a, tx_out]);

    // setup computers
    let mut cpu_a = IntcodeComputer::new(&mut program.clone(), io_a);
    let mut cpu_b = IntcodeComputer::new(&mut program.clone(), io_b);
    let mut cpu_c = IntcodeComputer::new(&mut program.clone(), io_c);
    let mut cpu_d = IntcodeComputer::new(&mut program.clone(), io_d);
    let mut cpu_e = IntcodeComputer::new(&mut program.clone(), io_e);

    // start output thread
    let output_thread = thread::spawn(move || {
        from_fn(|| rx_out.recv().ok()).last().unwrap()
    });

    // start intcode cpu threads
    let threads = vec![
        thread::spawn(move || cpu_a.run()),
        thread::spawn(move || cpu_b.run()),
        thread::spawn(move || cpu_c.run()),
        thread::spawn(move || cpu_d.run()),
        thread::spawn(move || cpu_e.run()),
        thread::spawn(move || pipe_ab.run()),
        thread::spawn(move || pipe_bc.run()),
        thread::spawn(move || pipe_cd.run()),
        thread::spawn(move || pipe_de.run()),
        thread::spawn(move || pipe_eao.run()),
    ];

    for t in threads {
        t.join().unwrap();
    }

    output_thread.join().unwrap()
}