use bevy::prelude::*;

#[derive(SystemLabel, Clone, Hash, Debug, PartialEq, Eq)]
enum MotionSystems {
    MotionApplied,
    GroundCollision,
}

#[derive(AmbiguitySetLabel, Clone, Hash, Debug, PartialEq, Eq)]
struct DinoMotionModifiers;

pub(crate) fn movement_plugin(app: &mut App) {
    app.add_startup_system(create_dino)
        // This ambiguity set labels are 'optional' - probably best not covered
        .add_system_set(
            SystemSet::new()
                .in_ambiguity_set(DinoMotionModifiers)
                .before(MotionSystems::MotionApplied)
                .with_system(jump)
                .with_system(snap)
                .with_system(gravity),
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

use crate::{Dinosaur, GROUND_Y, WIDTH};

struct VerticalVelocity(f32);

const DINO_HEIGHT: f32 = 60.;
const DINO_GROUND_Y: f32 = GROUND_Y + DINO_HEIGHT / 2.;

fn create_dino(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    // Spawn our dinosaur
    commands
        .spawn_bundle(SpriteBundle {
            // What do you mean a blue block isn't a dinosaur?
            material: materials.add(Color::BLUE.into()),
            sprite: Sprite::new(Vec2::new(40.0, DINO_HEIGHT)),
            transform: Transform::from_xyz(-WIDTH / 2. + 40., DINO_GROUND_Y, 0.),
            ..Default::default()
        })
        .insert_bundle((Dinosaur, GroundState::OnGround, VerticalVelocity(0.)));
}

fn jump(
    mut dino: Query<(&mut VerticalVelocity, &mut GroundState), With<Dinosaur>>,
    keyboard: Res<Input<KeyCode>>,
) {
    let (mut dino_velocity, mut grounded) = dino.single_mut().unwrap();
    if matches!(*grounded, GroundState::OnGround) && keyboard.pressed(KeyCode::Space) {
        *grounded = GroundState::InAir;
        (*dino_velocity).0 = 300.;
    }
}

fn vertical_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &GroundState, &VerticalVelocity)>,
) {
    for (mut pos, ground, velocity) in query.iter_mut() {
        if matches!(ground, GroundState::InAir) {
            pos.translation.y += velocity.0 * time.delta_seconds();
        }
    }
}

fn gravity(time: Res<Time>, mut query: Query<(&GroundState, &mut VerticalVelocity)>) {
    for (ground, mut velocity) in query.iter_mut() {
        if matches!(ground, GroundState::InAir) {
            velocity.0 -= 500. * time.delta_seconds();
        }
    }
}

fn grounding(mut query: Query<(&mut GroundState, &mut Transform, &Sprite)>) {
    for (mut grounded, mut transform, sprite) in query.iter_mut() {
        let base_height = GROUND_Y + sprite.size.y / 2.;
        if transform.translation.y < base_height {
            *grounded = GroundState::OnGround;
            transform.translation.y = base_height;
        }
    }
}

fn snap(
    mut dino: Query<(&mut VerticalVelocity, &GroundState), With<Dinosaur>>,
    keyboard: Res<Input<KeyCode>>,
) {
    let (mut velocity, grounded) = dino.single_mut().unwrap();
    if (keyboard.pressed(KeyCode::Down) || keyboard.pressed(KeyCode::S))
        && matches!(grounded, GroundState::InAir)
    {
        velocity.0 -= 600.;
    }
}
