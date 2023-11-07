use git2::FetchOptions;

use crate::{defaults::UPSTREAM_NAME, errors::GitErrors, GitResult};

use super::repository::{callback_with_credentials_get, Repository};

pub struct RemoteHelper<'a> {
  repository: &'a Repository,
}

impl<'a> RemoteHelper<'a> {
  pub fn new(repository: &'a Repository) -> Self {
    Self { repository }
  }

  pub fn remote_add(&self) -> GitResult<()> {
    let mut fetch_options = FetchOptions::new();

    fetch_options.remote_callbacks(callback_with_credentials_get(&self.repository.options.auth));

    self
      .repository
      .repository
      .remote(
        UPSTREAM_NAME,
        self
          .repository
          .options
          .upstream_url
          .clone()
          .ok_or(GitErrors::RemoteError)?
          .0
          .as_str(),
      )?
      .fetch(
        &[&self.repository.options.default_branch],
        Some(&mut fetch_options),
        None,
      )?;

    Ok(())
  }
}
