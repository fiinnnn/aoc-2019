use std::error::Error;
use std::fs;
use std::fs::read_dir;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;
use std::str;

fn days(input_dir: &str) -> io::Result<Vec<u32>> {
    Ok(read_dir(input_dir)?
        .flatten()
        .filter(|e| e.path().is_file())
        .flat_map(|e| e.file_name().into_string())
        .flat_map(|s| {
            str::from_utf8(&s.into_bytes()[3..])
                .ok()
                .and_then(|v| v.parse::<u32>().ok())
        })
        .collect())
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

fn main() -> Result<(), Box<dyn Error>> {
    let input_dir = "./input";
    let output_dir = "./src/solutions";
    let solutions_mod_output_path = Path::new(&output_dir).join("mod.rs");

    let days = days(input_dir)?;

    gen_solutions_mod(&solutions_mod_output_path, &days)?;

    Ok(())
}
