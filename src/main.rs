mod commands;
mod executor;
mod opts;
mod types;

use clap::Clap;

#[async_std::main]
async fn main() -> types::Result<()> {
    let opts = opts::Opts::parse();
    let executor = executor::Executor::init();
    for cmd in commands::commands(opts) {
        if let Err(e) = executor.run_command(cmd.to_string(), || cmd.run()).await {
            // see https://github.com/target/lorri/blob/master/src/ops/mod.rs for where these codes come from
            match e {
                types::Error::MissingExecutable { name } => {
                    eprintln!("executable was not available on path: {}", name);
                    std::process::exit(127)
                }

                types::Error::BadArgs { message } => {
                    eprintln!("bad arguments supplied to program: {}", message);
                    std::process::exit(100)
                }

                types::Error::CommandFailed { message } => {
                    eprintln!("could not execute sub-command. {}", message);
                    std::process::exit(111)
                }

                types::Error::BadShit { message } => {
                    eprintln!(
                        "something totally unexpected occurred, processing terminated: {}",
                        message
                    );
                    std::process::exit(101)
                }
            }
        }
    }

    // TODO: exit with proper error code when complete
    executor.terminate().await?;

    Ok(())
}
