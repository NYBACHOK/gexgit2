use core::fmt::Debug;
use git2::{BranchType, Cred, FetchOptions, RemoteCallbacks, Repository as Git2Repository};
use std::fmt::Formatter;

use crate::{
  auth_type::AuthType,
  opt::{RepositoryOptions, Signature},
  GitResult,
};

use super::{branch::BranchHelper, commit::CommitHelper, index::IndexHelper, remote::RemoteHelper, merge::MergeHelper};

/// Structure for interacting with git repository
///
/// _Note:_ When a repository goes out of scope it is freed in memory but not deleted from the filesystem.
pub struct Repository {
  pub options: RepositoryOptions,
  pub(crate) repository: Git2Repository,
}

impl AsRef<Git2Repository> for Repository {
  fn as_ref(&self) -> &Git2Repository {
    &self.repository
  }
}

impl Debug for Repository {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "Repository: {:?}", self.options)
  }
}

impl Repository {
  /// Create repository with cloning into local machine
  pub fn clone(options: RepositoryOptions, clone_into: &str) -> GitResult<Self> {
    let repository = {
      let callbacks = callback_with_credentials_get(&options.auth);

      // Prepare fetch options.
      let mut fo = FetchOptions::new();
      fo.remote_callbacks(callbacks);

      // Prepare builder.
      let mut builder = git2::build::RepoBuilder::new();
      builder.fetch_options(fo);

      // Clone the project.
      builder.clone(
        options.url.0.as_str(),
        &options.working_dir.join(clone_into),
      )?
    };

    Ok(Self {
      options,
      repository,
    })
  }

  pub fn repository(&self) -> &Git2Repository {
    &self.repository
  }

  pub fn commit(&self) -> CommitHelper<'_> {
    CommitHelper::new(self)
  }

  pub fn index(&self) -> IndexHelper<'_> {
    IndexHelper::new(self)
  }

  pub fn branch(&self) -> BranchHelper<'_> {
    BranchHelper::new(self)
  }

  pub fn remote(&self) -> RemoteHelper<'_> {
    RemoteHelper::new(self)
  }

  pub fn merge(&self) -> MergeHelper<'_>
  {
    MergeHelper::new(self)
  }

  pub fn author_signature_get(&self) -> GitResult<Signature> {
    let sig = if let Some(ref var) = self.options.author_signature {
      var.clone()
    } else {
      let branch = self
        .repository
        .find_branch(&self.options.default_branch, BranchType::Local)?;

      let commit = branch.get().peel_to_commit()?;
      let author = commit.author();

      let sign = Signature::new(
        author.name().expect("Invalid author name."),
        author.email().expect("Invalid author email."),
      );

      sign
    };

    Ok(sig)
  }
}

/// Helper method for creating callback with auth
pub(crate) fn callback_with_credentials_get(auth: &AuthType) -> RemoteCallbacks<'_> {
  // Prepare callbacks.
  let mut callbacks = RemoteCallbacks::new();

  match auth {
    AuthType::None => (),
    AuthType::SshKey(creds) => {
      callbacks.credentials(|_, name, _| {
        Cred::ssh_key(
          {
            if let Some(ref new_name) = creds.name {
              new_name
            } else {
              name.unwrap_or("git")
            }
          },
          None,
          &creds.path_to_key,
          None,
        )
      });
    }
    AuthType::SshAgent => {
      callbacks.credentials(move |_, name, _| Cred::ssh_key_from_agent(name.unwrap_or("git")));
    }
    AuthType::Token(token) => {
      callbacks.credentials(|_url, _name, _allowed_types| {
        Cred::userpass_plaintext(&token.username, &token.password)
      });
    }
  }

  callbacks
}
