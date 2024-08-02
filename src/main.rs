use bevy::prelude::*;

pub mod shooting;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Movement { speed: 12.0 })
        .add_systems(Startup, (setup, load))
        .add_systems(Update, camera_movement)
        .add_systems(Update, shooting::shoot_bullet)
        .add_systems(Update, shooting::move_bullets)
        .run();
}

fn load(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((SceneBundle {
        scene: asset_server.load("Dungeon.glb#Scene0"),
        ..default()
    },));
}

#[derive(Component)]
struct Player;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(12.0, 20.0, 12.0))
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            ..default()
        },
        Player,
    ));
    commands.spawn((
        SpotLightBundle {
            spot_light: SpotLight {
                intensity: 8_000_000.0,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 3.0, 2.0)),
            ..default()
        },
        Player,
    ));
    commands.spawn((
        Transform::from_translation(Vec3::new(0.0, 1.0, 2.0)),
        shooting::Gun,
        Player,
    ));
    commands.spawn((
        PbrBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 1.0, 0.0)),
            mesh: meshes.add(Capsule3d {
                radius: 0.5,
                half_length: 0.5,
            }),
            material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
            ..default()
        },
        Player,
    ));
}

#[derive(Resource)]
struct Movement {
    speed: f32,
}

fn camera_movement(
    mut query: Query<&mut Transform, With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    movement: Res<Movement>,
) {
    for mut transform in query.iter_mut() {
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
