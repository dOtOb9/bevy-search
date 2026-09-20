# bevy-search

Bevyを1年ほど使ってきたが仕組みをちゃんと理解できていなかったため、自分用の学習用チュートリアルをmdBookでまとめているプロジェクト。

## 構成

- リポジトリルート = mdBook本体（`book.toml`, `src/SUMMARY.md`, `src/*.md`）
- `code/` = 実際に動かすBevyのコード（独立したCargoプロジェクト）
- ビルド出力の`book/`と`code/target`は`.gitignore`済み

## 役割分担・進め方

- **Claude**: `src/`配下のチュートリアル本文と`SUMMARY.md`の章構成を直接編集する。書き直しやリライトも都度行ってよい。
- **ユーザー**: `code/`内のBevyのコードは自分で書いて`cargo run`で試す。コードを書くこと自体が学習目的なので、Claudeはコードを書かず、必要なら説明・レビューに留める。
- **進める順番**: 新しいトピックに入るときは、まずClaudeがmdBook側にそのトピックの説明・手順（コード例込み）を章として書く。その後ユーザーが`code/`で実際に試す、という順序（本を読んでから手を動かすチュートリアル形式）。ユーザーが先にコードを書いて後から章にまとめる、という逆順にはしない。
- コード例が固まってきたら、本文中の説明用コードブロックは`{{#include ../code/examples/<章名>.rs:anchor名}}`のような`{{#include}}`＋ANCHORコメントに置き換えて、実際の`code/`のファイルと本文の内容がズレないようにする。
- `code/`のコードは、章ごとにCargoの`examples/`機能（`code/examples/<章名>.rs`、`cargo run --example <章名>`）で分ける。Bevy公式のexamplesリポジトリと同じ構成で、章ごとに独立して実行・見返せるようにするため。`code/src/main.rs`は章ごとの実験置き場としては使わない。

## mdBookに関する注意

- `code/`フォルダをVSCodeで別ウィンドウ/別フォルダとして開くと、Source Control拡張が自動で`.git`を初期化してしまうことがあった。`code/`単体では新たにgit初期化しない（リポジトリはルートの1つだけ）。
- GitHubリポジトリ: `https://github.com/dOtOb9/bevy-search` (Public)
