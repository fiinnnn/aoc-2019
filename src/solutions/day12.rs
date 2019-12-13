use crate::solver::Solver;
use std::{
    io::{Read, BufReader, BufRead},
    error::Error,
    ops::AddAssign,
    cmp::Ordering
};
use regex::Regex;
use itertools::Itertools;
use num::Integer;

pub struct Problem;

#[derive(Debug, Copy, Clone)]
struct Vec3 {
    x: isize,
    y: isize,
    z: isize,
}

impl Vec3 {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            z: 0,
        }
    }

    fn abs_sum(&self) -> isize {
        self.x.abs() + self.y.abs() + self.z.abs()
    }

    fn from_str(s: &str) -> Result<Self, Box<dyn Error>> {
        let regex = Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>")?;
        let cap = regex.captures(s).ok_or("No match")?;
        Ok(Self {
            x: cap[1].parse()?,
            y: cap[2].parse()?,
            z: cap[3].parse()?,
        })
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

#[derive(Copy, Clone)]
pub struct Moon {
    pos: Vec3,
    v: Vec3,
}

impl Moon {
    fn new(pos: Vec3) -> Self {
        Self {
            pos,
            v: Vec3::new(),
        }
    }

    fn update_position(&mut self) {
        self.pos += self.v;
    }

    fn update_velocity(&mut self, other: &mut Moon) {
        match self.pos.x.cmp(&other.pos.x) {
            Ordering::Less => {
                self.v.x += 1;
                other.v.x -= 1;
            },
            Ordering::Equal => {},
            Ordering::Greater => {
                self.v.x -= 1;
                other.v.x += 1;
            },
        }

        match self.pos.y.cmp(&other.pos.y) {
            Ordering::Less => {
                self.v.y += 1;
                other.v.y -= 1;
            },
            Ordering::Equal => {},
            Ordering::Greater => {
                self.v.y -= 1;
                other.v.y += 1;
            },
        }

        match self.pos.z.cmp(&other.pos.z) {
            Ordering::Less => {
                self.v.z += 1;
                other.v.z -= 1;
            },
            Ordering::Equal => {},
            Ordering::Greater => {
                self.v.z -= 1;
                other.v.z += 1;
            },
        }
    }

    fn get_energy(&self) -> isize {
        self.pos.abs_sum() * self.v.abs_sum()
    }
}

impl Solver for Problem {
    type Input = Vec<Moon>;
    type Output1 = isize;
    type Output2 = isize;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        BufReader::new(r)
            .lines()
            .flatten()
            .flat_map(|s| Vec3::from_str(&s))
            .map(Moon::new)
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let mut moons = input.clone();
        for _ in 0..1000 {
            step(&mut moons);
        }
        Iterator::sum(moons.iter().map(|m| m.get_energy()))
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let mut moons = input.clone();

        let mut i = 1;
        let mut per_x = 0;
        let mut per_y = 0;
        let mut per_z = 0;

        loop {
            step(&mut moons);

            if per_x == 0 && vx_zero(&moons) {
                per_x = i * 2;
            }

            if per_y == 0 && vy_zero(&moons) {
                per_y = i * 2;
            }

            if per_z == 0 && vz_zero(&moons) {
                per_z = i * 2;
            }

            if per_x != 0 && per_y != 0 && per_z != 0 {
                break;
            }

            i += 1;
        }

        per_x.lcm(&per_y.lcm(&per_z))
    }
}

fn step(moons: &mut Vec<Moon>) {
    for (a, b) in (0..moons.len()).tuple_combinations() {
        let (s1, s2) = moons.split_at_mut(a + 1);
        let m1 = &mut s1[a];
        let m2 = &mut s2[b - a - 1];
        m1.update_velocity(m2);
    }

    for m in moons {
        m.update_position();
    }
}

fn vx_zero(moons: &Vec<Moon>) -> bool {
    moons.iter().all(|m| m.v.x == 0)
}

fn vy_zero(moons: &Vec<Moon>) -> bool {
    moons.iter().all(|m| m.v.y == 0)
}

fn vz_zero(moons: &Vec<Moon>) -> bool {
    moons.iter().all(|m| m.v.z == 0)
}