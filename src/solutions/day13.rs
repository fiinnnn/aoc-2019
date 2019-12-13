use crate::solver::Solver;
use crate::intcode_computer::{IntcodeComputer, AsyncIO, read_program};
use std::{
    io::Read,
    sync::mpsc::{channel, Receiver},
    thread,
    collections::HashMap,
    error::Error
};
use std::cmp::Ordering;

pub struct Problem;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl Tile {
    fn to_char(&self) -> char {
        match self {
            Tile::Empty => ' ',
            Tile::Wall => '█',
            Tile::Block => '▭',
            Tile::Paddle => '▂',
            Tile::Ball => '●',
        }
    }
}

impl From<i64> for Tile {
    fn from(n: i64) -> Self {
        match n {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            4 => Tile::Ball,
            _ => panic!("Unknown tile"),
        }
    }
}

impl Solver for Problem {
    type Input = Vec<i64>;
    type Output1 = usize;
    type Output2 = i64;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        read_program(r)
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let mut tiles: HashMap<(i64, i64), Tile> = HashMap::new();

        let mut io = AsyncIO::new();
        let rx = io.get_receiver();
        let mut cpu = IntcodeComputer::new(&mut input.clone(), io);

        let handle = thread::spawn(move || { cpu.run() });

        while let Ok((x, y, tile)) = recv_tile(&rx) {
            tiles.insert((x, y), tile.into());
        }

        let _ = handle.join();

        tiles.values().filter(|&t| t.eq(&Tile::Block)).count()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let mut io = AsyncIO::new();
        let (tx, rx) = channel();
        io.set_receiver(rx);
        let rx = io.get_receiver();
        let mut cpu = IntcodeComputer::new(&mut input.clone(), io);
        cpu.write(0, 2);

        let handle = thread::spawn(move || { cpu.run() });

        let mut paddle_x = 0;
        let mut score = 0;

        while let Ok((x, y, tile)) = recv_tile(&rx) {
            if x == -1 {
                score = tile;
                continue;
            }

            let tile: Tile = tile.into();

            if tile == Tile::Paddle {
                paddle_x = x;
            }
            else if tile == Tile::Ball {
                let dir = match paddle_x.cmp(&x) {
                    Ordering::Less => 1,
                    Ordering::Equal => 0,
                    Ordering::Greater => -1,
                };

                tx.send(dir).unwrap();
            }
        }

        let _ = handle.join();

        score
    }
}

fn recv_tile(rx: &Receiver<i64>) -> Result<(i64, i64, i64), Box<dyn Error>> {
    Ok((rx.recv()?, rx.recv()?, rx.recv()?))
}