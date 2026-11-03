use std::f32::consts::FRAC_PI_2;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use bevy::input::mouse::MouseButton; 

use bevy::window::{CursorGrabMode, PrimaryWindow, WindowResized}; 
use bevy::render::camera::Viewport; 
use bevy::render::view::RenderLayers; 
use bevy_mod_raycast::prelude::*;
use crate::MyRaycastSet;
use crate::movement::{SourceMovement, apply_movement};
use crate::gamepad::{Eingabequelle, SpielerEingabe}; 
use crate::kampf::{Hitbox, HitboxArt, Kills, Leben, Nachricht, MAX_LEBEN, spawne_hud};


const KOERPER_DREHUNG_GRAD: f32 = 0.0;

pub struct SpielerPlugin;

impl Plugin for SpielerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_spieler);
        app.add_systems(
            Update,
            (bewege_kamera, maus_sperren, passe_viewports_an, setze_render_layer),
        ); 
    }
}

#[derive(Component)]
struct VertikaleGeschwindigkeit(f32);



/// Spieler 1 oder 2
#[derive(Component)]
pub struct Spieler {
    pub nummer: u8,
}

/// Die Kamera eines Spielers (zeigt auf den Spieler zurück)
#[derive(Component)]
pub struct SpielerKamera {
    pub besitzer: Entity,
}

/// 0 = linke Bildschirmhälfte, 1 = rechte Bildschirmhälfte
#[derive(Component)]
pub struct KameraPosition(pub u32);

/// Auf der Wurzel des Körper-Modells: auf welcher Render-Ebene soll das Modell liegen?
#[derive(Component)]
pub struct KoerperEbene(pub u8);

/// Startpunkt + Blickrichtung (pieler schauen sich gegenseitig an
pub fn spawn_transform(nummer: u8) -> Transform {
    if nummer == 1 {
        Transform::from_xyz(-4.0, 3.0, 5.0).with_rotation(Quat::from_rotation_y(-FRAC_PI_2))
    } else {
        Transform::from_xyz(4.0, 3.0, 5.0).with_rotation(Quat::from_rotation_y(FRAC_PI_2))
    }
}

fn halbes_viewport(platz: u32, groesse: UVec2) -> Viewport {
    let halbe_breite = groesse.x / 2;
    Viewport {
        physical_position: UVec2::new(platz * halbe_breite, 0),
        physical_size: UVec2::new(halbe_breite, groesse.y),
        ..default()
    }
}


pub fn setup_spieler(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    fenster: Query<&Window, With<PrimaryWindow>>,
) {
    let groesse = fenster
        .get_single()
        .map(|w| UVec2::new(w.physical_width(), w.physical_height()))
        .unwrap_or(UVec2::new(1280, 720));

    spawne_spieler(&mut commands, &asset_server, 1, Eingabequelle::TastaturMaus, groesse);
    spawne_spieler(&mut commands, &asset_server, 2, Eingabequelle::Controller, groesse);
}



fn spawne_spieler(
    commands: &mut Commands,
    asset_server: &AssetServer,
    nummer: u8,
    quelle: Eingabequelle,
    fenster_groesse: UVec2,
) {
    // Jeder Körper liegt auf seiner eigenen RenderEbene damit man den EIGENEN Körper
    // nicht von innen sieht den des Gegners aber schon
    let eigene_ebene = nummer;       // Spieler 1 -> Ebene 1, Spieler 2 -> Ebene 2
    let andere_ebene = 3 - nummer;   // die Ebene des Gegners
    let kamera_platz = (nummer - 1) as u32;

    let start = spawn_transform(nummer);

    let spieler_entity = commands
        .spawn((
            SpatialBundle::from_transform(start), 
            RigidBody::KinematicPositionBased,
            Collider::capsule_y(0.5, 0.5),
            KinematicCharacterController {
                snap_to_ground: Some(CharacterLength::Absolute(0.5)),
                filter_flags: QueryFilterFlags::EXCLUDE_SENSORS, 
                ..default()
            },
            SourceMovement::default(),
            Spieler { nummer },            
            quelle,                        
            SpielerEingabe::default(),     
            Leben(MAX_LEBEN),              
            Kills::default(),              
            Nachricht::default(),          
        ))
        .id();

    let mut kamera_entity = Entity::PLACEHOLDER;

    commands.entity(spieler_entity).with_children(|parent| {
        parent.spawn((
            SceneBundle {
                scene: asset_server.load(format!("spieler{}.glb#Scene0", nummer)),
                transform: Transform::from_xyz(0.0, -1.0, 0.0)
                    .with_rotation(Quat::from_rotation_y(KOERPER_DREHUNG_GRAD.to_radians())),
                ..default()
            },
            KoerperEbene(eigene_ebene),
        ));

        //Hitbox KOPF 
        parent.spawn((
            TransformBundle::from_transform(Transform::from_xyz(0.0, 0.72, 0.0)),
            Collider::ball(0.28),
            Sensor,
            Hitbox { art: HitboxArt::Kopf, besitzer: spieler_entity },
        ));

        //* Hitbox KÖRPER (Kapsel von den Füßen bis zur Brust)
        parent.spawn((
            TransformBundle::from_transform(Transform::from_xyz(0.0, -0.25, 0.0)),
            Collider::capsule_y(0.35, 0.4),
            Sensor,
            Hitbox { art: HitboxArt::Koerper, besitzer: spieler_entity },
        ));

        //die Kamera ist Kind des Spielers
        kamera_entity = parent
            .spawn((
                Camera3dBundle {
                    camera: Camera {
                        order: kamera_platz as isize, 
                        viewport: Some(halbes_viewport(kamera_platz, fenster_groesse)), //*
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 0.6, 0.0),
                    ..default()
                },
                RaycastSource::<MyRaycastSet>::new(),
                SpielerKamera { besitzer: spieler_entity }, //*
                KameraPosition(kamera_platz),               //*
                RenderLayers::from_layers(&[0, andere_ebene as usize]), 
            ))
            .with_children(|kamera| {
                //gun als kind der kamera spawnen
                kamera.spawn(SceneBundle {
                    scene: asset_server.load("gun.glb#Scene0"),
                    //Transform ist jetzt relativ zur Kamera, nicht mehr global
                    transform: Transform::from_xyz(0.2, -0.5, -0.2),
                    ..default()
                });
            })
            .id();
    });

    //HUD für diese Bildschirmhälfte
    spawne_hud(commands, kamera_entity, spieler_entity);
}

//split screen 
pub fn passe_viewports_an(
    mut resize_events: EventReader<WindowResized>,
    fenster: Query<&Window, With<PrimaryWindow>>,
    mut kameras: Query<(&KameraPosition, &mut Camera)>,
) {
    if resize_events.read().count() == 0 {
        return;
    }
    let Ok(window) = fenster.get_single() else { return; };
    let groesse = UVec2::new(window.physical_width(), window.physical_height());
    if groesse.x < 2 || groesse.y < 1 {
        return; // Fenster minimiert
    }
    for (platz, mut camera) in &mut kameras {
        camera.viewport = Some(halbes_viewport(platz.0, groesse));
    }
}

// Alle Nachfahren der Wurzel auf die Render-Ebene des Wurzel-Objekts setzen
fn setze_render_layer(
    mut commands: Commands,
    wurzeln: Query<(Entity, &KoerperEbene)>,
    kinder: Query<&Children>,
    layers: Query<&RenderLayers>,
) {
    for (wurzel, ebene) in &wurzeln {
        for nachfahre in kinder.iter_descendants(wurzel) {
            if layers.get(nachfahre).is_err() {
                commands.entity(nachfahre).insert(RenderLayers::layer(ebene.0 as usize));
            }
        }
    }
}

pub fn maus_sperren(
    mut fenster: Query<&mut Window, With<PrimaryWindow>>,
    maus: Res<ButtonInput<MouseButton>>,
) {
    let Ok(mut window) = fenster.get_single_mut() else { return; };

    if maus.just_pressed(MouseButton::Left) {
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;
    }
    if maus.just_pressed(MouseButton::Right) {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
    }
}

// Bewege die Kamera eines Spielers entsprechend der Eingaben des Spielers
pub fn bewege_kamera(
    mut kamera_query: Query<(&SpielerKamera, &mut Transform), Without<Spieler>>,
    mut spieler_query: Query<
        (
            &mut Transform,
            &mut KinematicCharacterController,
            &mut SourceMovement,
            &SpielerEingabe,
            Option<&KinematicCharacterControllerOutput>,
        ),
        With<Spieler>,
    >,
    zeit: Res<Time>,
    // Hitbox-Collider ausschließen sonst wäre "karte sofort nicht mehr leer
    karte: Query<(), (With<Collider>, Without<KinematicCharacterController>, Without<Hitbox>)>,
) {
    for (kamera, mut kamera_transform) in &mut kamera_query {
        let Ok((mut spieler_transform, mut controller, mut movement, eingabe, output)) =
            spieler_query.get_mut(kamera.besitzer)
        else {
            continue;
        };

        // blickrichtung der kamera auf den spieler übertragen
        spieler_transform.rotate_y(eingabe.blick.x);

        let aktuelle_pitch = kamera_transform.rotation.to_euler(EulerRot::YXZ).1;
        let neuer_pitch = (aktuelle_pitch + eingabe.blick.y).clamp(-1.55, 1.55);
        kamera_transform.rotation = Quat::from_rotation_x(neuer_pitch);

        //Bewegung
        let mut wish_dir = Vec3::ZERO;
        wish_dir += spieler_transform.forward().as_vec3() * eingabe.bewegung.y;
        wish_dir += spieler_transform.right().as_vec3() * eingabe.bewegung.x;
        wish_dir.y = 0.0;
        let wish_dir = wish_dir.normalize_or_zero();

        if karte.is_empty() { continue; }

        let auf_dem_boden = output.map(|o| o.grounded).unwrap_or(false);

        apply_movement(
            wish_dir,
            eingabe.springen,
            zeit.delta_seconds(),
            &mut controller,
            &mut movement,
            auf_dem_boden,
        );
    }
}