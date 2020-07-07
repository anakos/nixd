use {
    std::io::{self, Write},
    crate::{
        types as nixd,
    },
};

/// Runs `lorri init` from the current working directory and
/// sets the content of shell.nix to the provided content.
pub fn init()
-> nixd::Result<()>
{
    check()?;

    super::exec("lorri", |mut cmd| cmd.arg("init").output())?;
    // println!("lorri init");

    write_nixpkgs_nix()?;
    update_nix_shell()?;
    
    Ok(())
}

/// Runs `lorri init` from the current working directory and
/// sets the content of shell.nix to the provided content.
pub fn shell()
-> nixd::Result<()>
{
    super::exec("lorri", |mut cmd| cmd.arg("shell").output())?;
    // println!("lorri shell");
    Ok(())
}

/// Used to verify that lorri is installed on the current system.
fn check()
-> nixd::Result<()>
{
    super::exec("lorri", |mut cmd| cmd.arg("-V").output())
}

/// writes shell.nix to CWD.
fn write_nixpkgs_nix()
-> io::Result<()>
{
    let mut sources = std::fs::File::create("nix/nixpkgs.nix")?;
    sources.write_all(nixd::SOURCES_SRC.as_bytes())?;

    Ok(())
}

/// writes shell.nix to CWD.
fn update_nix_shell()
-> io::Result<()>
{
    std::fs::File::create("shell.nix")?
        .write_all(nixd::TRIVIAL_SHELL_SRC.as_bytes())?;

    Ok(())
}
