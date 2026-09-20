# Plugin

## Pluginとは

BevyのPluginは、`&mut App`を受け取って好きなようにセットアップする、ただの1メソッドのtraitです。

```rust
pub trait Plugin: Send + Sync + 'static {
    fn build(&self, app: &mut App);
}
```

例えば「起動時に一度だけ挨拶を出す」だけのPluginはこう書けます。

```rust
use bevy::prelude::*;

struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, say_hello);
    }
}

fn say_hello() {
    println!("Hello!");
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, HelloPlugin))
        .run();
}
```

`build`の中でやっていることは、`main`に直接`app.add_systems(...)`と書くのと本質的に同じです。Pluginは「Appのセットアップ処理をひとまとめにして名前を付け、使い回せるようにしたもの」に過ぎません。

## UE5のPluginとの違い

UE5のPluginを知っている前提で、対比すると分かりやすいです。

| | UE5のPlugin | BevyのPlugin |
|---|---|---|
| 実体 | `.uplugin`という独立したパッケージ（コード＋アセット＋Blueprintを含められる） | Rustの`trait`を実装しただけの型（コードのみ） |
| 有効/無効 | エディタのPlugin管理画面でON/OFF、ランタイムでもロード/アンロード可能 | コンパイル時に`.add_plugins()`で組み込むかどうかを決める。実行時の動的なON/OFFは無い |
| 配布 | Marketplaceや`.uplugin`ファイル単位で配布 | crates.ioに公開された普通のRustクレートとして配布（例: `bevy_egui`, `leafwing-input-manager`はいずれもPluginを提供するクレート） |
| 中身 | 独自のモジュールシステム、ビルドターゲットを持つ | 単なる「`&mut App`を受け取る関数」なので、特別な仕組みは無い |

つまりUE5のPluginが「独立した配布可能なパッケージ（アセットもコードも含む）」であるのに対し、BevyのPluginは「Appの初期化コードをまとめるための、ただの設計パターン」です。パッケージング・配布・ロード機構としての役割はRustのクレートシステム（Cargo）がそのまま担っていて、Plugin自体はその中でAppの組み立て方を宣言するための薄い規約でしかありません。

## PluginGroup: 複数のPluginをまとめる

これまで使ってきた`DefaultPlugins`は、実は単体のPluginではなく`PluginGroup`です。ウィンドウ表示・レンダリング・入力・アセット管理など、独立した多数のPluginをひとまとめにしたグループになっています。

`PluginGroup`は個別のPluginを無効化したり設定を上書きしたりできます。

```rust
App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "My Bevy Game".into(),
            ..default()
        }),
        ..default()
    }))
    .run();
```

`DefaultPlugins`自体も「複数のPluginをまとめて名前を付けたもの」という点では、自分で書く`HelloPlugin`と同じ発想の延長線上にあります。

## なぜPluginに分けるのか

- アプリが大きくなるにつれ、関連するSystem・Resource・Eventをひとまとまりにして見通しを良くするため
- サードパーティのcrateが機能を提供する際の標準的な公開単位になっているため（`cargo add`したcrateのPluginを`.add_plugins()`するだけで機能が組み込める）
- `DefaultPlugins`のように、まとめて追加しつつ個別に設定を上書きする、という柔軟な組み合わせ方ができるため

## 試してみる

`code/examples/plugin.rs`を作り、上記の`HelloPlugin`の例を書いてください。

```
cargo run --example plugin
```

ウィンドウが表示され、コンソールに`Hello!`が一度だけ出力されれば成功です。
