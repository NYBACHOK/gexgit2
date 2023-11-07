use git2::{IndexAddOption, IndexMatchedPath, Oid};

use crate::GitResult;

use super::repository::Repository;

pub struct IndexHelper<'a> {
  repository: &'a Repository,
}

impl<'a> IndexHelper<'a> {
  pub fn new(repository: &'a Repository) -> Self {
    Self { repository }
  }

  /// Write index into
  pub fn write(&self) -> GitResult<Oid> {
    let mut index = self.repository.repository().index()?;

    index.write()?;

    let id = index.write_tree()?;

    Ok(id)
  }

  pub fn add<'b>(
    &self,
    pathspecs: impl Iterator<Item = &'b str>,
    flag: Option<IndexAddOption>,
    cb: Option<&mut IndexMatchedPath<'_>>,
  ) -> GitResult<()> {
    let mut index = self.repository.repository().index()?;

    index.add_all(pathspecs, flag.unwrap_or(IndexAddOption::DEFAULT), cb)?;

    Ok(())
  }

  pub fn update<'b>(
    &self,
    pathspecs: impl Iterator<Item = &'b str>,
    cb: Option<&mut IndexMatchedPath<'_>>,
  ) -> GitResult<()> {
    let mut index = self.repository.repository().index()?;

    index.update_all(pathspecs, cb)?;

    Ok(())
  }

  pub fn remove<'b>(
    &self,
    pathspecs: impl Iterator<Item = &'b str>,
    cb: Option<&mut IndexMatchedPath<'_>>,
  ) -> GitResult<()> {
    let mut index = self.repository.repository().index()?;

    index.remove_all(pathspecs, cb)?;

    Ok(())
  }
}
