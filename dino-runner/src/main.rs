use bevy::{app::AppExit, prelude::*, sprite::collide_aabb};

mod movement;
mod obstacles;

use movement::movement_plugin;
use obstacles::obstacles_plugin;

/// The width of the game area
// We need to use the trailing `.` because these numbers are floats - won't be automatically inferred
const WIDTH: f32 = 600.;
/// The height of the game area
const HEIGHT: f32 = 200.;

/// The height of the ground above the bottom of the screen
const GROUND_HEIGHT: f32 = 15.;
/// The y level of the top of the ground
const GROUND_Y: f32 = (-HEIGHT) / 2. + GROUND_HEIGHT;
/// The movement speed stationary objects move
const BASE_MOVEMENT_SPEED: f32 = -200.;

/// Marks the entity which is our player. There will only ever be one
/// entity with the [`Dinosaur`] component
struct Dinosaur;
/// Marks obstacles which kill our player when it collides with them
struct Obstacle;

fn main() {
    let mut app = App::new();
    // Set the details of the window
    app.insert_resource(WindowDescriptor {
        width: WIDTH,
        height: HEIGHT,
        // We override the scale factor to make the game's size better depending on your screen size.
        // Rule of thumb: 1080p: `1.`; 1440p: 2.; 4k: 3.
        scale_factor_override: Some(3.),
        // We turn the `&'static str` "Dinosaurs" into a heap allocated `String`, which is required for
        // this field of `WindowDescriptor`
        title: "Dinosaurs".into(),
        // Disabling resizing means that we don't have to worry about content being incorrectly off-screen
        // (barring https://github.com/bevyengine/bevy/issues/2751, which breaks with different scales)
        resizable: false,
        ..Default::default()
    })
    // Note that the setting the window details needs to be before adding [`DefaultPlugins`] - this might change later
    // Add the default plugins, so bevy sets up windows and rendering foro us to use
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup)
    .add_system(collision);
    // Initialise the custom modules. An alternative would be to use a 'proper' `Plugin` -
    // a plugin ends up being very
    obstacles_plugin(&mut app);
    movement_plugin(&mut app);
    // Run the app
    app.run();
}

fn setup(
    // Commands are use to run delayed operations on the `World`, when no systems are running
    // This is generally used for spawning entities, which requires manipulating the data structures in a
    // way which cannot be done concurrently
    // Note that this means that the entities are not spawned until the end of the relevant stage
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Create the default kind of 2d camera for sprite
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    // Create the ground
    commands.spawn_bundle(SpriteBundle {
        // Materials are stored using handles, which let you reuse the same data on the GPU for multiple sprites.
        // For example, consider a 2d puzzle game - each tile appears multiple times, but can use the same GPU texture
        // This deduplication is managed 'manually' by using the `Assets` resource for given material
        material: materials.add(Color::YELLOW.into()),
        // The sprite component stores how large the sprite is on screen. For sprites with images (in the material),
        // the size is automatically determined (by default), but because we do not have a `Texture`, we must specify it manually
        sprite: Sprite::new(Vec2::new(WIDTH, GROUND_HEIGHT)),
        // A transform is a location in 3d space. This is used to determine where it should be rendered
        // A sprite is rendered with its center at the transform. This means that to place the bottom of the sprite at a point
        // we need to add an offset of half its height
        transform: Transform::from_xyz(0., (-HEIGHT + GROUND_HEIGHT) / 2., 0.),
        ..Default::default()
    });
}

fn collision(
    // An `EventWriter` 'sends'/writes events to the global events tracker for a given type
    // Reading events uses `EventReader`, which ensures that each system only reads the event once.
    mut exit: EventWriter<AppExit>,
    dino: Query<(&Transform, &Sprite), With<Dinosaur>>,
    obstacles: Query<(&Transform, &Sprite), With<Obstacle>>,
) {
    // We use the `single` method here because we know that there will only be one dinosaur.
    // The unwrap will panic (broadly, stop the program) if that isn't true
    let (dino_pos, dino_sprite) = dino.single().unwrap();
    for (obstacle_pos, obstacle_sprite) in obstacles.iter() {
        // collide_aabb is a very simple, built in utility for collision detection
        if let Some(_) = collide_aabb::collide(
            obstacle_pos.translation,
            obstacle_sprite.size,
            dino_pos.translation,
            dino_sprite.size,
        ) {
            // When the dino collides with an obstacle, we stop the program
            // The main loop reads the `AppExit` events at the end of each frame, and closes if one is sent
            // (note that closing the window using operating system facilities works as expected)
            exit.send(AppExit);
        };
    }
}
