# 起動方法

Bevyアプリの最小構成は、`App`を作ってプラグインを登録し、`run()`するだけです。

```rust
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .run();
}
```

それぞれの役割は以下の通りです。

- `App::new()` — Entity・Component・System・Pluginなどを保持する、アプリ全体の入れ物を作ります。
- `.add_plugins(DefaultPlugins)` — `DefaultPlugins`は、ウィンドウ表示・レンダリング・入力・アセット管理・時間管理など、GUIアプリとして動くために必要な一式のプラグインをまとめたグループです。これを登録しないとウィンドウは一切表示されません。
- `.run()` — イベントループを開始します。ウィンドウを閉じるかプロセスを終了するまで動き続けます。

## DefaultPluginsとMinimalPluginsの違い

`DefaultPlugins`の代わりに`MinimalPlugins`を使うこともできます。こちらはスケジューラや時間管理など最低限のものしか含まず、ウィンドウもレンダリングも行いません。CIでのヘッドレステストや、描画不要なサーバー用途で使うものなので、最初の一歩としては`DefaultPlugins`を使います。

## 試してみる

このチュートリアルでは、章ごとの実行可能な例をCargoの`examples/`機能で管理します（1章＝1ファイルの独立した実行可能サンプル。Bevy公式のexamplesリポジトリと同じ構成です）。

`code/examples/getting_started.rs`を作り、上記の内容を書いてください。

```
cargo run --example getting_started
```

初回はBevy本体と依存クレートのビルドが走るため、数分かかることがあります。ウィンドウ（デフォルトでは何も描画されていない黒い画面）が表示されたら成功です。

`code/src/main.rs`はこれ以降、章ごとの実験置き場としては使わず、`examples/`配下に章ごとのファイルを作っていきます。
