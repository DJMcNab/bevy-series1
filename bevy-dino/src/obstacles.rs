use bevy::prelude::*;

use crate::{Obstacle, BASE_MOVEMENT_SPEED, GROUND_Y, WIDTH};

pub(crate) fn obstacles(app: &mut App) {
    app.add_system(spawn_obstacles)
        .add_system(move_obstacles)
        .add_system(despawn_obstacles)
        .insert_resource(EnemyTimer(Timer::from_seconds(3., true)));
}

struct EnemyTimer(pub Timer);
struct ObstacleVelocity(f32);

const CACTUS_HEIGHT: f32 = 45.;

fn spawn_obstacles(
    time: Res<Time>,
    mut timer: ResMut<EnemyTimer>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        commands
            .spawn_bundle(SpriteBundle {
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

fn despawn_obstacles(mut enemies: Query<(Entity, &Transform)>, mut commands: Commands) {
    for (enemy, transform) in enemies.iter_mut() {
        if transform.translation.x.abs() > WIDTH * 2. {
            commands.entity(enemy).despawn();
        }
    }
}
