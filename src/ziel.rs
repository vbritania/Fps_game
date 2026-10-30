use bevy::prelude::*;
use rand::Rng;

#[derive(Component)]
pub struct Ziel;

pub struct ZielPlugin;

impl Plugin for ZielPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawne_ziel_start);
    }
}

fn spawne_ziel_start(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    spawne_ziel(&mut commands, &mut meshes, &mut materials);
}
pub fn spawne_ziel (
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
){
    let mut rng = rand::thread_rng();

    let x = rng.gen_range(-4.0..4.0);
    let y = rng.gen_range(0.5..3.0);
    let z = rng.gen_range(-8.0..-3.0);

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(Color::srgb(1.0, 0.0, 0.0)),
            transform: Transform::from_xyz(x, y, z),
            ..default()
        },
        Ziel,
    ));
}