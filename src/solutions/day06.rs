use crate::solver::Solver;
use std::{
    io::{self, BufRead, BufReader},
    collections::HashMap,
};
use std::collections::VecDeque;

pub struct Problem;

#[derive(Debug, Eq, PartialEq)]
pub struct Planet {
    name: String,
    children: Vec<Planet>,
    depth: i64,
}

impl Solver for Problem {
    type Input = Planet;
    type Output1 = i64;
    type Output2 = i64;

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let r = BufReader::new(r);

        let mut planets: HashMap<String, Vec<String>> = HashMap::new();

        for line in r.lines().map(|l| l.expect("Unable to read line")) {
            let item: Vec<&str> = line.split(')').collect();
            let parent = item[0].to_string();
            let child = item[1].to_string();

            if planets.contains_key(&parent) {
                planets.get_mut(&parent).expect("Unable to get parent").push(child);
            }
            else {
                planets.insert(parent, vec![child]);
            }
        }

        let mut root = Planet {
            name: "COM".into(),
            children: vec![],
            depth: 0,
        };

        add_children(&mut root, &planets, 1);

        root
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        sum_depths(&input)
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let mut queue = VecDeque::new();
        let mut path_to_you = vec![];
        let mut path_to_san = vec![];
        let mut one_found = false;

        let mut you_depth = 0;
        let mut san_depth = 0;

        queue.push_back(vec![input]);
        while !queue.is_empty() {
            let path = queue.pop_back().expect("Unable to get path");
            let planet = path.last().expect("Unexpected empty path");
            if planet.name.as_str() == "YOU" {
                path_to_you = path.clone();
                you_depth = planet.depth;
                if one_found {
                    break;
                }
                one_found = true;
            }
            else if planet.name.as_str() == "SAN" {
                path_to_san = path.clone();
                san_depth = planet.depth;
                if one_found {
                    break;
                }
                one_found = true;
            }

            for child in &planet.children {
                let mut new_path = path.clone();
                new_path.push(child);
                queue.push_back(new_path);
            }
        }

        let common_parent_depth = path_to_you.iter()
            .filter(|p| path_to_san.contains(p))
            .map(|p| p.depth)
            .max()
            .expect("Can't find common parent");

        you_depth + san_depth - 2 * common_parent_depth - 2
    }
}

fn add_children(planet: &mut Planet, planets: &HashMap<String, Vec<String>>, depth: i64) {
    if let Some(children) = planets.get(&planet.name) {
        for child in children {
            let mut child_planet = Planet{name: child.clone(), children: vec![], depth};
            add_children(&mut child_planet, planets, depth+1);
            planet.children.push(child_planet);
        }
    }
}

fn sum_depths(node: &Planet) -> i64 {
    let mut count = node.depth;

    for child in &node.children {
        count += sum_depths(child);
    }

    count
}