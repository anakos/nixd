use {
    clap::Clap,
    crate::types::{
        NixBranch,
    },
};

/// Initialize the project sandbox with the provided version of nixpkgs and user-defined github packages.
#[derive(Clap, Debug, PartialEq,)]
#[clap(
    version = "0.1.0",
    author  = "Alex N. <anakos@gmail.com>",
    about   = "Initializes a nix-shell based development environment (niv + lorri + direnv) in the current working directory"
)]
pub struct Opts
{
    /// pins nixpkgs to the indicated branch.
    #[clap(short, long, default_value)]
    pub branch: NixBranch,

    /// packages to include via calls to `niv add package1, package 2, ...`
    #[clap(short, long,)]
    pub packages: Option<Vec<String>>,
}
impl Default for Opts
{
    fn default() -> Opts
    {
        Opts {
            branch: NixBranch::default(),
            packages: None,
        }
    }    
}

#[cfg(test)]
mod tests {
    use {
        clap::Clap,
        crate::types::{
            NixBranch,
            Result,
        },
    };

    #[test]
    fn default_opts() -> Result<()>
    {
        let empty_args = vec![] as Vec<&str>; 
        let result     = super::Opts::try_parse_from(empty_args)?;
        
        assert_eq!(
            result,
            super::Opts::default(),
        );

        Ok(())
    }

    #[test]
    fn opts_long()
    -> Result<()>
    {
        let args   = vec!["nixd", "--branch", "nixos-19.09", "-p", "mozilla/nixpkgs-mozilla", "-p", "stedolan/jq", ];
        let actual = super::Opts::try_parse_from(args)?;
        
        assert_eq!(
            actual,
            super::Opts {
                branch: NixBranch::Version("nixos-19.09".to_owned()),
                packages: Some(vec!["mozilla/nixpkgs-mozilla".to_owned(), "stedolan/jq".to_owned()]),
            },
        );

        Ok(())
    }

    #[test]
    fn opts_short()
    -> Result<()>
    {
        let branch = "nixos-19.09";
        let args   = vec!["nixd", "-b", branch ];
        let actual = super::Opts::try_parse_from(args)?;
        
        assert_eq!(
            actual,
            super::Opts {
                branch: NixBranch::Version(branch.to_owned()),
                packages: None,
            },
        );

        Ok(())
    }
}