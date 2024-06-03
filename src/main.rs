mod api;
mod args;
mod read_command;
mod webhook;

extern crate reqwest;

use std::{env, fs};

use anyhow::{Context, Result};
use args::{TriggerWebhookCommandsArgs, WriteCommentArgs};

use crate::{args::Subcommands, webhook::Webhook};

#[tokio::main]
async fn main() -> Result<()> {
    let args = args::parse();
    let api_token = env::var("GITHUB_COMMENT_TOKEN")
        .context("GITHUB_COMMENT_TOKEN environment variable must contain a GitHub API token")?;
    match args.command {
        Subcommands::WriteComment(args) => write(&args, api_token).await?,
        Subcommands::TriggerWebhookCommands(args) => read(&args, api_token).await?,
    }
    Ok(())
}

async fn write(args: &WriteCommentArgs, api_token: String) -> Result<()> {
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

async fn read(args: &TriggerWebhookCommandsArgs, api_token: String) -> Result<()> {
    let webhook = Webhook::parse(&args.webhook)?;
    let commands = read_command::extract_commands(&webhook);
    if commands.is_empty() {
        println!("No commands found");
        return Ok(());
    }

    // get pull request and assosiated branch
    let api = api::GitHubApi::init(&webhook.repo.owner, &webhook.repo.name, api_token)?;

    let pull_request = api.get_pull_request_by_id(webhook.issue_number).await?;
    let branch = pull_request.head.ref_field;
    println!("Found PR branch: {branch}");

    let glapi = api::GitLabAPI::init(
        &args.job_token,
        &args.gl_instance,
        &webhook.repo.owner,
        &webhook.repo.name,
    );
    for command in commands {
        println!("Triggering pipeline with command {}", command);
        let res = glapi
            .trigger_pipeline_with_command(&branch, &command, &webhook.comment_id)
            .await?;
        println!("Pipeline response for command {}: \n{}", command, res);
    }

    Ok(())
}
