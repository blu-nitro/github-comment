mod api;
mod args;
mod read_command;
mod webhook;

extern crate reqwest;

use std::{env, fs};

use anyhow::{Context, Result};
use args::{ReadCommandArgs, WriteArgs};

use crate::{args::Subcommands, webhook::WebhookParser};

#[tokio::main]
async fn main() -> Result<()> {
    let args = args::parse();
    let api_token = env::var("GITHUB_COMMENT_TOKEN")
        .context("GITHUB_COMMENT_TOKEN environment variable must contain a GitHub API token")?;
    match args.command {
        Subcommands::Write(args) => write(&args, api_token).await?,
        Subcommands::ReadCommand(args) => read(&args, api_token).await?,
    }
    Ok(())
}

async fn write(args: &WriteArgs, api_token: String) -> Result<()> {
    let text = fs::read_to_string(&args.text).with_context(|| {
        format!(
            "failed to read comment text from file '{}'",
            args.text.display()
        )
    })?;
    let api = api::GitHubApi::init(&args.owner, &args.repo, api_token)?;
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

async fn read(args: &ReadCommandArgs, api_token: String) -> Result<()> {
    let parser = WebhookParser::init(&args.webhook)?;

    let (owner, repo, issue_id, commands) = read_command::parse_webhook(parser).await?;

    // get pull request and assosiated branch
    let api = api::GitHubApi::init(&owner, &repo, api_token)?;

    let pull_request = api.get_pull_request_by_id(issue_id).await?;
    let branch = pull_request.head.ref_field;
    println!("Found PR branch: {branch}");

    let glapi = api::GitLabAPI::init(&args.job_token, &args.gl_instance, &owner, &repo);
    for command in commands {
        println!("Triggering pipeline with command {}", command);
        let res = glapi
            .trigger_pipeline_with_command(&branch, &command)
            .await?;
        println!("Pipeline response for command {}: \n{}", command, res);
    }

    Ok(())
}
