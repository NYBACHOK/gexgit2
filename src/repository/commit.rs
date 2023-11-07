use super::repository::Repository;
use git2::{Branch, BranchType, Oid, Signature as Git2Signature};

use crate::{defaults::DEFAULT_COMMIT_NAME, opt::Signature, GitResult};

pub struct CommitHelper<'a> {
  repository: &'a Repository,
}

impl<'a> CommitHelper<'a> {
  pub fn new(repository: &'a Repository) -> Self {
    Self { repository }
  }

  pub fn commit_all_parameters(
    &self,
    msg: &str,
    update_ref: Option<&str>,
    author: &Git2Signature,
    commiter: &Git2Signature,
    tree_id: Oid,
    commit_into: Branch<'_>,
  ) -> GitResult<Oid> {
    let commit_oid = self.repository.repository().commit(
      update_ref, // Use HEAD to automatically point to the current branch
      &author,    // Commit author
      &commiter,  // Commit committer
      msg,        // Commit message
      &self.repository.repository().find_tree(tree_id)?, // The tree to commit
      &[&commit_into.get().peel_to_commit()?], // Parent commit
    )?;

    Ok(commit_oid)
  }

  /// Creates a new commit.
  pub fn commit(&self, msg: Option<&str>) -> GitResult<Oid> {
    let msg = msg.unwrap_or(DEFAULT_COMMIT_NAME);

    let Signature { username, email } = self.repository.author_signature_get()?;
    let author = Git2Signature::now(&username, &email)?;

    let commiter = Git2Signature::now(
      &self.repository.options.commiter_signature.username,
      &self.repository.options.commiter_signature.email,
    )?;

    let tree_id = self.repository.index().write()?;
    let commit_into = self
      .repository
      .repository()
      .find_branch(&self.repository.options.default_branch, BranchType::Local)?;

    let commit_id =
      self.commit_all_parameters(msg, Some("HEAD"), &author, &commiter, tree_id, commit_into)?;

    Ok(commit_id)
  }
}
