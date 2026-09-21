# Atmosphere

Bevyの`Atmosphere`は、UE5のSky Atmosphereと同じ発想の、物理ベースの大気散乱を再現する機能です。中身の実装（GPU compute shaderでのLUT計算など）は3Dの基礎とAtmosphere章の前段で触れた通りかなり専門的ですが、使う側としてはComponentを付けるだけで使えます。

> 補足: 最初、Bevy本体のドキュメントコメントを頼りに書こうとしたところ、そこに書かれていた説明と実際のソースコードの構造が食い違っていました（ドキュメントが実装のリファクタに追いついていなかったようです）。`cargo doc`で実際にこのバージョン向けにドキュメントを生成して型定義を確認しましたが、それでも関数の**引数の中身**まではきちんと見ていなかったため、最初に書いたコードは実際には`cargo run`できないバグ入りのものでした。以下は、実際に動かして見つかったそのバグを直した後の内容です。ドキュメントもAIの説明も、実際にコンパイルが通るところまで確認しないと100%は信用できない、という良い実例です。

## 必要なEntityは2つ

ドキュメントによれば、`Atmosphere`は「大気（惑星）そのもの」を表すEntityと、それを描画する「カメラ」用のEntityを別々に用意する構成になっています。

**1. 大気そのもの**

`Atmosphere::earth()`は、地球っぽい大気のパラメータ（半径など）をまとめて設定してくれる専用のコンストラクタですが、実際には大気の「組成」を表す`ScatteringMedium`という別のAssetへの`Handle`を1つ引数に取ります。これはAsset章で見た`Handle<Image>`と同じ考え方で、`Assets<ScatteringMedium>`に追加して`Handle`を取得します。

```rust
use bevy::light::Atmosphere;
use bevy::light::atmosphere::ScatteringMedium;

fn setup(mut commands: Commands, mut media: ResMut<Assets<ScatteringMedium>>) {
    let medium = media.add(ScatteringMedium::default());

    commands.spawn((
        Atmosphere::earth(medium),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}
```

`ScatteringMedium::default()`は、内部的に`ScatteringMedium::earth(256, 256)`（地球の大気組成データを、それなりの解像度で生成したもの）を返すようになっているので、細かい値を自分で決めなくても、これだけで地球っぽい大気になります。

このEntityの`Transform`（正確には`GlobalTransform`）が「惑星の中心」の座標として扱われます。他に`Atmosphere::mars()`もあります（`ScatteringMedium`にも同様に`mars()`があります）。

## 補足: スケールに要注意

`Atmosphere::earth()`は、`inner_radius`（地表）を`6_360_000.0`（メートル単位、つまり6360km）という**実際の地球の半径**に設定します。大気Entityの座標は「惑星の中心」なので、これを原点`(0.0, 0.0, 0.0)`に置くと、地表は原点から638万メートルも離れた場所になってしまいます。

これまでの章のカメラは原点から数メートルの距離に置いていましたが、そのままだとカメラは地表よりずっと内側（惑星の中心＝岩盤の中）にいることになり、大気の層（地表からさらに10万メートル上空まで）に全く届きません。大気を見るには、カメラを「地表からの高さ」の位置に置く必要があります。

簡単な対処法は、大気Entity自体を`inner_radius`の分だけ下にずらして、原点付近が「ちょうど地表」になるようにすることです。

```rust
commands.spawn((
    Atmosphere::earth(medium),
    Transform::from_xyz(0.0, -6_360_000.0, 0.0),
));
```

こうしておけば、原点は地表からの高さ`0`の地点になるので、これまで通り`Transform::from_xyz(0.0, 2.0, 5.0)`のような小さい数値でカメラを置くだけで、地表付近から空を見上げる形になります。

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
use bevy::light::atmosphere::ScatteringMedium;
use bevy::pbr::AtmosphereSettings;
use bevy::prelude::*;

fn setup(mut commands: Commands, mut media: ResMut<Assets<ScatteringMedium>>) {
    let medium = media.add(ScatteringMedium::default());

    commands.spawn((
        Atmosphere::earth(medium),
        Transform::from_xyz(0.0, -6_360_000.0, 0.0),
    ));

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
