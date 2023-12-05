use crate::auth::current_token;
use crate::puzzle::{AnswerStatus, Run};
use crate::{client, Day, Part, Year};
use anyhow::{anyhow, bail, Context, Result};
use colored::Colorize;
use itertools::Itertools;
use regex::Regex;
use reqwest::header::COOKIE;
use std::fmt::Display;
use std::io::Write;
use std::os::unix::prelude::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

lazy_static::lazy_static! {
    static ref EXE_RE: Regex = Regex::new(r"^(\d{4})_(\d{1,2})$").unwrap();
    static ref CODE_RE: Regex = Regex::new(r"(?s)<code>(.+?)</code>").unwrap();
    static ref AOC_YEAR_RE: Regex = Regex::new(r"^aoc(20\d\d)$").unwrap();
    static ref ANSWER_RE: Regex = Regex::new(r"Your puzzle answer was <code>(.+?)</code>").unwrap();
}

pub fn year() -> Result<Year> {
    let path = std::env::current_dir()?;
    for (aoc_year, aoc) in path.iter().rev().tuple_windows() {
        if aoc == "aoc" {
            if let Some(cap) = AOC_YEAR_RE.captures(&aoc_year.to_string_lossy()) {
                return Ok(Year(cap[1].parse()?));
            }
        }
    }
    bail!("run me in aoc/aoc20??");
}

pub fn parse_day(file: &str) -> Day {
    let path = Path::new(file);
    let day = path
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .split('_')
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
    Day(day)
}

#[macro_export]
macro_rules! xaoc {
    ( $($i:ident = $e:expr),* ) => {
        use anyhow::Result;

        fn main() -> Result<()> {
            let year = xaoc::runner::year()?;
            let day = xaoc::runner::parse_day(std::file!());
            let mut opts = xaoc::runner::RunOptions::default();
            $(
                opts.$i = $e;
            )*
            xaoc::runner::run(year, day, part1, part2, opts)?;
            Ok(())
        }
    };
}

use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(long)]
    p1: bool,

    #[arg(long)]
    p2: bool,

    #[arg(long)]
    dev: bool,

    #[arg(long)]
    input_file: Option<PathBuf>,
}

#[derive(Default)]
pub struct RunOptions {
    pub sample_idx: usize,
    pub sample_idx2: Option<usize>,
    pub sample: &'static str,
    pub sample2: &'static str,
    pub no_sample: bool,
}

impl RunOptions {
    pub fn sample_idx(&mut self, sample_idx: usize) {
        self.sample_idx = sample_idx;
    }

    pub fn sample_idx2(&mut self, sample_idx2: usize) {
        self.sample_idx2 = Some(sample_idx2);
    }

    pub fn sample(&mut self, sample: &'static str) {
        self.sample = sample;
    }

    pub fn sample2(&mut self, sample2: &'static str) {
        self.sample2 = sample2;
    }

    pub fn no_sample(&mut self, no_sample: bool) {
        self.no_sample = no_sample;
    }
}

fn get_sample(text: &str, idx: usize) -> Option<String> {
    let mut s = String::new();
    let cap = CODE_RE.captures_iter(text).nth(idx)?;
    html_escape::decode_html_entities_to_string(
        cap.get(1)?.as_str().trim_end_matches('\n'),
        &mut s,
    );
    Some(s)
}

pub fn run<F1, D1, F2, D2>(
    year: Year,
    day: Day,
    part1: F1,
    part2: F2,
    opts: RunOptions,
) -> Result<()>
where
    F1: Fn(&str) -> D1,
    D1: Display,
    F2: Fn(&str) -> D2,
    D2: Display,
{
    let cli = Cli::parse();
    let mut p1 = cli.p1;
    let mut p2 = cli.p2;
    if !p1 && !p2 {
        p1 = true;
        p2 = true;
    }

    let token = crate::auth::current_token().context("get token")?;
    let run = Run::new(token.clone(), year, day, Part::One)?;
    let puzzle = run.get_puzzle().context("get puzzle")?;
    let input = if let Some(path) = cli.input_file {
        let s = String::from_utf8(std::fs::read(path)?)?;
        let s = s.trim_end_matches('\n');
        s.to_string()
    } else {
        run.get_input().context("get input")?
    };
    println!("{}", puzzle.title.green().bold());
    if cli.dev {
        (|| {
            if puzzle.text.is_empty() {
                println!("no puzzle text, run prepare first");
                return;
            }
            if opts.no_sample {
                println!("skipping sample");
                return;
            }
            let input_p1 = if !opts.sample.is_empty() {
                opts.sample.to_string()
            } else if let Some(input_p1) = get_sample(&puzzle.text, opts.sample_idx) {
                input_p1
            } else {
                println!("no p1 input");
                return;
            };
            let input_p2 = if !opts.sample2.is_empty() {
                opts.sample2.to_string()
            } else if let Some(sample_idx2) = opts.sample_idx2 {
                match get_sample(&puzzle.text, sample_idx2) {
                    Some(i) => i,
                    None => {
                        println!("no p2 input");
                        return;
                    }
                }
            } else {
                input_p1.to_string()
            };
            if p1 {
                println!("{}", "input:".red().bold());
                println!("{input_p1}");
                println!("{}", "part 1:".red().bold());
                println!("{}", part1(&input_p1));
            }
            if p2 {
                if !p1 || input_p2 != input_p1 {
                    println!("{}", "part 2 input:".red().bold());
                    println!("{input_p2}");
                }
                println!("{}", "part 2:".red().bold());
                println!("{}", part2(&input_p2));
            }
        })();
    }
    type Boxed<'a> = Box<dyn Fn() -> String + 'a>;
    for (n, f, b) in [
        (1, Box::new(|| part1(&input).to_string()) as Boxed, p1),
        (2, Box::new(|| part2(&input).to_string()) as Boxed, p2),
    ] {
        if b {
            let start = Instant::now();
            let mut res = f();
            let time = start.elapsed().as_millis();
            let mut ml = res.chars().any(|c| c == '\n');
            if ml {
                match crate::puzzle::map_get(&res)? {
                    Some(s) => {
                        res = s;
                        ml = false;
                    }
                    None => std::fs::write("/tmp/xaoc_multiline", res.as_bytes())?,
                }
            }
            let ml_s = if ml {
                ", multiline output saved to /tmp/xaoc_multiline\n"
            } else {
                ""
            };
            let mut out = String::new();
            out.push_str(&format!("part {n}({time}ms){ml_s}"));
            if ml {
                out.push_str(&format!("\n{res}\n"));
            } else {
                let run = Run::new(token.clone(), year, day, Part::new(n)?)?;
                let sym = match run.check_answer(&res)? {
                    AnswerStatus::Good => "✅",
                    AnswerStatus::Bad => "❌",
                    AnswerStatus::Unknown => "❓",
                };
                out.push_str(&format!(" {sym} [ {res} ]"));
            }
            print!("{out:<40}");
            let _ = std::io::stdout().flush();
        }
        print!("\t\t\t\t");
    }
    println!();
    Ok(())
}

pub fn prepare(day: Day) -> Result<()> {
    let year = year()?;
    let token = current_token()?;
    let run = Run::new(token, year, day, Part::One)?;
    let code = PathBuf::from(format!("src/bin/{year}_{day}.rs"));
    if !code.exists() {
        let template = include_bytes!("../fixtures/template.rs");
        std::fs::write(&code, template)?;
        println!("Created {:?}", code);
    } else {
        println!("Code already exists {:?}", code);
    }
    let title = run.get_or_fetch_puzzle()?.title;
    println!("{}", title.green().bold());
    let input = run.get_or_fetch_input()?;
    println!("input {} bytes", input.len());
    Ok(())
}

pub fn sync_answers(force: bool) -> Result<()> {
    let year = year()?;
    let token = current_token()?;
    println!("syncing answers for year {year}");
    for day in 1..=25 {
        println!("day {day} ");
        let mut answers = vec![];
        for part in 1..=2 {
            print!("  part {part}: ");
            let run = Run::new(token.clone(), year, Day(day), Part::new(part)?)?;
            if run.get_answer().is_ok() && !force {
                println!("already exists, skipping");
                continue;
            }
            if answers.is_empty() {
                let client = client()?;
                let page = client
                    .get(format!("https://adventofcode.com/{year}/day/{day}"))
                    .header(COOKIE, format!("session={}", token.token))
                    .send()?
                    .error_for_status()?
                    .text()?;
                answers = ANSWER_RE
                    .captures_iter(&page)
                    .map(|cap| html_escape::decode_html_entities(&cap[1]).into_owned())
                    .collect::<Vec<_>>();
                if answers.is_empty() {
                    println!("no answers on puzzle page");
                    break;
                }
            }
            let Some(answer) = answers.get(part as usize - 1) else {
                println!("no answer");
                break;
            };
            match run.check_answer(answer)? {
                AnswerStatus::Good => println!("good"),
                AnswerStatus::Bad => bail!("bad {answer}"),
                AnswerStatus::Unknown => {
                    run.set_answer(answer)?;
                    println!("updated");
                }
            }
        }
    }
    Ok(())
}

pub fn run_all(debug: bool) -> Result<()> {
    let year = year()?;
    let mut cmd = Command::new("cargo");
    cmd.arg("build");
    if !debug {
        cmd.arg("--release");
    }
    if !cmd.status()?.success() {
        bail!("build failed");
    }

    let base = if debug {
        Path::new("target/debug/")
    } else {
        Path::new("target/release/")
    };

    let mut present = vec![];
    for entry in std::fs::read_dir(base)? {
        if let Ok(Some(n)) = (|| {
            let entry = entry?;
            // check if it's an executable file
            if !entry.file_type()?.is_file() || entry.metadata()?.permissions().mode() & 0o111 == 0
            {
                return Ok(None);
            }
            let name = entry
                .file_name()
                .into_string()
                .map_err(|_| anyhow!("bad file name"))?;
            let Some(caps) = EXE_RE.captures(&name) else {
                return Ok(None);
            };
            if caps.get(1).unwrap().as_str() != year.to_string() {
                return Ok(None);
            }
            anyhow::Ok(Some(caps.get(2).unwrap().as_str().parse::<u16>()?))
        })() {
            present.push(n);
        }
    }

    present.sort();
    for n in present {
        Command::new(base.join(format!("{year}_{n}"))).status()?;
    }
    Ok(())
}
