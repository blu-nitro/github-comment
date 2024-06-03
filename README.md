# nitrokey-ci

CI tooling for the Nitrokey repositories.

```
$ nitrokey-ci --help
Post or update a comment to a GitHub PR.

This command tries to find a pull request that wants to merge the given commit.
Then it checks if there is a comment that is tagged with the given ID. If yes,
the comment is updated with the provided text. Otherwise, a new comment is
created.

The GITHUB_COMMENT_TOKEN environment variable must be set to a token with the
write permission for issues or pull requests on the target repository.

Usage: nitrokey-ci --owner <OWNER> --repo <REPO> --commit <COMMIT> --id <ID> <TEXT>

Arguments:
  <TEXT>
          A file containing the text of the comment

Options:
      --owner <OWNER>
          The owner of the repository to post the comment to

      --repo <REPO>
          The repository to post the comment to

      --commit <COMMIT>
          The commit that should be used to identify the PR

      --id <ID>
          The ID of the comment that is embedded into the comment body

  -h, --help
          Print help (see a summary with '-h')
```
