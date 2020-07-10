use crate::types as nixd;

/// Runs `direnv allow` from the current working directory.
pub fn allow() -> nixd::Result<()> {
    super::exec("direnv", |mut cmd| cmd.arg("allow").output())?;

    Ok(())
}
