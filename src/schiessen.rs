use bevy::prelude::*;
use bevy::input::mouse::MouseButton;
use bevy_mod_raycast::prelude::*;
use crate::ziel::Ziel;

pub fn schiessen(
    maus: Res<ButtonInput<MouseButton>>,
    cursor_ray: Res<CursorRay>,
    mut raycast: Raycast,
    ziele: Query<Entity, With<Ziel>>,
) {
    if maus.just_pressed(MouseButton::Left) {
        if let Some(ray) = **cursor_ray {
            let filter = |entity| ziele.contains(entity);
            let settings = RaycastSettings::default().with_filter(&filter);
            let hits = raycast.cast_ray(ray, &settings);

            for (entity, hit) in hits {
                println!("Getroffen! Entity: {:?}, Distanz: {}", entity, hit.distance());
            }
        }
    }
}