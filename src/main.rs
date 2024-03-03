use bevy::prelude::*;

use cool_particles::{config::*, gui::GuiPlugin, particle::ParticlePlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (WORLD_WIDTH, WORLD_HEIGHT).into(),
                title: "cool-particles".to_owned(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(GuiPlugin)
        .add_plugins(ParticlePlugin)
        .run();
}
