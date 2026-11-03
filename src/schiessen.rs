use bevy::prelude::*;
use bevy::{ecs::bundle, prelude::*};
use bevy_rapier3d::prelude::*; 

use bevy::input::mouse::MouseButton;
use crate::ziel::{Ziel, spawne_ziel};
use crate::gamepad::SpielerEingabe;                      
use crate::kampf::{Hitbox, Treffer};                      
use crate::spieler::{Spieler, SpielerKamera};             

const MAX_REICHWEITE: f32 = 500.0; 

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
    weg: f32, 
}

#[derive(Component)]
struct Bullet {
    direction: Vec3,
    speed: f32,
    traveled: f32,
    max_weg: f32, //*
}


fn schiessen(
    spieler_query: Query<&SpielerEingabe, With<Spieler>>,
    kameras: Query<(&GlobalTransform, &SpielerKamera)>,
    hitboxen: Query<&Hitbox>,
    spieler_marker: Query<(), With<Spieler>>,
    rapier: Res<RapierContext>,
    mut schuss_events: EventWriter<SchussAbgefeuert>,
    mut treffer_events: EventWriter<Treffer>,
) {
    for (cam_transform, kamera) in &kameras {
        let schuetze = kamera.besitzer;
        let Ok(eingabe) = spieler_query.get(schuetze) else { continue; };
        if !eingabe.schiessen {
            continue;
        }

        let muendung_lokal = Vec3::new(1.2, -1., -8.5); // x =1 y = -1.2 z = -4.5
        let rotation = cam_transform.to_scale_rotation_translation().1;
        let kamera_pos = cam_transform.translation();
        let muendung = kamera_pos + rotation * muendung_lokal;
        let richtung: Vec3 = *cam_transform.forward();

        
        let ist_treffbar = |entity: Entity| -> bool {
            if let Ok(hitbox) = hitboxen.get(entity) {
                hitbox.besitzer != schuetze
            } else {
                spieler_marker.get(entity).is_err()
            }
        };
        let filter = QueryFilter::default().predicate(&ist_treffbar);

        let mut reichweite = MAX_REICHWEITE;
        if let Some((entity, abstand)) =
            rapier.cast_ray(kamera_pos, richtung, MAX_REICHWEITE, true, filter)
        {
            reichweite = abstand;
            // Hitbox getroffen (und keine Wand)? -> Treffer melden
            if let Ok(hitbox) = hitboxen.get(entity) {
                treffer_events.send(Treffer {
                    opfer: hitbox.besitzer,
                    schuetze,
                    art: hitbox.art, // Kopf oder Körper
                });
            }
        }

        // Sichtbarer Tracer: von der Mündung zum Treffpunkt
        let ziel_punkt = kamera_pos + richtung * reichweite;
        let zum_ziel = ziel_punkt - muendung;
        let (tracer_richtung, tracer_weg) = if zum_ziel.dot(richtung) > 0.1 {
            (zum_ziel.normalize(), zum_ziel.length())
        } else {
            (richtung, 0.0) 
        };

        schuss_events.send(SchussAbgefeuert {
            origin: muendung,
            direction: tracer_richtung,
            weg: tracer_weg,
        });
    }
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
                max_weg: event.weg, 
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


            if distanz < 1. && bullet.traveled > 0.5 {
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

        
        if !getroffen && bullet.traveled > bullet.max_weg { 
            commands.entity(bullet_entity).despawn();
        }
    }
}