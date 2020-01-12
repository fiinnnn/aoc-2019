use crate::solver::Solver;
use crate::intcode_computer::{IntcodeComputer, AsyncIO, read_program};
use std::{io::Read, collections::HashSet, iter::repeat, sync::mpsc::channel, thread};
use std::time::Duration;

pub struct Problem;

impl Solver for Problem {
    type Input = Grid;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        let mut program = read_program(r);

        // intcode setup
        let mut io = AsyncIO::new();
        let (tx, rx) = channel();
        io.set_receiver(rx);
        let rx = io.get_receiver();
        let mut cpu = IntcodeComputer::new(&mut program, io);

        // start intcode computer
        let handle = thread::spawn(move || cpu.run());

        let mut grid = Grid::new();

        let mut pos = (0, 0);
        grid.set_visited(pos);
        let mut path = vec![];

        let mut dir = grid.get_possible_direction(pos).unwrap();

        tx.send(dir.into()).unwrap();

        while let Ok(reply) = rx.recv() {
            match reply {
                0 => grid.hit_wall(pos, dir),
                1 => {
                    pos = dir.move_from_pos(pos);
                    if !grid.visited(&pos) {
                        path.push(dir);
                    }
                    grid.set_visited(pos);
                },
                2 => {
                    pos = dir.move_from_pos(pos);
                    if !grid.visited(&pos) {
                        path.push(dir);
                    }
                    grid.set_visited(pos);
                    grid.oxygen_system = Some(pos);
                    grid.path_length = path.len() as u64;
                },
                _ => panic!("Invalid reply: {}", reply),
            }

            if let Some(next_dir) = grid.get_possible_direction(pos) {
                dir = next_dir;
                tx.send(dir.into()).unwrap();
            }
            else if let Some(next_dir) = path.pop() {
                dir = next_dir.get_opposite();
                tx.send(dir.into()).unwrap();
            }
            else {
                break;
            }

            grid.display(Some(pos));
            println!("Path length: {}", path.len());
            thread::sleep(Duration::from_millis(40));
        }

        drop(tx);
        let _ = handle.join();

        grid.oxygen.insert(grid.oxygen_system.unwrap());

        grid
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        input.path_length
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let mut minutes = 0;
        let mut grid = input.clone();

        while grid.spread_oxygen() {
            minutes += 1;
            grid.display(None);
            println!("Minutes: {}", minutes);
            thread::sleep(Duration::from_millis(40));
        }

        minutes
    }
}

#[derive(Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn move_from_pos(&self, (x, y): (isize, isize)) -> (isize, isize) {
        match self {
            Direction::North => (x, y + 1),
            Direction::South => (x, y - 1),
            Direction::West  => (x - 1, y),
            Direction::East  => (x + 1, y),
        }
    }

    fn get_opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::West  => Direction::East,
            Direction::East  => Direction::West,
        }
    }
}

impl Into<i64> for Direction {
    fn into(self) -> i64 {
        match self {
            Direction::North => 1,
            Direction::South => 2,
            Direction::West  => 3,
            Direction::East  => 4,
        }
    }
}


#[derive(Clone)]
pub struct Grid {
    walls: HashSet<(isize, isize)>,
    visited: HashSet<(isize, isize)>,
    oxygen: HashSet<(isize, isize)>,
    oxygen_system: Option<(isize, isize)>,
    path_length: u64,
}

impl Grid {
    fn new() -> Self {
        Grid {
            walls: HashSet::new(),
            visited: HashSet::new(),
            oxygen: HashSet::new(),
            oxygen_system: None,
            path_length: 0,
        }
    }

    fn get_possible_direction(&self, pos: (isize, isize)) -> Option<Direction> {
        let dirs = vec![Direction::North, Direction::South, Direction::West, Direction::East];
        dirs
            .iter()
            .filter(|&d| {
                let new_pos = d.move_from_pos(pos);
                !(self.walls.contains(&new_pos) || self.visited.contains(&new_pos))
            })
            .cloned()
            .nth(0)
    }

    fn hit_wall(&mut self, pos: (isize, isize), dir: Direction) {
        self.walls.insert(dir.move_from_pos(pos));
    }

    fn set_visited(&mut self, pos: (isize, isize)) {
         self.visited.insert(pos);
    }

    fn visited(&self, pos: &(isize, isize)) -> bool {
        self.visited.contains(pos)
    }

    fn spread_oxygen(&mut self) -> bool {
        let mut spread = false;
        let mut new_positions = vec![];

        for &pos in self.oxygen.iter() {
            new_positions.append(&mut self.spread_from_pos(pos));
        }

        if new_positions.len() > 0 {
            spread = true;
        }

        for pos in new_positions {
            self.oxygen.insert(pos);
        }

        spread
    }

    fn spread_from_pos(&self, pos: (isize, isize)) -> Vec<(isize, isize)> {
        let dirs = vec![Direction::North, Direction::South, Direction::West, Direction::East];
        dirs
            .iter()
            .map(|d| d.move_from_pos(pos))
            .filter(|p| !(self.walls.contains(p) || self.oxygen.contains(p)))
            .collect()
    }

    fn display(&self, pos: Option<(isize, isize)>) {
        print!("\x1B[2J");
        let mut min_x = 0;
        let mut max_x = 0;
        let mut min_y = 0;
        let mut max_y = 0;

        for &(x, y) in self.walls.iter() {
            if x < min_x { min_x = x; }
            else if x > max_x { max_x = x; }
            if y < min_y { min_y = y; }
            else if y > max_y { max_y = y; }
        }

        if let Some((x, y)) = pos {
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

        for &(x, y) in self.walls.iter() {
            chars[(height - (y + yoff) - 1) as usize][(x + xoff) as usize] = '█';
        }

        for &(x, y) in self.oxygen.iter() {
            chars[(height - (y + yoff) - 1) as usize][(x + xoff) as usize] = '░';
        }

        if let Some((x, y)) = pos {
            chars[(height - (y + yoff) - 1) as usize][(x + xoff) as usize] = '@';
        }

        if let Some((x, y)) = self.oxygen_system {
            chars[(height - (y + yoff) - 1) as usize][(x + xoff) as usize] = 'X';
        }

        for row in chars {
            for c in row {
                print!("{}", c);
            }
            println!();
        }
    }
}