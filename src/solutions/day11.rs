use crate::solver::Solver;
use crate::intcode_computer::{IntcodeComputer, AsyncIO, read_program};
use std::io::Read;
use std::iter::repeat;
use std::collections::HashSet;
use std::sync::mpsc;
use std::thread;

pub struct Problem;

struct Grid {
    white_panels: HashSet<(isize, isize)>,
    changed_panels: HashSet<(isize, isize)>,
}

impl Grid {
    fn new() -> Self {
        Self{
            white_panels: HashSet::new(),
            changed_panels: HashSet::new(),
        }
    }

    fn display(&self) {
        let mut min_x = 0;
        let mut max_x = 0;
        let mut min_y = 0;
        let mut max_y = 0;

        for &(x, y) in self.white_panels.iter() {
            if x < min_x { min_x = x; }
            else if x > max_x { max_x = x; }
            if y < min_y { min_y = y; }
            else if y > max_y { max_y = y; }
        }

        let xoff = -min_x;
        let yoff = -min_y;
        let width = max_x - min_x + 1;
        let height = max_y - min_y + 1;

        let mut chars: Vec<Vec<char>> = repeat(repeat(' ').take(width as usize).collect())
            .take(height as usize).collect();

        for &(x, y) in self.white_panels.iter() {
            chars[(height - (y + yoff) - 1) as usize][(x + xoff) as usize] = '#';
        }

        for row in chars {
            for c in row {
                print!("{}", c);
            }
            println!();
        }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn get_direction(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

struct Robot {
    pos: (isize, isize),
    dir: Direction,
}

impl Robot {
    fn new() -> Self {
        Self {
            pos: (0, 0),
            dir: Direction::Up,
        }
    }

    fn rotate_left(&mut self) {
        match self.dir {
            Direction::Up => self.dir = Direction::Left,
            Direction::Down => self.dir = Direction::Right,
            Direction::Left => self.dir = Direction::Down,
            Direction::Right => self.dir = Direction::Up,
        }
    }

    fn rotate_right(&mut self) {
        match self.dir {
            Direction::Up => self.dir = Direction::Right,
            Direction::Down => self.dir = Direction::Left,
            Direction::Left => self.dir = Direction::Up,
            Direction::Right => self.dir = Direction::Down,
        }
    }

    fn move_forward(&mut self) {
        let (x, y) = self.pos;
        let (dx, dy) = self.dir.get_direction();
        self.pos = (x + dx, y + dy);
    }
}

impl Solver for Problem {
    type Input = Vec<i64>;
    type Output1 = usize;
    type Output2 = String;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        read_program(r)
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let mut grid = Grid::new();
        paint_grid(&mut grid, &mut input.clone());
        grid.changed_panels.len()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let mut grid = Grid::new();
        grid.white_panels.insert((0,0));
        paint_grid(&mut grid, &mut input.clone());
        grid.display();
        String::from("BCKFPCRA")
    }
}

fn paint_grid(grid: &mut Grid, program: &mut Vec<i64>) {
    let mut io = AsyncIO::new();
    let (tx, rx) = mpsc::channel();
    io.set_receiver(rx);
    let rx = io.get_receiver();

    let mut cpu = IntcodeComputer::new(program, io);

    let handle = thread::spawn(move || cpu.run());

    let mut input = 0;
    if grid.white_panels.contains(&(0,0)) {
        input = 1;
    }
    tx.send(input).unwrap();

    let mut robot = Robot::new();

    while let Ok(mut val) = rx.recv() {
        if val == 0 && grid.white_panels.contains(&robot.pos) {
            grid.white_panels.remove(&robot.pos);
        }
        else if val == 1 {
            grid.white_panels.insert(robot.pos);
            grid.changed_panels.insert(robot.pos);
        }

        val = rx.recv().unwrap();

        if val == 0 {
            robot.rotate_left();
        } else {
            robot.rotate_right();
        }
        robot.move_forward();

        if grid.white_panels.contains(&robot.pos) {
            val = 1
        } else {
            val = 0
        }
        let _ = tx.send(val);
    }

    handle.join().unwrap();
}