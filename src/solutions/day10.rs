use crate::solver::Solver;
use std::io::{Read, BufRead, BufReader};
use std::collections::{HashSet, HashMap};
use num::Integer;
use std::f64::consts::PI;

pub struct Problem;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Cell {
    Empty,
    Asteroid,
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '.' => Cell::Empty,
            '#' => Cell::Asteroid,
            _ => panic!("Invalid input"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Vec<Cell>>,
}

impl Grid {
    fn read_grid<R: BufRead>(r: R) -> Self {
        let cells = r
            .lines()
            .filter_map(|l| l.ok())
            .map(|l| l.chars().map(Cell::from).collect::<Vec<Cell>>())
            .collect::<Vec<Vec<Cell>>>();

        let height = cells.len();
        let width = cells[0].len();

        Grid {
            width,
            height,
            cells,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn to(&self, other: &Point) -> Vec2 {
        Vec2::new(
            other.x as isize - self.x as isize,
            other.y as isize - self.y as isize)
    }

    fn dist_to(&self, other: &Point) -> usize {
        ((self.x as isize - other.x as isize).abs() +
         (self.y as isize - other.x as isize).abs()) as usize
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Vec2 {
    x: isize,
    y: isize,
}

impl Vec2 {
    fn new(x: isize, y: isize) -> Self {
        let gcd = x.gcd(&y);
        Vec2 {
            x: x / gcd,
            y: y / gcd,
        }
    }

    fn angle(&self) -> f64 {
        let rad = (self.y as f64).atan2(self.x as f64);
        let mut deg = (rad * 180.0 / PI) + 90.0;

        if deg < 0.0 { deg += 360.0; }
        else if deg > 360.0 { deg -= 360.0; }

        deg
    }
}

impl Solver for Problem {
    type Input = Grid;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        Grid::read_grid(BufReader::new(r))
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let (_, c) = find_best_position(&input);
        c
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let vaporized = vaporize(&mut input.clone());
        let p = &vaporized[199];
        p.x * 100 + p.y
    }
}

fn find_best_position(grid: &Grid) -> (Point, usize) {
    let mut positions = vec![];
    for y in 0..grid.height {
        for x in 0..grid.width {
            if grid.cells[y][x] == Cell::Empty {
                continue;
            }

            let origin = Point{ x, y };
            let visible = find_visible(grid, &origin).len();
            positions.push((origin, visible));
        }
    }
    positions
        .into_iter()
        .max_by_key(|(_, c)| *c)
        .unwrap()
}

fn vaporize(grid: &mut Grid) -> Vec<Point> {
    let (origin, _) = find_best_position(grid);
    let mut vaporized = vec![];

    loop {
        let mut visible = find_visible(grid, &origin);
        if visible.is_empty() {
            break;
        }

        visible.sort_by(|(_, v1),(_, v2)| v1.angle().partial_cmp(&v2.angle()).unwrap());

        visible.iter().for_each(|(p, _)| {
            vaporized.push(p.clone());
            grid.cells[p.y][p.x] = Cell::Empty;
        });
    }

    vaporized
}

fn find_visible(grid: &Grid, origin: &Point) -> Vec<(Point, Vec2)> {
    let mut nearest: HashMap<Vec2, Point> = HashMap::new();

    for y in 0..grid.height {
        for x in 0..grid.width {
            if grid.cells[y][x] == Cell::Empty {
                continue;
            }

            let asteroid = Point { x, y };

            if asteroid == *origin {
                continue;
            }

            let vec = origin.to(&asteroid);

            if let Some(point) =  nearest.get(&vec) {
                let new_dist = origin.dist_to(&asteroid);
                let prev_dist = origin.dist_to(point);
                if new_dist < prev_dist {
                    nearest.insert(vec, asteroid);
                }
            }
            else {
                nearest.insert(vec, asteroid);
            }
        }
    }

    nearest.iter().map(|(&v, &p)| (p, v)).collect()
}