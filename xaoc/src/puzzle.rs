use crate::auth::{current_token, Token};
use crate::runner::year;
use crate::{client, setup_dir, Day, Part, Year};
use anyhow::{anyhow, bail, Context, Result};
use regex::Regex;
use reqwest::header::COOKIE;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

lazy_static::lazy_static! {
    static ref TITLE_RE: Regex = Regex::new(r"--- (Day \d+?: .*?) ---").unwrap();
    static ref MAIN_RE: Regex = Regex::new(r"(?i)(?s)<main>(.*)</main>").unwrap();
}

#[derive(Serialize, Deserialize)]
pub struct Puzzle {
    pub title: String,
    #[serde(default)]
    pub text: String,
}

pub struct Run {
    token: Token,
    year: Year,
    day: Day,
    part: Part,
}

impl Run {
    pub fn new(token: Token, year: Year, day: Day, part: Part) -> Result<Self> {
        if !(2015..=2024).contains(&year.0) {
            bail!("bad year {year}");
        }
        if !(1..=25).contains(&day.0) {
            bail!("bad day {day}");
        }
        Ok(Run {
            token,
            year,
            day,
            part,
        })
    }

    fn puzzle_path(&self) -> Result<PathBuf> {
        let mut base = setup_dir()?;
        base.push(format!("puzzle/{}/{}/info.json", self.year, self.day));
        Ok(base)
    }

    pub fn get_puzzle(&self) -> Result<Puzzle> {
        let puzzle: Puzzle = serde_json::from_reader(File::open(self.puzzle_path()?)?)?;
        Ok(puzzle)
    }

    pub fn get_or_fetch_puzzle(&self) -> Result<Puzzle> {
        let path = self.puzzle_path()?;
        if path.exists() {
            let puzzle = self.get_puzzle()?;
            if !puzzle.text.is_empty() {
                return Ok(puzzle);
            }
        }
        create_dir_all(path.parent().unwrap())?;
        let client = client()?;
        let page = client
            .get(format!(
                "https://adventofcode.com/{}/day/{}",
                self.year, self.day
            ))
            .send()?
            .error_for_status()?
            .text()?;
        let title = TITLE_RE
            .captures(&page)
            .ok_or_else(|| anyhow!("no title in puzzle page"))?
            .get(1)
            .unwrap()
            .as_str();
        let puzzle = Puzzle {
            title: title.to_string(),
            text: page,
        };
        serde_json::to_writer(&mut File::create(path)?, &puzzle)?;
        Ok(puzzle)
    }

    fn base(&self) -> Result<PathBuf> {
        let mut base = setup_dir()?;
        base.push(format!(
            "user/{}/{}/{}/",
            self.token.id, self.year, self.day
        ));
        Ok(base)
    }

    fn input_path(&self) -> Result<PathBuf> {
        let mut path = self.base()?;
        path.push("input");
        Ok(path)
    }

    pub fn get_input(&self) -> Result<String> {
        let s = String::from_utf8(std::fs::read(self.input_path()?)?)?;
        let s = s.trim_end_matches('\n');
        Ok(s.to_string())
    }

    pub fn get_or_fetch_input(&self) -> Result<String> {
        let path = self.input_path()?;
        if path.exists() {
            return self.get_input();
        }
        let client = client()?;
        let input = client
            .get(format!(
                "https://adventofcode.com/{}/day/{}/input",
                self.year, self.day
            ))
            .header(COOKIE, format!("session={}", self.token.token))
            .send()?
            .error_for_status()?
            .text()?;
        create_dir_all(path.parent().unwrap())?;
        std::fs::write(path, &input)?;
        Ok(input)
    }

    fn answer_path(&self) -> Result<PathBuf> {
        let mut path = self.base()?;
        path.push(format!("{}/answer", self.part));
        Ok(path)
    }

    pub fn get_answer(&self) -> Result<String> {
        let s = String::from_utf8(std::fs::read(self.answer_path()?)?)?;
        Ok(s)
    }

    fn bad_answers_path(&self) -> Result<PathBuf> {
        let mut path = self.base()?;
        path.push(format!("{}/bad_answers", self.part));
        Ok(path)
    }

    fn is_bad_answer(&self, res: &str) -> Result<bool> {
        let path = self.bad_answers_path()?;
        if !path.exists() {
            return Ok(false);
        }
        for line in BufReader::new(File::open(path)?).lines() {
            if line? == res {
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub fn check_answer(&self, res: &str) -> Result<AnswerStatus> {
        let path = self.answer_path()?;
        if path.exists() {
            let answer = self.get_answer()?;
            if answer == res {
                return Ok(AnswerStatus::Good);
            }
        }
        if self.is_bad_answer(res)? {
            return Ok(AnswerStatus::Bad);
        }
        Ok(AnswerStatus::Unknown)
    }

    fn submit(&self, res: &str) -> Result<()> {
        match self.check_answer(res)? {
            AnswerStatus::Good => bail!("answer is good"),
            AnswerStatus::Bad => bail!("answer is bad"),
            AnswerStatus::Unknown => (),
        }
        let client = client()?;
        let params = HashMap::<&str, String>::from_iter([
            ("level", self.part.to_string()),
            ("answer", res.to_string()),
        ]);
        let resp = client
            .post(format!(
                "https://adventofcode.com/{}/day/{}/answer",
                self.year, self.day
            ))
            .header(COOKIE, format!("session={}", self.token.token))
            .form(&params)
            .send()?
            .error_for_status()?
            .text()?;
        let main = MAIN_RE
            .captures(&resp)
            .ok_or_else(|| anyhow!("no <main> in submit response"))?
            .get(1)
            .unwrap()
            .as_str();
        if main.contains("not the right answer") {
            self.add_bad_answer(res)?;
            println!("added bad answer {res}");
        } else if main.contains("That's the right answer") {
            self.set_answer(res)?;
        }
        let main = html2text::from_read(main.as_bytes(), 80)?;
        println!("\n{main}");
        Ok(())
    }

    fn add_bad_answer(&self, res: &str) -> Result<()> {
        let path = self.bad_answers_path()?;
        create_dir_all(path.parent().unwrap())?;
        let mut f = OpenOptions::new().append(true).create(true).open(path)?;
        writeln!(f, "{res}")?;
        Ok(())
    }

    pub fn set_answer(&self, res: &str) -> Result<()> {
        let path = self.answer_path()?;
        create_dir_all(path.parent().unwrap())?;
        std::fs::write(path, res)?;
        Ok(())
    }
}

pub enum AnswerStatus {
    Good,
    Bad,
    Unknown,
}

pub fn submit(day: u16, part: u16, res: &str) -> Result<()> {
    let year = year()?;
    let token = current_token()?;
    let run = Run::new(token, year, Day(day), Part::new(part)?)?;
    run.submit(res)?;
    Ok(())
}

struct Map {
    path: PathBuf,
    map: HashMap<String, String>,
}

impl Map {
    fn new() -> Result<Self> {
        let path = setup_dir()?.join("map.json");
        let map = if path.exists() {
            serde_json::from_reader(File::open(&path)?)?
        } else {
            HashMap::new()
        };
        Ok(Map { path, map })
    }

    fn write(&self) -> Result<()> {
        let mut name = self
            .path
            .file_name()
            .unwrap_or_else(|| OsStr::new(""))
            .to_owned();
        name.push(".temp");
        let temp = self.path.with_file_name(name);
        {
            let mut f = File::create(&temp)?;
            serde_json::to_writer(&mut f, &self.map)?;
            f.sync_all()?;
        }
        std::fs::rename(temp, &self.path)?;
        Ok(())
    }
}

pub fn map(res: &str) -> Result<()> {
    let mut map = Map::new()?;
    let ml = std::fs::read_to_string("/tmp/xaoc_multiline").context("read /tmp/xaoc_multiline")?;
    println!("mapping\n{ml}\nto");
    println!("{res}");
    map.map.insert(ml, res.to_string());
    map.write()?;
    Ok(())
}

pub fn unmap() -> Result<()> {
    let mut map = Map::new()?;
    let ml = std::fs::read_to_string("/tmp/xaoc_multiline").context("read /tmp/xaoc_multiline")?;
    println!("unmapping\n{ml}");
    map.map.remove(&ml);
    map.write()?;
    Ok(())
}

pub fn map_get(res: &str) -> Result<Option<String>> {
    let map = Map::new()?;
    Ok(map.map.get(res).map(|s| s.to_string()))
}
