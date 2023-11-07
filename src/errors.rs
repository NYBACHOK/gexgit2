/// Error enum
#[derive(Debug, thiserror::Error)]
pub enum GitErrors {
  /// Errors from git2
  #[error("Git error: {0}")]
  Git2Error(#[from] git2::Error),
  /// Merge contains errors
  #[error("Merge contains conflicts")]
  MergeConflicts,
  /// Merge contains no changes
  #[error("Repository already up to date")]
  UpToDate,
  #[error("Remote url is `None`")]
  RemoteError,
  /// Custom error
  #[error("Custom error: {0}")]
  CustomError(String),
}
