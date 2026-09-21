# FixedUpdateとシミュレーションのステップ実行

## なぜUpdateだけでは足りないか

Time章で`delta_secs()`を掛けて「1秒あたりの移動量」を一定にする方法を紹介しましたが、これでも解決しない問題が残っています。それは、**1秒間に`Update`が何回実行されるか自体は環境によって変わる**ということです。60fpsのマシンと144fpsのマシン、あるいは負荷で20fpsまで落ちた瞬間とでは、同じ1秒間でもシミュレーションの計算が何回行われるかがバラバラになります。

見た目の移動速度は`delta_secs()`のおかげで揃っても、「何回目のステップで何が起きたか」という計算の刻み自体が環境依存になるのは、避難シミュレーションのような再現性が重要な用途では困ります。同じ初期条件なら、どのマシンで実行しても同じ結果になってほしいはずです。

## FixedUpdateとは

`FixedUpdate`は、`Update`とは別に用意されている、**常に一定の間隔**で実行されるScheduleです。デフォルトでは64Hz（1回あたり約15.625ミリ秒）に固定されています。実際の描画フレームレートが何であっても、`FixedUpdate`に登録したSystemは常に同じ刻みで呼ばれます。

```rust
app.add_systems(FixedUpdate, simulate_step);
```

## 仕組み: 1フレームで複数回・あるいは0回実行される

`FixedUpdate`は「1描画フレームにつき1回」ではありません。Bevyは実際に経過した時間を毎フレーム積み立てておき、その残高が固定タイムステップ（デフォルト約15.6ms）を超えるたびに`FixedUpdate`を1回消費して実行する、という仕組みになっています。

そのため、描画が重くて1フレームに30ms掛かった場合は、その1フレームの間に`FixedUpdate`が2回実行されて帳尻を合わせますし、逆に描画が速すぎる場合はその描画フレームでは`FixedUpdate`が1回も実行されないこともあります。「描画の回数」と「シミュレーションの計算回数」が、意図的に切り離されているということです。

## FixedUpdate内での`Time`

`FixedUpdate`に登録したSystemの中で`Res<Time>`を使うと、Time章で見た可変の`delta_secs()`ではなく、**常に一定の値**（デフォルトなら常に約0.015625秒）が返ってきます。`Update`と`FixedUpdate`とで、同じ`Res<Time>`という書き方のまま中身が自動的に切り替わっている、というのがポイントです。

## タイムステップの変更

デフォルトの64Hzから変更したい場合は、`Time<Fixed>`というResourceを差し替えます。

```rust
app.insert_resource(Time::<Fixed>::from_hz(60.0));
```

## UE5との対比

UE5にも似た発想があります。ゲーム全体のTickは可変フレームレートで動く一方、物理演算（Chaos/PhysX）は安定性のために独自の固定サブステップで計算されることが多いです。BevyがUE5と違うのは、この「固定刻みで計算する」という仕組みを物理演算専用にせず、`FixedUpdate`という一般的なScheduleとして誰でも使えるようにしている点です。物理に限らず、シミュレーションのステップ実行そのものにこの仕組みをそのまま使えます。

## シミュレーションでの使い分け

- **`FixedUpdate`** — シミュレーション本体のロジック（Agentの移動計算、状態の更新、出口に到達したかの判定など）。再現性が必要な計算はすべてここに置きます。
- **`Update`** — 描画に関わるもの（カメラの補間、UIのアニメーションなど）。こちらは可変フレームレートのままで問題ありません。

こうして「シミュレーションの時計」と「描画の時計」を分けておくと、描画が重くなってもシミュレーション結果自体は変わらない、という状態を作れます。

## 試してみる

`code/examples/fixed_update.rs`を作り、以下を書いてください。`Update`と`FixedUpdate`それぞれの`delta_secs()`を見比べる例です。

```rust
use bevy::prelude::*;

#[derive(Resource, Default)]
struct TickCount(u32);

fn setup(mut commands: Commands) {
    commands.insert_resource(TickCount::default());
}

fn fixed_tick(mut tick: ResMut<TickCount>, time: Res<Time>) {
    tick.0 += 1;
    println!("fixed tick {} (dt={:.5}s)", tick.0, time.delta_secs());
}

fn update_tick(time: Res<Time>) {
    println!("update frame (dt={:.5}s)", time.delta_secs());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, fixed_tick)
        .add_systems(Update, update_tick)
        .run();
}
```

```
cargo run --example fixed_update
```

`fixed tick`側の`dt`は常にほぼ同じ値（約`0.01563`）で固定されているのに対し、`update frame`側の`dt`は環境によって多少ばらつくはずです。ウィンドウをドラッグして意図的に描画を一瞬止めてみると、その直後に`fixed tick`が複数回まとめて出力される（遅れを取り戻す）様子も観察できます。
