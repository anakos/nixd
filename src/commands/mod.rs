use {
    crate::{opts, types as nixd},
    std::process::Command as PCommand,
};

mod direnv;
mod lorri;
mod niv;

pub fn commands(opts: opts::Opts) -> impl Iterator<Item = Command> {
    let opts::Opts { branch, packages } = opts;
    let packages = packages.unwrap_or_default();

    vec![
        Command::NivInit,
        Command::NivUpdate { branch },
        Command::NivAdd { packages },
        Command::Lorri,
        Command::Direnv,
        Command::NewShell,
    ]
    .into_iter()
}

#[derive(Debug)]
pub enum Command {
    NivInit,
    NivUpdate { branch: nixd::NixBranch },
    NivAdd { packages: Vec<String> },
    Lorri,
    Direnv,
    NewShell,
}
impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::NivInit => write!(f, "niv init"),
            Command::NivUpdate { branch } => write!(f, "niv update -b {}", branch),
            Command::NivAdd { packages } => write!(f, "niv add {:?}", packages),
            Command::Lorri => write!(f, "lorri"),
            Command::Direnv => write!(f, "direnv"),
            Command::NewShell => write!(f, "lorri shell"),
        }
    }
}
impl Command {
    pub fn run(self) -> nixd::Result<()> {
        match self {
            Command::NivInit => niv::init(),
            Command::NivUpdate { branch } => niv::update(branch),
            Command::NivAdd { packages } => niv::add_packages(&packages),
            Command::Lorri => lorri::init(),
            Command::Direnv => direnv::allow(),
            Command::NewShell => lorri::shell(),
        }
    }
}

fn exec<F>(executable: &str, cmd: F) -> nixd::Result<()>
where
    F: FnOnce(PCommand) -> std::io::Result<std::process::Output>,
{
    let output = cmd(PCommand::new(executable)).map_err(|a| match a.kind() {
        std::io::ErrorKind::NotFound => nixd::Error::MissingExecutable {
            name: executable.to_owned(),
        },
        _ => {
            let message = format!("Could not run `{}`: {}", executable, a,);
            nixd::Error::CommandFailed { message }
        }
    })?;

    if output.status.success() {
        return Ok(());
    }

    let message = format!(
        r#"Could not run `{}`:
           - stderr: {}
        "#,
        executable,
        std::str::from_utf8(&output.stderr)?,
    );

    Err(nixd::Error::CommandFailed { message })
}
