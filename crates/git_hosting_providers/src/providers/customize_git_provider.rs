use url::Url;

use git::{BuildCommitPermalinkParams, BuildPermalinkParams, GitHostingProvider, ParsedGitRemote};
use project::project_settings::GitHostProvider;

pub struct CustomizeGitProvider {
    host_provider: GitHostProvider,
}

impl CustomizeGitProvider {
    pub fn new(git_host_provider_settings: &GitHostProvider) -> CustomizeGitProvider {
        CustomizeGitProvider {
            host_provider: git_host_provider_settings.clone(),
        }
    }
}

impl GitHostingProvider for CustomizeGitProvider {
    fn name(&self) -> String {
        "GitLab".to_string()
    }

    fn base_url(&self) -> Url {
        Url::parse(self.host_provider.host.to_owned().unwrap().as_str()).unwrap()
    }

    fn supports_avatars(&self) -> bool {
        false
    }

    fn format_line_number(&self, line: u32) -> String {
        format!("L{line}")
    }

    fn format_line_numbers(&self, start_line: u32, end_line: u32) -> String {
        format!("L{start_line}-{end_line}")
    }

    fn parse_remote_url<'a>(&self, url: &'a str) -> Option<ParsedGitRemote<'a>> {
        if self.host_provider.host.is_none() {
            return None;
        }

        if url.starts_with("git@gitlab.com:") || url.starts_with("https://gitlab.com/") {
            let repo_with_owner = url
                .trim_start_matches("git@gitlab.com:")
                .trim_start_matches("https://gitlab.com/")
                .trim_end_matches(".git");

            let (owner, repo) = repo_with_owner.split_once('/')?;

            return Some(ParsedGitRemote { owner, repo });
        }

        None
    }

    fn build_commit_permalink(
        &self,
        remote: &ParsedGitRemote,
        params: BuildCommitPermalinkParams,
    ) -> Url {
        let BuildCommitPermalinkParams { sha } = params;
        let ParsedGitRemote { owner, repo } = remote;

        self.base_url()
            .join(&format!("{owner}/{repo}/-/commit/{sha}"))
            .unwrap()
    }

    fn build_permalink(&self, remote: ParsedGitRemote, params: BuildPermalinkParams) -> Url {
        let ParsedGitRemote { owner, repo } = remote;
        let BuildPermalinkParams {
            sha,
            path,
            selection,
        } = params;

        let mut permalink = self
            .base_url()
            .join(&format!("{owner}/{repo}/-/blob/{sha}/{path}"))
            .unwrap();
        if path.ends_with(".md") {
            permalink.set_query(Some("plain=1"));
        }
        permalink.set_fragment(
            selection
                .map(|selection| self.line_fragment(&selection))
                .as_deref(),
        );
        permalink
    }
}
