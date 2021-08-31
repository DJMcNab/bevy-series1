use bevy::prelude::*;

#[derive(SystemLabel, Clone, Hash, Debug, PartialEq, Eq)]
enum MotionSystems {
    MotionApplied,
    GroundCollision,
}

#[derive(AmbiguitySetLabel, Clone, Hash, Debug, PartialEq, Eq)]
struct DinoMotionModifiers;

pub(crate) fn movement(app: &mut App) {
    app.add_startup_system(create_dino)
        // This ambiguity set labels are 'optional' - probably best not covered
        .add_system(
            jump.in_ambiguity_set(DinoMotionModifiers)
                .before(MotionSystems::MotionApplied),
        )
        .add_system(
            snap.in_ambiguity_set(DinoMotionModifiers)
                .before(MotionSystems::MotionApplied),
        )
        .add_system(
            gravity
                .in_ambiguity_set(DinoMotionModifiers)
                .before(MotionSystems::MotionApplied),
        )
        .add_system(
            vertical_movement
                .label(MotionSystems::MotionApplied)
                .before(MotionSystems::GroundCollision),
        )
        .add_system(grounding.label(MotionSystems::GroundCollision));
}

enum GroundState {
    OnGround,
    InAir,
}
use GroundState::*;

use crate::{Dinosaur, GROUND_Y, WIDTH};
struct Falls {
    ground_height: f32,
}

struct VerticalVelocity(f32);

const DINO_HEIGHT: f32 = 60.;
const DINO_GROUND_Y: f32 = GROUND_Y + DINO_HEIGHT / 2.;

fn create_dino(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    // Spawn our dinosaur
    commands
        .spawn_bundle(SpriteBundle {
            // Details? Where we're going, we don't need details
            material: materials.add(Color::BLUE.into()),
            sprite: Sprite::new(Vec2::new(40.0, DINO_HEIGHT)),
            transform: Transform::from_xyz(-WIDTH / 2. + 40., DINO_GROUND_Y, 0.),
            ..Default::default()
        })
        .insert_bundle((
            Dinosaur,
            GroundState::OnGround,
            VerticalVelocity(0.),
            Falls {
                ground_height: DINO_GROUND_Y,
            },
        ));
}

fn jump(
    mut dino: Query<(&mut VerticalVelocity, &mut GroundState), With<Dinosaur>>,
    keyboard: Res<Input<KeyCode>>,
) {
    let (mut dino_velocity, mut grounded) = dino.single_mut().expect("Dinosaur should exist");
    if matches!(*grounded, OnGround) && keyboard.pressed(KeyCode::Space) {
        *grounded = GroundState::InAir;
        (*dino_velocity).0 = 300.;
    }
}

fn vertical_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &GroundState, &VerticalVelocity)>,
) {
    for (mut pos, ground, velocity) in query.iter_mut() {
        if matches!(ground, InAir) {
            pos.translation.y += velocity.0 * time.delta_seconds();
        }
    }
}

fn gravity(time: Res<Time>, mut query: Query<(&GroundState, &mut VerticalVelocity)>) {
    for (ground, mut velocity) in query.iter_mut() {
        if matches!(ground, InAir) {
            velocity.0 -= 500. * time.delta_seconds();
        }
    }
}

fn grounding(mut query: Query<(&mut GroundState, &mut Transform, &Falls)>) {
    for (mut grounded, mut transform, fallness) in query.iter_mut() {
        if transform.translation.y < fallness.ground_height {
            *grounded = OnGround;
            transform.translation.y = fallness.ground_height;
        }
    }
}

fn snap(
    mut dino: Query<(&mut VerticalVelocity, &GroundState), With<Dinosaur>>,
    keyboard: Res<Input<KeyCode>>,
) {
    let (mut velocity, grounded) = dino.single_mut().expect("Dino exists");
    if (keyboard.pressed(KeyCode::Down) || keyboard.pressed(KeyCode::S))
        && matches!(grounded, InAir)
    {
        velocity.0 -= 600.;
    }
}
