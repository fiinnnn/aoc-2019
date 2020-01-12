use crate::solver::Solver;
use crate::intcode_computer::{IntcodeComputer, AsyncIO, read_program};
use std::{
    io::Read,
    sync::mpsc::channel,
    thread,
    iter::from_fn,
    str::FromStr,
    convert::TryFrom,
    fmt::{Display, Formatter, Error}
};
use itertools::Itertools;

pub struct Problem;


impl Solver for Problem {
    type Input = Vec<i64>;
    type Output1 = usize;
    type Output2 = i64;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        read_program(r)
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        // intcode setup
        let mut io = AsyncIO::new();
        let (tx, rx) = channel();
        io.set_receiver(rx);
        let rx = io.get_receiver();
        let mut cpu = IntcodeComputer::new(&mut input.clone(), io);

        let handle = thread::spawn(move || cpu.run());

        let s = String::from_utf8(
            from_fn(|| rx.recv().ok())
                .map(|v| v as u8)
                .collect::<Vec<_>>()
        )
        .unwrap();

        drop(tx);
        let _ = handle.join();

        let grid = Grid::from_str(&s).unwrap();

        println!("{}", grid);

        (1..grid.width - 1)
            .into_iter()
            .cartesian_product((1..grid.height - 1).into_iter())
            .filter(|&pos| grid.is_intersecion(pos))
            .map(|(x, y)| x * y)
            .sum()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        1
    }
}

#[derive(Eq, PartialEq)]
enum Cell {
    Empty,
    Wall,
    Bot(Direction),
}

#[derive(Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl TryFrom<u8> for Cell {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'.' => Ok(Cell::Empty),
            b'#' => Ok(Cell::Wall),
            b'^' => Ok(Cell::Bot(Direction::Up)),
            b'v' => Ok(Cell::Bot(Direction::Down)),
            b'<' => Ok(Cell::Bot(Direction::Left)),
            b'>' => Ok(Cell::Bot(Direction::Right)),
            c => Err(()),
        }
    }
}

struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Grid {
    pub fn get(&self, (x, y): (usize, usize)) -> Option<&Cell> {
        self.cells.get(x + y * self.width)
    }

    pub fn is_intersecion(&self, (x, y): (usize, usize)) -> bool {
        (
            self.get((x, y)),
            self.get((x + 1, y)),
            self.get((x - 1, y)),
            self.get((x, y + 1)),
            self.get((x, y - 1)),
        )
            ==
        (
            Some(&Cell::Wall),
            Some(&Cell::Wall),
            Some(&Cell::Wall),
            Some(&Cell::Wall),
            Some(&Cell::Wall),
        )
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cells = s
            .lines()
            .map(|l| l.bytes().map(Cell::try_from).collect::<Result<Vec<_>, _>>())
            .collect::<Result<Vec<_>, _>>()?;

        let height = cells.len();
        let width = cells.first().unwrap().len();

        Ok(Grid {
            width,
            height,
            cells: cells.into_iter().flatten().collect(),
        })
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        for row in self.cells.chunks(self.width) {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}",
            match self {
                Cell::Empty => '.',
                Cell::Wall => '#',
                Cell::Bot(dir) => {
                    match dir {
                        Direction::Up => '^',
                        Direction::Down => 'v',
                        Direction::Left => '<',
                        Direction::Right => '>',
                    }
                }
            }
        )
    }
}