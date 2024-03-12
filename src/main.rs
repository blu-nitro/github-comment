mod api;
mod args;

use std::{env, fs};

use anyhow::{Context as _, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let args = args::parse();
    let api_token = env::var("GITHUB_COMMENT_TOKEN")
        .context("GITHUB_COMMENT_TOKEN environment variable must contain a GitHub API token")?;
    let text = fs::read_to_string(&args.text).with_context(|| {
        format!(
            "failed to read comment text from file '{}'",
            args.text.display()
        )
    })?;
    let api = api::init(&args, api_token)?;
    let pull_request = api.find_pull_request(&args.commit).await?;
    println!("Found pull request #{}", pull_request.number);

    let tag = format!("<!-- github-comment: {} -->", args.id);
    let body = format!("{}\n{}", tag, text);
    if let Some(comment) = api.find_comment(&pull_request, &tag).await? {
        println!("Found existing comment #{}", comment.id);
        if Some(&body) == comment.body.as_ref() {
            println!("Comment is up to date, skipping.");
        } else {
            println!("Updating comment ...");
            api.update_comment(&comment, &body).await?;
            println!("Comment #{} updated.", comment.id);
        }
    } else {
        println!("Creating comment ...");
        let comment = api.create_comment(&pull_request, &body).await?;
        println!("Comment #{} created.", comment.id);
    }
    Ok(())
}
