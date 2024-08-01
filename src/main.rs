use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, load))
        .add_systems(Update, camera_movement)
        .run();
}

fn load(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((SceneBundle {
        scene: asset_server.load("Dungeon.glb#Scene0"),
        ..default()
    },));
}

#[derive(Bundle)]
struct PlayerBundle {}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn a light and the camera
    commands.spawn((
        MoveWithWASD { speed_multiplier: 1.0 },
        PointLightBundle {
            transform: Transform::from_translation(Vec3::new(1.0, 3.0, 1.0)),
            ..default()
        },
    ));
    commands.spawn((
        MoveWithWASD { speed_multiplier: 1.0 },
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(12.0, 20.0, 12.0))
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            ..default()
        },
    ));
    commands.spawn((
        MoveWithWASD { speed_multiplier: 1.0 },
        PbrBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 1.0, 0.0)),
            mesh: meshes.add(Capsule3d {
                radius: 0.5,
                half_length: 0.5,
            }),
            material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
            ..default()
        },
    ));
}

#[derive(Component)]
struct MoveWithWASD {
    speed_multiplier: f32,
}

fn camera_movement(
    mut query: Query<(&mut Transform, &MoveWithWASD)>,
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

        transform.translation += direction * 8.0 * movement.speed_multiplier * time.delta_seconds();
    }
}
