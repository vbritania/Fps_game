use bevy::prelude::*;
use bevy::{ecs::bundle, prelude::*};



use bevy::input::mouse::MouseButton;
use crate::ziel::{Ziel, spawne_ziel};

pub struct SchiessenPlugin;

impl Plugin for SchiessenPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SchussAbgefeuert>();
        app.add_systems(Update, (schiessen, spawne_tracer, bewege_bullets));
    }
}

#[derive(Event)]
struct SchussAbgefeuert {
    origin: Vec3,
    direction: Vec3,
}

#[derive(Component)]
struct Bullet {
    direction: Vec3,
    speed: f32,
    traveled: f32,
}

fn schiessen(
    maus: Res<ButtonInput<MouseButton>>,
    kamera: Query<&GlobalTransform, With<Camera3d>>,
    mut schuss_events: EventWriter<SchussAbgefeuert>,
) {
    if !maus.just_pressed(MouseButton::Left) {
        return;
    }

    let Ok(cam_transform) = kamera.get_single() else { return; };
    let origin = cam_transform.translation();
    let direction = cam_transform.forward();

    schuss_events.send(SchussAbgefeuert {
        origin,
        direction: *direction,
    });
}

fn spawne_tracer(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut schuss_events: EventReader<SchussAbgefeuert>
) {
        for event in schuss_events.read() {
            let rotation = Transform::from_translation(event.origin)
            .looking_to(event.direction, Vec3::Y).rotation;
        
        commands.spawn((
            SceneBundle {
                scene: asset_server.load("bullet.glb#Scene0"),
                transform: Transform::from_translation(event.origin)
                    .with_rotation(rotation),
                ..default()
            },
            Bullet {
                direction: event.direction,
                speed: 200.0,
                traveled: 0.0,
            },
        ));
    }
    }


fn bewege_bullets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut bullets: Query<(Entity, &mut Transform, &mut Bullet), Without<Ziel>>,
    ziele: Query<(Entity, &Transform), (With<Ziel>, Without<Bullet>)>,
    zeit: Res<Time>,
) {
    for (bullet_entity, mut bullet_transform, mut bullet) in &mut bullets {
        let schritt = bullet.direction * bullet.speed * zeit.delta_seconds();
        bullet_transform.translation += schritt;
        bullet.traveled += schritt.length();

        let mut getroffen = false;
        for (ziel_entity, ziel_transform) in &ziele {
            let distanz = bullet_transform.translation
                .distance(ziel_transform.translation);

            if distanz < 1.0 {
                // Kugel und getroffenes Ziel entfernen
                commands.entity(bullet_entity).despawn();
                commands.entity(ziel_entity).despawn();

                // Neues Ziel an zufälliger Stelle spawnen
                spawne_ziel(&mut commands, &asset_server);

                getroffen = true;
                println!("Ziel getroffen! Neues Ziel gespawnt.");
                break;
            }
        }

        if !getroffen && bullet.traveled > 100.0 {
            commands.entity(bullet_entity).despawn();
        }
    }
}