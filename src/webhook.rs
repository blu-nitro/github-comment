use std::{fs, path::PathBuf};

use anyhow::Context;
use anyhow::Result;
use serde_json::Value;

pub struct WebhookParser {
    json_object: Value,
}

impl WebhookParser {
    pub fn init(file: &PathBuf) -> Result<Self> {
        let text = fs::read_to_string(file).with_context(|| {
            format!(
                "Error: Failed to read issue_comment data from file '{}'",
                &file.display()
            )
        })?;
        let json_object: Value = serde_json::from_str(&text)
            .with_context(|| "Error: Failed to read json data from issue_comment data")?;
        // let json_object = json_object["payload"].clone();
        Ok(Self { json_object })
    }

    pub async fn action(&self) -> String {
        self.json_object["action"].to_string().replace('"', "")
    }

    pub async fn comment(&self) -> String {
        self.json_object["comment"]["body"]
            .to_string()
            .replace('"', "")
    }

    pub async fn author_assosiation(&self) -> String {
        self.json_object["comment"]["author_association"]
            .to_string()
            .replace('"', "")
    }

    pub async fn issue_number(&self) -> Result<u64> {
        self.json_object["issue"]["number"]
            .as_u64()
            .context("Error: unpacking issue id: not a number")
    }

    pub async fn repository(&self) -> Result<(String, String)> {
        let full_name = self.json_object["repository"]["full_name"]
            .to_string()
            .replace('"', "");
        let mut full_name = full_name.split('/');

        let owner = full_name.next().context("Error: unpacking repo owner")?;
        let repo = full_name.next().context("Error: unpacking repo name")?;
        Ok((owner.to_string(), repo.to_string()))
    }
}
