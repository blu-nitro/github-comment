use std::{fs, path::PathBuf};

use anyhow::Context;
use anyhow::Result;
use serde::Deserialize;

#[derive(PartialEq, Debug)]
pub struct Repository {
    pub owner: String,
    pub name: String,
}

pub struct Webhook {
    pub action: String,
    pub comment: String,
    pub comment_id: String,
    pub author_association: String,
    pub issue_number: u64,
    pub repo: Repository,
}

#[derive(Deserialize)]
struct WebhookRaw {
    action: String,
    comment: WebhookComment,
    issue: WebhookIssue,
    repository: WebhookRepository,
}

#[derive(Deserialize)]
struct WebhookComment {
    body: String,
    id: u64,
    author_association: String,
}

#[derive(Deserialize)]
struct WebhookIssue {
    number: u64,
}

#[derive(Deserialize)]
struct WebhookRepository {
    owner: WebhookRepositoryOwner,
    name: String,
}

#[derive(Deserialize)]
struct WebhookRepositoryOwner {
    login: String,
}

impl Webhook {
    pub fn parse(file: &PathBuf) -> Result<Self> {
        let text = fs::read_to_string(file).with_context(|| {
            format!(
                "Error: Failed to read issue_comment data from file '{}'",
                &file.display()
            )
        })?;
        Self::parse_from_str(&text)
    }

    pub fn parse_from_str(text: &str) -> Result<Self> {
        let raw: WebhookRaw = serde_json::from_str(text)
            .with_context(|| "Error: Failed to read json data from issue_comment data")?;
        let action = raw.action;
        let comment = raw.comment.body;
        let comment_id = raw.comment.id.to_string();
        let author_association = raw.comment.author_association;

        let issue_number = raw.issue.number;

        let owner = raw.repository.owner.login;
        let repo = raw.repository.name;
        let repo = Repository { owner, name: repo };

        Ok(Self {
            action,
            comment,
            comment_id,
            author_association,
            issue_number,
            repo,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const WEBHOOK_DATA: &str = r#"
        {
            "action": "created",
            "issue": {
              "url": "https://api.github.com/repos/namespace/reponame/issues/3",
              "repository_url": "https://api.github.com/repos/namespace/reponame",
              "labels_url": "https://api.github.com/repos/namespace/reponame/issues/3/labels{/name}",
              "comments_url": "https://api.github.com/repos/namespace/reponame/issues/3/comments",
              "events_url": "https://api.github.com/repos/namespace/reponame/issues/3/events",
              "html_url": "https://github.com/namespace/reponame/pull/3",
              "id": 2317872330,
              "node_id": "PR_kwDOMA3tsc5wmKOf",
              "number": 3,
              "title": "branch",
              "user": {
                "login": "namespace",
                "id": 156798802,
                "node_id": "U_kgDOCViPUg",
                "avatar_url": "https://avatars.githubusercontent.com/u/123456789?v=4",
                "gravatar_id": "",
                "url": "https://api.github.com/users/namespace",
                "html_url": "https://github.com/namespace",
                "followers_url": "https://api.github.com/users/namespace/followers",
                "following_url": "https://api.github.com/users/namespace/following{/other_user}",
                "gists_url": "https://api.github.com/users/namespace/gists{/gist_id}",
                "starred_url": "https://api.github.com/users/namespace/starred{/owner}{/repo}",
                "subscriptions_url": "https://api.github.com/users/namespace/subscriptions",
                "organizations_url": "https://api.github.com/users/namespace/orgs",
                "repos_url": "https://api.github.com/users/namespace/repos",
                "events_url": "https://api.github.com/users/namespace/events{/privacy}",
                "received_events_url": "https://api.github.com/users/namespace/received_events",
                "type": "User",
                "site_admin": false
              },
              "labels": [],
              "state": "open",
              "locked": false,
              "assignee": null,
              "assignees": [],
              "milestone": null,
              "comments": 2,
              "created_at": "2024-05-26T18:11:07Z",
              "updated_at": "2024-05-27T08:48:08Z",
              "closed_at": null,
              "author_association": "OWNER",
              "active_lock_reason": null,
              "draft": false,
              "pull_request": {
                "url": "https://api.github.com/repos/namespace/reponame/pulls/3",
                "html_url": "https://github.com/namespace/reponame/pull/3",
                "diff_url": "https://github.com/namespace/reponame/pull/3.diff",
                "patch_url": "https://github.com/namespace/reponame/pull/3.patch",
                "merged_at": null
              },
              "body": null,
              "reactions": {
                "url": "https://api.github.com/repos/namespace/reponame/issues/3/reactions",
                "total_count": 0,
                "+1": 0,
                "-1": 0,
                "laugh": 0,
                "hooray": 0,
                "confused": 0,
                "heart": 0,
                "rocket": 0,
                "eyes": 0
              },
              "timeline_url": "https://api.github.com/repos/namespace/reponame/issues/3/timeline",
              "performed_via_github_app": null,
              "state_reason": null
            },
            "comment": {
              "url": "https://api.github.com/repos/namespace/reponame/issues/comments/1111111111",
              "html_url": "https://github.com/namespace/reponame/pull/3#issuecomment-1111111111",
              "issue_url": "https://api.github.com/repos/namespace/reponame/issues/3",
              "id": 4242424242,
              "node_id": "node_id",
              "user": {
                "login": "namespace",
                "id": 111111111,
                "node_id": "U_kgDOCViPUg",
                "avatar_url": "https://avatars.githubusercontent.com/u/111111111?v=4",
                "gravatar_id": "",
                "url": "https://api.github.com/users/namespace",
                "html_url": "https://github.com/namespace",
                "followers_url": "https://api.github.com/users/namespace/followers",
                "following_url": "https://api.github.com/users/namespace/following{/other_user}",
                "gists_url": "https://api.github.com/users/namespace/gists{/gist_id}",
                "starred_url": "https://api.github.com/users/namespace/starred{/owner}{/repo}",
                "subscriptions_url": "https://api.github.com/users/namespace/subscriptions",
                "organizations_url": "https://api.github.com/users/namespace/orgs",
                "repos_url": "https://api.github.com/users/namespace/repos",
                "events_url": "https://api.github.com/users/namespace/events{/privacy}",
                "received_events_url": "https://api.github.com/users/namespace/received_events",
                "type": "User",
                "site_admin": false
              },
              "created_at": "2024-05-27T08:48:06Z",
              "updated_at": "2024-05-27T08:48:06Z",
              "author_association": "OWNER",
              "body": "@bot test_command test2\r\n@bot command2 3\r\nthis is a reponame",
              "reactions": {
                "url": "https://api.github.com/repos/namespace/reponame/issues/comments/1111111111/reactions",
                "total_count": 0,
                "+1": 0,
                "-1": 0,
                "laugh": 0,
                "hooray": 0,
                "confused": 0,
                "heart": 0,
                "rocket": 0,
                "eyes": 0
              },
              "performed_via_github_app": null
            },
            "repository": {
              "id": 111111111,
              "node_id": "node_id",
              "name": "reponame",
              "full_name": "namespace/reponame",
              "private": true,
              "owner": {
                "login": "namespace",
                "id": 111111111,
                "node_id": "node_id",
                "avatar_url": "https://avatars.githubusercontent.com/u/111111111?v=4",
                "gravatar_id": "",
                "url": "https://api.github.com/users/namespace",
                "html_url": "https://github.com/namespace",
                "followers_url": "https://api.github.com/users/namespace/followers",
                "following_url": "https://api.github.com/users/namespace/following{/other_user}",
                "gists_url": "https://api.github.com/users/namespace/gists{/gist_id}",
                "starred_url": "https://api.github.com/users/namespace/starred{/owner}{/repo}",
                "subscriptions_url": "https://api.github.com/users/namespace/subscriptions",
                "organizations_url": "https://api.github.com/users/namespace/orgs",
                "repos_url": "https://api.github.com/users/namespace/repos",
                "events_url": "https://api.github.com/users/namespace/events{/privacy}",
                "received_events_url": "https://api.github.com/users/namespace/received_events",
                "type": "User",
                "site_admin": false
              },
              "html_url": "https://github.com/namespace/reponame",
              "description": null,
              "fork": false,
              "url": "https://api.github.com/repos/namespace/reponame",
              "forks_url": "https://api.github.com/repos/namespace/reponame/forks",
              "keys_url": "https://api.github.com/repos/namespace/reponame/keys{/key_id}",
              "collaborators_url": "https://api.github.com/repos/namespace/reponame/collaborators{/collaborator}",
              "teams_url": "https://api.github.com/repos/namespace/reponame/teams",
              "hooks_url": "https://api.github.com/repos/namespace/reponame/hooks",
              "issue_events_url": "https://api.github.com/repos/namespace/reponame/issues/events{/number}",
              "events_url": "https://api.github.com/repos/namespace/reponame/events",
              "assignees_url": "https://api.github.com/repos/namespace/reponame/assignees{/user}",
              "branches_url": "https://api.github.com/repos/namespace/reponame/branches{/branch}",
              "tags_url": "https://api.github.com/repos/namespace/reponame/tags",
              "blobs_url": "https://api.github.com/repos/namespace/reponame/git/blobs{/sha}",
              "git_tags_url": "https://api.github.com/repos/namespace/reponame/git/tags{/sha}",
              "git_refs_url": "https://api.github.com/repos/namespace/reponame/git/refs{/sha}",
              "trees_url": "https://api.github.com/repos/namespace/reponame/git/trees{/sha}",
              "statuses_url": "https://api.github.com/repos/namespace/reponame/statuses/{sha}",
              "languages_url": "https://api.github.com/repos/namespace/reponame/languages",
              "stargazers_url": "https://api.github.com/repos/namespace/reponame/stargazers",
              "contributors_url": "https://api.github.com/repos/namespace/reponame/contributors",
              "subscribers_url": "https://api.github.com/repos/namespace/reponame/subscribers",
              "subscription_url": "https://api.github.com/repos/namespace/reponame/subscription",
              "commits_url": "https://api.github.com/repos/namespace/reponame/commits{/sha}",
              "git_commits_url": "https://api.github.com/repos/namespace/reponame/git/commits{/sha}",
              "comments_url": "https://api.github.com/repos/namespace/reponame/comments{/number}",
              "issue_comment_url": "https://api.github.com/repos/namespace/reponame/issues/comments{/number}",
              "contents_url": "https://api.github.com/repos/namespace/reponame/contents/{+path}",
              "compare_url": "https://api.github.com/repos/namespace/reponame/compare/{base}...{head}",
              "merges_url": "https://api.github.com/repos/namespace/reponame/merges",
              "archive_url": "https://api.github.com/repos/namespace/reponame/{archive_format}{/ref}",
              "downloads_url": "https://api.github.com/repos/namespace/reponame/downloads",
              "issues_url": "https://api.github.com/repos/namespace/reponame/issues{/number}",
              "pulls_url": "https://api.github.com/repos/namespace/reponame/pulls{/number}",
              "milestones_url": "https://api.github.com/repos/namespace/reponame/milestones{/number}",
              "notifications_url": "https://api.github.com/repos/namespace/reponame/notifications{?since,all,participating}",
              "labels_url": "https://api.github.com/repos/namespace/reponame/labels{/name}",
              "releases_url": "https://api.github.com/repos/namespace/reponame/releases{/id}",
              "deployments_url": "https://api.github.com/repos/namespace/reponame/deployments",
              "created_at": "2024-05-26T17:48:24Z",
              "updated_at": "2024-05-26T17:49:23Z",
              "pushed_at": "2024-05-26T18:11:07Z",
              "git_url": "git://github.com/namespace/reponame.git",
              "ssh_url": "git@github.com:namespace/reponame.git",
              "clone_url": "https://github.com/namespace/reponame.git",
              "svn_url": "https://github.com/namespace/reponame",
              "homepage": null,
              "size": 2,
              "stargazers_count": 0,
              "watchers_count": 0,
              "language": null,
              "has_issues": true,
              "has_projects": true,
              "has_downloads": true,
              "has_wiki": false,
              "has_pages": false,
              "has_discussions": false,
              "forks_count": 0,
              "mirror_url": null,
              "archived": false,
              "disabled": false,
              "open_issues_count": 3,
              "license": null,
              "allow_forking": true,
              "is_template": false,
              "web_commit_signoff_required": false,
              "topics": [],
              "visibility": "private",
              "forks": 0,
              "open_issues": 3,
              "watchers": 0,
              "default_branch": "main"
            },
            "sender": {
              "login": "namespace",
              "id": 111111111,
              "node_id": "node_id",
              "avatar_url": "https://avatars.githubusercontent.com/u/111111111?v=4",
              "gravatar_id": "",
              "url": "https://api.github.com/users/namespace",
              "html_url": "https://github.com/namespace",
              "followers_url": "https://api.github.com/users/namespace/followers",
              "following_url": "https://api.github.com/users/namespace/following{/other_user}",
              "gists_url": "https://api.github.com/users/namespace/gists{/gist_id}",
              "starred_url": "https://api.github.com/users/namespace/starred{/owner}{/repo}",
              "subscriptions_url": "https://api.github.com/users/namespace/subscriptions",
              "organizations_url": "https://api.github.com/users/namespace/orgs",
              "repos_url": "https://api.github.com/users/namespace/repos",
              "events_url": "https://api.github.com/users/namespace/events{/privacy}",
              "received_events_url": "https://api.github.com/users/namespace/received_events",
              "type": "User",
              "site_admin": false
            }
          }"#;

    #[test]
    fn test_parse_webhook() {
        let webhook = Webhook::parse_from_str(WEBHOOK_DATA).unwrap();
        assert_eq!(webhook.action, "created".to_string());
        assert_eq!(webhook.comment_id, "4242424242".to_string());
        assert_eq!(webhook.author_association, "OWNER".to_string());
        assert_eq!(webhook.issue_number, 3);
        assert_eq!(
            webhook.repo,
            Repository {
                owner: "namespace".to_string(),
                name: "reponame".to_string()
            }
        );
    }
}
