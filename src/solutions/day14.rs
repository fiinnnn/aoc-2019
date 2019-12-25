use crate::solver::Solver;
use std::io::{self, BufRead, BufReader};
use regex::Regex;
use std::collections::{HashMap, VecDeque};

pub struct Problem;

impl Solver for Problem {
    type Input = HashMap<String, Reaction>;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        BufReader::new(r)
            .lines()
            .flatten()
            .map(|s| Reaction::parse(&s))
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let mut refinery = Refinery::new(input.clone());
        refinery.make_fuel(1);
        refinery.ore_amount
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let mut refinery = Refinery::new(input.clone());
        let mut fuel_amount = 0;
        let estimate = 1_850_000;
        refinery.make_fuel(estimate);
        fuel_amount += estimate;

        while refinery.make_fuel(1) {
            fuel_amount += 1;
        }

        fuel_amount
    }
}

#[derive(Debug, Clone)]
pub struct Reaction {
    output_amount: u64,
    input: Vec<(String, u64)>,
}

impl Reaction {
    pub fn parse(s: &str) -> (String, Self) {
        let regex = Regex::new(r"(\d+) (\w+)").unwrap();
        let elements = regex.captures_iter(s)
            .map(|c| (c[2].to_string(), c[1].parse().unwrap()))
            .collect::<Vec<_>>();

        let ((elem, amount), input) = elements.split_last().unwrap();

        let reaction = Reaction {
            output_amount: *amount,
            input: input.to_vec(),
        };

        (elem.clone(), reaction)
    }
}

struct Refinery {
    reactions: HashMap<String, Reaction>,
    excess: HashMap<String, u64>,
    ore_amount: u64,
}

impl Refinery {
    fn new(reactions: HashMap<String, Reaction>) -> Self {
        let mut excess = HashMap::new();
        excess.insert("ORE".into(), 1_000_000_000_000);

        Self {
            reactions,
            excess,
            ore_amount: 0,
        }
    }

    fn make_fuel(&mut self, amount: u64) -> bool {
        let mut queue = VecDeque::new();
        queue.push_back(("FUEL".to_string(), amount));

        while let Some((elem, required)) = queue.pop_front() {
            let available = self.get_excess(&elem, required);
            let build_amount = required - available;

            if build_amount == 0 {
                continue;
            }

            if let Some(reaction) = self.reactions.get(&elem).cloned() {
                let reaction_amount = Self::calc_reaction_amount(build_amount, reaction.output_amount);

                let excess = reaction_amount * reaction.output_amount - build_amount;
                self.add_excess(&elem, excess);

                for (e, n) in reaction.input {
                    queue.push_back((e, reaction_amount * n));
                }
            }
            else {
                return false;
            }
        }

        true
    }

    fn add_excess(&mut self, elem: &str, excess: u64) {
        let amount = self.excess.entry(elem.to_string()).or_insert(0);
        *amount += excess;
    }

    fn get_excess(&mut self, elem: &str, required: u64) -> u64 {
        if elem == "ORE" {
            self.ore_amount += required;
        }

        if let Some(amount) = self.excess.get_mut(elem) {
            if *amount >= required {
                *amount -= required;
                required
            }
            else {
                let remaining = *amount;
                *amount = 0;
                remaining
            }
        }
        else {
            0
        }
    }

    fn calc_reaction_amount(required: u64, reaction_output: u64) -> u64 {
        (required as f64 / reaction_output as f64).ceil() as u64
    }
}