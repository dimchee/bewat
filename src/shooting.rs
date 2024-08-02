use bevy::prelude::*;

#[derive(Component)]
pub struct Gun;

#[derive(Component)]
pub struct Bullet {
    velocity: Vec3,
    lifetime: Timer,
}

pub fn shoot_bullet(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    gun_query: Query<(&Gun, &Transform)>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        if let Ok((_gun, gun_transform)) = gun_query.get_single() {
            let bullet_direction = gun_transform.forward();
            let bullet_spawn_position = gun_transform.translation + bullet_direction * 2.0;

            commands.spawn((
                Bullet {
                    velocity: bullet_direction * 5.0,
                    lifetime: Timer::from_seconds(3.0, TimerMode::Once),
                },
                PbrBundle {
                    transform: Transform::from_translation(bullet_spawn_position),
                    mesh: meshes.add(Capsule3d {
                        radius: 0.2,
                        half_length: 0.0,
                    }),
                    material: materials.add(Color::srgb(0.1, 0.1, 0.1)),
                    ..default()
                },
            ));
        }
    }
}

pub fn move_bullets(
    mut commands: Commands,
    time: Res<Time>,
    mut bullets: Query<(Entity, &mut Bullet, &mut Transform)>,
) {
    for (entity, mut bullet, mut transform) in bullets.iter_mut() {
        bullet.lifetime.tick(time.delta());
        if bullet.lifetime.finished() {
            commands.entity(entity).despawn();
        } else {
            transform.translation += bullet.velocity * time.delta_seconds();
        }
    }
}
