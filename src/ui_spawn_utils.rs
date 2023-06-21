use bevy::prelude::*;

use crate::game_components::ScoreCounter;

const FONT_ASSET: &str = "fonts/Roboto-Regular.ttf";

pub fn spawn_hud(cmd: Commands, asset_sv: Res<AssetServer>) {
    spawn_score_counter(cmd, asset_sv);
}

fn spawn_score_counter(mut cmd: Commands, asset_sv: Res<'_, AssetServer>) {
    cmd.spawn((TextBundle::from_section(
        "999",
        TextStyle {
            font: asset_sv.load(FONT_ASSET),
            font_size: 50.0,
            color: Color::WHITE,
        },
    )
    .with_text_alignment(TextAlignment::Center)
    .with_style(Style {
        position_type: PositionType::Absolute,
        margin: UiRect {
            left: Val::Percent(2.5),
            top: Val::Percent(2.5),
            ..default()
        },
        ..default()
    }),))
        .insert(ScoreCounter {});
}
