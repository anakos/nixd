use {
    crate::types as nixd,
    std::io::{self, Write},
    stream_cancel::{StreamExt,Trigger, Tripwire,},
    termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor},
    tokio::{
        spawn,
        sync::mpsc,
        task,
        time::interval,
    },
    tokio_stream::wrappers::IntervalStream,
};

/// Responsible for executing commands and drawing progress on the screen.
#[derive(Debug)]
pub struct Executor {
    sender: mpsc::Sender<Status>,
    handle: task::JoinHandle<io::Result<()>>,
}
impl Executor {
    pub fn init() -> Self {
        let (sender, mut rx) = mpsc::channel::<Status>(10);
        let handle = spawn(async move {
            let mut stdout = StandardStream::stdout(ColorChoice::Always);
            while let Some(val) = rx.recv().await {
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
        self.sender.send(Status::Start(label)).await.unwrap();

        let progress_indicator = ProgressIndicator::start(self.sender.clone());
        let result = match cmd() {
            Ok(_) => Status::Success,
            Err(e) => Status::Failure(e),
        };

        drop(progress_indicator);
        self.sender.send(result).await?;

        Ok(())
    }

    pub async fn terminate(self) -> nixd::Result<()> {
        drop(self.sender);

        self.handle.await??;
        Ok(())
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
    _kill_switch: Trigger,
}
impl ProgressIndicator {
    fn start(my_sender: mpsc::Sender<Status>) -> ProgressIndicator {
        let (_kill_switch, tripwire) = Tripwire::new();

        let mut my_stream =
            IntervalStream::new(interval(std::time::Duration::from_millis(500)))
                .take_until_if(tripwire);

        let _handle = spawn(async move {
            loop {
                if let None = tokio_stream::StreamExt::next(&mut my_stream).await {
                    break;
                }
                my_sender.send(Status::Continue).await.unwrap();
            }
        });

        ProgressIndicator {
            _handle,
            _kill_switch,
        }
    }
}
