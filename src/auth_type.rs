use std::path::PathBuf;

#[derive(Debug, Clone)]
/// Credentials to auth with SSH
pub struct ZshCredentials {
  /// Optional name for auth
  pub name: Option<String>,
  /// Path to zsh key
  pub path_to_key: PathBuf,
}

#[derive(Debug, Clone)]
/// Credentials to auth with HTTP
pub struct HttpCredentials {
  /// Username for git
  pub username: String,
  /// Password or token for git
  pub password: String,
}

#[derive(Debug, Clone, Default)]
/// Type auth
pub enum AuthType {
  /// No Auth
  #[default]
  None,
  /// Auth with SSH
  SshKey(ZshCredentials),
  /// Auth with SSH using credentials from agent
  SshAgent,
  /// Auth with git token/password
  Token(HttpCredentials),
}
