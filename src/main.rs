use std::ffi::{OsStr, OsString};
use std::io::{self, ErrorKind};
use std::os::unix::process::CommandExt as _;
use std::process::{self, Command};

#[derive(Debug, clap::Parser)]
#[clap(name = "$", author, version, about, long_about = None)]
pub(crate) struct Cli {
    #[clap(allow_hyphen_values = true)]
    pub(crate) args: Vec<OsString>,
}

impl Cli {
    pub(crate) fn parse() -> Self {
        <Self as clap::Parser>::parse()
    }
}

fn main() {
    let cli = Cli::parse();

    if cli.args.is_empty() {
        let mut cmd = <Cli as clap::CommandFactory>::command();
        eprint!("{}", cmd.render_help());
        process::exit(2);
    }

    if let Err(err) = exec(&cli.args[0], &cli.args[1..]) {
        let cmd = cli.args[0].to_string_lossy();
        let code = match err.kind() {
            ErrorKind::NotFound => {
                eprintln!("$: command not found: {cmd}");
                127
            }
            ErrorKind::PermissionDenied => {
                eprintln!("$: permission denied: {cmd}");
                126
            }
            _ => {
                eprintln!("$: {cmd}: {err}");
                126
            }
        };
        process::exit(code);
    }
}

fn exec(
    cmd: impl AsRef<OsStr>,
    args: impl IntoIterator<Item = impl AsRef<OsStr>>,
) -> io::Result<()> {
    let mut cmd = Command::new(cmd);
    let cmd = cmd.args(args);

    let err = cmd.exec();
    Err(err)
}
