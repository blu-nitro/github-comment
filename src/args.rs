use std::path::PathBuf;

use clap::Parser;

pub fn parse() -> Args {
    Args::parse()
}

/// Post or update a comment to a GitHub PR.
///
/// This command tries to find a pull request that wants to merge the given commit. Then it checks
/// if there is a comment that is tagged with the given ID. If yes, the comment is updated with the
/// provided text. Otherwise, a new comment is created.
///
/// The GITHUB_COMMENT_TOKEN environment variable must be set to a token with the write permission
/// for issues or pull requests on the target repository.
#[derive(Parser)]
pub struct Args {
    /// The owner of the repository to post the comment to.
    #[clap(long)]
    pub owner: String,

    /// The repository to post the comment to.
    #[clap(long)]
    pub repo: String,

    /// The commit that should be used to identify the PR.
    #[clap(long)]
    pub commit: String,

    /// The ID of the comment that is embedded into the comment body.
    #[clap(long)]
    pub id: String,

    /// A file containing the text of the comment.
    pub text: PathBuf,
}
