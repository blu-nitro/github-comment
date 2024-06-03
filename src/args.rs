use std::path::PathBuf;

use clap::{Args as Cargs, Parser, Subcommand};

pub fn parse() -> Args {
    Args::parse()
}

/// Interact with GitHub PR comments
#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: Subcommands,
}

#[derive(Cargs)]
pub struct WriteCommentArgs {
    /// The owner of the repository to post the comment to.
    #[arg(long)]
    pub owner: String,

    /// The repository to post the comment to.
    #[arg(long)]
    pub repo: String,

    /// The commit that should be used to identify the PR.
    #[arg(long)]
    pub commit: String,

    /// The ID of the comment that is embedded into the comment body.
    #[arg(long)]
    pub id: String,

    /// A file containing the text of the comment.
    pub text: PathBuf,
}

#[derive(Cargs)]
pub struct TriggerWebhookCommandsArgs {
    /// A json file containing the webhook contents
    #[arg(long)]
    pub webhook: PathBuf,

    /// The GitLab instance the commands are to be executed on
    #[arg(long)]
    pub gl_instance: String,

    /// Your GitLab job token
    #[arg(long)]
    pub job_token: String,
}

#[derive(Subcommand)]
pub enum Subcommands {
    /// Post or update a comment to a GitHub PR.
    ///
    /// This command tries to find a pull request that wants to merge the given commit. Then it checks
    /// if there is a comment that is tagged with the given ID. If yes, the comment is updated with the
    /// provided text. Otherwise, a new comment is created.
    ///
    /// The GITHUB_COMMENT_TOKEN environment variable must be set to a token with the write permission
    /// for issues or pull requests on the target repository.
    WriteComment(WriteCommentArgs),
    /// Reads commands from github issue_comment webhook and triggers gitlab pipelines.
    /// Command and comment_id are provided in the $COMMAND and $COMMENT_ID CI variables.
    ///
    /// The GITHUB_COMMENT_TOKEN environment variable must be set to a token with the read permission
    /// for issues or pull requests on the target repository.
    TriggerWebhookCommands(TriggerWebhookCommandsArgs),
}
