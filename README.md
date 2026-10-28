# nodelana

`nodelana` is a Rust CLI for managing Agave and Firedancer validator versions.

The first implemented command lists every non-draft GitHub release from:

- [`anza-xyz/agave`](https://github.com/anza-xyz/agave/releases)
- [`firedancer-io/firedancer`](https://github.com/firedancer-io/firedancer/releases)

The daemon, validator startup, version switching, and credential inspection commands are currently command-line scaffolds only.

## Build

```sh
cargo build
```

## Commands

```sh
# Show the full command tree
cargo run -- --help

# List all Agave and Firedancer releases
cargo run -- versions

# List only stable Agave releases
cargo run -- versions --client agave --stable-only

# Inspect planned command options
cargo run -- daemon --help
cargo run -- validator start --help
cargo run -- switch --help
cargo run -- credentials --help
```

`GITHUB_TOKEN` is optional but recommended to increase the GitHub API rate limit. `GITHUB_BASE_URL` can override `https://api.github.com`, which is useful for testing or GitHub Enterprise.

The release fetcher follows the pattern in `../changelog-fetch`: GitHub's REST releases endpoint, API version `2022-11-28`, 100 results per page, and pagination until all releases are loaded.
