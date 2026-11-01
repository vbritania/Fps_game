use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy::input::keyboard::KeyCode;
use bevy::input::mouse::{MouseButton, MouseMotion};
use bevy::window::{CursorGrabMode, PrimaryWindow};
use bevy_mod_raycast::prelude::*;
use crate::MyRaycastSet;

pub struct SpielerPlugin;

impl Plugin for SpielerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_spieler);
        app.add_systems(Update, (bewege_kamera, maus_sperren));
    }
}


#[derive(Component)]
struct VertikaleGeschwindigkeit(f32);

pub fn setup_spieler(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let spieler_entity = commands.spawn((
        TransformBundle::from_transform(Transform::from_xyz(0.0, 3.0, 5.0)),
        RigidBody::KinematicPositionBased,
        Collider::capsule_y(0.5, 0.5),
        KinematicCharacterController {                                       
            snap_to_ground: Some(CharacterLength::Absolute(0.5)),            
            ..default()                                                      
        },                                                                   
        VertikaleGeschwindigkeit(0.0),
    )).id();

    //die Kamera ist Kind des Spielers
    commands.entity(spieler_entity).with_children(|parent| {
        parent.spawn((
            Camera3dBundle {
                transform: Transform::from_xyz(0.0, 0.6, 0.0),               
                ..default()
            },
            RaycastSource::<MyRaycastSet>::new(),
        ))
        .with_children(|kamera| {
            //gun als kind der kamera spawnen
            kamera.spawn(SceneBundle {
                scene: asset_server.load("gun.glb#Scene0"),
                //Transform ist jetzt relativ zur Kamera, nicht mehr global
                transform: Transform::from_xyz(1., -1.2, -4.5),
                ..default()
            });
        });
    });
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

pub fn bewege_kamera(
    mut query: Query<&mut Transform, With<Camera3d>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut maus_bewegung: EventReader<MouseMotion>,
    zeit: Res<Time>,
    //Query fuer den Character Controller und die vertikale Geschwindigkeit
    mut spieler_query: Query<(&mut KinematicCharacterController, &mut VertikaleGeschwindigkeit)>,
    //Query fuer den Ground-Check
    output_query: Query<&KinematicCharacterControllerOutput>,
    karte: Query<(), (With<Collider>, Without<KinematicCharacterController>)>,
) {
    let Ok(mut transform) = query.get_single_mut() else { return; };
    let geschwindigkeit = 5.0;
    let sens = 0.002;

    let mut yaw = 0.0;
    let mut pitch = 0.0;

    for b in maus_bewegung.read() {
        yaw -= b.delta.x * sens;
        pitch -= b.delta.y * sens;
    }

    if yaw != 0.0 || pitch != 0.0 {
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

    //statt transform.translation direkt zu aendern geben wir die Bewegung an den Controller
    let Ok((mut controller, mut vert_geschw)) = spieler_query.get_single_mut() else { return; };

    //warten bis die Map ihre Collider hat sonst faellt der Spieler durch die noch ladende Map
    if karte.is_empty() {                                                    
        return;                                                              
    }                                                                        

    //pruefen ob der Spieler auf dem Boden steht
    let auf_dem_boden = output_query
        .get_single()
        .map(|output| output.grounded)
        .unwrap_or(false);

    //Gravitation anwenden
    if auf_dem_boden && vert_geschw.0 <= 0.0 {
        vert_geschw.0 = 0.0;
    } else {
        vert_geschw.0 -= 9.81 * zeit.delta_seconds();
    }

    //Springen mit Space
    if auf_dem_boden && keyboard.just_pressed(KeyCode::Space) {
        vert_geschw.0 = 6.0;                                                 
    }

    //horizontale Bewegung berechnen
    let horizontale_bewegung = richtung.normalize_or_zero() * geschwindigkeit * zeit.delta_seconds();

    //vertikale Bewegung berechnen
    let vertikale_bewegung = Vec3::new(0.0, vert_geschw.0 * zeit.delta_seconds(), 0.0);

    //gesamte Bewegung an den Controller geben (statt transform.translation)
    controller.translation = Some(horizontale_bewegung + vertikale_bewegung);
}