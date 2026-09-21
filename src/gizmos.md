# Gizmos: 座標を見ながら確認する

コードで座標を指定して`cargo run`で確認、を繰り返すのはつらいので、せめて「今どこに何があるか」を画面上で直接確認できるようにしておくと楽になります。`Gizmos`は、そのためのデバッグ描画専用の仕組みです。

## Gizmosとは

`Gizmos`は、`Query`や`Commands`と同じように**Systemの引数として使えるSystemParamの一種**です。特別な設定は不要で、これまで使ってきた`DefaultPlugins`の中に最初から含まれています。

```rust
fn draw_debug(mut gizmos: Gizmos) {
    gizmos.line_2d(Vec2::new(-100.0, 0.0), Vec2::new(100.0, 0.0), Color::srgb(0.5, 0.5, 0.5));
    gizmos.circle_2d(Vec2::new(0.0, 0.0), 20.0, Color::srgb(0.9, 0.2, 0.2));
}
```

線（`line`/`line_2d`）や円（`circle`/`circle_2d`）など、よく使う図形を描く関数が揃っています。3D用は`Vec3`、2D用は`Vec2`を使う`_2d`付きのメソッドを使います。

## 「その場限り」の描画であること

`Gizmos`で描いた図形はEntityとして残るものではなく、**その1フレームだけ**表示される一時的な描画です。ずっと表示し続けたい場合は、毎フレーム実行される`Update`のSystemの中で、毎回描き直す必要があります。EntityとComponentのように「1回spawnすれば存在し続ける」ものとは仕組みが違う、という点に注意してください。

## シミュレーションでの使い道

避難シミュレーションなら、例えば以下のような使い方ができます。

- 各Agentの座標に円を描いて、実際にどこにいるか一目で確認する
- Agentの検知範囲（センサー半径）を円で重ねて可視化する
- 壁や出口の位置を線で描いて、Agentの動きと照らし合わせる

コードで座標だけ見て想像するより、実際に描いて確認しながら数値を調整できるので、Transform章やQuery章で書いた移動ロジックのデバッグにもそのまま使えます。

## UE5との対比

UE5の`DrawDebugSphere`/`DrawDebugLine`（`DrawDebugHelpers.h`、あるいはBlueprintの`Draw Debug`系ノード）とほぼ同じ発想です。永続的なActorを置かずに、その場限りのデバッグ用図形を描く、という目的も使い方もよく似ています。

## 試してみる

`code/examples/gizmos.rs`を作り、以下を書いてください。3体のAgent（円）と、境界線（線）を描く例です。

```rust
use bevy::prelude::*;

#[derive(Component)]
struct Position(f32, f32);

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn(Position(-100.0, 0.0));
    commands.spawn(Position(100.0, 0.0));
    commands.spawn(Position(0.0, 80.0));
}

fn draw_agents(mut gizmos: Gizmos, query: Query<&Position>) {
    for position in &query {
        gizmos.circle_2d(
            Vec2::new(position.0, position.1),
            20.0,
            Color::srgb(0.9, 0.2, 0.2),
        );
    }

    gizmos.line_2d(
        Vec2::new(-200.0, -150.0),
        Vec2::new(200.0, -150.0),
        Color::srgb(0.5, 0.5, 0.5),
    );
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, draw_agents)
        .run();
}
```

```
cargo run --example gizmos
```

赤い円が3つと、下の方に灰色の線が1本表示されれば成功です。`Position`の値を変えて、円の位置がどう動くか確認してみてください。
