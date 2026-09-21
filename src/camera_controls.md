# カメラをマウスで動かす

Bevy本体には、UE5やUnityのViewportにあるような「マウスでカメラをぐりぐり動かす」機能は用意されていません。毎回自分で実装するのは面倒なので、サードパーティのcrateを使うのが一般的です。ここでは`bevy_panorbit_camera`という、広く使われているcrateを紹介します（実際にこのバージョンのBevyでビルドが通ることを確認済みです）。

## 導入

```
cargo add bevy_panorbit_camera
```

Pluginを登録し、カメラEntityに`PanOrbitCamera`を付けるだけです。

```rust
use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.0, 5.0),
        PanOrbitCamera::default(),
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        .run();
}
```

`looking_at`で向きを計算しておく必要はありません。`PanOrbitCamera`が、渡された`Transform`の位置から自動的に注視点や距離を計算してくれます。

## 操作方法（デフォルト設定）

- **左ドラッグ** — 注視点を中心にカメラを回転（オービット）
- **右ドラッグ** — 視点を平行移動（パン）
- **スクロール** — ズームイン・アウト

Blenderや、UnityのSceneビューのマウス操作にかなり近い感覚です。

## UE5との対比

UE5のエディタViewportでのナビゲーション（右クリック+WASDでの視点移動、Altキー+ドラッグでのオービットなど）と役割は同じです。ただしUE5ではエディタに標準搭載されているのに対し、Bevyにはエディタ自体が無いので、実行中のアプリにこの機能を自分で組み込む必要がある、という違いがあります。

## 試してみる

3Dの基礎章で作った立方体の例を、固定カメラから`PanOrbitCamera`に差し替えてみましょう。`code/examples/camera_controls.rs`を作り、以下を書いてください。

```rust
use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.0, 5.0),
        PanOrbitCamera::default(),
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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        .run();
}
```

```
cargo run --example camera_controls
```

マウスをドラッグして立方体の周りを自由に回り込めれば成功です。これで座標を勘だけで書く必要がぐっと減るはずです。
