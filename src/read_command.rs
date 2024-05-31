use crate::webhook::Webhook;

pub fn extract_commands(webhook: &Webhook) -> Vec<String> {
    // only continue if comment was not deleted
    if webhook.action.eq("deleted") {
        println!("Exiting: comment deleted");
        return vec![];
    }

    // ensure author of command has sufficient rights
    let rights = &webhook.author_association;
    if !(rights.eq("OWNER") || rights.eq("COLLABORATOR")) {
        println!("permission denied: {}", rights);
        return vec![];
    }

    parse_commands(&webhook.comment)
}

fn parse_commands(text: &str) -> Vec<String> {
    text.lines()
        .filter(|line| line.starts_with("@bot "))
        .flat_map(|s| -> Vec<String> {
            s.to_string()
                .replace("@bot ", "")
                .split(' ')
                .map(|s| s.to_string())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::parse_commands;
    #[test]
    fn test_parse_commands() {
        assert_eq!(
            parse_commands("@bot test_command test2\r\n@bot command2 3\r\nthis is a reponame"),
            vec![
                "test_command".to_string(),
                "test2".to_string(),
                "command2".to_string(),
                "3".to_string()
            ]
        );
    }
}
