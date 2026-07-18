# dollar-cmd

[![GitHub License](https://img.shields.io/github/license/cffnpwr/dollar-cmd?style=flat)](./LICENSE)

`$ cmd args` を貼り付けてもそのまま動くようにするジョークコマンドです。

[README.md for English is available here](./README.md).

## What is This

技術記事のシェル例は、たいていプロンプト記号付きで書かれています。

```sh
$ cargo build --release
```

これをサイトのコピーボタンでコピーすると `$` ごとコピーされることが多く、シェルに貼り付けると失敗します。

```console
zsh: command not found: $
```

`dollar-cmd` は、`PATH` 上に置く `$` という名前の実行ファイルです。
シェルがリテラルの `$` を実行しようとしたときにこのコマンドが起動し、残りを独立したコマンドとして実行するため、貼り付けた行がそのまま動きます。

## How to Install

このクレートがビルドするバイナリ名は `dollar-cmd` です。インストール後に `PATH` 上へ `$` としてリンクしてください。

### cargo install

```sh
cargo install --git https://github.com/cffnpwr/dollar-cmd
ln -s ~/.cargo/bin/dollar-cmd ~/.cargo/bin/'$'
```

### ソースからビルド

```sh
git clone https://github.com/cffnpwr/dollar-cmd
cd dollar-cmd
cargo build --release
```

ビルドした `target/release/dollar-cmd` を、`$` という名前で `PATH` 上の任意の場所に配置します。

## How to Use

プロンプト記号付きのコマンドをそのまま貼り付けて実行します。

```sh
$ echo hello
```

```console
hello
```

先頭の引数が実行するコマンドで、それ以降の引数はそのまま渡されます。
コマンドの前に置いた `--` はオプション終端として扱われるため、オプションのような名前のコマンドも実行できます。

```sh
$ -- ls -al
```

`$` は `execvp` で自身を対象プロセスに置き換えるため、終了コードとシグナルは実行したコマンドのものになります。

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
