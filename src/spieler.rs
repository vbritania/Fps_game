use bevy::prelude::*;
use bevy::core_pipeline::core_3d::Camera3d;
use bevy::input::keyboard::KeyCode;
use bevy::input::mouse::MouseMotion;
use bevy_mod_raycast::prelude::*;
use crate::MyRaycastSet;


pub fn setup_spieler(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 3.0, 5.0)
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
    let sens = 0.008;
    let mut yaw = 0.0;
    let mut pitch = 0.0;

    for b in maus_bewegung.read() {
        yaw -= b.delta.x * sens;
        pitch -= b.delta.y * sens;
    }

    if yaw != 0.0 || pitch != 0. {
        transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(yaw));
        transform.rotate(Quat::from_rotation_y(yaw));

        let aktuelle_pitch = transform.rotation.to_euler(EulerRot::YXZ).1;
        let neuer_pitch = (aktuelle_pitch + pitch).clamp(-1.55, 1.55);
        let delta_pitch = neuer_pitch - aktuelle_pitch;
        transform.rotate_local_x(delta_pitch);
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