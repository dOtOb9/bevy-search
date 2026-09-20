# mdBookについて

このチュートリアル自体は[mdBook](https://rust-lang.github.io/mdBook/)というツールで書かれています。本編（Bevy）に入る前に、道具であるmdBook自体の仕様をまとめておきます。

## mdBookとは

RustでできたMarkdown専用の静的サイトジェネレータです。RustのMarkdownパーサーで本文を読み込み、HTMLの本（左サイドバーに章立て、検索、テーマ切り替え付き）を生成します。Rust公式の["The Rust Programming Language"](https://doc.rust-lang.org/book/)やBevy公式のBevy Bookもこのツールで書かれており、Rustエコシステムでは事実上の標準的なドキュメントツールです。

Docusaurus（React/Node.jsベース）と比べると機能は絞られていますが、その分ビルドが速く、依存関係もCargo一つで完結します。

## インストールと初期化

```
cargo install mdbook
```

プロジェクトのルートで以下を実行すると雛形が生成されます。

```
mdbook init
```

対話式で本のタイトルを聞かれ、以下が生成されます。

- `book.toml` — 本全体の設定ファイル
- `src/SUMMARY.md` — 章立て（目次）を定義するファイル
- `src/chapter_1.md` — 最初の章のスタブ
- `.gitignore` — ビルド出力先（`book`）を無視する設定

## book.toml

```toml
[book]
title = "bevy-tutorial"
authors = ["dotob9"]
language = "en"
```

`authors`はローカルのgit設定（`git config user.name`）から自動で拾われます。`cargo init`と同じ仕組みです。日本語で書く場合は`language`を`"ja"`にしておくと、HTMLの`lang`属性や検索の挙動が日本語向けになります。

## プレビュー

```
mdbook serve --open
```

ローカルサーバーが立ち上がり、ブラウザが自動で開きます。ファイルを保存するたびにライブリロードされます。

Docusaurusよりも体感でかなり速いですが、これは設計思想の違いによるものです。mdBookはRustのネイティブバイナリが「Markdownをパースしてテンプレートに流し込むだけ」の単純な処理をするのに対し、DocusaurusはNode.js上でReact/MDXのコンパイルとwebpack（またはRspack）によるバンドルという重い処理を毎回行っています。生成物もmdBookは素朴な静的HTMLですが、Docusaurusは各ページがReactコンポーネントとしてハイドレーションされるSPA相当の作りになっている、という違いもあります。

## SUMMARY.mdの文法

`src/SUMMARY.md`が唯一の目次定義ファイルです。ここに書いた構造がそのままサイドバーになります。

**基本（リスト＝章）**

```markdown
- [表示タイトル](./path/to/file.md)
```

**ネスト（親子関係のある章）**

```markdown
- [ECSの基礎](./ecs/index.md)
  - [Entity](./ecs/entity.md)
  - [Component](./ecs/component.md)
```

2スペースインデントで階層化できます。ネストした親章は、サイドバー上で自動的に折りたたみ可能なツリー表示になります（これは標準機能であり裏技ではありません）。ただし本当に親子関係がある内容にだけ使うべきで、見た目を畳みたいだけの理由でネストするのは避けます。

**パート分け（グループの見出し）**

```markdown
# Summary

- [はじめに](./intro.md)

# 基礎編

- [ECS](./ecs/index.md)
```

リストの間に`#`見出しを挟むと、リンクを持たない「区切り」としてサイドバーに表示されます。

**未着手の章（ドラフト）**

```markdown
- [未着手の章]()
```

リンク先を空にすると、クリックできないグレー表示のドラフト章になります。目次だけ先に作っておきたいときに使えます。

## {{#include}}によるコードの埋め込み

このチュートリアルでは、Bevyの実コードは本文（`src/`）とは別に`code/`フォルダのCargoプロジェクトに置いています（Rust公式の"The Book"と同じ構成）。本文からは`{{#include}}`記法で外部ファイルの中身をそのまま埋め込めます。

```markdown
{{#include ../code/src/main.rs}}
```

行範囲を指定することもできます。

```markdown
{{#include ../code/src/main.rs:1:10}}
```

コード中に`// ANCHOR: setup` 〜 `// ANCHOR_END: setup`という目印コメントを置いておけば、その区間だけを名前で埋め込むこともできます。

```markdown
{{#include ../code/src/main.rs:setup}}
```

これにより、本文と実際に動くコードを別々に管理しつつ、本文側は常に最新のコードを表示できます。

`{{#include}}`とANCHORの仕組みはRust専用ではなく、単に「テキストの中に`ANCHOR: 名前`という文字列を含む行を探す」だけの言語非依存の機能です。Python（`# ANCHOR: xxx`）でもJS（`// ANCHOR: xxx`）でもHTML（`<!-- ANCHOR: xxx -->`）でも同じように使えます。コードのシンタックスハイライトも、フェンス直後に言語名を書けば多言語に対応します（\`\`\`python など）。mdBookはRustコミュニティ発のツールですが、中身は汎用のMarkdown本ビルダーです。

なお、Rustのコードブロックだけは`# `で始まる行を「コンパイルはされるが本文には表示しない」隠し行として扱う慣習があります（rustdocのdoctestと同じ発想）。これはANCHORとは別の、Rust固有の機能です。

## 拡張性の範囲

mdBookは「本」という形（線形の章立て＋１つのサイドバー＋検索）に用途を絞ったツールで、Docusaurusや一般的な静的サイトジェネレータのように、トップページ・ブログ・ドキュメントを組み合わせた個人サイトのような自由なページ構成を作る拡張性はありません。カスタマイズできる範囲は以下に限られます。

- `book.toml`の`additional-css`/`additional-js`で見た目や挙動に手を加える
- `theme/`フォルダでHTML/CSS/JSテンプレート自体を丸ごと差し替える（ただし「本」という構造自体は変わらない）
- プリプロセッサ（`mdbook-mermaid`で図、`mdbook-katex`で数式、`mdbook-admonish`で注意書きボックスなど）でMarkdownの表現力を拡張する

これらはあくまで本の中身をリッチにするための拡張であり、ページごとに全く違うレイアウトを作ったり、Reactコンポーネントを自由に埋め込んだりはできません。ただし今回のように「Bevyを理解するための本」という用途には過不足なく合っています。

具体的に標準機能に無いものとしては以下があります。必要になったら都度プリプロセッサ導入などを検討します。

- ページ内アウトライン（Docusaurusの「On this page」に相当する、右側の見出し一覧＋スクロール追従）
- MDXのようなコンポーネント埋め込み（動くデモをページに埋め込むといったこと）
