use git2::{build::CheckoutBuilder, BranchType, MergeOptions};

use crate::{defaults::UPSTREAM_NAME, errors::GitErrors, GitResult};

use super::repository::Repository;

pub struct MergeHelper<'a> {
  repository: &'a Repository,
}

impl<'a> MergeHelper<'a> {
  pub fn new(repository: &'a Repository) -> Self {
    Self { repository }
  }

  pub fn merge(
    &self,
    allow_conflict: bool,
    fail_on_conflict: bool,
    is_diff3: bool,
    use_theirs: bool,
  ) -> GitResult<()> {
    let merge_from_name = format!("{UPSTREAM_NAME}/{}", self.repository.options.default_branch);
    let merge_from = self
      .repository
      .repository
      .find_branch(&merge_from_name, BranchType::Remote)?;

    let merge_from_commit = self
      .repository
      .repository
      .reference_to_annotated_commit(merge_from.get())?;

    let (merge_analysis, _merge_preference) = self
      .repository
      .repository
      .merge_analysis(&[&merge_from_commit])?;

    if merge_analysis.is_up_to_date() {
      return Err(GitErrors::UpToDate);
    }

    if merge_analysis.is_empty() {
      return Err(GitErrors::Empty);
    }

    let mut checkout_opt = CheckoutBuilder::default();
    checkout_opt.update_index(true);
    checkout_opt.use_theirs(use_theirs);
    checkout_opt.allow_conflicts(allow_conflict);
    checkout_opt.remove_untracked(true);

    let mut merge_opt = MergeOptions::default();
    merge_opt.find_renames(true);
    merge_opt.fail_on_conflict(fail_on_conflict);
    merge_opt.diff3_style(is_diff3);

    self
      .repository
      .repository
      .merge(
        &[&merge_from_commit],
        Some(&mut merge_opt),
        Some(&mut checkout_opt),
      )
      .map_err(GitErrors::Git2Error)?;

    Ok(())
  }
}
