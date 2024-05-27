mod api;
mod args;

extern crate reqwest;

use std::{env, fs};

use anyhow::{ensure, Context, Result};
use args::{ReadCommandArgs, WriteArgs};
use serde_json::Value;

use crate::args::Subcommands;

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
    let text = fs::read_to_string(&args.webhook).with_context(|| {
        format!(
            "Error: Failed to read issue_comment data from file '{}'",
            args.webhook.display()
        )
    })?;

    let json_object: &Value = &serde_json::from_str(&text)
        .with_context(|| "Error: Failed to read json data from issue_comment data")?;

    let payload = &json_object["payload"];

    // only continue if comment was not deleted
    let action = payload["action"].to_string().replace('"', "");
    if action.eq("deleted") {
        println!("Exiting: comment deleted");
        return Ok(());
    }

    // if comment has no command exit
    let comment: &Value = &payload["comment"];
    let body: String = comment["body"].to_string().replace('"', "");
    let commands: Vec<&str> = body
        .split("\\r\\n")
        .filter(|line| line.starts_with('!'))
        .collect();
    if commands.is_empty() {
        println!("Exiting: no command found: {body:?}");
        return Ok(());
    } else {
        println!("Found commands: {commands:?}");
    }

    // ensure author of command has sufficient rights
    let rights: String = comment["author_association"].to_string().replace('"', "");
    ensure!(
        rights.eq("OWNER") || rights.eq("COLLABORATOR"),
        "Exiting: Commenter does not have sufficient rights: {}",
        rights
    );

    // get issue and repo properties
    let issue = &payload["issue"];
    let issue_id = issue["number"]
        .as_u64()
        .context("Error: unpacking issue id: not a number")?;

    let repo = &payload["repository"];
    let full_name = repo["full_name"].to_string().replace('"', "");
    let mut full_name = full_name.split('/');

    let owner = full_name.next().context("Error: unpacking repo owner")?;
    let repo = full_name.next().context("Error: unpacking repo name")?;

    // get pull request and assosiated branch
    let api = api::GitHubApi::init(owner, repo, api_token)?;

    let pull_request = api.get_pull_request_by_id(issue_id).await?;
    let branch = pull_request.head.ref_field;
    println!("Found PR branch: {branch}");

    let glapi = api::GitLabAPI::init(&args.job_token, &args.gl_instance, owner, repo);
    for command in commands {
        println!("Triggering pipeline with command {}", command);
        let res = glapi
            .trigger_pipeline_with_command(&branch, command)
            .await?;
        println!("Pipeline response for command {}: \n{}", command, res);
    }

    Ok(())
}
