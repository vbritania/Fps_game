// use bevy::prelude::*;

// pub struct WeltPlugin;

// impl Plugin for WeltPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_systems(Startup, setup_welt);
//     }
// }

// pub fn setup_welt(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     // licht
//     commands.spawn(DirectionalLightBundle {
//         directional_light: DirectionalLight {   // <-- FIX: DirectionalLight
//             illuminance: 12000.0,
//             ..default()
//         },
//         transform: Transform::from_rotation(Quat::from_euler(
//             EulerRot::XYZ,
//             -std::f32::consts::FRAC_PI_4,
//             -std::f32::consts::FRAC_PI_4,
//             0.0,
//         )),
//         ..default()
//     });

    
//     commands.insert_resource(AmbientLight {   // <-- FIX: insert_resource statt spawn
//         color: Color::srgb(0.6, 0.65, 0.75),
//         brightness: 200.0,
//     });

//     // materialen
//     let mat_boden = materials.add(StandardMaterial {
//         base_color: Color::srgb(0.22, 0.24, 0.28),
//         perceptual_roughness: 0.95,
//         ..default()
//     });
//     // waende
//     let mat_wand = materials.add(StandardMaterial {
//         base_color: Color::srgb(0.75, 0.68, 0.55),
//         perceptual_roughness: 0.85,
//         ..default()
//     });
//     //rot
//     let mat_container = materials.add(StandardMaterial {
//         base_color: Color::srgb(0.65, 0.20, 0.15),
//         perceptual_roughness: 0.7,
//         ..default()
//     });
//     // Blau
//     let mat_blau = materials.add(StandardMaterial {
//         base_color: Color::srgb(0.15, 0.35, 0.65),
//         perceptual_roughness: 0.7,
//         ..default()
//     });
//     //  Grün
//     let mat_gruen = materials.add(StandardMaterial {
//         base_color: Color::srgb(0.20, 0.50, 0.25),
//         perceptual_roughness: 0.7,
//         ..default()
//     });
//     // Holz-Braun
//     let mat_holz = materials.add(StandardMaterial {
//         base_color: Color::srgb(0.55, 0.38, 0.22),
//         perceptual_roughness: 0.9,
//         ..default()
//     });
//     // Glas
//     let mat_glas = materials.add(StandardMaterial {
//         base_color: Color::srgba(0.7, 0.85, 0.95, 0.25),
//         alpha_mode: AlphaMode::Blend,
//         perceptual_roughness: 0.05,
//         metallic: 0.1,
//         ..default()
//     });
//     // hier dummy spawnen
//     let mat_dummy = materials.add(StandardMaterial {
//         base_color: Color::srgb(0.9, 0.1, 0.1),
//         emissive: LinearRgba::rgb(2.0, 0.0, 0.0),
//         ..default()
//     });
//     // deko neon-cyan
//     let mat_neon = materials.add(StandardMaterial {
//         base_color: Color::srgb(0.0, 0.9, 0.9),
//         emissive: LinearRgba::rgb(0.0, 3.0, 3.0),
//         ..default()
//     });

//     // BODEN 
//     commands.spawn(PbrBundle {
//         mesh: meshes.add(Plane3d::default().mesh().size(100.0, 100.0)),
//         material: mat_boden.clone(),
//         ..default()
//     });

    
//     // Nordwand
//     commands.spawn(PbrBundle {
//         mesh: meshes.add(Cuboid::new(100.0, 8.0, 1.0)),
//         material: mat_wand.clone(),
//         transform: Transform::from_xyz(0.0, 4.0, -50.0),
//         ..default()
//     });
//     // Sueddwand
//     commands.spawn(PbrBundle {
//         mesh: meshes.add(Cuboid::new(100.0, 8.0, 1.0)),
//         material: mat_wand.clone(),
//         transform: Transform::from_xyz(0.0, 4.0, 50.0),
//         ..default()
//     });
//     // Westwand
//     commands.spawn(PbrBundle {
//         mesh: meshes.add(Cuboid::new(1.0, 8.0, 100.0)),
//         material: mat_wand.clone(),
//         transform: Transform::from_xyz(-50.0, 4.0, 0.0),
//         ..default()
//     });
//     // Ostwand
//     commands.spawn(PbrBundle {
//         mesh: meshes.add(Cuboid::new(1.0, 8.0, 100.0)),
//         material: mat_wand.clone(),
//         transform: Transform::from_xyz(50.0, 4.0, 0.0),
//         ..default()
//     });

//     //Mittewand /fenster
//     commands.spawn(PbrBundle {
//         mesh: meshes.add(Cuboid::new(60.0, 3.0, 1.0)),
//         material: mat_wand.clone(),
//         transform: Transform::from_xyz(0.0, 1.5, 0.0),
//         ..default()
//     });
//     // ueber fenster
//     commands.spawn(PbrBundle {
//         mesh: meshes.add(Cuboid::new(60.0, 2.0, 1.0)),
//         material: mat_wand.clone(),
//         transform: Transform::from_xyz(0.0, 7.0, 0.0),
//         ..default()
//     });
//     // Fenster-Glas
//     commands.spawn(PbrBundle {
//         mesh: meshes.add(Cuboid::new(60.0, 3.0, 0.05)),
//         material: mat_glas.clone(),
//         transform: Transform::from_xyz(0.0, 4.5, 0.0),
//         ..default()
//     });

   
//     // Rot
//     commands.spawn(PbrBundle {
//         mesh: meshes.add(Cuboid::new(8.0, 4.0, 3.0)),
//         material: mat_container.clone(),
//         transform: Transform::from_xyz(-30.0, 2.0, -20.0),
//         ..default()
//     });
//     // Blau gestappelt
//     commands.spawn(PbrBundle {
//         mesh: meshes.add(Cuboid::new(8.0, 4.0, 3.0)),
//         material: mat_blau.clone(),
//         transform: Transform::from_xyz(-30.0, 6.0, -20.0),
//         ..default()
//     });
//     //gruun rehct oben
//     commands.spawn(PbrBundle {
//         mesh: meshes.add(Cuboid::new(8.0, 4.0, 3.0)),
//         material: mat_gruen.clone(),
//         transform: Transform::from_xyz(30.0, 2.0, -20.0),
//         ..default()
//     });
//     //Rot rechts unten
//     commands.spawn(PbrBundle {
//         mesh: meshes.add(Cuboid::new(8.0, 4.0, 3.0)),
//         material: mat_container.clone(),
//         transform: Transform::from_xyz(30.0, 2.0, 20.0),
//         ..default()
//     });
//     //Blau, links unten
//     commands.spawn(PbrBundle {
//         mesh: meshes.add(Cuboid::new(8.0, 4.0, 3.0)),
//         material: mat_blau.clone(),
//         transform: Transform::from_xyz(-30.0, 2.0, 20.0),
//         ..default()
//     });
//     //Grün, Mitte rechts 
//     commands.spawn(PbrBundle {
//         mesh: meshes.add(Cuboid::new(6.0, 3.0, 3.0)),
//         material: mat_gruen.clone(),
//         transform: Transform::from_xyz(20.0, 1.5, 15.0),
//         ..default()
//     });

//     //Kisten holz
//     commands.spawn(PbrBundle {
//         mesh: meshes.add(Cuboid::new(2.0, 2.0, 2.0)),
//         material: mat_holz.clone(),
//         transform: Transform::from_xyz(-15.0, 1.0, -10.0),
//         ..default()
//     });
//     commands.spawn(PbrBundle {
//         mesh: meshes.add(Cuboid::new(2.0, 2.0, 2.0)),
//         material: mat_holz.clone(),
//         transform: Transform::from_xyz(-13.0, 1.0, -10.0),
//         ..default()
//     });
//     commands.spawn(PbrBundle {
//         mesh: meshes.add(Cuboid::new(2.0, 2.0, 2.0)),
//         material: mat_holz.clone(),
//         transform: Transform::from_xyz(-14.0, 3.0, -10.0),
//         ..default()
//     });
//     commands.spawn(PbrBundle {
//         mesh: meshes.add(Cuboid::new(2.0, 2.0, 2.0)),
//         material: mat_holz.clone(),
//         transform: Transform::from_xyz(15.0, 1.0, -12.0),
//         ..default()
//     });
//     commands.spawn(PbrBundle {
//         mesh: meshes.add(Cuboid::new(2.0, 2.0, 2.0)),
//         material: mat_holz.clone(),
//         transform: Transform::from_xyz(12.0, 1.0, 10.0),
//         ..default()
//     });

//     ///kurze wand links
//     commands.spawn(PbrBundle {
//         mesh: meshes.add(Cuboid::new(1.0, 4.0, 12.0)),
//         material: mat_wand.clone(),
//         transform: Transform::from_xyz(-20.0, 2.0, -35.0),
//         ..default()
//     });
//     // Kurze Wand rechts
//     commands.spawn(PbrBundle {
//         mesh: meshes.add(Cuboid::new(1.0, 4.0, 12.0)),
//         material: mat_wand.clone(),
//         transform: Transform::from_xyz(20.0, 2.0, -35.0),
//         ..default()
//     });
//     // Wand mit Fenster (Süd-Bereich)
//     commands.spawn(PbrBundle {
//         mesh: meshes.add(Cuboid::new(15.0, 3.0, 1.0)),
//         material: mat_wand.clone(),
//         transform: Transform::from_xyz(-25.0, 1.5, 35.0),
//         ..default()
//     });
//     commands.spawn(PbrBundle {
//         mesh: meshes.add(Cuboid::new(15.0, 1.5, 1.0)),
//         material: mat_wand.clone(),
//         transform: Transform::from_xyz(-25.0, 3.75, 35.0),
//         ..default()
//     });
//     // Glas dazwischen
//     commands.spawn(PbrBundle {
//         mesh: meshes.add(Cuboid::new(15.0, 1.0, 0.05)),
//         material: mat_glas.clone(),
//         transform: Transform::from_xyz(-25.0, 3.0, 35.0),
//         ..default()
//     });

//     //Dach
//     commands.spawn(PbrBundle {
//         mesh: meshes.add(Cuboid::new(10.0, 0.5, 10.0)),
//         material: mat_wand.clone(),
//         transform: Transform::from_xyz(-20.0, 5.0, -40.0),
//         ..default()
//     });
//     //Stuetzeze dafür
//     commands.spawn(PbrBundle {
//         mesh: meshes.add(Cuboid::new(1.0, 5.0, 1.0)),
//         material: mat_wand.clone(),
//         transform: Transform::from_xyz(-24.0, 2.5, -44.0),
//         ..default()
//     });
//     commands.spawn(PbrBundle {
//         mesh: meshes.add(Cuboid::new(1.0, 5.0, 1.0)),
//         material: mat_wand.clone(),
//         transform: Transform::from_xyz(-16.0, 2.5, -44.0),
//         ..default()
//     });

//     //neon steifen nord
//     commands.spawn(PbrBundle {
//         mesh: meshes.add(Cuboid::new(40.0, 0.3, 0.2)),
//         material: mat_neon.clone(),
//         transform: Transform::from_xyz(0.0, 6.0, -49.4),
//         ..default()
//     });
    

    
//     // Hier sollen die Dummys spawnen 
// //     commands.spawn(PbrBundle {
// //         mesh: meshes.add(Cuboid::new(2.0, 2.0, 2.0)),
// //         material: mat_dummy,
// //         transform: Transform::from_xyz(0.0, 1.0, -40.0),
// //         ..default()
// //     });
// }

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct WeltPlugin;

impl Plugin for WeltPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_welt);
    }
}

pub fn setup_welt(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Licht
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 7000.0,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            -std::f32::consts::FRAC_PI_4,
            -std::f32::consts::FRAC_PI_4,
            0.0,
        )),
        ..default()
    });

    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.6, 0.65, 0.75),
        brightness: 150.0,
    });

    //Map
    commands.spawn((                                              
        SceneBundle {
            scene: asset_server.load("map.glb#Scene0"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),//.with_scale(Vec3::splat(4.7)), //with_scale(Vec3::splat(4.7) bei dust2
            ..default()
        },
        AsyncSceneCollider::default(), //Collider fur alle maps automatisch
    ));

    // unsichtbarer Boden: ENTFERNT
}