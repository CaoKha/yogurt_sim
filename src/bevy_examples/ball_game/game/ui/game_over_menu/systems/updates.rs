use bevy::prelude::*;

use crate::bevy_examples::ball_game::events::GameOver;
use crate::bevy_examples::ball_game::game::ui::game_over_menu::components::FinalScoreText;

pub fn update_final_score_text(
    mut game_over_event_reader: EventReader<GameOver>,
    mut text_query: Query<&mut Text, With<FinalScoreText>>,
) {
    for event in game_over_event_reader.iter() {
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("Final Score: {}", event.score.to_string());
        }
    }
}
