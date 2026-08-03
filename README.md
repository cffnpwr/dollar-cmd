# dollar-cmd

[![GitHub License](https://img.shields.io/github/license/cffnpwr/dollar-cmd?style=flat)](./LICENSE)

A joke command that makes a pasted `$ cmd args` line just work.

[日本語のREADMEはこちら](./README-ja.md)

## What is This

Technical articles usually write shell examples with a prompt marker:

```sh
$ cargo build --release
```

Which marker appears depends on the shell the author used: `$` for sh and bash, `%` for csh and zsh, and `#` for a root shell.

Copying that block with the site's copy button often takes the marker along with it, and pasting it into a shell fails:

```console
zsh: command not found: $
```

`dollar-cmd` is an executable placed on your `PATH` under those marker names.
When the shell tries to run the literal marker, this command takes over, executes the rest of the line as an independent command, and the pasted line works as written.

## How to Install

The crate builds a binary named `dollar-cmd`, so link it as `$`, `%`, and `#` on your `PATH` after installing.

### cargo install

```sh
cargo install --git https://github.com/cffnpwr/dollar-cmd
ln -s ~/.cargo/bin/dollar-cmd ~/.cargo/bin/'$'
ln -s ~/.cargo/bin/dollar-cmd ~/.cargo/bin/'%'
ln -s ~/.cargo/bin/dollar-cmd ~/.cargo/bin/'#'
```

### Build from source

```sh
git clone https://github.com/cffnpwr/dollar-cmd
cd dollar-cmd
cargo build --release
```

Then place `target/release/dollar-cmd` anywhere on your `PATH` under the names `$`, `%`, and `#`.

### Shell setup for `%` and `#`

`$` works with the links alone.
The other two markers are intercepted by the shell before it searches your `PATH`, so each needs one line in your shell startup file.

`%` introduces a job specification in both bash and zsh, where `%1` is a synonym for `fg %1`.
Alias expansion runs before that, so alias the marker to the absolute path of its link:

```sh
alias '%'="$HOME/.cargo/bin/%"
```

`#` starts a comment in bash, because `interactive_comments` is enabled by default for interactive shells.
Comments are stripped before aliases are expanded, so an alias cannot help and the option has to be turned off:

```sh
shopt -u interactive_comments
```

Trailing comments such as `ls # list files` stop working in interactive bash while that option is off.
zsh leaves `INTERACTIVE_COMMENTS` unset by default, so `#` works there with no setup.

## How to Use

Paste a command with its prompt marker and run it:

```sh
$ echo hello
% echo hello
# echo hello
```

```console
hello
```

The first argument is the command to run, and everything after it is passed through untouched.
`--` before the command is accepted as an option terminator, so a command named like an option can still be run:

```sh
$ -- ls -al
```

The marker replaces itself with the target process via `execvp`, so the exit code and signals are those of the command you ran.
Help and error messages are reported under the marker the command was invoked with.

### Options

| Option | Description |
| --- | --- |
| `-h`, `--help` | Print help |
| `-V`, `--version` | Print version |

These are recognized only when they appear before the command name.
After a command is determined, every remaining argument is passed to that command as-is.

### Exit Codes

| Code | Condition |
| --- | --- |
| `2` | No command was given |
| `126` | The command was found but could not be executed |
| `127` | The command was not found |

Anything else is the exit code of the executed command.

## License

[MIT License](./LICENSE)
