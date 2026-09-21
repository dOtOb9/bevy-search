# 画面に表示する（Camera・Sprite・Assets）

ここまでの章はすべて`println!`で確認してきましたが、ここで初めて画面に何か描画します。必要な要素は3つ、**Camera**（映す視点）・**AssetServer**（画像を読み込む）・**Sprite**（画像を表示するComponent）です。

## Camera

2Dで何かを描画するには、`Camera2d`を持つEntityが最低1つ必要です。これが無いと、Spriteを配置しても何も映りません。

```rust
commands.spawn(Camera2d);
```

Plugin章のRequired Componentsの仕組みにより、`Camera2d`を付けるだけで、実際に必要な内部的なComponent（レンダリング対象の設定など）は自動的に補われます。

## AssetServerとHandle

画像などのファイルを読み込むには、Resource章で名前だけ挙げた`AssetServer`を使います。

```rust
fn setup(asset_server: Res<AssetServer>) {
    let handle: Handle<Image> = asset_server.load("icon.png");
}
```

`.load(path)`はファイルを指定するとすぐに`Handle<Image>`を返しますが、この時点では読み込みはまだ終わっていません（バックグラウンドで非同期に読み込まれます）。`Handle<T>`は「その資産（アセット）への参照券」のようなもので、実データそのものではありません。同じ`Handle`を複数のEntityで使い回す（`.clone()`する）と、画像データ自体は1つだけロードされたまま共有されます。多数のAgentに同じアイコンを表示するような場面では、Entityの数だけ`load`し直す必要はなく、1回読み込んだ`Handle`を使い回すのが基本です。

パスの基準は、実行ディレクトリ（`cargo run`する場所、通常は`code/`）からの`assets/`フォルダです。例えば`code/assets/icon.png`を置けば、`"icon.png"`という指定で読み込めます。

## Sprite

`Sprite`は、`Handle<Image>`を実際に画面に表示するためのComponentです。

```rust
commands.spawn(Sprite::from_image(handle));
```

これまで通り`Transform`と組み合わせれば、画面上の位置を指定できます。

```rust
commands.spawn((
    Sprite::from_image(handle),
    Transform::from_xyz(100.0, 0.0, 0.0),
));
```

## 試してみる

まず`code/assets/icon.png`に、適当な小さいPNG画像を1枚置いてください（手持ちの画像で構いません）。そのうえで`code/examples/rendering_basics.rs`を作り、以下を書いてください。

```rust
use bevy::prelude::*;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    let handle: Handle<Image> = asset_server.load("icon.png");

    commands.spawn((
        Sprite::from_image(handle.clone()),
        Transform::from_xyz(-100.0, 0.0, 0.0),
    ));
    commands.spawn((
        Sprite::from_image(handle),
        Transform::from_xyz(100.0, 0.0, 0.0),
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}
```

```
cargo run --example rendering_basics
```

同じ画像が左右2箇所に表示されれば成功です。2つ目の`Sprite`は1つ目と同じ`Handle`を`.clone()`して使っているので、画像自体は1回しか読み込まれていません。
