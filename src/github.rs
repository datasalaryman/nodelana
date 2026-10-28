use std::env;

use anyhow::{Context, Result, bail};
use reqwest::blocking::Client;
use reqwest::header::{ACCEPT, AUTHORIZATION, HeaderMap, HeaderValue};
use serde::Deserialize;

const DEFAULT_BASE_URL: &str = "https://api.github.com";
const API_VERSION: &str = "2022-11-28";
const PER_PAGE: usize = 100;

#[derive(Clone, Copy, Debug)]
pub struct Repository {
    pub display_name: &'static str,
    pub owner: &'static str,
    pub name: &'static str,
}

pub const AGAVE: Repository = Repository {
    display_name: "Agave",
    owner: "anza-xyz",
    name: "agave",
};

pub const FIREDANCER: Repository = Repository {
    display_name: "Firedancer",
    owner: "firedancer-io",
    name: "firedancer",
};

#[derive(Debug, Deserialize)]
pub struct Release {
    pub tag_name: String,
    pub name: Option<String>,
    pub html_url: String,
    pub draft: bool,
    pub prerelease: bool,
    pub published_at: Option<String>,
}

pub struct GitHubClient {
    client: Client,
    base_url: String,
}

impl GitHubClient {
    pub fn from_env() -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(
            ACCEPT,
            HeaderValue::from_static("application/vnd.github+json"),
        );
        headers.insert(
            "X-GitHub-Api-Version",
            HeaderValue::from_static(API_VERSION),
        );

        if let Ok(token) = env::var("GITHUB_TOKEN") {
            let value = HeaderValue::from_str(&format!("Bearer {token}"))
                .context("GITHUB_TOKEN cannot be represented as an HTTP header")?;
            headers.insert(AUTHORIZATION, value);
        }

        let client = Client::builder()
            .user_agent(concat!("nodelana/", env!("CARGO_PKG_VERSION")))
            .default_headers(headers)
            .build()
            .context("failed to create the GitHub HTTP client")?;

        let base_url = env::var("GITHUB_BASE_URL")
            .unwrap_or_else(|_| DEFAULT_BASE_URL.to_owned())
            .trim_end_matches('/')
            .to_owned();

        Ok(Self { client, base_url })
    }

    /// Fetch every release page, matching the GitHub REST approach in changelog-fetch.
    pub fn releases(&self, repository: Repository) -> Result<Vec<Release>> {
        let mut releases = Vec::new();
        let mut page = 1;

        loop {
            let url = format!(
                "{}/repos/{}/{}/releases?per_page={PER_PAGE}&page={page}",
                self.base_url, repository.owner, repository.name
            );
            let response = self.client.get(&url).send().with_context(|| {
                format!(
                    "failed to fetch {}/{} releases",
                    repository.owner, repository.name
                )
            })?;

            if !response.status().is_success() {
                let status = response.status();
                let remaining = response
                    .headers()
                    .get("x-ratelimit-remaining")
                    .and_then(|value| value.to_str().ok())
                    .unwrap_or("unknown")
                    .to_owned();
                let body = response.text().unwrap_or_default();
                let body = body.chars().take(500).collect::<String>();
                bail!(
                    "GitHub API returned {status} for {}/{} (rate limit remaining: {remaining}): {body}",
                    repository.owner,
                    repository.name
                );
            }

            let mut page_items: Vec<Release> = response.json().with_context(|| {
                format!(
                    "GitHub returned invalid release data for {}/{}",
                    repository.owner, repository.name
                )
            })?;
            let page_len = page_items.len();
            releases.append(&mut page_items);

            if page_len < PER_PAGE {
                break;
            }
            page += 1;
        }

        releases.retain(|release| !release.draft);
        Ok(releases)
    }
}
