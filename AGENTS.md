# dollar-cmd

## 概要

`$`という名前でPATHに置くための実行ファイル。
記事から貼り付けた`$ cmd args`の行を、プロンプト記号ごとそのまま実行できるようにするジョークコマンド。

Rust製の単一バイナリで、実装は`src/main.rs`のみ。
LinuxとmacOS向けにリリースする。

## 開発コマンド

ツールはmiseで管理する（`mise install`で導入）。
ツールチェインは`rust-toolchain.toml`でnightlyに固定する。
`rustfmt.toml`がnightly限定のオプションを使うため、stableのrustfmtは警告を出して設定を無視する。

- ビルド: `cargo build --release --locked`
- テスト: `cargo test --all-features`
- lint: `cargo clippy --all-targets --all-features -- -D warnings`
- フォーマット: `mise exec -- treefmt`（CIは`--fail-on-change`付きで検査する）
- カバレッジ: `mise exec -- cargo llvm-cov`

## 設計方針

シェルとしての透過性を最優先する。
`exec`で自身を対象プロセスに置き換え、終了コード・シグナル・引数を加工せずそのまま渡す。
コマンド名が確定した後の引数は一切解釈しない。

## コーディング規約

- フォーマットは手で行わず`mise exec -- treefmt`に任せる
- テストは`tests/cli.rs`に、ビルド済みバイナリを起動する結合テストとして追加する。
  関数名は`positive_`/`negative_`を接頭辞にし、rstestの`#[case::...]`名にも同じ接頭辞を付ける
- 外部から見える挙動（終了コード、オプション解釈、使い方）を変えたら、`README.md`と`README-ja.md`を同じPRで両方更新する

## リリース・依存更新

- バージョン・`CHANGELOG.md`・`.github/release-please/manifest.json`はrelease-pleaseが所有する
- 依存の更新はRenovateが担う
