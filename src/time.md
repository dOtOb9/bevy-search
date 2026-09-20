# Time

前章の`resource.rs`の`movement`は、`Velocity`を毎フレームそのまま足していたため、フレームレートが変わると1秒あたりの実際の移動距離まで変わってしまう、という簡略化をしていました。それを解決するのが`Time`です。

## Timeとは

`Time`は、Resource章で少し触れた通り`DefaultPlugins`が最初から用意してくれているResourceで、時間に関する情報を保持しています。自分で登録する必要はなく、`Res<Time>`とSystemの引数に書くだけで使えます。

代表的なメソッドは以下の2つです。

- `time.delta_secs()` — 直前のフレームからの経過時間（秒）。フレームによって値が変わります。
- `time.elapsed_secs()` — アプリ起動からの累計経過時間（秒）。

## フレームレートに依存しない移動

`Velocity`を「1フレームあたりの移動量」ではなく「1秒あたりの移動量」として扱い、`delta_secs()`を掛けることで、フレームレートが変わっても1秒間に動く距離が変わらないようにできます。

```rust
fn movement(mut query: Query<(&mut Position, &Velocity)>, time: Res<Time>) {
    let dt = time.delta_secs();
    for (mut position, velocity) in &mut query {
        position.0 += velocity.0 * dt;
        position.1 += velocity.1 * dt;
    }
}
```

こうしておけば、60fpsでも30fpsでも「1秒間にどれだけ動くか」は変わらなくなります。

## 補足: `dt`を2回使っても所有権は移譲されない

`dt`は`velocity.0 * dt`と`velocity.1 * dt`の2箇所で使われていますが、これで所有権の移譲（ムーブ）が起きて2回目が使えなくなる、ということはありません。理由は`dt`の型`f32`が**`Copy`型**だからです。

Rustでムーブが起きるのは、「その型が`Copy`を実装していない」場合に代入や値渡しなど所有権を奪う操作をしたときだけです。`f32`・`i32`・`bool`・`char`のようなプリミティブ型は`Copy`を実装しているので、使うたびに実際にはビットがコピーされるだけで、元の変数はそのまま使い続けられます。

```rust
let dt: f32 = 0.016;
let a = velocity.0 * dt; // dtが使われるが、コピーされるだけ
let b = velocity.1 * dt; // なので2回目もそのまま使える
```

もし`dt`が`String`のような`Copy`を実装していない型だったら、1回目の使用で所有権が移動し、2回目に使おうとした時点で「use of moved value」というコンパイルエラーになります。`&`/`&mut`（参照）は、値をコピーせず所有権も奪わずに一時的に借用したい場合に使うものですが、今回のように単純な数値を読むだけなら`Copy`のおかげで参照すら不要です。

## UE5との対比

UE5では`Tick(float DeltaTime)`のように、`DeltaTime`が**すべての**Tick関数に強制的に渡されます。使う使わないに関わらず、シグネチャとして常に受け取る形です。

Bevyでは、Systemの引数は必要なものだけを宣言する、というこれまでの章で見てきた設計がここでも一貫していて、時間情報が欲しいSystemだけが`Res<Time>`を引数に加えます。時間を使わないSystemには一切登場しません。

## 試してみる

`code/examples/time.rs`を作り、以下を書いてください。`resource.rs`の`movement`を、`Velocity`を「1秒あたりの移動量」として扱うように書き直した例です。

```rust
use bevy::prelude::*;

#[derive(Component)]
struct Position(f32, f32);

#[derive(Component)]
struct Velocity(f32, f32);

fn setup(mut commands: Commands) {
    commands.spawn((Position(0.0, 0.0), Velocity(50.0, 0.0)));
}

fn movement(mut query: Query<(&mut Position, &Velocity)>, time: Res<Time>) {
    let dt = time.delta_secs();
    for (mut position, velocity) in &mut query {
        position.0 += velocity.0 * dt;
        position.1 += velocity.1 * dt;
    }
}

fn print_position(query: Query<&Position>, time: Res<Time>) {
    for position in &query {
        println!(
            "t={:.2}s ({:.1}, {:.1})",
            time.elapsed_secs(),
            position.0,
            position.1
        );
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (movement, print_position).chain())
        .run();
}
```

```
cargo run --example time
```

`t=1.00s`あたりでx座標がだいたい`50.0`付近になっていれば、「1秒あたり50動く」という設定通りに、フレームレートに関係なく動いていることが確認できます。
