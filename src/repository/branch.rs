use git2::PushOptions;

use crate::{defaults::REMOTE_NAME, GitResult};

use super::repository::{callback_with_credentials_get, Repository};

pub struct BranchHelper<'a> {
  repository: &'a Repository,
}

impl<'a> BranchHelper<'a> {
  pub fn new(repository: &'a Repository) -> Self {
    Self { repository }
  }

  pub fn push_branch(&self, force: bool, branch_name: &str) -> GitResult<()> {
    let mut remote = self.repository.repository.find_remote(REMOTE_NAME)?;

    let mut push_options = PushOptions::new();
    push_options.remote_callbacks(callback_with_credentials_get(&self.repository.options.auth));

    let ref_spec_format = match force {
      true => format!("+refs/heads/{}:refs/heads/{}", branch_name, branch_name),
      false => format!("refs/heads/{}:refs/heads/{}", branch_name, branch_name),
    };

    let refspec: &[&str] = &[&ref_spec_format];

    remote.push(refspec, Some(&mut push_options))?;

    Ok(())
  }

  /// Push changes to remote
  pub fn push(&self, force: bool) -> GitResult<()> {
    self.push_branch(force, &self.repository.options.default_branch)?;

    Ok(())
  }
}
