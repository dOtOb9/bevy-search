# State（シミュレーションのフェーズ管理）

避難シミュレーションのようなものには、たいてい「準備中」「実行中」「一時停止」「終了」のような、はっきりした局面（フェーズ）があります。Bevyの`States`は、こうした「今どの局面にいるか」を管理するための仕組みです。

## Stateとは

`enum`に`#[derive(States, ...)]`を付けて定義します。

```rust
#[derive(States, Default, Clone, Eq, PartialEq, Hash, Debug)]
enum SimPhase {
    #[default]
    Setup,
    Running,
    Paused,
    Finished,
}
```

`#[default]`を付けたバリアントが、Appの起動直後の初期状態になります。

## 登録

```rust
app.init_state::<SimPhase>();
```

## 特定の状態のときだけSystemを実行する

`.run_if(in_state(...))`を付けると、そのSystemは指定した状態のときだけ実行されるようになります。

```rust
app.add_systems(Update, simulate.run_if(in_state(SimPhase::Running)));
```

`Paused`や`Setup`のときは`simulate`が呼ばれなくなるので、if文で毎回状態をチェックする代わりに、そもそも「実行すること自体」をスケジュール側で制御できます。

## 状態を切り替える

現在の状態は`Res<State<S>>`で読み取り、切り替えは`ResMut<NextState<S>>`に対して行います。

```rust
fn toggle_pause(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<SimPhase>>,
    mut next_state: ResMut<NextState<SimPhase>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        match state.get() {
            SimPhase::Running => next_state.set(SimPhase::Paused),
            SimPhase::Paused => next_state.set(SimPhase::Running),
            _ => {}
        }
    }
}
```

`next_state.set(...)`を呼んだ時点ではまだ切り替わらず、次のタイミングでBevyがまとめて状態遷移を処理します。同じフレーム中に複数箇所で`set`を呼んでも、最後に呼ばれたものが採用されます。

## 状態が切り替わった瞬間だけ実行する: `OnEnter`/`OnExit`

「その状態に入った瞬間」「その状態から出た瞬間」だけ実行したい初期化・後片付け処理には`OnEnter`/`OnExit`を使います。

```rust
app.add_systems(OnEnter(SimPhase::Running), on_start_running)
   .add_systems(OnExit(SimPhase::Running), on_stop_running);
```

`Running`に切り替わった最初の1回だけ`on_start_running`が呼ばれ、`Running`から他の状態に移った瞬間に`on_stop_running`が呼ばれます。

## 補足: 普通のResourceに`enum`を持たせるのと何が違うか

`Paused`が1個あるだけのような単純なケースなら、正直大差はありません。`States`を使う価値は、以下の3点にあります。

1. **`run_if(in_state(X))`はSystemそのものを呼ばせない** — 自前Resourceだと、Systemの中身で`if resource.phase == Running { ... }`と毎回チェックする必要があります。`States`ならSystem自体がスケジューラの時点で丸ごとスキップされるので、「どのSystemがどの局面で動くか」がApp構築のコードを見ただけで分かります（Systemの中身を読まなくていい）。
2. **`OnEnter`/`OnExit`は「切り替わった瞬間」を自動検知してくれる** — 自前Resourceでこれをやろうとすると、「前回の値」を別途持っておいて毎フレーム比較する、という手書きの仕組みが必要になります。`States`ならフレームワーク側がその比較をやってくれます。
3. **`DespawnOnExit<S>`という専用Componentがある** — Entityにこれを付けておくと、指定した状態から抜けた瞬間にBevyが自動でそのEntityを`despawn`してくれます。「シミュレーション終了時に表示していたAgentを全部消す」のような後片付けを、自分でSystemを書かずに済ませられます。

逆に言うと、状態が1〜2個で、切り替わった瞬間の処理も特に無いなら、`States`を使わず素朴な`Resource`のboolやenumで十分です。フェーズの数が増えたり、「入った瞬間だけ初期化」「出た瞬間だけ後片付け」が増えてくるタイミングで`States`に切り替える価値が出てくる、という感覚です。

## UE5との対比

UE5には`States`にそのまま相当する汎用の仕組みはなく、多くの場合`GameMode`/`GameState`に自前で`enum`を持たせ、遷移のたびに手でBroadcastやチェックを書く、という実装になりがちです（AI用途なら`Gameplay State Tree`や`Behavior Tree`がありますが、これはアプリ全体のフェーズ管理とは別物です）。

Bevyの`States`は、「今の状態を保持する」「状態ごとにSystemの実行有無を切り替える」「状態が変わった瞬間のフック（OnEnter/OnExit）」までがフレームワーク側に組み込まれているため、UE5で自前実装していた配線の多くを書かずに済みます。

## 試してみる

`code/examples/state.rs`を作り、以下を書いてください。スペースキーで`Running`/`Paused`を切り替えると、シミュレーションのカウンタが動いたり止まったりする例です。

```rust
use bevy::prelude::*;

#[derive(States, Default, Clone, Eq, PartialEq, Hash, Debug)]
enum SimPhase {
    #[default]
    Running,
    Paused,
}

#[derive(Resource, Default)]
struct StepCount(u32);

fn setup(mut commands: Commands) {
    commands.insert_resource(StepCount::default());
}

fn toggle_pause(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<SimPhase>>,
    mut next_state: ResMut<NextState<SimPhase>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        match state.get() {
            SimPhase::Running => next_state.set(SimPhase::Paused),
            SimPhase::Paused => next_state.set(SimPhase::Running),
        }
    }
}

fn simulate(mut step: ResMut<StepCount>) {
    step.0 += 1;
    println!("step: {}", step.0);
}

fn on_enter_paused() {
    println!("--- paused ---");
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<SimPhase>()
        .add_systems(Startup, setup)
        .add_systems(Update, toggle_pause)
        .add_systems(FixedUpdate, simulate.run_if(in_state(SimPhase::Running)))
        .add_systems(OnEnter(SimPhase::Paused), on_enter_paused)
        .run();
}
```

```
cargo run --example state
```

`step:`が増え続けている間にスペースキーを押すと`--- paused ---`が1回だけ表示されて`step:`の出力が止まり、もう一度スペースキーを押すと再開する、という動きになれば成功です。
