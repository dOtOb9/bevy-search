# EntityとComponent

これまでの章で`Commands::spawn`や`#[derive(Component)]`を説明抜きで使ってきたので、ここで正式に整理します。

## Entity

Entityは、それ自体は何のデータも持たない、ただの**ID**です。実体はインデックスと世代番号（同じインデックスが再利用されたときに区別するための番号）を組み合わせた数値で、名前や座標のような情報は一切含みません。

「このEntityが何であるか」は、Entity自体ではなく、**そのEntityに紐づいているComponentの組み合わせ**によって決まります。Positionを持てば位置がある何か、PlayerとPositionの両方を持てばプレイヤー、という具合です。

## Component

Componentは`#[derive(Component)]`を付けた、普通のRustのstruct（またはenum）です。特別なことは何もなく、ただの型にEntityへ付与できる印を付けているだけです。

```rust
#[derive(Component)]
struct Position(f32, f32);

#[derive(Component)]
struct Player;
```

## 生成・追加・削除

`Commands::spawn`にタプルを渡すと、複数のComponentを同時に持つEntityを1つ作れます。

```rust
fn setup(mut commands: Commands) {
    commands.spawn((Position(0.0, 0.0), Player));
}
```

あとからComponentを付け外ししたい場合は、`Entity`を指定して操作します。

```rust
fn add_and_remove(mut commands: Commands, entity: Entity) {
    commands.entity(entity).insert(Player);      // 追加
    commands.entity(entity).remove::<Player>();  // 削除
    commands.entity(entity).despawn();           // Entityごと削除
}
```

## マーカーComponent

`Player`のように中身を持たないComponent（ユニット構造体）は、データを持たせる目的ではなく、「このEntityは○○である」という印を付けるためだけに使います。Query章で紹介した`With<Player>`・`Without<Player>`は、まさにこのマーカーComponentの有無で絞り込むためのものです。

## EntityそのものをQueryで取る

Queryの型引数に`Entity`を含めると、Component自体ではなくEntityのIDを受け取れます。`Commands`と組み合わせて、条件に合うEntityを操作したいときに使います。

```rust
fn despawn_non_players(
    mut commands: Commands,
    query: Query<Entity, (With<Position>, Without<Player>)>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
```

## UE5との対比

UE5のActorは、識別子・自分自身のふるまい（`Tick`などの仮想関数）・Componentの集合をすべて1つのクラスインスタンスとして抱え込む、比較的重量級のオブジェクトです。ActorComponentも同様に、自分の`Tick`や仮想関数を持てるクラスです。

BevyのEntityは本当にただの数値のIDで、ふるまいを一切持ちません。Componentもただのデータ（struct）で、`Tick`のような自分自身の処理を持つことはできません。ふるまいは完全にSystem側に外出しされていて、「データ（Entity/Component）」と「ふるまい（System）」がはっきり分離しているのがBevy（ECS）の特徴です。

## 試してみる

`code/examples/entity_component.rs`を作り、以下を書いてください。

```rust
use bevy::prelude::*;

#[derive(Component)]
struct Position(f32, f32);

#[derive(Component)]
struct Player;

fn setup(mut commands: Commands) {
    commands.spawn((Position(0.0, 0.0), Player));
    commands.spawn(Position(5.0, 5.0));
}

fn list_entities(query: Query<(Entity, &Position)>) {
    for (entity, position) in &query {
        println!("{entity:?}: ({}, {})", position.0, position.1);
    }
}

fn despawn_non_players(
    mut commands: Commands,
    query: Query<Entity, (With<Position>, Without<Player>)>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (list_entities, despawn_non_players).chain())
        .run();
}
```

```
cargo run --example entity_component
```

最初のフレームでは2つのEntityが表示されますが、`Player`を持たない方のEntityが`despawn_non_players`で削除されるため、2フレーム目以降は1つ（Playerの方）だけが表示され続けるはずです。

## 補足: `println!("{entity:?}")`という書き方

`list_entities`の中で使った`{entity:?}`は、2つの要素が組み合わさった書き方です。

**変数名を直接埋め込む記法** — Rust 2021以降、`{}`の中に変数名を直接書けます。

```rust
println!("{entity}");     // 新しい書き方
println!("{}", entity);   // 従来の書き方。意味は同じ
```

**`:?`はDebugフォーマット指定** — `{}`だけだと`Display`（人間向けのきれいな表示）というtraitが使われますが、`Entity`のような多くの型は`Display`を実装しておらず、`#[derive(Debug)]`で自動生成される`Debug`（デバッグ用の内部表現）だけを持っています。`:?`を付けると`Debug`の方でフォーマットされます。

```rust
println!("{entity}");    // Entityの場合コンパイルエラー（Displayが無い）
println!("{entity:?}");  // OK。Entity(0v1) のような内部表現が出力される
```

`{entity:?}`は「`entity`をDebugフォーマットで埋め込む」という意味で、`println!("{:?}", entity)`と同じことをより短く書いたものです。
