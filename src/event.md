# Event

## Eventとは

ResourceやComponentが「今、何であるか・どういう状態か」という**持続する状態**を表すのに対し、Eventは「今、何かが起きた」という**一瞬の出来事の通知**です。一度読まれる（あるいは一定時間読まれない）と役目を終えて消えます。

例えば「プレイヤーが壁にぶつかった」というのはEvent向きです。ぶつかった瞬間だけ意味があり、「ぶつかっている状態」をずっと保持しておきたいわけではないからです。

## 使い方

**1. 型を定義する**

```rust
#[derive(Event)]
struct Bounced;
```

**2. Appに登録する**

```rust
App::new()
    .add_event::<Bounced>()
    // ...
```

**3. 送る側（EventWriter）**

```rust
fn movement(mut bounced: EventWriter<Bounced>) {
    bounced.send(Bounced);
}
```

**4. 受け取る側（EventReader）**

```rust
fn on_bounced(mut events: EventReader<Bounced>) {
    for _ in events.read() {
        println!("bounced!");
    }
}
```

`.read()`は、前回このSystemが読んでから新しく送られたEventすべてを返すイテレータです。

## なぜEventを使うのか（Systemの疎結合）

`movement`が直接`println!`を呼んでもいいのに、わざわざEventを経由する理由は、**送る側が受け取る側を知らなくていい**ようにするためです。`movement`は「跳ね返ったこと」を`Bounced`として送るだけで、それを誰が・いくつのSystemが受け取って何をするかには一切関知しません。効果音を鳴らすSystem、スコアを加算するSystem、画面を揺らすSystemなど、後から受け取る側だけをいくつでも追加できます。

## UE5との対比

これはUE5のDelegate（特にマルチキャストデリゲート／Event Dispatcher）とほぼ同じ発想です。UE5でも、あるActorが「何かが起きた」ことをBroadcastし、それを購読（Bind）している側がいくつあっても、Broadcastする側は購読者の存在を意識しません。

Bevyの`EventWriter`/`EventReader`は、このBroadcast/Bindの関係をECSの中で表現したものだと考えると分かりやすいです。ただしUE5のDelegateが「関数ポインタ的にBindした相手を直接呼び出す」のに対し、Bevyの`EventReader`は「毎フレーム、溜まっているEventを能動的に見に行く（ポーリングする）」形になっている点は違います。

## 試してみる

`code/examples/event.rs`を作り、以下を書いてください。Positionが端まで来たら跳ね返り、跳ね返った瞬間に`Bounced`イベントを送る例です。

```rust
use bevy::prelude::*;

#[derive(Component)]
struct Position(f32, f32);

#[derive(Component)]
struct Velocity(f32, f32);

#[derive(Event)]
struct Bounced;

fn setup(mut commands: Commands) {
    commands.spawn((Position(0.0, 0.0), Velocity(50.0, 0.0)));
}

fn movement(
    mut query: Query<(&mut Position, &mut Velocity)>,
    time: Res<Time>,
    mut bounced: EventWriter<Bounced>,
) {
    let dt = time.delta_secs();
    for (mut position, mut velocity) in &mut query {
        position.0 += velocity.0 * dt;
        if position.0 > 100.0 || position.0 < 0.0 {
            velocity.0 = -velocity.0;
            bounced.send(Bounced);
        }
    }
}

fn on_bounced(mut events: EventReader<Bounced>) {
    for _ in events.read() {
        println!("bounced!");
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<Bounced>()
        .add_systems(Startup, setup)
        .add_systems(Update, (movement, on_bounced).chain())
        .run();
}
```

```
cargo run --example event
```

x座標が0〜100の間を往復しつつ、端に着いて跳ね返るたびにコンソールに`bounced!`が出力されれば成功です。
