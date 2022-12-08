use anyhow::Result;
use git2::Repository;

fn main() -> Result<()> {
    let repo = Repository::open_from_env()?;
    let config = repo.config()?;
    let email = config.get_string("user.email")?;
    println!("cargo:rustc-env=GIT_EMAIL={}", email);
    Ok(())
}
