# Query

## Queryとは

Queryは、Systemの引数として「どんなComponentを持つEntityが欲しいか」を宣言するための型です。

```rust
fn print_positions(query: Query<&Position>) {
    for position in &query {
        println!("({}, {})", position.0, position.1);
    }
}
```

`Query<&Position>`と書くだけで、`Position`というComponentを持つ**すべてのEntity**が対象になります。ループの中身は「1つのEntityに対して何をするか」だけを書けばよく、Entityが何個あるか・どのEntityが対象かはBevy側が管理します。

## 複数のComponentを取る・書き換える

タプルで書けば複数のComponentを同時に取得できます。書き換えたい場合は`&mut`を付けます。

```rust
fn movement(mut query: Query<(&mut Position, &Velocity)>) {
    for (mut position, velocity) in &mut query {
        position.0 += velocity.0;
        position.1 += velocity.1;
    }
}
```

これは「`Position`と`Velocity`の両方を持つEntityすべてに対して、Velocity分だけPositionを動かす」という処理です。`Position`だけ持っていて`Velocity`を持たないEntityは、このQueryの対象になりません。

## 絞り込み（Filter）

型引数の2番目に`With<T>`・`Without<T>`を指定すると、「Componentのデータは使わないが、持っているかどうかだけで絞り込みたい」場合に使えます。

```rust
fn player_only(query: Query<&Position, With<Player>>) {
    // Playerを持つEntityのPositionだけが対象
}
```

## 1件だけ取得する

対象が1つしかないと分かっている場合（例えばプレイヤーが1人だけのゲーム）は`.single()`が使えます。これは`Result<T, QuerySingleError>`を返し、対象が0件または2件以上だと`Err`になります（panicはしません）。特定のEntityを名指しで取得したい場合は`.get(entity)`を使います。

## 補足: `&mut`で取っているのに、並列実行と衝突しないのか

`movement`は`Query<(&mut Position, &Velocity)>`のように可変参照でComponentを取っていますが、これは実行時のロック（Mutex的な仕組み）で守られているわけではありません。Bevyは**事前の静的解析＋スケジューリング**でこれを解決しています。

1. Systemを登録する際、Bevyは各Systemの引数の型（`Query<&mut Position>`や`Res<Time>`など）を見て、「このSystemはどのComponent型・Resource型を読むか／書くか」を型情報だけから洗い出します。
2. スケジューラは、System同士のこのアクセス一覧を比較し、書き込みが重なる場合は「衝突する」と判断します。
3. 衝突しないSystem同士だけがスレッドプール上で実際に同時実行されます。衝突するSystemは、ロック待ちで止まるのではなく、**そもそも同時に走らないようにスケジュール自体が順序付け**られます。

つまり「ロックを取って待つ」のではなく「最初から衝突する組み合わせを実行しない」という設計です。ロックのオーバーヘッドや競合待ちが原理的に発生しないため、ECSが高速な理由の一つになっています。

なお、**同じSystemの中**で衝突する可能性のある複数のQueryを同時に引数に取った場合（例えば`Query<&mut Position>`と`Query<&mut Position, With<Enemy>>`を両方引数に取る、など）は、この静的解析にひっかかって実行時にpanicします。その場合は`ParamSet`という仕組みを使い、明示的にどちらか一方ずつアクセスするように書き直す必要があります。

## UE5との対比

UE5で「特定のコンポーネントを持つActorを全部集めたい」場合、`TActorIterator`で全Actorを回してキャストや`FindComponentByClass`で都度チェックする、という書き方になりがちです。「何を対象にするか」の条件と「対象に対して何をするか」の処理が、ループの中に混在します。

Bevyでは「何を対象にするか」はQueryの型（`Query<(&mut Position, &Velocity), With<Player>>`のような部分）にすべて宣言し、「対象に対して何をするか」だけをSystemの中身として書きます。対象の絞り込み自体はBevy内部のデータ構造（アーキタイプ）で効率よく処理されるので、手でループを回して都度条件判定する必要がありません。

## 試してみる

`code/examples/query.rs`を作り、以下を書いてください。`#[derive(Component)]`をつけた型はComponentとして`Commands::spawn`でEntityに付与できます（Componentの詳細はECSの章で改めて扱います）。

```rust
use bevy::prelude::*;

#[derive(Component)]
struct Position(f32, f32);

#[derive(Component)]
struct Velocity(f32, f32);

fn setup(mut commands: Commands) {
    commands.spawn((Position(0.0, 0.0), Velocity(1.0, 0.5)));
    commands.spawn((Position(10.0, 10.0), Velocity(-1.0, 0.0)));
}

fn movement(mut query: Query<(&mut Position, &Velocity)>) {
    for (mut position, velocity) in &mut query {
        position.0 += velocity.0;
        position.1 += velocity.1;
    }
}

fn print_positions(query: Query<&Position>) {
    for position in &query {
        println!("({}, {})", position.0, position.1);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (movement, print_positions).chain())
        .run();
}
```

```
cargo run --example query
```

2つのEntityそれぞれの座標が、毎フレームVelocity分だけ動きながらコンソールに出力されれば成功です。
