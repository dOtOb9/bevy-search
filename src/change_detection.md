# 変更検知（Changed / Added）

## Changed / Addedとは

Query章で紹介した`With<T>`・`Without<T>`は「Componentを持っているかどうか」で絞り込むフィルタでした。`Changed<T>`・`Added<T>`は、それに加えて**時間の経過**を条件にする特殊なフィルタです。

- `Added<T>` — 前回このSystemが実行されてから、`T`が新しく付与されたEntityだけが対象
- `Changed<T>` — 前回このSystemが実行されてから、`T`の中身が変更されたEntityだけが対象

```rust
fn on_position_changed(query: Query<&Position, Changed<Position>>) {
    for position in &query {
        println!("moved to ({}, {})", position.0, position.1);
    }
}
```

何も変更されていないフレームでは、このQueryは空になり、ループの中身は一度も実行されません。毎フレーム全Entityを見て回るのではなく、「実際に変わったものだけ」を効率よく処理できます。

## 仕組み

Bevyは、各Componentのデータに「いつ変更されたか」を示すタイムスタンプのようなもの（tick）を内部で付けています。Systemが`&mut`でComponentにアクセスするたび、Bevyはこのtickを更新します。各Systemは自分が「前回実行されたときのtick」を覚えていて、`Changed<T>`は「Componentのtickが、このSystemの前回実行時より新しいか」で判定しています。

これはMessage章で説明した`MessageReader`の「自分専用の読み取り位置（カーソル）」と同じ発想です。「絶対的な直前のフレーム」ではなく「このSystemが最後に見たとき」を基準にしている点が共通しています。

## 注意点: `&mut`でアクセスしただけでChangedになる

`Changed<T>`は「実際に値が変わったか」ではなく「`&mut`でアクセスされたか」で判定しています。つまり、以下のように値を書き換えていなくても、`&mut`で触れただけで次回`Changed<Position>`はtrueになります。

```rust
fn touch_only(mut query: Query<&mut Position>) {
    for mut position in &mut query {
        let _ = &mut position; // 値は変えていない
    }
}
```

このため、値を書き換える予定がないSystemでは`&mut`ではなく`&`（読み取り専用）を使うようにしないと、意図せず他のSystemの`Changed<T>`フィルタを誤発火させてしまうことがあります。

## UE5との対比

UE5には`Changed<T>`にそのまま相当する汎用の仕組みはなく、多くの場合「変更があったら自分でdirtyフラグを立てて、後で自分でチェックしてクリアする」という処理を手で書く必要があります（レプリケーションの`OnRep`はネットワーク同期に特化した別の仕組みです）。Bevyの`Changed<T>`は、このdirtyフラグ管理をフレームワーク側が自動でやってくれるようなものだと考えると分かりやすいです。

## 試してみる

`code/examples/change_detection.rs`を作り、以下を書いてください。

```rust
use bevy::prelude::*;

#[derive(Component)]
struct Position(f32, f32);

fn setup(mut commands: Commands) {
    commands.spawn(Position(0.0, 0.0));
}

fn on_added(query: Query<&Position, Added<Position>>) {
    for position in &query {
        println!("added: ({}, {})", position.0, position.1);
    }
}

fn move_right(mut query: Query<&mut Position>, time: Res<Time>) {
    for mut position in &mut query {
        position.0 += 10.0 * time.delta_secs();
    }
}

fn on_changed(query: Query<&Position, Changed<Position>>) {
    for position in &query {
        println!("changed: ({:.1}, {:.1})", position.0, position.1);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (move_right, on_added, on_changed).chain())
        .run();
}
```

```
cargo run --example change_detection
```

`added:`は最初の1回だけ、`changed:`は`move_right`が毎フレーム`&mut`でアクセスしているため毎フレーム出力されるはずです。試しに`move_right`をコメントアウトすると、`changed:`が一切出なくなることも確認してみてください。
