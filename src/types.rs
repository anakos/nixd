pub type Result<A> = std::result::Result<A, Error>;

pub(crate) const SOURCES_SRC: &str = include_str!("./nixpkgs.nix");
pub(crate) const TRIVIAL_SHELL_SRC: &str = include_str!("./trivial-shell.nix");

/// Defines the type of errors we can expect to occur when running this program.
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    BadArgs { message: String },
    MissingExecutable { name: String },
    CommandFailed { message: String },
    BadShit { message: String },
}
impl From<clap::Error> for Error {
    fn from(err: clap::Error) -> Self {
        let message = format!("{}", err);
        Self::BadArgs { message }
    }
}
impl From<std::str::Utf8Error> for Error {
    fn from(err: std::str::Utf8Error) -> Self {
        Self::BadShit {
            message: format!("utf8 decoding failure: {}", err),
        }
    }
}
impl From<std::io::Error> for Error
{
    fn from(err: std::io::Error) -> Self {
        let message = format!("io error: {}", err);
        Self::CommandFailed { message }
    }
}
impl<T> From<tokio::sync::mpsc::error::SendError<T>> for Error
{
    fn from(err: tokio::sync::mpsc::error::SendError<T>) -> Self
    {
        let message = format!("error sending to channel: {}", err);
        Self::BadShit { message }        
    }
}
impl From<tokio::task::JoinError> for Error
{
    fn from(err: tokio::task::JoinError) -> Self
    {
        let message = format!("error joining to current thread: {}", err);
        Self::BadShit { message }        
    }
}

impl std::fmt::Display for Error
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::BadArgs { message } => message,
            Self::MissingExecutable { name } => name,
            Self::CommandFailed { message } => message,
            Self::BadShit { message } => message,
        };

        write!(f, "{}", msg)
    }
}
impl std::error::Error for Error {}

/// The nix branch to pin the project to.
#[derive(PartialEq, Debug)]
pub enum NixBranch {
    Version(String),
    Unstable,
    Stable,
}

impl std::str::FromStr for NixBranch {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let value = match s.to_lowercase().trim() {
            "" => NixBranch::Unstable,
            "stable" | "nixos-stable" | "nixpkgs-stable" => NixBranch::Stable,
            "unstable" | "nixos-unstable" | "nixpkgs-unstable" => NixBranch::Unstable,
            _ => NixBranch::Version(s.to_owned()),
        };

        Ok(value)
    }
}

impl std::fmt::Display for NixBranch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Stable => write!(f, "nixos-stable"),
            Self::Unstable => write!(f, "nixos-unstable"),
            Self::Version(s) => write!(f, "{}", s),
        }
    }
}

impl Default for NixBranch {
    fn default() -> Self {
        NixBranch::Unstable
    }
}
