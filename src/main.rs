//! Shows how to set the solid color that is used to paint the window before the frame gets drawn.
//!
//! Acts as background color, since pixels that are not drawn in a frame remain unchanged.

use bevy::{color::palettes::css::PURPLE, prelude::*};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.5, 0.5, 0.9)))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, load))
        .add_systems(Update, change_clear_color)
        .add_systems(Update, camera_movement)
        .run();
}

fn load(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((SceneBundle {
        scene: asset_server.load("Dungeon.glb#Scene0"),
        ..default()
    },));
}
fn setup(mut commands: Commands) {
    // Spawn a light and the camera
    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(3.0, 4.0, 3.0)),
        ..default()
    });
    commands.spawn((
        Movement {
            speed: 150.0,
        },
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(100.0, 150.0, 100.0))
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            ..default()
        }
    ));
}

fn change_clear_color(input: Res<ButtonInput<KeyCode>>, mut clear_color: ResMut<ClearColor>) {
    if input.just_pressed(KeyCode::Space) {
        clear_color.0 = PURPLE.into();
    }
}

#[derive(Component)]
struct Movement {
    speed: f32,
}

fn camera_movement(
    mut query: Query<(&mut Transform, &Movement)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, movement) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if input.pressed(KeyCode::KeyW) {
            direction.z -= 1.0;
        }
        if input.pressed(KeyCode::KeyS) {
            direction.z += 1.0;
        }
        if input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }

        if direction.length_squared() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * movement.speed * time.delta_seconds();
    }
}
