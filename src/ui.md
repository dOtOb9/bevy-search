# UI（最低限の使い方）

避難シミュレーションのような用途だと、UIは凝った演出よりも「状態を表示する」「ボタンで操作する」くらいの、コントロールパネル的な最低限の使い方が中心になります。ここではその範囲だけを扱います。

## Node: レイアウトの基本単位

`Node`は、HTMLでいう`<div>`に近い、UIのレイアウトを表すComponentです。Flexboxと同じ考え方（`width`/`height`/`justify_content`/`align_items`など）でレイアウトを組みます。

```rust
commands.spawn(Node {
    width: Val::Percent(100.0),
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    ..default()
});
```

Hierarchy章で紹介した`with_children`がそのまま使えます。UIの入れ子構造も、普通のEntityの親子関係として組み立てます。

## Text: 文字を表示する

```rust
commands.spawn(Text::new("Running"));
```

`Text`は中身が`String`1個だけのタプル構造体です（System章で扱ったタプル構造体そのままです）。

## Button: クリックを検知する

`Button`は中身を持たないマーカーComponent（Entity/Component章で紹介したパターンと同じです）で、これを付けたUI要素はクリック判定の対象になります。実際に押されたかどうかは、自動的に付与される`Interaction`というComponentで判定します。

```rust
fn button_system(
    mut query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut color) in &mut query {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::srgb(0.6, 0.2, 0.2));
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgb(0.3, 0.3, 0.3));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::srgb(0.2, 0.2, 0.2));
            }
        }
    }
}
```

`Changed<Interaction>`を使っているのは、変更検知章で見た通り「マウスの状態が変わった瞬間だけ」処理すれば十分で、毎フレーム全ボタンをチェックし直す必要が無いからです。

## UE5との対比

UE5の`UMG`（Widget Blueprint）は、Widget自体が独自のクラス階層・独自のイベントシステム（`OnClicked`のようなDelegate）を持つ、UI専用の別世界という色合いが強いです。BevyのUIは、普通のEntity・Component・Systemの延長線上にあり、ボタンも「`Button`というマーカーを持ったEntity」でしかありません。UI専用の特別な仕組みを新しく覚える必要が少ない、というのがBevyのUIの特徴です。

## 試してみる

`code/examples/ui.rs`を作り、以下を書いてください。中央にボタンを1つ配置し、状態（ホバー/押下/なし）に応じて色が変わる例です。

```rust
use bevy::prelude::*;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(150.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                ))
                .with_children(|parent| {
                    parent.spawn(Text::new("Toggle"));
                });
        });
}

fn button_system(
    mut query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut color) in &mut query {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::srgb(0.6, 0.2, 0.2));
                println!("clicked");
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgb(0.3, 0.3, 0.3));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::srgb(0.2, 0.2, 0.2));
            }
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, button_system)
        .run();
}
```

```
cargo run --example ui
```

ボタンにカーソルを乗せると色が変わり、クリックするとコンソールに`clicked`が出力されれば成功です。
