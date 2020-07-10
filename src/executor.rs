use {
    crate::types as nixd,
    async_std::{
        prelude::*,
        stream,
        sync::{self, channel},
        task,
    },
    std::io::{self, Write},
    termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor},
};

/// Responsible for executing commands and drawing progress on the screen.
#[derive(Debug)]
pub struct Executor {
    sender: sync::Sender<Status>,
    handle: task::JoinHandle<io::Result<()>>,
}
impl Executor {
    pub fn init() -> Self {
        let (sender, mut recv) = channel::<Status>(10);
        let handle = task::spawn(async move {
            let mut stdout = StandardStream::stdout(ColorChoice::Always);
            while let Some(val) = recv.next().await {
                match val {
                    Status::Start(text) => {
                        stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
                        write!(&mut stdout, "{} ", text)?;
                        stdout.flush()?;
                    }
                    Status::Continue => {
                        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
                        write!(&mut stdout, ".")?;
                        stdout.flush()?;
                    }
                    Status::Success => {
                        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
                        writeln!(&mut stdout, " success!!!")?;
                        stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
                    }
                    Status::Failure(e) => {
                        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
                        writeln!(&mut stdout, " failed: {}", e)?;
                        stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
                    }
                }
            }

            Ok(())
        });

        Executor { sender, handle }
    }

    pub async fn run_command<F>(&self, label: String, cmd: F) -> nixd::Result<()>
    where
        F: FnOnce() -> nixd::Result<()>,
    {
        self.sender.send(Status::Start(label)).await;

        let progress_indicator = ProgressIndicator::start(self.sender.clone());
        let result = match cmd() {
            Ok(_) => Status::Success,
            Err(e) => Status::Failure(e),
        };

        drop(progress_indicator);
        self.sender.send(result).await;

        Ok(())
    }

    pub async fn terminate(self) -> nixd::Result<()> {
        drop(self.sender);

        if let Err(e) = self.handle.await {
            Err(nixd::Error::BadShit {
                message: format!("error writing to stdout: {}", e),
            })
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, PartialEq)]
enum Status {
    Start(String),
    Continue,
    Success,
    Failure(nixd::Error),
}

/// This is a visual progress indicator
struct ProgressIndicator {
    _handle: task::JoinHandle<()>,
    _kill_switch: stop_token::StopSource,
}
impl ProgressIndicator {
    fn start(my_sender: sync::Sender<Status>) -> ProgressIndicator {
        let _kill_switch = stop_token::StopSource::new();
        let mut my_stream = _kill_switch
            .stop_token()
            .stop_stream(stream::interval(std::time::Duration::from_millis(500)).fuse());
        let _handle = task::spawn(async move {
            loop {
                if my_stream.next().await.is_none() {
                    break;
                }
                my_sender.send(Status::Continue).await;
            }
        });

        ProgressIndicator {
            _handle,
            _kill_switch,
        }
    }
}
