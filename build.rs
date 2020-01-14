use std::error::Error;
use std::fs::read_dir;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::str;

fn days(input_dir: &str) -> io::Result<Vec<u32>> {
    let mut days = read_dir(input_dir)?
        .flatten()
        .filter(|e| e.path().is_file())
        .flat_map(|e| e.file_name().into_string())
        .flat_map(|s| s[3..].parse::<u32>())
        .collect::<Vec<_>>();
    days.sort();
    Ok(days)
}

fn gen_solutions_mod<P: AsRef<Path>>(p: P, days: &Vec<u32>) -> io::Result<()> {
    let mut f = File::create(p)?;

    writeln!(
        f,
        "// GENERATED ON BUILD, DO NOT EDIT"
    )?;

    writeln!(f, "use crate::solver::Solver;")?;

    writeln!(f)?;
    for day in days {
        writeln!(f, "mod day{0:02};", day)?;
    }

    writeln!(f)?;

    writeln!(
        f,
        "pub fn exec_day(day: i32) {{
    match day {{"
    )?;
    for day in days {
        writeln!(f, "        {0} => day{0:02}::Problem {{}}.solve(day),", day)?;
    }
    writeln!(
        f,
        "        d => println!(\"Day {{}} hasn't been solved yet :(\", d),
    }}
}}"
    )?;

    Ok(())
}

fn gen_solutions(dir: &str, days: &[u32]) -> io::Result<()> {
    for day in days {
        let file = PathBuf::from(format!("{}/day{:02}.rs", dir, day));
        if file.exists() {
            continue;
        }

        let mut f = File::create(file)?;
        writeln!(
            f,
            "use crate::solver::Solver;
use std::io::Read;

pub struct Problem;

impl Solver for Problem {{
    type Input = ();
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {{}}

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {{
        0
    }}

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {{
        0
    }}
}}"
        )?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let input_dir = "./input";
    let output_dir = "./src/solutions";
    let solutions_mod_output_path = Path::new(&output_dir).join("mod.rs");

    let days = days(input_dir)?;

    gen_solutions_mod(&solutions_mod_output_path, &days)?;

    gen_solutions(&output_dir, &days)?;

    Ok(())
}
