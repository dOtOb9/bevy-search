# bevy-search

Bevyを1年ほど使ってきたが仕組みをちゃんと理解できていなかったため、自分用の学習用チュートリアルをmdBookでまとめているプロジェクト。

## 構成

- リポジトリルート = mdBook本体（`book.toml`, `src/SUMMARY.md`, `src/*.md`）
- `code/` = 実際に動かすBevyのコード（独立したCargoプロジェクト）
- ビルド出力の`book/`と`code/target`は`.gitignore`済み

## 役割分担・進め方

**フェーズ1: Bevy基礎のチュートリアル執筆（完了）**

- **Claude**: `src/`配下のチュートリアル本文と`SUMMARY.md`の章構成を直接編集する。書き直しやリライトも都度行ってよい。
- **ユーザー**: `code/`内のBevyのコードは自分で書いて`cargo run`で試す。コードを書くこと自体が学習目的なので、Claudeはコードを書かず、必要なら説明・レビューに留める。
- **進める順番**: 新しいトピックに入るときは、まずClaudeがmdBook側にそのトピックの説明・手順（コード例込み）を章として書く。その後ユーザーが`code/`で実際に試す、という順序（本を読んでから手を動かすチュートリアル形式）。
- `code/`のコードは、章ごとにCargoの`examples/`機能（`code/examples/<章名>.rs`、`cargo run --example <章名>`）で分ける。Bevy公式のexamplesリポジトリと同じ構成。

ECSの基礎からカメラ操作（Gizmos、`bevy_panorbit_camera`）までを一通りこの形式でカバーし終えた時点で、ユーザーがこのフェーズを完了と判断した。

**フェーズ2: 避難シミュレーションのプロトタイプ実装（現在）**

- ユーザーが「コードを書いていくだけにしたい」と明言。以降、新しいトピックのたびにmdBookの章を先に書く、という進め方はしない。
- Claudeは新しい章を自発的には書かず、質問に答える・コードをレビューする・頼まれたときだけ手伝う、という反応的な関わり方に切り替える。ドキュメント化してほしいとユーザーが明示的に頼んだ場合のみ、章を書く。
- プロトタイプの構成は機能ごとにPluginへ分ける方針（例: `AgentPlugin`＝Agent生成と移動、`SimulationPlugin`＝State管理、`DebugVisualsPlugin`＝Gizmos可視化とカメラ操作）。
- コードの置き場所は、実質のロジック（各Plugin）を`code/src/`配下にライブラリとして置き、`code/examples/`は`App`を組み立てて`.run()`するだけの薄いエントリポイントにする。`bevy_panorbit_camera`のような外部Pluginクレート自体と同じ構造。チュートリアル期の「1章1ファイル完結」の`examples/`運用とは役割が変わる。

## mdBookに関する注意

- `code/`フォルダをVSCodeで別ウィンドウ/別フォルダとして開くと、Source Control拡張が自動で`.git`を初期化してしまうことがあった。`code/`単体では新たにgit初期化しない（リポジトリはルートの1つだけ）。
- GitHubリポジトリ: `https://github.com/dOtOb9/bevy-search` (Public)
