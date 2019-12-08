use crate::solver::Solver;
use std::{
    io::{self, BufReader, BufRead},
    str::FromStr,
    collections::HashSet,
    iter::FromIterator,
    hash::{Hash, Hasher}
};

pub struct Problem;

pub struct Segment {
    dir: Direction,
    dist: isize,
}

impl FromStr for Segment {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Segment {
            dir: s[0..1].parse::<Direction>().expect("Unable to parse direction"),
            dist: s[1..].parse::<isize>().expect("Unable to parse distance"),
        })
    }
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn get_dir(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Eq)]
pub struct Point {
    x: isize,
    y: isize,
    path_length: isize,
}

impl Point {
    fn manhattan_distance(&self) -> isize {
        self.x.abs() + self.y.abs()
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl Solver for Problem {
    type Input = (HashSet<Point>, HashSet<Point>);
    type Output1 = isize;
    type Output2 = isize;

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let mut r = BufReader::new(r);

        let mut buf = String::new();
        let _ = r.read_line(&mut buf).expect("Unable to read first line");
        let wire1 = parse_segment(&buf);
        let wire1_coords= HashSet::from_iter(segments_to_points(&wire1).clone().into_iter());

        buf = String::new();
        let _ = r.read_line(&mut buf).expect("Unable to read second line");
        let wire2 = parse_segment(&buf);
        let wire2_coords: HashSet<Point>= HashSet::from_iter(segments_to_points(&wire2).clone().into_iter());

        (wire1_coords, wire2_coords)
    }

    fn solve_first(&self, (w1, w2): &Self::Input) -> Self::Output1 {
        w1.intersection(&w2)
            .map(|i| i.manhattan_distance())
            .min()
            .unwrap()
    }

    fn solve_second(&self, (w1, w2): &Self::Input) -> Self::Output2 {
        w1.intersection(&w2)
            .map(|p| {
                let p1 = w1.get(p).unwrap();
                let p2 = w2.get(p).unwrap();
                (p1.path_length + p2.path_length)
            })
            .min()
            .unwrap()
    }
}

fn parse_segment(s: &String) -> Vec<Segment> {
    s.trim()
        .split(',')
        .map(|s| s.parse::<Segment>().expect("Unable to parse segment")).collect()
}

fn segments_to_points(segments: &Vec<Segment>) -> Vec<Point> {
    let mut points = vec!();

    let mut origin = Point {
        x: 0,
        y: 0,
        path_length: 0,
    };

    for segment in segments {
        let mut segment_points = segment_to_points(&origin, segment);
        origin = segment_points.last().expect("no last point").clone();
        points.append(&mut segment_points);
    }

    points
}

fn segment_to_points(origin: &Point, segment: &Segment) -> Vec<Point> {
    let mut points = vec!();
    let (dx, dy) = segment.dir.get_dir();
    let mut point = origin.clone();

    for _ in 0..segment.dist {
        point = Point {
            x: point.x + dx,
            y: point.y + dy,
            path_length: point.path_length + 1,
        };
        points.push(point.clone());
    }

    points
}