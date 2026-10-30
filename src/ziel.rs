use bevy::prelude::*;

#[derive(Component)]
pub struct Ziel;

pub struct ZielPlugin;

impl Plugin for ZielPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ziele);
    }
}

pub fn setup_ziele(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for i in 0..4 {
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
                material: materials.add(Color::srgb(1.0, 0.0, 0.0)),
                transform: Transform::from_xyz(i as f32 * 1.5, 0.5, -10.0),
                ..default()
            },
            Ziel,
        ));
    }
}