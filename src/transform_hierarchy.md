# TransformとHierarchy（親子関係）

## Transformとは

`Transform`は、Entityの位置・回転・拡縮を持つComponentです。

```rust
pub struct Transform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}
```

一番簡単な使い方は、位置だけ指定するコンストラクタです。

```rust
commands.spawn(Transform::from_xyz(10.0, 0.0, 0.0));
```

`Transform`を付けると、自動的に`GlobalTransform`というComponentも一緒に付与されます。これは「Required Components」と呼ばれる仕組みで、ある型を定義する側で「このComponentを付けるときは、このComponentも一緒に必要」と宣言しておくと、`spawn`時にBevyが足りない分を自動で補ってくれます（`Transform`自体の定義に`#[require(GlobalTransform)]`のような指定がされています）。手動で`GlobalTransform`を付け忘れる心配をしなくていい、というのがこの仕組みの狙いです。

`Transform`は「親から見た相対位置」、`GlobalTransform`は「実際のワールド上の絶対位置」で、親がいないEntityではこの2つは常に同じ値になります。`GlobalTransform`は毎フレームBevyが`Transform`と親の位置から自動計算するもので、通常は自分で書き換えません。

## Hierarchy（親子関係）

あるEntityを別のEntityの子にするには、子になる側に`ChildOf`というComponentを付けます。

```rust
commands.entity(child).insert(ChildOf(parent));
```

もう少し簡単に書ける専用のメソッドも用意されています。

```rust
commands.entity(parent).add_child(child);

// まとめて複数の子を追加
commands.entity(parent).add_children(&[child_a, child_b]);

// 親を作りながら子も一緒に生成
commands.spawn(Transform::default()).with_children(|parent| {
    parent.spawn(Transform::from_xyz(10.0, 0.0, 0.0));
    parent.spawn(Transform::from_xyz(-10.0, 0.0, 0.0));
});
```

親側には`Children`というComponentが自動的に付き、今の子Entity一覧を保持しますが、これは`ChildOf`から自動的に同期されるものなので、`Children`を自分で書き換えることはありません。

親子関係があると、`Transform`（相対位置）は親からの相対座標として扱われ、親が動けば子の`GlobalTransform`（絶対位置）も自動的について動きます。

## シミュレーション用途での使い方・注意点

避難シミュレーションのようにAgentを多数配置する場合、`Transform.translation`をそのままAgentの座標として使うのが基本です。Hierarchyが役立つのは、例えば「グループ（避難班）ごとにまとめて動かしたい」「Agent本体に、状態を示すラベルや矢印などの見た目だけの子Entityを付けたい」といった、明確な親子関係があるケースです。

逆に、数千〜数万のAgentを**すべて同じ1つの親にぶら下げる**ような使い方はおすすめしません。`GlobalTransform`の再計算はHierarchyをたどって行われるため、無闇に深い・広いHierarchyを作るとその分だけ計算コストが増えます。基本的にはAgentは親を持たないフラットなEntityとして配置し、本当に「一緒に動く」べきものだけを親子関係にする、という考え方がパフォーマンス面でも安全です。

## UE5との対比

UE5の`USceneComponent`階層（`AttachToComponent`/`AttachToActor`で親子付けする、いわゆるシーングラフ）と同じ発想です。UE5の「Relative Transform」と「World Transform」の違いが、そのままBevyの`Transform`（親からの相対）と`GlobalTransform`（絶対）の違いに対応します。

## 試してみる

`code/examples/transform_hierarchy.rs`を作り、以下を書いてください。1つの「班（Group）」を動かすと、その子である2体のメンバーが追従して動く例です。

```rust
use bevy::prelude::*;

#[derive(Component)]
struct Group;

fn setup(mut commands: Commands) {
    let group = commands
        .spawn((Group, Transform::from_xyz(0.0, 0.0, 0.0)))
        .id();

    let member_a = commands.spawn(Transform::from_xyz(10.0, 0.0, 0.0)).id();
    let member_b = commands.spawn(Transform::from_xyz(-10.0, 0.0, 0.0)).id();

    commands.entity(group).add_children(&[member_a, member_b]);
}

fn move_group(mut query: Query<&mut Transform, With<Group>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.translation.x += 20.0 * time.delta_secs();
    }
}

fn print_members(query: Query<&GlobalTransform, Without<Group>>) {
    for global_transform in &query {
        println!("member at {:?}", global_transform.translation());
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (move_group, print_members).chain())
        .run();
}
```

```
cargo run --example transform_hierarchy
```

メンバーの`Transform`自体は`(10.0, 0.0, 0.0)`/`(-10.0, 0.0, 0.0)`のまま変わりませんが、班（親）が動くにつれて、出力される`GlobalTransform`（絶対位置）のx座標が2体とも同じペースで増えていくのが確認できれば成功です。
