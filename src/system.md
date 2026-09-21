# System

## Systemとは

Systemは、ただのRust関数です。特別な基底クラスや実装すべきtraitはありません。関数の**引数の型**によって、Bevyがその関数にどんなデータを渡せばいいかを判断します。

```rust
fn print_count(counter: Res<Counter>) {
    println!("count: {}", counter.0);
}
```

`Res<Counter>`という引数を書いておくだけで、Bevyは実行時にリソース`Counter`を探して渡してくれます。関数側は「自分が何を必要としているか」を型で宣言するだけで、それをどう用意してどう渡すかはBevy側が面倒を見ます。よく使う引数の型には以下があります。

- `Query<...>` — 条件に合うComponentを持つEntity群への問い合わせ（詳細は次章で扱います）
- `Res<T>` / `ResMut<T>` — アプリ全体で1つだけ存在するリソースの読み取り／書き込み
- `Commands` — Entityの生成・削除やComponentの追加などを予約する
- `MessageReader<T>` / `MessageWriter<T>` — メッセージの受信／送信（Message章で扱います。以前は`EventReader`/`EventWriter`という名前でした）

## いつ実行されるか（Schedule）

Systemは`add_systems`で、どのタイミングで実行するかを指定して登録します。

```rust
app.add_systems(Startup, setup)
   .add_systems(Update, (count, print_count));
```

## 補足: 関数名にかっこ`()`を付けないのはなぜか

`setup`や`count`はかっこ無しで渡しています。これは「その関数を呼び出す」のではなく、**関数そのものを値として渡している**という意味です。

```rust
setup     // 関数そのもの（値）。「これを後で呼んでね」という参照
setup()   // 今すぐ呼び出す。戻り値（ここでは`()`）が返ってくる
```

Rustでは関数名だけを書くと、その関数を指す特別な型の値（関数アイテム）として扱われます。`add_systems`は「後で（該当のScheduleのタイミングで）呼び出すべき関数はどれか」を登録したいだけで、その場で実行結果が欲しいわけではないので、関数そのものを渡す必要があります。もし`setup()`のようにかっこを付けてしまうと、その場ですぐ`setup`が実行されてしまい、その戻り値（`()`という「何も無い」型）を渡そうとしてコンパイルエラーになります。

これはこの本で今まで書いてきた`add_systems`の呼び出し全部に共通する話です。「後で誰かに呼んでもらうための関数」を渡すときは、実行せずそのまま名前だけ渡す、という感覚です。

代表的なScheduleは以下の通りです。

- `Startup` — アプリ起動時に一度だけ実行
- `Update` — 毎フレーム実行
- `FixedUpdate` — 固定タイムステップで実行（物理演算など、フレームレートに依存させたくない処理向け）

## 実行順序

`Update`に複数のSystemを登録しても、Bevyはデフォルトでは実行順序を保証しません。むしろ、データの読み書きが衝突しないSystem同士は積極的に並列実行しようとします。

順序を保証したい場合は明示的に指定します。

```rust
app.add_systems(Update, (count, print_count).chain());
```

`.chain()`を付けると、タプル内のSystemが書いた順に実行されるようになります。個別に`.before()`・`.after()`で依存関係だけ指定することもできます。

## UE5との対比

UE5では、Actorや ActorComponentがそれぞれ自分の`Tick()`を持ち、毎フレーム呼ばれます。処理はオブジェクトに紐づいていて、「このオブジェクトが自分で動く」というオブジェクト指向のモデルです。

Bevyには「Entityごとのtick関数」という概念はありません。代わりに、1つのSystem関数が、それに合致する**すべてのEntityをまとめて**毎フレーム処理します。振る舞い（System）とデータ（Component）が分離していて、System側はデータの塊に対してバルクで処理を行う、というのがECS（データ指向）の考え方です。Entityが増えても「動かし方」を書いたコードは1箇所のままで済みます。

## 試してみる

`code/examples/system.rs`を作り、以下を書いてください。

```rust
use bevy::prelude::*;

#[derive(Resource, Default)]
struct Counter(u32);

fn setup(mut commands: Commands) {
    commands.insert_resource(Counter::default());
}

fn count(mut counter: ResMut<Counter>) {
    counter.0 += 1;
}

fn print_count(counter: Res<Counter>) {
    println!("count: {}", counter.0);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (count, print_count).chain())
        .run();
}
```

```
cargo run --example system
```

コンソールに`count: 1`, `count: 2`, ...と毎フレーム増えていく値が出力されれば成功です。試しに`.chain()`を外してみると、`count`と`print_count`のどちらが先に実行されるか保証されなくなるので、出力される値がどう変わるか（あるいは変わらないか）を確認してみてください。

## 補足: `struct Counter(u32)`という書き方

`Counter`はフィールドに名前を付けない「タプル構造体（tuple struct）」です。Rustのstructには3種類あります。

```rust
struct Point { x: f32, y: f32 }  // 名前付きフィールド。 point.x でアクセス
struct Counter(u32);             // タプル構造体。counter.0 でアクセス
struct Player;                   // ユニット構造体。フィールドを持たない
```

`Counter(u32)`のように中身が1つだけのタプル構造体は、「ただの`u32`だと他の数値と混ざりやすいので、専用の型にして意味を持たせる」ためによく使われます（newtypeパターンと呼ばれます）。値を作るときの`Counter(5)`のような書き方は関数呼び出しに見えますが、実際にはRustがタプル構造体に対して自動生成するコンストラクタ関数を呼んでいます。

ちなみにフィールドを持たない`struct Player;`のようなユニット構造体は、Bevyでは中身を持たない目印用のComponent（マーカーコンポーネント）としてよく使われます。ECS編で改めて登場します。
