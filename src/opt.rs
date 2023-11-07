use std::path::PathBuf;

use derive_builder::Builder;

use crate::auth_type::AuthType;

/// Newtype struct for url of git repository
#[derive(Clone, Debug)]
pub struct GitUrl(pub String);

#[derive(Clone, Debug)]
pub struct Signature {
  pub username: String,
  pub email: String,
}

impl Signature {
  pub fn new(name: &str, email: &str) -> Self {
    Self {
      username: name.to_string(),
      email: email.to_string(),
    }
  }
}

/// Options of repository
#[derive(Builder, Clone, Debug)]
pub struct RepositoryOptions {
  /// Woring dir for cloning repository into it
  #[builder(default = "default_dir()")]
  pub working_dir: PathBuf,
  /// Default branch for sync
  #[builder(default = "default_branch()")]
  pub default_branch: String,
  /// Url to upstream repo
  pub upstream_url: Option<GitUrl>,
  /// Url to fork
  pub url: GitUrl,
  /// Type of auth to use
  pub auth: AuthType,
  #[builder(default)]
  pub author_signature: Option<Signature>,
  pub commiter_signature: Signature,
}

fn default_dir() -> PathBuf {
  std::env::temp_dir()
}

fn default_branch() -> String {
  crate::defaults::DEFAULT_BRANCH.to_string()
}
