use anyhow::{bail, Context, Result};
use clap::{Args, Parser, Subcommand};
use itertools::Itertools;
use std::os::unix::net::UnixListener;
use std::path::Path;
use uds::UnixListenerExt;
use xaoc::{auth, puzzle, runner};

#[derive(Parser, Debug)]
struct Cli {
    #[arg(long)]
    path: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Prepare {
        day: u16,
    },
    Auth(Auth),
    Submit {
        day: u16,
        part: u16,
        answer: String,
    },
    Map {
        answer: String,
    },
    Unmap,
    Run {
        #[clap(long)]
        debug: bool,
    },
    FixDeps,
    FixUse,
}

#[derive(Args, Debug)]
struct Auth {
    #[command(subcommand)]
    command: AuthCommands,
}

#[derive(Subcommand, Debug)]
enum AuthCommands {
    #[clap(aliases = &["list", "ls"])]
    Show,
    #[clap(aliases = &["a"])]
    Add { token: String },
    #[clap(aliases = &["rm", "r", "delete", "del"])]
    Remove { idx: usize },
    #[clap(aliases = &["sw"])]
    Switch { idx: usize },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let _g = lock().context("acquire lock")?;

    if let Some(path) = cli.path {
        std::env::set_current_dir(path)?;
    }

    match cli.command {
        Commands::Prepare { day } => runner::prepare(xaoc::Day(day))?,
        Commands::Auth(auth) => match auth.command {
            AuthCommands::Show => auth::show()?,
            AuthCommands::Add { token } => {
                auth::add(token)?;
                auth::show()?;
            }
            AuthCommands::Remove { idx } => {
                auth::remove(idx)?;
                auth::show()?;
            }
            AuthCommands::Switch { idx } => {
                auth::switch(idx)?;
                auth::show()?;
            }
        },
        Commands::Submit { day, part, answer } => {
            puzzle::submit(day, part, &answer)?;
        }
        Commands::Run { debug } => {
            runner::run_all(debug)?;
        }
        Commands::Map { answer } => puzzle::map(&answer)?,
        Commands::Unmap => puzzle::unmap()?,
        Commands::FixUse => fix_use()?,
        Commands::FixDeps => fix_deps()?,
    }
    Ok(())
}

fn lock() -> Result<UnixListener> {
    let addr =
        uds::UnixSocketAddr::from_abstract(format!("{}_xaoc_lock", whoami::username()).as_bytes())?;
    let g = UnixListener::bind_unix_addr(&addr)?;
    Ok(g)
}

fn check_dir() -> Result<()> {
    let base = std::env::current_dir()?;
    if !base.ends_with("projects/aoc") {
        bail!("run me in ~/projects/aoc");
    }
    Ok(())
}

fn fix_use_one(path: &Path) -> Result<()> {
    let mut xaoc = None;
    let mut lines = vec![];
    let contents = std::fs::read_to_string(path)?;
    for line in contents.lines() {
        if line.starts_with("xaoc::xaoc!(") {
            assert!(xaoc.is_none());
            xaoc = Some(line);
            continue;
        }
        lines.push(line);
    }
    while lines.first() == Some(&"") {
        lines.remove(0);
    }
    while lines.last() == Some(&"") {
        lines.pop();
    }
    lines.push("");
    lines.push(xaoc.unwrap());
    lines.push("");
    std::fs::write(path, lines.into_iter().join("\n"))?;
    Ok(())
}

fn fix_use() -> Result<()> {
    check_dir()?;
    let paths = glob::glob("aoc20??/src/bin/20??_*.rs")?.collect::<Result<Vec<_>, _>>()?;
    for path in paths {
        eprintln!("fixing {path:?}");
        fix_use_one(&path)?;
    }
    Ok(())
}

fn fix_deps() -> Result<()> {
    check_dir()?;
    let paths = glob::glob("aoc20??/Cargo.toml")?.collect::<Result<Vec<_>, _>>()?;
    let mut deps = toml::value::Table::new();
    for path in &paths {
        let cargo = std::fs::read_to_string(path)?;
        let cargo: toml::Value = toml::from_str(&cargo)?;
        deps.extend(
            cargo
                .get("dependencies")
                .unwrap()
                .as_table()
                .unwrap()
                .clone(),
        );
    }
    for path in paths {
        let cargo = std::fs::read_to_string(&path)?;
        let mut cargo: toml::Value = toml::from_str(&cargo)?;
        cargo
            .as_table_mut()
            .unwrap()
            .insert("dependencies".to_owned(), toml::Value::Table(deps.clone()));
        std::fs::write(path, toml::to_string(&cargo)?)?;
    }
    Ok(())
}
