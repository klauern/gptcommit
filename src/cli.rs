use crate::actions::{config::ConfigArgs, prepare_commit_msg::PrepareCommitMsgArgs};
use clap::{Parser, Subcommand};

/// Represents the main command-line interface for the application.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub(crate) struct GptcommitCLI {
    /// The action to perform (subcommand).
    #[command(subcommand)]
    pub action: Action,
    /// Enable verbose logging.
    #[arg(short, long, global = true)]
    pub verbose: bool,
}

/// Actions the application can perform.
#[derive(Subcommand, Debug)]
pub(crate) enum Action {
    /// Install the git hook
    Install,
    /// Uninstall the git hook
    Uninstall,
    /// Read and modify settings
    Config(ConfigArgs),
    /// Set a custom prompt
    SetPrompt { prompt: String },
    /// Run on the prepare-commit-msg hook
    PrepareCommitMsg(PrepareCommitMsgArgs),
}

#[derive(Subcommand, Debug)]
pub(crate) enum ConfigAction {
    /// Show the current configuration
    Show,
    /// Set a configuration value
    Set { key: String, value: String },
    /// Delete a configuration value
    Delete { key: String },
    /// Set a custom prompt
    SetPrompt { prompt: String },
}
