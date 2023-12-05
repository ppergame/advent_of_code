pub mod auth;
pub mod puzzle;
pub mod runner;

use anyhow::{bail, Context, Result};
use md5::{Digest, Md5};
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, USER_AGENT};
use std::fmt::Display;
use std::path::PathBuf;

pub fn md5(
    s: impl AsRef<[u8]>,
) -> md5::digest::generic_array::GenericArray<u8, md5::digest::typenum::U16> {
    let mut md5 = Md5::new();
    md5.update(s);
    md5.finalize()
}

pub fn client() -> Result<Client> {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, "xaoc by ppergame@gmail.com".parse()?);
    let client = Client::builder().default_headers(headers).build()?;
    Ok(client)
}

pub fn setup_dir() -> Result<PathBuf> {
    (|| {
        let path = xdg::BaseDirectories::new()?.create_config_directory("xaoc")?;
        anyhow::Ok(path)
    })()
    .context("setup_dir")
}

#[derive(Copy, Clone)]
pub struct Year(pub u16);

impl Display for Year {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Copy, Clone)]
pub struct Day(pub u16);

impl Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Part {
    One,
    Two,
}

impl Part {
    pub fn new(part: u16) -> Result<Self> {
        match part {
            1 => Ok(Part::One),
            2 => Ok(Part::Two),
            _ => bail!("bad part {part}"),
        }
    }
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let n = match self {
            Part::One => 1,
            Part::Two => 2,
        };
        write!(f, "{}", n)
    }
}
