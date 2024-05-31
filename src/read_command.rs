use std::process::exit;

use anyhow::{ensure, Result};

use crate::webhook::WebhookParser;

pub async fn parse_webhook(
    parser: WebhookParser,
) -> Result<(String, String, u64, String, Vec<String>)> {
    // only continue if comment was not deleted
    let action = parser.action().await;
    if action.eq("deleted") {
        println!("Exiting: comment deleted");
        exit(0);
    }

    // if comment has no command exit
    let commands = parser.extract_commands().await;
    if commands.is_empty() {
        println!("Exiting: no command found");
        exit(0);
    } else {
        println!("Found commands: {commands:?}");
    }

    // ensure author of command has sufficient rights
    let rights = parser.author_association().await;
    ensure!(
        rights.eq("OWNER") || rights.eq("COLLABORATOR"),
        "Exiting: Commenter does not have sufficient rights: {}",
        rights
    );

    // get issue and repo properties
    let issue_id = parser.issue_number().await?;

    let comment_id = parser.comment_id().await;

    let (owner, repo) = parser.repository().await?;

    Ok((owner, repo, issue_id, comment_id, commands))
}
