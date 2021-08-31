//! This program can be used to check whether bevy programs can run on your machine

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(Color::BISQUE.into()),
        sprite: Sprite::new(Vec2::new(120.0, 30.0)),
        ..Default::default()
    });
}
