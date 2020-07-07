use {
    crate::types as nixd,
};

/// Runs `niv init`
pub fn init()
-> nixd::Result<()>
{
    check()?;
    super::exec("niv", |mut cmd| cmd.arg("init").output())?;
    
    Ok(())
}

/// Runs `niv add ${package} for each provided package`
pub fn add_packages(packages: &[String])
-> nixd::Result<()>
{
    for package in packages {
        super::exec(
            "niv",
            |mut cmd| cmd.args(&["add", package]).output()
        )?;
    }

    Ok(())
}

/// Runs niv update nixpkgs -b ${branch}` if branch is not set to stable
pub fn update(branch: nixd::NixBranch)
-> nixd::Result<()>
{
    super::exec(
        "niv",
        |mut cmd| cmd.args(&["update", "nixpkgs", "-b", &branch.to_string()]).output()
    )?;

    Ok(())
}

/// verify that niv is installed in the current environment
fn check() -> nixd::Result<()>
{
    super::exec("niv", |mut cmd| cmd.arg("-h").output())?;
    // println!("niv exists");

    Ok(())
}

#[cfg(test)]
mod tests
{
    #[test]
    fn check_requires_niv()
    -> crate::types::Result<()>
    {
        super::check()?;
        Ok(())
    }
}