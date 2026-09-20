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

## 補足: `EventReader`はなぜ読むだけなのに`mut`が必要か

Query章・Resource章では「`mut`が付く＝他のSystemからも見える共有データを書き換える」という説明をしましたが、`EventReader`はデータを書き換えないのに`mut events: EventReader<Bounced>`のように`mut`が必要で、一見矛盾しています。

理由は、`EventReader`が**自分専用の「どこまで読んだか」を覚えておくカーソル（位置情報）**を内部に持っていて、`.read()`を呼ぶたびにそのカーソルを進めるからです。カーソルを進める＝自分の内部状態を書き換える操作なので`mut`が必要になります。ただしこのカーソルは、あくまで**そのSystem専用のローカルな状態**であり、他のSystemが持つ`EventReader`のカーソルには一切影響しません。

つまりBevyでの`mut`の正確な意味は「このSystem param自身が何かを書き換える必要がある」であって、「他のSystemから見える共有データを書き換える」とは限りません。

- `ResMut<T>` / `Query<&mut T>` — 他のSystemからも見える共有データそのものを書き換える。書き込み同士・読み書き同士は衝突判定の対象になる。
- `EventReader<T>` — 自分専用の読み取り位置だけを書き換える。共有のEventキューへのアクセス自体は読み取り専用なので、複数のSystemがそれぞれ独立して同じ型の`EventReader`を持っていても互いに干渉せず並列に動ける。

一方`EventWriter<T>`は実際に共有のキューへ書き込むため、これは`ResMut`と同じ扱いになります。

## 補足: 書いたEventはいつ読める側に届くか

`movement`（送る側）と`on_bounced`（受け取る側）を`.chain()`で順序付けしているのは、確実に同じフレーム内で読み取らせるためです。

`EventWriter`は内部的に`ResMut<Events<T>>`、`EventReader`は`Res<Events<T>>`へのアクセスを必要とします。これは片方が書き込み・もう片方が読み取りなので、System章で説明した衝突判定に引っかかり、Bevyのスケジューラはこの2つを同時に（別スレッドで）実行することはありません。なので「読み取っている最中に書き込まれて中身が壊れる」という意味でのデータ競合は起こりません。

ただし`.chain()`のような順序指定を書かなければ、**そのフレーム内でどちらが先に実行されるかは保証されません**。もし読み取り側がそのフレームで先に実行されてしまった場合、そのフレームでは拾えません。しかし消えてなくなるわけでもありません。BevyのEventは内部で直近2フレーム分のバッファを保持しており（ダブルバッファリング）、拾いそこねても次のフレームで改めて読み取れるようになっています。

つまり「Writeされてから必ず次のフレームで反映される」わけではなく、順序次第で「同じフレーム中に読める」場合と「次のフレームにずれ込む」場合があり、後者になってもイベント自体が失われることはない、という設計です。`.chain()`で順序を明示しておけば、常に同じフレーム内で確実に拾えます。

## UE5との対比

これはUE5のDelegate（特にマルチキャストデリゲート／Event Dispatcher）とほぼ同じ発想です。UE5でも、あるActorが「何かが起きた」ことをBroadcastし、それを購読（Bind）している側がいくつあっても、Broadcastする側は購読者の存在を意識しません。

Bevyの`EventWriter`/`EventReader`は、このBroadcast/Bindの関係をECSの中で表現したものだと考えると分かりやすいです。ただしUE5のDelegateが「関数ポインタ的にBindした相手を直接呼び出す」のに対し、Bevyの`EventReader`は「毎フレーム、溜まっているEventを能動的に見に行く（ポーリングする）」形になっている点は違います。

## 補足: Eventはいつ使うべきか

Delegateと同じ発想である以上、Delegateが持つ弱点もそのまま引き継ぎます。送る側のコードだけを読んでも、実際に何が起きるかは「どこかにいる`EventReader`次第」なので、追跡性が落ちます。

UE5で「Delegateでばら撒く代わりに、`UWorldSubsystem`のような中央管理者に直接呼びに行く」という設計を選ぶことがあるように、Bevyでも同じ選択肢があります。対応関係としては以下のようになります。

- **Subsystem的（直接呼ぶ）** → `Resource`に状態を持たせ、反応する側のSystemが`Res`/`ResMut`で直接読み書き・処理する。誰が読み書きするかはコードを読めば追える。
- **Delegate的（ばら撒く）** → `Event`で通知だけ送り、誰が拾うかは送る側が知らない。疎結合だが追跡性は落ちる。

Eventを使う価値があるのは、以下のように「疎結合であること自体に意味がある」場合に限られます。

- 複数の独立したSystemが同じ出来事に反応する必要がある（効果音・スコア加算・実績解除など、無関係などうしが同じきっかけで動く）
- Pluginの境界をまたぐ（サードパーティのPluginは自分のゲームロジックを知らないので、Eventが唯一の拡張ポイントになる）

逆に、反応するSystemが1つしかないなら、Eventを経由せず直接処理してしまった方が素直に読めて追跡性も高いです。「デカップリングしたい明確な理由がない限り、まずはResourceベースで直接呼ぶ」くらいの温度感で使い分けるのがおすすめです。

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
