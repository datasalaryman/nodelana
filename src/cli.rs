use clap::{Parser, Subcommand, ValueEnum};

/// Manage Agave and Firedancer validator installations.
#[derive(Debug, Parser)]
#[command(name = "nodelana", version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// List releases published by the Agave and Firedancer GitHub repositories.
    Versions {
        /// Only show releases for one validator client.
        #[arg(long, value_enum, default_value_t = ClientSelection::All)]
        client: ClientSelection,

        /// Exclude GitHub releases marked as prereleases.
        #[arg(long)]
        stable_only: bool,
    },

    /// Run the release-monitoring daemon (planned; polls every 5 seconds).
    Daemon,

    /// Manage a validator process.
    #[command(subcommand)]
    Validator(ValidatorCommand),

    /// Seamlessly switch the validator client or installed version (planned).
    Switch {
        /// Validator client to switch to.
        #[arg(value_enum)]
        client: ValidatorClient,

        /// Release tag to activate; the latest release will be used when omitted.
        #[arg(long)]
        version: Option<String>,
    },

    /// Show validator identity, vote account, and authorized-withdrawer information (planned).
    Credentials,
}

#[derive(Debug, Subcommand)]
pub enum ValidatorCommand {
    /// Start a validator instance (planned).
    Start {
        /// Validator implementation to start.
        #[arg(long, value_enum)]
        client: ValidatorClient,

        /// Release tag to run; the active version will be used when omitted.
        #[arg(long)]
        version: Option<String>,
    },
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, ValueEnum)]
pub enum ClientSelection {
    #[default]
    All,
    Agave,
    Firedancer,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum ValidatorClient {
    Agave,
    Firedancer,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn command_tree_is_valid() {
        Cli::command().debug_assert();
    }
}
