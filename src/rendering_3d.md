# 3Dの基礎（Mesh・Material・Light）

2Dでは`Camera2d`＋`Sprite`だけで画面に何か表示できましたが、3Dでは表示するために必要な要素がもう少し増えます。**Camera3d**（視点）・**Mesh**（形状）・**Material**（見た目）・**Light**（光源）の4つです。

## Camera3d

2Dの`Camera2d`と同じ感覚で、3Dでは`Camera3d`を使います。3Dはカメラの位置と向きに意味があるので、`Transform`で配置します。

```rust
commands.spawn((
    Camera3d::default(),
    Transform::from_xyz(0.0, 3.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
));
```

`.looking_at(target, up)`は、指定した座標の方向を向くように回転を計算してくれる便利メソッドです。TransformとHierarchy章で触れた通り、Bevyの上方向は`Y`軸です。

## Mesh: 形状

3Dオブジェクトの「形」は`Mesh`というAssetです。立方体や球のような基本図形は、`Assets<Mesh>`に追加するだけで作れます。

```rust
fn setup(mut meshes: ResMut<Assets<Mesh>>) {
    let handle: Handle<Mesh> = meshes.add(Cuboid::default());
}
```

`Cuboid`（直方体）や`Sphere`（球）のような図形の型は、`Mesh`に変換できるようになっているので、そのまま`.add()`に渡せます。画面に表示するには、これを`Mesh3d`というComponentに包んでEntityに付けます。

```rust
commands.spawn(Mesh3d(handle));
```

## Material: 見た目

`Mesh`が「形」だけを持つのに対し、色や質感は`StandardMaterial`という別のAssetが担当します。これも`Assets<StandardMaterial>`に追加して`Handle`を取得し、`MeshMaterial3d`というComponentに包みます。

```rust
fn setup(mut materials: ResMut<Assets<StandardMaterial>>) {
    let handle = materials.add(StandardMaterial {
        base_color: Color::srgb(0.3, 0.6, 0.9),
        ..default()
    });
}
```

`Mesh3d`と`MeshMaterial3d`を同じEntityに両方付けて、初めて3Dオブジェクトとして表示されます。

```rust
commands.spawn((
    Mesh3d(mesh_handle),
    MeshMaterial3d(material_handle),
    Transform::default(),
));
```

## Light: 光源

`StandardMaterial`はPBR（物理ベースレンダリング）用の材質で、光が当たらないと真っ黒に見えます。2DのSpriteとは違い、3Dでは光源が無いと基本的に何も見えません。

```rust
commands.spawn((
    PointLight {
        intensity: 2_000_000.0,
        ..default()
    },
    Transform::from_xyz(4.0, 8.0, 4.0),
));
```

`PointLight`は電球のような点光源です。他に、太陽光のように特定方向から平行に届く`DirectionalLight`もあります。

## UE5との対比

考え方自体はUE5とほぼ同じです。UE5の`StaticMeshComponent`が「形（Static Mesh）」と「見た目（Material）」を1つのComponentの中でスロットとして持つのに対し、Bevyでは`Mesh3d`と`MeshMaterial3d`という**別々の小さなComponent**として同じEntityに付けます。1つの万能なComponentに機能を詰め込むのではなく、小さなComponentを組み合わせる、というECSらしい設計の違いです。光源をActorとして別に配置する感覚（`PointLight`/`DirectionalLight`のActor）は、UE5とほぼそのまま対応します。

## 試してみる

`code/examples/rendering_3d.rs`を作り、以下を書いてください。ライトが当たった立方体が回転する例です。

```rust
use bevy::prelude::*;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 3.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        PointLight {
            intensity: 2_000_000.0,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.6, 0.9),
            ..default()
        })),
        Transform::default(),
    ));
}

fn spin(mut query: Query<&mut Transform, With<Mesh3d>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y(1.0 * time.delta_secs());
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, spin)
        .run();
}
```

```
cargo run --example rendering_3d
```

光の当たった水色の立方体が表示され、ゆっくり回転していれば成功です。`PointLight`の`intensity`を`0.0`にしてみると、光が無いとどれだけ真っ暗になるかも確認してみてください。
