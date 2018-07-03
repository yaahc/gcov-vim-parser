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

    let gcov_re = Regex::new(r"^\s*#####:\s*(\d*):(.*)").unwrap();
    let lcov_re = Regex::new(r"LCOV_EXCL_(LINE|START|STOP)").unwrap();

    let gcov_lines: Vec<String> = BufReader::new(rdr)
        .lines()
        .filter_map(|line| line.ok().filter(|line| line.starts_with("    #####:")))
        .collect();

    let uncovered_lines: Vec<(&str, usize)> = gcov_lines
        .iter()
        .filter_map(|line| gcov_re.captures(&line))
        .map(|caps| {
            (
                caps.get(2).map(|m| m.as_str()).unwrap(),
                caps[1].parse().unwrap(),
            )
        })
        .collect();

    let mut disabled = false;

    let src_lines: Vec<_> = BufReader::new(src_rdr)
        .lines()
        .enumerate()
        .filter_map(|(lineno, line)| match line {
            Ok(line) => Some((lineno + 1, line)),
            _ => None,
        })
        .filter_map(|(lineno, line)| {
            if lcov_re.is_match(&line) {
                if line.contains("LCOV_EXCL_START") {
                    disabled = true
                } else if line.contains("LCOV_EXCL_STOP") {
                    disabled = false;
                }
                None
            } else if disabled {
                None
            } else {
                Some((lineno, line))
            }
        })
        .collect();

    let mut source_map = BTreeMap::new();

    src_lines.iter().for_each(|(lineno, line)| {
        source_map
            .entry(line.as_str())
            .or_insert_with(Vec::new)
            .push(lineno)
    });

    let diff = |a, b| max(a, b) - min(a, b);

    uncovered_lines
        .iter()
        .filter_map(|(line, num)| {
            source_map.get(line).map(|linenums| {
                linenums
                    .iter()
                    .min_by_key(|a| diff(**a, num))
                    .map(|num| (line, *num))
                    .unwrap()
            })
        })
        .for_each(|(line, linenum)| {
            if arg_matches.is_present("vimgrep") {
                let start_ind = line.find(|c: char| !c.is_whitespace()).unwrap_or(0);
                println!("{}:{}:{}:{}", input, linenum, start_ind, line)
            } else {
                println!("[{}:{}]: (uncovered) uncovered:[{}]", fname, linenum, line)
            };
        });
}
