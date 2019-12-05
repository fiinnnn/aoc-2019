use crate::solver::Solver;
use std::{
    io::{self, BufRead, BufReader},
};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<i32>;
    type Output1 = i32;
    type Output2 = i32;

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let r = BufReader::new(r);
        let s: String = r.lines().flatten().next().expect("Unable to read line");
        let range: Vec<i32> = s.split('-').map(|x| x.parse().expect("Unable to parse")).collect();
        (range[0]..range[1]).collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        input.iter().filter(|p| validate_password_first(p)).count() as i32
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        input.iter().filter(|p| validate_password_second(p)).count() as i32
    }
}

fn validate_password_first(pass: &i32) -> bool {
    let mut repeated = false;

    let n = pass.to_string().into_bytes();

    for pair in n.windows(2) {
        if pair[0] == pair[1] {
            repeated = true;
        }

        if pair[0] > pair[1] {
            return false;
        }
    }

    repeated
}

fn validate_password_second(pass: &i32) -> bool {
    let mut repeated = false;

    let n = pass.to_string().into_bytes();

    let mut i = 0;
    while i < n.len() {
        let n1 = *n.get(i).unwrap_or(&0);
        let n2 = *n.get(i+1).unwrap_or(&0);
        let n3 = *n.get(i+2).unwrap_or(&0);

        if n2 != 0 && n1 > n2 {
            return false;
        }

        if n1 == n2 {
            if n2 != n3 {
                repeated = true;
                i += 1;
            }
            else {
                i += 2;
                while n1 == *n.get(i+1).unwrap_or(&0) {
                    i += 1;
                }
            }
        }
        else {
            i += 1;
        }
    }

    repeated
}