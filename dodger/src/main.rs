extern crate amethyst;

mod dodger;

use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    renderer::{DisplayConfig, DrawFlat, Pipeline, PosNormTex, RenderBundle, Stage},
    utils::application_root_dir,
};
use dodger::Dodger;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let path = format!("{}/resources/display_config.ron", application_root_dir());
    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.00196, 0.23726, 0.21765, 1.0], 1.0)
            .with_pass(DrawFlat::<PosNormTex>::new()),
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?
        .with_bundle(TransformBundle::new())?;
    let mut game = Application::new("./", Dodger, game_data)?;

    game.run();

    Ok(())
}
