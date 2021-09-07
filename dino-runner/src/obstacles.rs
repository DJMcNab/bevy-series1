use bevy::prelude::*;

use crate::{Obstacle, BASE_MOVEMENT_SPEED, GROUND_Y, WIDTH};

pub(crate) fn obstacles_plugin(app: &mut App) {
    app.add_system(spawn_obstacles)
        .add_system(move_obstacles)
        .add_system(despawn_obstacles)
        // A timer lets you act after a certain amount of time has passed
        .insert_resource(EnemyTimer(Timer::from_seconds(3., true)));
}

struct EnemyTimer(Timer);
struct ObstacleVelocity(f32);

const CACTUS_HEIGHT: f32 = 45.;

fn spawn_obstacles(
    time: Res<Time>,
    mut timer: ResMut<EnemyTimer>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Timers need to be 'ticked' manually - for example, this allows them to be more usable for
    // slowing down your simulation
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        commands
            .spawn_bundle(SpriteBundle {
                // Note that in this example, we don't deduplicate the materials
                // However, this does not create a memory leak because when there are no remaining references to an asset, it is
                // freed
                material: materials.add(Color::GREEN.into()),
                sprite: Sprite::new(Vec2::new(20., CACTUS_HEIGHT)),
                transform: Transform::from_xyz(
                    WIDTH / 2. + 100.,
                    GROUND_Y + CACTUS_HEIGHT / 2.,
                    0.,
                ),
                ..Default::default()
            })
            .insert_bundle((ObstacleVelocity(BASE_MOVEMENT_SPEED), Obstacle));
    }
}

fn move_obstacles(mut enemies: Query<(&mut Transform, &ObstacleVelocity)>, time: Res<Time>) {
    for (mut transform, velocity) in enemies.iter_mut() {
        transform.translation.x += velocity.0 * time.delta_seconds();
    }
}

fn despawn_obstacles(
    mut enemies: Query<(Entity, &Transform), With<Obstacle>>,
    mut commands: Commands,
) {
    for (enemy, transform) in enemies.iter_mut() {
        if transform.translation.x.abs() > WIDTH * 2. {
            commands.entity(enemy).despawn();
        }
    }
}
