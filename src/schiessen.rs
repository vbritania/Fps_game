use bevy::prelude::*;
use bevy::input::mouse::MouseButton;
use bevy_mod_raycast::prelude::*;
use crate::ziel::Ziel;

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
    mut commands: Commands,
    maus: Res<ButtonInput<MouseButton>>,
    mut raycast: Raycast,
    kamera: Query<&GlobalTransform, With<Camera3d>>,
    ziele: Query<Entity, With<Ziel>>,
    mut schuss_events: EventWriter<SchussAbgefeuert>,
) {
    if !maus.just_pressed(MouseButton::Left) {
        return;
    }

    let Ok(cam_transform) = kamera.get_single() else { return; };
    let origin = cam_transform.translation();
    let direction = cam_transform.forward();
    let ray = Ray3d::new(origin, *direction);

    let filter = |entity| ziele.contains(entity);
    let settings = RaycastSettings::default().with_filter(&filter);
    let hits = raycast.cast_ray(ray, &settings);

    schuss_events.send(SchussAbgefeuert {
        origin,
        direction: *direction,
    });

    for (entity, hit) in hits {
        println!("Getroffen! Entity: {:?}, Distanz: {}", entity, hit.distance());
        commands.entity(*entity).despawn();
    }
}

fn spawne_tracer(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut schuss_events: EventReader<SchussAbgefeuert>,
) {
    for event in schuss_events.read() {
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Cuboid::new(0.05, 0.05, 0.05)),
                material: materials.add(Color::srgb(1.0, 0.8, 0.0)),
                transform: Transform::from_translation(event.origin),
                ..default()
            },
            Bullet {
                direction: event.direction,
                speed: 50.0,
                traveled: 0.0,
            },
        ));
    }
}

fn bewege_bullets(
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Transform, &mut Bullet)>,
    zeit: Res<Time>,
) {
    for (entity, mut transform, mut bullet) in &mut bullets {
        let schritt = bullet.direction * bullet.speed * zeit.delta_seconds();
        transform.translation += schritt;
        bullet.traveled += schritt.length();

        if bullet.traveled > 100.0 {
            commands.entity(entity).despawn();
        }
    }
}