extern crate clap;
extern crate regex;

use clap::{App, Arg};
use regex::Regex;
use std::cmp::{max, min};
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let arg_matches = App::new("gcov vim parser")
        .version("1.0")
        .author("Jane Lusby. <jlusby42@gmail.com>")
        .about(
            "Parse gcov data and output in vim friendly formats. Expects gcov files to be stored \
             in ~/gcov directory. Default format is cppcheck style format that is compatible with \
             the included ale configuration files.",
        )
        .arg(
            Arg::with_name("vimgrep")
                .long("vimgrep")
                .help("configures vimgrep style output"),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    let input = arg_matches.value_of("INPUT").unwrap();

    let source_file = Path::new(&input);

    let fname = Path::new(&input)
        .file_name()
        .unwrap()
        .to_os_string()
        .into_string()
        .unwrap();

    let gcov_file = env::home_dir()
        .unwrap()
        .join("gcov")
        .join(fname.clone() + ".gcov");

    let rdr = fs::File::open(gcov_file).unwrap();
    let src_rdr = fs::File::open(source_file).unwrap();

    let re = Regex::new(r"^\s*#####:\s*(?P<linenum>\d*):(?P<linetext>.*)").unwrap();

    let gcov_lines: Vec<String> = BufReader::new(rdr)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let uncovered_lines: Vec<(&str, usize)> = gcov_lines
        .iter()
        .filter_map(|line| re.captures(&line))
        .map(|caps| {
            (
                caps.get(2).map(|m| m.as_str()).unwrap(),
                caps[1].parse().unwrap(),
            )
        })
        .collect();

    let mut disabled = false;

    let src_lines: Vec<(usize, String)> = BufReader::new(src_rdr)
        .lines()
        .enumerate()
        .filter_map(|(lineno, line)| match line {
            Ok(line) => Some((lineno + 1, line)),
            _ => None,
        })
        .filter_map(|(lineno, line)| {
            if line.contains("LCOV_EXCL_LINE") {
                None
            } else if line.contains("LCOV_EXCL_START") {
                disabled = true;
                None
            } else if line.contains("LCOV_EXCL_STOP") {
                disabled = false;
                None
            } else if disabled {
                None
            } else {
                Some((lineno, line))
            }
        })
        .collect();

    let diff = |a, b| max(a, b) - min(a, b);

    let mut matches = BTreeMap::new();
    for (src_lineno, src_line) in &src_lines {
        for (gcov_line, linenum) in &uncovered_lines {
            if src_line == gcov_line {
                let output = if arg_matches.is_present("vimgrep") {
                    let start_ind = src_line.find(|c: char| !c.is_whitespace()).unwrap_or(0);
                    format!("{}:{}:{}:{}", input, src_lineno, start_ind, src_line)
                } else {
                    format!(
                        "[{}:{}]: (uncovered) uncovered:[{}]",
                        fname, src_lineno, src_line
                    )
                };

                if let Some((other_lineno, other_output)) =
                    matches.insert(linenum, (src_lineno, output))
                {
                    if diff(linenum, other_lineno) < diff(linenum, src_lineno) {
                        matches.insert(linenum, (other_lineno, other_output));
                    }
                }
            }
        }
    }

    for (_, output) in matches.values() {
        println!("{}", output);
    }
}
