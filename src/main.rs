use std::ffi::{OsStr, OsString};
use std::io::{self, ErrorKind};
use std::os::unix::process::CommandExt as _;
use std::path::Path;
use std::process::{self, Command};

const DEFAULT_PROGRAM_NAME: &str = "$";

#[derive(Debug, clap::Parser)]
#[clap(author, version, about, long_about = None)]
pub(crate) struct Cli {
    #[clap(allow_hyphen_values = true)]
    pub(crate) args: Vec<OsString>,
}

impl Cli {
    pub(crate) fn parse(program: &str) -> Self {
        let mut matches = Self::command(program).get_matches();

        <Self as clap::FromArgMatches>::from_arg_matches_mut(&mut matches)
            .unwrap_or_else(|err| err.exit())
    }

    fn command(program: &str) -> clap::Command {
        <Self as clap::CommandFactory>::command()
            .name(program.to_owned())
            .bin_name(program.to_owned())
    }
}

fn main() {
    let program = program_name();
    let cli = Cli::parse(&program);

    if cli.args.is_empty() {
        let mut cmd = Cli::command(&program);
        eprint!("{}", cmd.render_help());
        process::exit(2);
    }

    if let Err(err) = exec(&cli.args[0], &cli.args[1..]) {
        let cmd = cli.args[0].to_string_lossy();
        let code = match err.kind() {
            ErrorKind::NotFound => {
                eprintln!("{program}: command not found: {cmd}");
                127
            }
            ErrorKind::PermissionDenied => {
                eprintln!("{program}: permission denied: {cmd}");
                126
            }
            _ => {
                eprintln!("{program}: {cmd}: {err}");
                126
            }
        };
        process::exit(code);
    }
}

/// 起動時の名前を返す。`$`・`%`・`#` のどれで呼ばれてもその名前を名乗る。
fn program_name() -> String {
    std::env::args_os()
        .next()
        .and_then(|arg0| {
            Path::new(&arg0)
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
        })
        .filter(|name| !name.is_empty())
        .unwrap_or_else(|| DEFAULT_PROGRAM_NAME.to_owned())
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
