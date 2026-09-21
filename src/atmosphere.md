# Atmosphere（最低限の使い方）

Bevyの`Atmosphere`は、UE5のSky Atmosphereと同じ発想の、物理ベースの大気散乱を再現する機能です。中身の実装（GPU compute shaderでのLUT計算など）は3Dの基礎とAtmosphere章の前段で触れた通りかなり専門的ですが、使う側としてはComponentを付けるだけで使えます。

> 補足: 最初、Bevy本体のドキュメントコメントを頼りに書こうとしたところ、そこに書かれていた説明と実際のソースコードの構造が食い違っていました（ドキュメントが実装のリファクタに追いついていなかったようです）。最終的には`cargo doc`で実際にこのバージョン向けにドキュメントを生成し、そこに書かれた型定義を直接確認して裏を取っています。ドキュメントコメントも100%は信用できないことがある、という一例です。

## 必要なEntityは2つ

ドキュメントによれば、`Atmosphere`は「大気（惑星）そのもの」を表すEntityと、それを描画する「カメラ」用のEntityを別々に用意する構成になっています。

**1. 大気そのもの**

```rust
use bevy::light::Atmosphere;

commands.spawn((
    Atmosphere::earth(),
    Transform::from_xyz(0.0, 0.0, 0.0),
));
```

このEntityの`Transform`（正確には`GlobalTransform`）が「惑星の中心」の座標として扱われます。`Atmosphere::earth()`という、地球っぽい大気のパラメータをまとめて設定してくれる専用のコンストラクタが用意されています（他に`Atmosphere::mars()`もあります）。

**2. カメラ側**

```rust
use bevy::pbr::AtmosphereSettings;

commands.spawn((
    Camera3d::default(),
    AtmosphereSettings::default(),
    Transform::from_xyz(0.0, 2.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
));
```

`AtmosphereSettings`をカメラに付けると、そのカメラで大気散乱が描画されるようになります（LUTの解像度などの品質設定もここに入っています）。付けるだけで、必要な内部設定（HDRレンダリングなど）はRequired Componentsの仕組みで自動的に補われます。

`Atmosphere`も`AtmosphereSettings`も`bevy::prelude`には含まれていないため、上記のように個別に`use`する必要があります。

## 太陽が要る

大気散乱は「光がどう散らばるか」を計算するものなので、散らばる元になる光源（`DirectionalLight`、太陽に見立てたもの）が無いと効果が分かりません。

```rust
commands.spawn((
    DirectionalLight::default(),
    Transform::from_xyz(0.0, 5.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
));
```

## 試してみる

`code/examples/atmosphere.rs`を作り、以下を書いてください。

```rust
use bevy::light::Atmosphere;
use bevy::pbr::AtmosphereSettings;
use bevy::prelude::*;

fn setup(mut commands: Commands) {
    commands.spawn((Atmosphere::earth(), Transform::from_xyz(0.0, 0.0, 0.0)));

    commands.spawn((
        Camera3d::default(),
        AtmosphereSettings::default(),
        Transform::from_xyz(0.0, 2.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(0.0, 5.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}
```

```
cargo run --example atmosphere
```

空らしいグラデーション（水平線に近いほど白っぽく、天頂は青い）が描画されれば成功です。うまく見えない場合は、GPUがcompute shaderに対応していない可能性があります（`AtmospherePlugin`がその場合は警告を出して自動的に無効化される仕様です）。
