use anyhow::{Context as _, Result};
use octocrab::{
    commits::PullRequestTarget,
    models::{issues::Comment, pulls::PullRequest, IssueState},
    Octocrab,
};

pub fn init(owner: String, repo: String, api_token: String) -> Result<Api> {
    let octocrab = Octocrab::builder()
        .personal_token(api_token)
        .build()
        .context("failed to create API client")?;
    Ok(Api {
        octocrab,
        owner,
        repo,
    })
}

pub struct Api {
    octocrab: Octocrab,
    owner: String,
    repo: String,
}

impl Api {
    pub async fn find_pull_request(&self, commit: &str) -> Result<PullRequest> {
        let mut pull_requests = self
            .octocrab
            .commits(self.owner.clone(), self.repo.clone())
            .associated_pull_requests(PullRequestTarget::Sha(commit.to_owned()))
            .send()
            .await
            .context("failed to fetch matching pull requests")?
            .items;
        pull_requests.retain(|pr| pr.state == Some(IssueState::Open));
        anyhow::ensure!(
            pull_requests.len() <= 1,
            "multiple open pull request found for commit {} in repository {}/{}",
            commit,
            self.owner,
            self.repo,
        );
        pull_requests.pop().with_context(|| {
            format!(
                "no open pull request found for commit {} in repository {}/{}",
                commit, self.owner, self.repo
            )
        })
    }

    pub async fn find_comment(
        &self,
        pull_request: &PullRequest,
        filter: &str,
    ) -> Result<Option<Comment>> {
        let mut comments = self
            .octocrab
            .issues(self.owner.clone(), self.repo.clone())
            .list_comments(pull_request.number)
            .send()
            .await
            .context("failed to fetch pull request comments")?
            .items;
        comments.retain(|comment| {
            comment
                .body
                .as_ref()
                .map(|body| body.starts_with(filter))
                .unwrap_or_default()
        });
        anyhow::ensure!(
            comments.len() <= 1,
            "multiple match comments found for pull request #{} in repository {}/{}",
            pull_request.number,
            self.owner,
            self.repo,
        );
        Ok(comments.pop())
    }

    pub async fn create_comment(&self, pull_request: &PullRequest, body: &str) -> Result<Comment> {
        self.octocrab
            .issues(self.owner.clone(), self.repo.clone())
            .create_comment(pull_request.number, body)
            .await
            .with_context(|| {
                format!(
                    "failed to add comment to pull request #{} in repo {}/{}",
                    pull_request.number, self.owner, self.repo
                )
            })
    }

    pub async fn update_comment(&self, comment: &Comment, body: &str) -> Result<()> {
        self.octocrab
            .issues(self.owner.clone(), self.repo.clone())
            .update_comment(comment.id, body)
            .await
            .with_context(|| {
                format!(
                    "failed to update comment #{} in repo {}/{}",
                    comment.id, self.owner, self.repo
                )
            })?;
        Ok(())
    }
}
