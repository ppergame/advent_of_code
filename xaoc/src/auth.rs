use crate::{client, setup_dir};
use anyhow::{anyhow, bail, Context, Result};
use regex::Regex;
use reqwest::header::COOKIE;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::fs::File;
use std::path::{Path, PathBuf};

lazy_static::lazy_static! {
    static ref SPAN_RE: Regex = Regex::new(r"([^>]+)</span").unwrap();
    static ref ANON_RE: Regex = Regex::new(r"\(anonymous user #(\d+)\)").unwrap();
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Token {
    pub id: u64,
    pub name: String,
    pub token: String,
    #[serde(default)]
    current: bool,
}

impl Token {
    fn new(id: u64, name: &str, token: &str, current: bool) -> Self {
        Token {
            id,
            name: name.to_string(),
            token: token.to_string(),
            current,
        }
    }
}

type Tokens = Vec<Token>;

struct Auth {
    path: PathBuf,
    tokens: Tokens,
}

impl Auth {
    fn new(root: &Path) -> Result<Self> {
        let mut tokens = vec![];
        let path = root.join("tokens.json");
        if path.exists() {
            tokens = serde_json::from_reader(File::open(&path)?)?;
        }
        Ok(Auth { path, tokens })
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
            serde_json::to_writer(&mut f, &self.tokens)?;
            f.sync_all()?;
        }
        std::fs::rename(temp, &self.path)?;
        Ok(())
    }
}

pub fn show() -> Result<()> {
    let base = setup_dir()?;
    let auth = Auth::new(&base)?;
    println!("Tokens:");
    for (i, token) in auth.tokens.iter().enumerate() {
        let cur = if token.current { '*' } else { ' ' };
        println!("{cur}{i}: {} {}", token.id, token.name);
    }
    Ok(())
}

pub fn add(token: String) -> Result<()> {
    let base = setup_dir()?;
    let base_url = "https://adventofcode.com/";
    _add(&base, base_url, &token)
}

fn get_id_name(resp: &str) -> Result<(u64, String)> {
    let mut id = 0;
    let mut found_anon = false;
    for cap in SPAN_RE.captures_iter(resp) {
        let text = cap.get(1).unwrap().as_str();
        if !found_anon {
            if let Some(cap) = ANON_RE.captures(text) {
                id = cap.get(1).unwrap().as_str().parse()?;
                found_anon = true;
            }
        } else {
            return Ok((id, text.to_string()));
        }
    }
    bail!("no spans");
}

fn _add(base: &Path, base_url: &str, token: &str) -> Result<()> {
    let mut auth = Auth::new(base)?;
    let client = client()?;
    let mut url = Url::parse(base_url)?;
    url.set_path("/2015/settings");
    let resp = client
        .get(url)
        .header(COOKIE, format!("session={token}"))
        .send()?
        .error_for_status()
        .context("check token")?
        .text()?;
    let (id, name) = get_id_name(&resp).context("couldn't parse id and name from settings")?;
    let mut changed = false;
    for t in &mut auth.tokens {
        if t.id == id || t.token == token {
            t.id = id;
            t.name = name.to_string();
            t.token = token.to_string();
            changed = true;
            break;
        }
    }
    if !changed {
        let current = auth.tokens.is_empty();
        auth.tokens.push(Token::new(id, &name, token, current));
    }
    auth.write()?;
    Ok(())
}

pub fn remove(idx: usize) -> Result<()> {
    let base = setup_dir()?;
    let mut auth = Auth::new(&base)?;
    if idx >= auth.tokens.len() {
        bail!("idx out of range");
    }
    auth.tokens.remove(idx);
    auth.write()?;
    Ok(())
}

pub fn switch(idx: usize) -> Result<()> {
    let base = setup_dir()?;
    let mut auth = Auth::new(&base)?;
    if idx >= auth.tokens.len() {
        bail!("idx out of range");
    }
    for (i, t) in auth.tokens.iter_mut().enumerate() {
        t.current = i == idx;
    }
    auth.write()?;
    Ok(())
}

pub fn current_token() -> Result<Token> {
    let base = setup_dir()?;
    let auth = Auth::new(&base)?;
    auth.tokens
        .into_iter()
        .find(|t| t.current)
        .ok_or_else(|| anyhow!("no current token"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_fs::fixture::{FileWriteStr, PathChild};
    use assert_fs::TempDir;

    fn tokens_mock() -> Result<TempDir> {
        let base = TempDir::new()?;
        base.child("tokens.json")
            .write_str(r#"[{"id": 123456, "name": "Vasily", "token": "F000", "current": true}]"#)?;
        Ok(base)
    }

    #[test]
    fn read_no_file() -> Result<()> {
        let base = TempDir::new()?;
        let auth = Auth::new(&base)?;
        assert!(auth.tokens.is_empty());
        Ok(())
    }

    #[test]
    fn read() -> Result<()> {
        let base = tokens_mock()?;
        let auth = Auth::new(&base)?;
        assert_eq!(
            auth.tokens,
            vec![Token::new(123456, "Vasily", "F000", true)]
        );
        Ok(())
    }

    #[test]
    fn add() -> Result<()> {
        let base = TempDir::new()?;
        let mut server = mockito::Server::new();
        let base_url = &server.url();
        let _m = server
            .mock("GET", "/2015/settings")
            .with_status(200)
            .with_body(include_bytes!("../fixtures/settings_good.html"))
            .create();
        _add(&base, base_url, "F111")?;
        {
            let auth = Auth::new(&base)?;
            assert_eq!(
                auth.tokens,
                vec![Token::new(2279184, "John Kent", "F111", true)]
            );
        }
        let _m = server
            .mock("GET", "/2015/settings")
            .with_status(200)
            .with_body(include_bytes!("../fixtures/settings_good_new_name.html"))
            .create();
        _add(&base, base_url, "F222")?;
        {
            let auth = Auth::new(&base)?;
            assert_eq!(
                auth.tokens,
                vec![Token::new(2279184, "Schmon Fent", "F222", true)]
            );
        }
        Ok(())
    }
}
