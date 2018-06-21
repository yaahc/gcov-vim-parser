extern crate regex;

use regex::Regex;
use std::env;
use std::fs;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
    let mut input = env::args().nth(1).unwrap();
    input.push_str(".gcov");
    let gcov_file = env::home_dir().unwrap().join("gcov").join(input.clone());
    let rdr = fs::File::open(gcov_file).unwrap();

    let re =
        Regex::new(r"^\s*#####:\s*(?P<linenum>\d*):(?P<whitespace>\s*)(?P<linetext>.*)").unwrap();

    for line in BufReader::new(rdr).lines() {
        let string = line?;
        let caps = re.captures(&string);
        if let Some(caps) = caps {
            let start = caps["whitespace"].chars().count();
            println!(
                "{}:{}:{}: {}{}",
                input, &caps["linenum"], start, &caps["whitespace"], &caps["linetext"]
            );
        }
    }
    Ok(())
}
