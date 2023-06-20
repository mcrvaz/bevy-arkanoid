use bevy::prelude::*;

const FONT_ASSET: &str = "fonts/Roboto-Regular.ttf";

pub fn spawn_hud(mut cmd: Commands, asset_sv: Res<AssetServer>) {
    cmd.spawn((
        TextBundle::from_section(
            "hello\nbevy!",
            TextStyle {
                font: asset_sv.load(FONT_ASSET),
                font_size: 100.0,
                color: Color::WHITE,
            },
        ) // Set the alignment of the Text
        .with_text_alignment(TextAlignment::Center),
        // // Set the style of the TextBundle itself.
        // .with_style(Style {
        //     position_type: PositionType::Absolute,
        //     bottom: Val::Px(5.0),
        //     right: Val::Px(15.0),
        //     ..default()
        // }),
    ));
}
