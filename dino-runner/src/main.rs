use bevy::{app::AppExit, prelude::*, sprite::collide_aabb};

mod movement;
mod obstacles;

use movement::movement;
use obstacles::obstacles;

const WIDTH: f32 = 600.;
const HEIGHT: f32 = 200.;

const GROUND_HEIGHT: f32 = 15.;
const GROUND_Y: f32 = (-HEIGHT) / 2. + GROUND_HEIGHT;
const BASE_MOVEMENT_SPEED: f32 = -200.;

struct Dinosaur;
struct Obstacle;

fn main() {
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        width: WIDTH,
        height: HEIGHT,
        scale_factor_override: Some(3.),
        title: "Dinosaurs".into(),
        resizable: false,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup)
    .add_system(collision);
    obstacles(&mut app);
    movement(&mut app);
    app.run();
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    // Create the ground
    commands.spawn_bundle(SpriteBundle {
        // Details? Where we're going, we don't need details
        material: materials.add(Color::YELLOW.into()),
        sprite: Sprite::new(Vec2::new(WIDTH, GROUND_HEIGHT)),
        transform: Transform::from_xyz(0., (-HEIGHT + GROUND_HEIGHT) / 2., 0.),
        ..Default::default()
    });
}

fn collision(
    mut exit: EventWriter<AppExit>,
    dino: Query<(&Transform, &Sprite), With<Dinosaur>>,
    obstacles: Query<(&Transform, &Sprite), With<Obstacle>>,
) {
    let (dino_pos, dino_sprite) = dino.single().expect("Dinosaur Exists");
    for (obstacle_pos, obstacle_sprite) in obstacles.iter() {
        if let Some(_) = collide_aabb::collide(
            obstacle_pos.translation,
            obstacle_sprite.size,
            dino_pos.translation,
            dino_sprite.size,
        ) {
            exit.send(AppExit);
        };
    }
}
