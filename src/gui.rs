use bevy::prelude::*;
use bevy::window::{close_on_esc, PrimaryWindow};

use crate::config::{WORLD_HEIGHT, WORLD_WIDTH};

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, close_on_esc)
            .add_systems(Update, show_world_border);
    }
}

fn spawn_camera(mut commands: Commands, window: Query<&Window, With<PrimaryWindow>>) {
    let window = window.get_single().unwrap();
    let x = window.width() / 2.0;
    let y = window.height() / 2.0;

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(x, y, 0.0),
        ..default()
    });
}

fn show_world_border(mut gizmos: Gizmos) {
    gizmos.rect_2d(
        Vec2 {
            x: WORLD_WIDTH / 2.0,
            y: WORLD_HEIGHT / 2.0,
        },
        0.0,
        Vec2 {
            x: WORLD_WIDTH,
            y: WORLD_HEIGHT,
        },
        Color::WHITE,
    );
}
