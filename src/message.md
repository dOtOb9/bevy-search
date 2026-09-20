# Message（旧: Event）

> このプロジェクトで使っている`bevy 0.19.1`では、以前`Event`/`EventWriter`/`EventReader`と呼ばれていた仕組みが`Message`/`MessageWriter`/`MessageReader`にリネームされています。ネット上の記事や過去のドキュメントの多くは古い名前（`Event`系）で書かれているので、検索するときは読み替えが必要です。また新しい`Event`/`EntityEvent`という名前は、このバージョンでは後述するObserverという別の仕組みに使われています。混同しないよう注意してください。

## Messageとは

ResourceやComponentが「今、何であるか・どういう状態か」という**持続する状態**を表すのに対し、Messageは「今、何かが起きた」という**一瞬の出来事の通知**です。一度読まれる（あるいは一定時間読まれない）と役目を終えて消えます。

例えば「プレイヤーが壁にぶつかった」というのはMessage向きです。ぶつかった瞬間だけ意味があり、「ぶつかっている状態」をずっと保持しておきたいわけではないからです。

## 使い方

**1. 型を定義する**

```rust
#[derive(Message)]
struct Bounced;
```

**2. Appに登録する**

```rust
App::new()
    .add_message::<Bounced>()
    // ...
```

**3. 送る側（MessageWriter）**

```rust
fn movement(mut bounced: MessageWriter<Bounced>) {
    bounced.write(Bounced);
}
```

**4. 受け取る側（MessageReader）**

```rust
fn on_bounced(mut messages: MessageReader<Bounced>) {
    for _ in messages.read() {
        println!("bounced!");
    }
}
```

`.read()`は、前回このSystemが読んでから新しく送られたMessageすべてを返すイテレータです。

## なぜMessageを使うのか（Systemの疎結合）

`movement`が直接`println!`を呼んでもいいのに、わざわざMessageを経由する理由は、**送る側が受け取る側を知らなくていい**ようにするためです。`movement`は「跳ね返ったこと」を`Bounced`として送るだけで、それを誰が・いくつのSystemが受け取って何をするかには一切関知しません。効果音を鳴らすSystem、スコアを加算するSystem、画面を揺らすSystemなど、後から受け取る側だけをいくつでも追加できます。

## 補足: `MessageReader`はなぜ読むだけなのに`mut`が必要か

Query章・Resource章では「`mut`が付く＝他のSystemからも見える共有データを書き換える」という説明をしましたが、`MessageReader`はデータを書き換えないのに`mut messages: MessageReader<Bounced>`のように`mut`が必要で、一見矛盾しています。

理由は、`MessageReader`が**自分専用の「どこまで読んだか」を覚えておくカーソル（位置情報）**を内部に持っていて、`.read()`を呼ぶたびにそのカーソルを進めるからです。カーソルを進める＝自分の内部状態を書き換える操作なので`mut`が必要になります。ただしこのカーソルは、あくまで**そのSystem専用のローカルな状態**であり、他のSystemが持つ`MessageReader`のカーソルには一切影響しません。

つまりBevyでの`mut`の正確な意味は「このSystem param自身が何かを書き換える必要がある」であって、「他のSystemから見える共有データを書き換える」とは限りません。

- `ResMut<T>` / `Query<&mut T>` — 他のSystemからも見える共有データそのものを書き換える。書き込み同士・読み書き同士は衝突判定の対象になる。
- `MessageReader<T>` — 自分専用の読み取り位置だけを書き換える。共有のMessageキューへのアクセス自体は読み取り専用なので、複数のSystemがそれぞれ独立して同じ型の`MessageReader`を持っていても互いに干渉せず並列に動ける。

一方`MessageWriter`は実際に共有のキューへ書き込むため、これは`ResMut`と同じ扱いになります。

## 補足: 書いたMessageはいつ読める側に届くか

`movement`（送る側）と`on_bounced`（受け取る側）を`.chain()`で順序付けしているのは、確実に同じフレーム内で読み取らせるためです。

`MessageWriter`は内部的に`ResMut<Messages<T>>`、`MessageReader`は`Res<Messages<T>>`へのアクセスを必要とします。これは片方が書き込み・もう片方が読み取りなので、System章で説明した衝突判定に引っかかり、Bevyのスケジューラはこの2つを同時に（別スレッドで）実行することはありません。なので「読み取っている最中に書き込まれて中身が壊れる」という意味でのデータ競合は起こりません。

ただし`.chain()`のような順序指定を書かなければ、**そのフレーム内でどちらが先に実行されるかは保証されません**。もし読み取り側がそのフレームで先に実行されてしまった場合、そのフレームでは拾えません。しかし消えてなくなるわけでもありません。BevyのMessageは内部で直近2フレーム分のバッファを保持しており（ダブルバッファリング）、拾いそこねても次のフレームで改めて読み取れるようになっています。

つまり「Writeされてから必ず次のフレームで反映される」わけではなく、順序次第で「同じフレーム中に読める」場合と「次のフレームにずれ込む」場合があり、後者になってもメッセージ自体が失われることはない、という設計です。`.chain()`で順序を明示しておけば、常に同じフレーム内で確実に拾えます。

## UE5との対比

これはUE5のDelegate（特にマルチキャストデリゲート／Event Dispatcher）とほぼ同じ発想です。UE5でも、あるActorが「何かが起きた」ことをBroadcastし、それを購読（Bind）している側がいくつあっても、Broadcastする側は購読者の存在を意識しません。

Bevyの`MessageWriter`/`MessageReader`は、このBroadcast/Bindの関係をECSの中で表現したものだと考えると分かりやすいです。ただしUE5のDelegateが「関数ポインタ的にBindした相手を直接呼び出す」のに対し、Bevyの`MessageReader`は「毎フレーム、溜まっているMessageを能動的に見に行く（ポーリングする）」形になっている点は違います。

## 補足: Messageはいつ使うべきか

Delegateと同じ発想である以上、Delegateが持つ弱点もそのまま引き継ぎます。送る側のコードだけを読んでも、実際に何が起きるかは「どこかにいる`MessageReader`次第」なので、追跡性が落ちます。

UE5で「Delegateでばら撒く代わりに、`UWorldSubsystem`のような中央管理者に直接呼びに行く」という設計を選ぶことがあるように、Bevyでも同じ選択肢があります。対応関係としては以下のようになります。

- **Subsystem的（直接呼ぶ）** → `Resource`に状態を持たせ、反応する側のSystemが`Res`/`ResMut`で直接読み書き・処理する。誰が読み書きするかはコードを読めば追える。
- **Delegate的（ばら撒く）** → `Message`で通知だけ送り、誰が拾うかは送る側が知らない。疎結合だが追跡性は落ちる。

Messageを使う価値があるのは、以下のように「疎結合であること自体に意味がある」場合に限られます。

- 複数の独立したSystemが同じ出来事に反応する必要がある（効果音・スコア加算・実績解除など、無関係などうしが同じきっかけで動く）
- Pluginの境界をまたぐ（サードパーティのPluginは自分のゲームロジックを知らないので、Messageが唯一の拡張ポイントになる）

逆に、反応するSystemが1つしかないなら、Messageを経由せず直接処理してしまった方が素直に読めて追跡性も高いです。「デカップリングしたい明確な理由がない限り、まずはResourceベースで直接呼ぶ」くらいの温度感で使い分けるのがおすすめです。

## 試してみる

`code/examples/message.rs`を作り、以下を書いてください。Positionが端まで来たら跳ね返り、跳ね返った瞬間に`Bounced`メッセージを送る例です。

```rust
use bevy::prelude::*;

#[derive(Component)]
struct Position(f32, f32);

#[derive(Component)]
struct Velocity(f32, f32);

#[derive(Message)]
struct Bounced;

fn setup(mut commands: Commands) {
    commands.spawn((Position(0.0, 0.0), Velocity(50.0, 0.0)));
}

fn movement(
    mut query: Query<(&mut Position, &mut Velocity)>,
    time: Res<Time>,
    mut bounced: MessageWriter<Bounced>,
) {
    let dt = time.delta_secs();
    for (mut position, mut velocity) in &mut query {
        position.0 += velocity.0 * dt;
        if position.0 > 100.0 || position.0 < 0.0 {
            velocity.0 = -velocity.0;
            bounced.write(Bounced);
        }
    }
}

fn on_bounced(mut messages: MessageReader<Bounced>) {
    for _ in messages.read() {
        println!("bounced!");
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_message::<Bounced>()
        .add_systems(Startup, setup)
        .add_systems(Update, (movement, on_bounced).chain())
        .run();
}
```

```
cargo run --example message
```

x座標が0〜100の間を往復しつつ、端に着いて跳ね返るたびにコンソールに`bounced!`が出力されれば成功です。
