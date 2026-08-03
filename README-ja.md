# dollar-cmd

[![GitHub License](https://img.shields.io/github/license/cffnpwr/dollar-cmd?style=flat)](./LICENSE)

`$ cmd args` を貼り付けてもそのまま動くようにするジョークコマンドです。

[README.md for English is available here](./README.md).

## What is This

技術記事のシェル例は、たいていプロンプト記号付きで書かれています。

```sh
$ cargo build --release
```

どの記号が付くかは筆者の使ったシェルによって変わり、sh・bashなら `$`、csh・zshなら `%`、rootシェルなら `#` になります。

これをサイトのコピーボタンでコピーすると記号ごとコピーされることが多く、シェルに貼り付けると失敗します。

```console
zsh: command not found: $
```

`dollar-cmd` は、これらの記号の名前で `PATH` 上に置く実行ファイルです。
シェルがリテラルの記号を実行しようとしたときにこのコマンドが起動し、残りを独立したコマンドとして実行するため、貼り付けた行がそのまま動きます。

## How to Install

このクレートがビルドするバイナリ名は `dollar-cmd` です。インストール後に `PATH` 上へ `$`・`%`・`#` としてリンクしてください。

### cargo install

```sh
cargo install --git https://github.com/cffnpwr/dollar-cmd
ln -s ~/.cargo/bin/dollar-cmd ~/.cargo/bin/'$'
ln -s ~/.cargo/bin/dollar-cmd ~/.cargo/bin/'%'
ln -s ~/.cargo/bin/dollar-cmd ~/.cargo/bin/'#'
```

### ソースからビルド

```sh
git clone https://github.com/cffnpwr/dollar-cmd
cd dollar-cmd
cargo build --release
```

ビルドした `target/release/dollar-cmd` を、`$`・`%`・`#` という名前で `PATH` 上の任意の場所に配置します。

### `%`・`#` のためのシェル設定

`$` はリンクを張るだけで動きます。
残る2つはシェルが `PATH` を探すより前に横取りするため、シェルの起動ファイルに1行ずつ設定が必要です。

`%` はbash・zshのどちらでもジョブ指定の開始として解釈され、`%1` は `fg %1` と同義になります。
alias展開はそれより前に行われるので、リンクの絶対パスへのaliasを張ります。

```sh
alias '%'="$HOME/.cargo/bin/%"
```

`#` はbashではコメントの開始になります。対話シェルでは `interactive_comments` がデフォルトで有効なためです。
コメントの除去はalias展開より前段なのでaliasでは回避できず、このオプションを無効にする必要があります。

```sh
shopt -u interactive_comments
```

無効にしている間は、対話中のbashで `ls # list files` のような行末コメントが使えなくなります。
zshは `INTERACTIVE_COMMENTS` がデフォルトで未設定なので、`#` は設定なしで動きます。

## How to Use

プロンプト記号付きのコマンドをそのまま貼り付けて実行します。

```sh
$ echo hello
% echo hello
# echo hello
```

```console
hello
```

先頭の引数が実行するコマンドで、それ以降の引数はそのまま渡されます。
コマンドの前に置いた `--` はオプション終端として扱われるため、オプションのような名前のコマンドも実行できます。

```sh
$ -- ls -al
```

記号は `execvp` で自身を対象プロセスに置き換えるため、終了コードとシグナルは実行したコマンドのものになります。
ヘルプとエラーメッセージは、起動に使われた記号の名前で表示されます。

### Options

| オプション | 説明 |
| --- | --- |
| `-h`, `--help` | ヘルプを表示する |
| `-V`, `--version` | バージョンを表示する |

これらはコマンド名より前に現れた場合にのみ解釈されます。
コマンドが確定した後の引数は、すべてそのままコマンドへ渡されます。

### Exit Codes

| コード | 条件 |
| --- | --- |
| `2` | コマンドが指定されていない |
| `126` | コマンドは見つかったが実行できない |
| `127` | コマンドが見つからない |

これら以外は、実行したコマンドの終了コードです。

## License

[MIT License](./LICENSE)
