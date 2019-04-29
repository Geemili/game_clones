use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;

// You'll have to mark PADDLE_HEIGHT as public in pong.rs
use crate::dodger::{Player, ARENA_HEIGHT, ARENA_WIDTH, PLAYER_HEIGHT, PLAYER_WIDTH};

pub struct PlayerSystem;

impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (mut transforms, players, input): Self::SystemData) {
        for (player, transform) in (&players, &mut transforms).join() {
            let vertical_move = input.axis_value("vertical");
            if let Some(mv_amount) = vertical_move {
                let scaled_amount = 1.2 * mv_amount as f32;
                let y = transform.translation().y;
                transform.set_y(
                    (y + scaled_amount)
                        .min(ARENA_HEIGHT - PLAYER_HEIGHT * 0.5)
                        .max(PLAYER_HEIGHT * 0.5),
                );
            }

            let horizontal_move = input.axis_value("horizontal");
            if let Some(mv_amount) = horizontal_move {
                let scaled_amount = 1.2 * mv_amount as f32;
                let x = transform.translation().x;
                transform.set_x(
                    (x + scaled_amount)
                        .min(ARENA_WIDTH - PLAYER_WIDTH * 0.5)
                        .max(PLAYER_WIDTH * 0.5),
                );
            }
        }
    }
}
