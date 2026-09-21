# Input（キーボード・マウス）

## キー入力: `ButtonInput<KeyCode>`

キーの状態は`ButtonInput<KeyCode>`というResourceから取得します。Resource章で見た通り、これも`Res<T>`でSystemの引数に書くだけで使えます。

```rust
fn toggle_pause(keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Space) {
        println!("toggled");
    }
}
```

代表的なメソッドは3つあります。

- `.pressed(key)` — そのキーが**今、押されている間**ずっと`true`（押しっぱなしを検知したい場合）
- `.just_pressed(key)` — 押した**その1フレームだけ**`true`（トグル操作やクリック的な単発の反応に向いている）
- `.just_released(key)` — 離した**その1フレームだけ**`true`

シミュレーションの一時停止/再開のような「1回押したら切り替わる」操作には`just_pressed`が向いています。

## マウス入力: `ButtonInput<MouseButton>`とカーソル位置

マウスボタンもキーボードと同じ`ButtonInput<MouseButton>`で扱います。

```rust
fn click(mouse: Res<ButtonInput<MouseButton>>) {
    if mouse.just_pressed(MouseButton::Left) {
        println!("clicked");
    }
}
```

カーソルの座標は、`Window`Componentの`cursor_position()`から取得します。`DefaultPlugins`が起動時に自動でメインウィンドウのEntityを1つ作ってくれているので、`Query<&Window>`で取得できます。

```rust
fn click_position(mouse: Res<ButtonInput<MouseButton>>, windows: Query<&Window>) {
    if mouse.just_pressed(MouseButton::Left) {
        if let Ok(window) = windows.single() {
            if let Some(position) = window.cursor_position() {
                println!("clicked at {:?}", position);
            }
        }
    }
}
```

Query章で説明した通り`.single()`は`Result`を返すので、`if let Ok(...)`で受けます。カーソルがウィンドウの外にある場合は`cursor_position()`自体が`None`を返すため、さらに`if let Some(...)`で受けています。

## シミュレーションでの使い道

避難シミュレーションのようなものだと、Inputはゲームのような複雑な操作というより、**観察・操作パネル的な役割**が中心になりやすいです。

- スペースキーでシミュレーションの一時停止/再開
- クリックで出口や障害物を配置する
- ドラッグでカメラを動かして俯瞰する

「一時停止中/実行中」のような状態の持ち方自体は、次の「State」章でもう少しちゃんと扱います。

## UE5との対比

UE5にはレガシーな`InputComponent`のバインド方式と、より新しい「Enhanced Input」（Input Action/Mapping Contextでキー割り当てをデータ化する仕組み）がありますが、いずれも「入力→バインドされた関数を呼ぶ」というイベント駆動の設計です。

Bevyの`ButtonInput<T>`はそれよりずっと素朴で、「今このキーはどういう状態か」を毎フレーム自分から見に行く（ポーリングする）だけの仕組みです。Enhanced InputのようなアクションマッピングやコンテキストごとのRebind機能はBevy本体には無く、必要なら`leafwing-input-manager`のようなサードパーティのcrate（Plugin章で名前だけ挙げたものです）を使うのが一般的です。

## 試してみる

`code/examples/input.rs`を作り、以下を書いてください。

```rust
use bevy::prelude::*;

#[derive(Resource, Default)]
struct Running(bool);

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.insert_resource(Running(true));
}

fn toggle_pause(keys: Res<ButtonInput<KeyCode>>, mut running: ResMut<Running>) {
    if keys.just_pressed(KeyCode::Space) {
        running.0 = !running.0;
        println!("running: {}", running.0);
    }
}

fn click_position(mouse: Res<ButtonInput<MouseButton>>, windows: Query<&Window>) {
    if mouse.just_pressed(MouseButton::Left) {
        if let Ok(window) = windows.single() {
            if let Some(position) = window.cursor_position() {
                println!("clicked at {:?}", position);
            }
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (toggle_pause, click_position))
        .run();
}
```

```
cargo run --example input
```

スペースキーを押すたびに`running: true`/`running: false`が切り替わり、ウィンドウ内をクリックするたびに座標が出力されれば成功です。
