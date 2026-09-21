# 大量のEntityを捌く（パフォーマンスと並列実行の深掘り）

System章では「衝突しない複数のSystem同士が並列実行される」という話をしました。これはSystem単位の並列化です。ここでは、避難シミュレーションのように**1つのSystemの中で数千〜数万のEntityを処理する**場合に関係してくる話を扱います。

## そもそもQueryのループが速い理由

BevyはComponentのデータを、Entityごとにバラバラに持つのではなく、同じComponentの組み合わせ（アーキタイプ）ごとにまとめて連続したメモリ上に配置しています。`Query`でループを回すとき、CPUのキャッシュに乗りやすい連続データを順番に読むだけなので、Entityの数が多くても1件あたりの処理は非常に軽いです。これは`par_iter`を使う・使わないに関わらず効いている、ECSの基本的な設計上の利点です。

## 1つのSystem内での並列化: `par_iter`

System同士の並列化とは別に、**1つのSystemが持つQueryのループそのものを複数スレッドに分散**することもできます。

```rust
fn movement(mut query: Query<(&mut Position, &Velocity)>, time: Res<Time>) {
    let dt = time.delta_secs();
    query.par_iter_mut().for_each(|(mut position, velocity)| {
        position.0 += velocity.0 * dt;
        position.1 += velocity.1 * dt;
    });
}
```

`.iter_mut()`の代わりに`.par_iter_mut().for_each(...)`を使うだけで、対象のEntity群がバックグラウンドのスレッドプールに分割され、複数のCPUコアで同時に処理されます。Entity数が少ないうちはスレッド分散のオーバーヘッドの方が大きく逆に遅くなることもありますが、数千〜数万規模になると効果がはっきり出てきます。

## 大量生成には`spawn_batch`

Entity/Component章で使った`commands.spawn(...)`をループで何千回も呼ぶより、`spawn_batch`でまとめて生成する方が効率的です。

```rust
let agents = (0..50_000).map(|i| {
    (Position(i as f32, 0.0), Velocity(1.0, 0.5))
});
commands.spawn_batch(agents);
```

`Component`のタプルを返すイテレータを渡すだけで、指定した数のEntityを一括で生成できます。

## UE5との対比

UE5では、Actorを数千体スポーンしてそれぞれに`Tick`を持たせると、仮想関数呼び出しやUObjectそのもののオーバーヘッドが積み重なり、パフォーマンスが急激に悪化することがよく知られています。そのため実務では「大量の同種オブジェクトを動かしたいときはActorを使わず、Niagara（パーティクル）やInstanced Static Mesh Componentのような、Actorの仕組みから外れた専用の軽量な手段に逃がす」という判断がよく行われます。

Bevyはこの逆で、そもそも「大量の同種データを一括処理する」ことを前提にECSが設計されているため、Actor相当の重い仕組みから逃げる必要がありません。数万体のAgentであっても、素朴にComponentとSystemで組んだ延長線上で扱えます。

## 試してみる

`code/examples/performance.rs`を作り、以下を書いてください。5万体のAgentを生成し、`par_iter_mut`で移動させながら処理時間を計測する例です。

```rust
use bevy::prelude::*;
use std::time::Instant;

#[derive(Component)]
struct Position(f32, f32);

#[derive(Component)]
struct Velocity(f32, f32);

fn setup(mut commands: Commands) {
    let agents = (0..50_000).map(|i| (Position(i as f32, 0.0), Velocity(1.0, 0.5)));
    commands.spawn_batch(agents);
}

fn movement(mut query: Query<(&mut Position, &Velocity)>, time: Res<Time>) {
    let dt = time.delta_secs();
    let start = Instant::now();

    query.par_iter_mut().for_each(|(mut position, velocity)| {
        position.0 += velocity.0 * dt;
        position.1 += velocity.1 * dt;
    });

    println!("movement took {:?}", start.elapsed());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, movement)
        .run();
}
```

```
cargo run --example performance
```

`movement took ...`の時間を確認したら、試しに`query.par_iter_mut().for_each(...)`の部分を、通常の`for (mut position, velocity) in &mut query { ... }`ループに書き換えて、同じ5万体でどれくらい時間が変わるか比べてみてください。Entity数（`0..50_000`の数字）を変えて、どのあたりから差が出てくるかを見るのもおすすめです。
