mod bevy_examples;

#[cfg(feature = "ball_game")]
use bevy_examples::ball_game::run_ball_game;
#[cfg(feature = "rapier2d")]
use bevy_examples::bevy_rapier2d::run_rapier2d_example;
#[cfg(feature = "rapier3d")]
use bevy_examples::bevy_rapier3d::run_rapier3d_example;
#[cfg(feature = "lighting")]
use bevy_examples::lighting::run_lighting_example;

fn main() {
    #[cfg(feature = "lighting")]
    {
        run_lighting_example();
    }
    #[cfg(feature = "ball_game")]
    {
        run_ball_game();
    }
    #[cfg(feature = "rapier3d")]
    {
        run_rapier3d_example();
    }
}
