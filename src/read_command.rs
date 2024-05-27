use std::process::exit;

use anyhow::{ensure, Result};

use crate::webhook::WebhookParser;

pub async fn parse_webhook(parser: WebhookParser) -> Result<(String, String, u64, Vec<String>)> {
    // only continue if comment was not deleted
    let action = parser.action().await;
    if action.eq("deleted") {
        println!("Exiting: comment deleted");
        exit(0);
    }

    // if comment has no command exit
    let body: String = parser.comment().await;
    let commands: Vec<String> = body
        .split("\\r\\n")
        .filter(|line| line.starts_with("@bot "))
        .flat_map(|s| -> Vec<String> {
            s.to_string()
                .replace("@bot ", "")
                .split(' ')
                .map(|s| s.to_string())
                .collect()
        })
        .collect();
    if commands.is_empty() {
        println!("Exiting: no command found: {body:?}");
        exit(0);
    } else {
        println!("Found commands: {commands:?}");
    }

    // ensure author of command has sufficient rights
    let rights = parser.author_assosiation().await;
    ensure!(
        rights.eq("OWNER") || rights.eq("COLLABORATOR"),
        "Exiting: Commenter does not have sufficient rights: {}",
        rights
    );

    // get issue and repo properties
    let issue_id = parser.issue_number().await?;

    let (owner, repo) = parser.repository().await?;

    Ok((owner, repo, issue_id, commands))
}
