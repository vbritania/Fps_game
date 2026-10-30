use bevy::prelude::*;
use bevy::core_pipeline::core_3d::Camera3d;
use bevy::input::keyboard::KeyCode;
use bevy::input::mouse::MouseMotion;
use bevy_mod_raycast::prelude::*;
use crate::MyRaycastSet;

pub fn setup_spieler(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 2.0, 5.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        RaycastSource::<MyRaycastSet>::new(),
    ));
}

pub fn bewege_kamera(
    mut query: Query<&mut Transform, With<Camera3d>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut maus_bewegung: EventReader<MouseMotion>,
    zeit: Res<Time>,
) {
    let Ok(mut transform) = query.get_single_mut() else { return; };
    let geschwindigkeit = 5.0;

    for b in maus_bewegung.read() {
        transform.rotate_y(-b.delta.x * 0.002);
    }

    let mut richtung = Vec3::ZERO;
    if keyboard.pressed(KeyCode::KeyW) {
        richtung += transform.forward().as_vec3();
    }
    if keyboard.pressed(KeyCode::KeyS) {
        richtung -= transform.forward().as_vec3();
    }
    if keyboard.pressed(KeyCode::KeyA) {
        richtung -= transform.right().as_vec3();
    }
    if keyboard.pressed(KeyCode::KeyD) {
        richtung += transform.right().as_vec3();
    }

    richtung.y = 0.0;
    if richtung.length() > 0.0 {
        let bewegung = richtung.normalize() * geschwindigkeit * zeit.delta_seconds();
        transform.translation += bewegung;
    }
}