use bevy::prelude::*;

#[derive(Component)]
pub struct Ziel;

pub fn setup_ziele(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1., 1.0, 1.0)),
            material: materials.add(Color::srgb(1.0, 0.0, 0.0)),
            transform: Transform::from_xyz(0., 0.5, 0.),
            ..default()
        },
        Ziel,
    ));
}