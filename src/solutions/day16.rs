use crate::solver::Solver;
use std::io::{Read, BufReader};
use itertools::Itertools;
use rayon::prelude::*;

pub struct Problem;


impl Solver for Problem {
    type Input = Vec<u8>;
    type Output1 = String;
    type Output2 = String;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        BufReader::new(r)
            .bytes()
            .flatten()
            .map(|b| b - b'0')
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let mut list = input.clone();
        for _ in 0..100 {
            list = fft(list);
        }

        list.iter().take(8).join("")
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let offset = input.iter()
            .take(7)
            .join("")
            .parse()
            .unwrap_or(0);

        let len = input.len();
        let mut list = input.iter()
            .cycle()
            .take(10000 * len)
            .skip(offset)
            .cloned()
            .collect::<Vec<_>>();

        for _ in 0..100 {
            for i in (0..list.len() - 1).rev() {
                list[i] = (list[i] + list[i + 1]) % 10;
            }
        }

        list.iter().take(8).join("")
    }
}

fn fft(input: Vec<u8>) -> Vec<u8> {
    (0..input.len())
        .into_par_iter()
        .map(|i| {
            let mut n = 0;
            for j in i..input.len() {
                n += get_pattern(i, j) * input[j] as isize;
            }
            (n.abs() % 10) as u8
        })
        .collect()
}

fn get_pattern(i: usize, j: usize) -> isize {
    let base = [0, 1, 0, -1];
    base[(((j + 1) / (i + 1)) % 4) as usize]
}