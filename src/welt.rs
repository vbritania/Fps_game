use bevy::prelude::*;

pub struct WeltPlugin;

impl Plugin for WeltPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_welt);
    }
}

pub fn setup_welt(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.0,
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

    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(20.0, 20.0)),
        material: materials.add(Color::srgb(0.3, 0.3, 0.3)),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(10.0, 10.0, 0.01)),
        material: materials.add(Color::srgb(0.3, 0.3, 0.3)),
        transform: Transform::from_xyz(0.0, 0.0, -10.0),
        ..default()
    });
}