mod providers;

use std::sync::Arc;

use git::GitHostingProviderRegistry;
use gpui::AppContext;
use project::project_settings::GitHostProvider;

pub use crate::providers::*;

/// Initializes the Git hosting providers.
pub fn init(cx: &mut AppContext, git_host_provider_settings: GitHostProvider) {
    let provider_registry = GitHostingProviderRegistry::global(cx);

    // The providers are stored in a `BTreeMap`, so insertion order matters.
    // GitHub comes first.
    provider_registry.register_hosting_provider(Arc::new(Github));

    // Then GitLab.
    provider_registry.register_hosting_provider(Arc::new(Gitlab));

    // Then the other providers, in the order they were added.
    provider_registry.register_hosting_provider(Arc::new(Gitee));
    provider_registry.register_hosting_provider(Arc::new(Bitbucket));
    provider_registry.register_hosting_provider(Arc::new(Sourcehut));
    provider_registry.register_hosting_provider(Arc::new(Codeberg));

    provider_registry.register_hosting_provider(Arc::new(CustomizeGitProvider::new(
        git_host_provider_settings,
    )));
}
