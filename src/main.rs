mod cli;
mod github;

use anyhow::{Result, bail};
use clap::Parser;
use cli::{Cli, ClientSelection, Command, ValidatorCommand};
use github::{AGAVE, FIREDANCER, GitHubClient, Release, Repository};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Versions {
            client,
            stable_only,
        } => list_versions(client, stable_only),
        Command::Daemon => bail!("the daemon command is scaffolded but not implemented yet"),
        Command::Validator(ValidatorCommand::Start { .. }) => {
            bail!("the validator start command is scaffolded but not implemented yet")
        }
        Command::Switch { .. } => {
            bail!("the switch command is scaffolded but not implemented yet")
        }
        Command::Credentials => {
            bail!("the credentials command is scaffolded but not implemented yet")
        }
    }
}

fn list_versions(selection: ClientSelection, stable_only: bool) -> Result<()> {
    let repositories: &[Repository] = match selection {
        ClientSelection::All => &[AGAVE, FIREDANCER],
        ClientSelection::Agave => &[AGAVE],
        ClientSelection::Firedancer => &[FIREDANCER],
    };
    let github = GitHubClient::from_env()?;

    for (index, repository) in repositories.iter().enumerate() {
        let releases = github.releases(*repository)?;
        let releases = releases
            .iter()
            .filter(|release| !stable_only || !release.prerelease)
            .collect::<Vec<_>>();

        if index > 0 {
            println!();
        }
        print_releases(*repository, &releases);
    }

    Ok(())
}

fn print_releases(repository: Repository, releases: &[&Release]) {
    println!(
        "{} releases ({}/{}) — {} found",
        repository.display_name,
        repository.owner,
        repository.name,
        releases.len()
    );
    println!("{:<24} {:<12} NAME", "VERSION", "PUBLISHED");

    for release in releases {
        let published = release.published_at.as_deref().unwrap_or("unpublished");
        let published = published.get(..10).unwrap_or(published);
        let prerelease = if release.prerelease {
            " [prerelease]"
        } else {
            ""
        };
        let name = release
            .name
            .as_deref()
            .map(str::trim)
            .filter(|name| !name.is_empty())
            .unwrap_or(&release.tag_name);

        println!(
            "{:<24} {:<12} {}{}\n    {}",
            release.tag_name, published, name, prerelease, release.html_url
        );
    }
}
