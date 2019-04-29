use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;

// You'll have to mark PADDLE_HEIGHT as public in pong.rs
use crate::{
    config::ArenaConfig,
    dodger::{Player, PLAYER_HEIGHT, PLAYER_SPEED, PLAYER_WIDTH},
};

pub struct PlayerSystem;

impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<String, String>>,
        Read<'s, ArenaConfig>,
    );

    fn run(&mut self, (mut transforms, players, input, arena_config): Self::SystemData) {
        let (arena_width, arena_height) = (arena_config.width, arena_config.height);
        for (player, transform) in (&players, &mut transforms).join() {
            let vertical_move = input.axis_value("vertical");
            if let Some(mv_amount) = vertical_move {
                let scaled_amount = PLAYER_SPEED * mv_amount as f32;
                let y = transform.translation().y;
                transform.set_y(
                    (y + scaled_amount)
                        .min(arena_height - PLAYER_HEIGHT * 0.5)
                        .max(PLAYER_HEIGHT * 0.5),
                );
            }

            let horizontal_move = input.axis_value("horizontal");
            if let Some(mv_amount) = horizontal_move {
                let scaled_amount = PLAYER_SPEED * mv_amount as f32;
                let x = transform.translation().x;
                transform.set_x(
                    (x + scaled_amount)
                        .min(arena_width - PLAYER_WIDTH * 0.5)
                        .max(PLAYER_WIDTH * 0.5),
                );
            }
        }
    }
}
