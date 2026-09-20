# Resource

System章で`Res<Counter>`・`ResMut<Counter>`を説明抜きで使ってきたので、ここで正式に整理します。

## Resourceとは

Componentが「Entityごとに何個でも存在しうるデータ」なのに対し、Resourceは**アプリ全体でただ1つだけ存在するデータ**です。プレイヤーの数だけ`Position`があるのは自然ですが、「現在のスコア」や「経過時間」はアプリ全体で1つあれば十分で、特定のEntityに属するものでもありません。こういったデータをResourceとして扱います。

```rust
#[derive(Resource, Default)]
struct Score(u32);
```

書き方はComponentとほぼ同じ（`#[derive(Resource)]`を付けた普通のstruct）で、Entityに付与するか、アプリ全体に1つだけ登録するかという使われ方の違いだけです。

## 登録方法

`Commands`経由か、`App`に直接、どちらでも登録できます。

```rust
// Systemの中から、Commands経由で登録
fn setup(mut commands: Commands) {
    commands.insert_resource(Score::default());
}

// main側で、Appに直接登録
App::new()
    .insert_resource(Score::default())
    .run();
```

`Default`を実装している型なら、値を書かずに`init_resource::<Score>()`でも登録できます。

## アクセス方法

Systemの引数に`Res<T>`（読み取り専用）・`ResMut<T>`（書き込み可能）を書くだけで、Bevyが該当のResourceを渡してくれます。これはQuery章・System章で説明した仕組みと同じで、`Res`/`ResMut`もComponentの読み書きと同様に、どのSystemがどのResourceを読み書きするかが型から分かるため、衝突しないSystem同士は並列実行されます。

```rust
fn add_score(mut score: ResMut<Score>) {
    score.0 += 10;
}
```

## 補足: `Res<T>`はなぜ`&T`ではないのか

Query章では`Query<&Position>`のように`&`が直接見える書き方をしましたが、`Res<Time>`のように`Res`/`ResMut`には`&`が出てきません。これは担っている役割のレイヤーが違うためです。

- `Query<&Position>`の`&Position`は「Entityごとにどんなデータの形が欲しいか」を表す指定（`WorldQuery`という仕組み）です。1つ1つのEntityを覗いた時に手に入るのが文字通り`&Position`という参照なので、そのまま`&`で書けます。
- `Res<T>`・`ResMut<T>`自体は、Systemの引数として渡される専用のラッパー構造体（`SystemParam`という仕組み。`Query`自体もこの`SystemParam`の一種です）です。ただの参照ではなく、「これは何のResourceへのアクセスか」「変更検知用のタイムスタンプ」といった付加情報を一緒に抱えているため、生の`&T`ではなく専用の型として定義されています。同じ理由で、Event章で扱う`EventWriter`/`EventReader`も`&`の付かない専用のラッパー型です。

ただし`Res<T>`は`Deref`（`ResMut<T>`は`DerefMut`も）を実装しているので、中身のメソッドをそのまま呼び出せます。書き方は違っても、使い勝手はほぼ「参照っぽく」扱えます。

## 組み込みのResource

`DefaultPlugins`は、自分で登録しなくても最初から使えるResourceをいくつも用意してくれています。代表的なものに、経過時間を管理する`Time`や、アセットの読み込みを行う`AssetServer`があります。これらも普通のResourceなので、使うときは同じように`Res<Time>`のようにSystemの引数へ書くだけです。

## UE5との対比

UE5でいう`GameInstance`や`GameState`、あるいはSubsystemに近い立ち位置です。レベル（シーン）をまたいで存在し続ける、アプリ全体で1つだけのオブジェクトにデータを持たせる、という発想は同じです。ただしBevyのResourceは「ただのデータ」でしかなく、UE5の`GameInstance`のように自分自身にふるまい（メソッド）を実装することは想定されていません。ふるまいはあくまでSystem側に外出しする、というBevy全体の設計方針がここでも一貫しています。

## 試してみる

`code/examples/resource.rs`を作り、以下を書いてください。Component（Entityごとの`Position`）とResource（全体で1つの`TotalDistance`）の違いが分かるように、複数のEntityの移動量をResourceに集計する例にしています。

```rust
use bevy::prelude::*;

#[derive(Component)]
struct Position(f32, f32);

#[derive(Component)]
struct Velocity(f32, f32);

#[derive(Resource, Default)]
struct TotalDistance(f32);

fn setup(mut commands: Commands) {
    commands.spawn((Position(0.0, 0.0), Velocity(1.0, 0.0)));
    commands.spawn((Position(0.0, 0.0), Velocity(0.0, 2.0)));
    commands.insert_resource(TotalDistance::default());
}

fn movement(mut query: Query<(&mut Position, &Velocity)>, mut total: ResMut<TotalDistance>) {
    for (mut position, velocity) in &mut query {
        position.0 += velocity.0;
        position.1 += velocity.1;
        total.0 += (velocity.0.powi(2) + velocity.1.powi(2)).sqrt();
    }
}

fn print_total(total: Res<TotalDistance>) {
    println!("total distance moved: {}", total.0);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (movement, print_total).chain())
        .run();
}
```

```
cargo run --example resource
```

2つのEntityがそれぞれ別方向に動きつつ、その移動量の合計が1つの`TotalDistance`にまとまって毎フレーム増えていけば成功です。
