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
     asset_server: Res<AssetServer>,
) {
    spawne_ziel(&mut commands, &asset_server);
}
pub fn spawne_ziel (
    commands: &mut Commands,
    asset_server: &AssetServer,
){
    let mut rng = rand::thread_rng();

    let x = rng.gen_range(-4.0..4.0);
    let y = rng.gen_range(0.5..8.0);
    let z = rng.gen_range(-8.5..-2.0);

    commands.spawn((
        SceneBundle {
            scene: asset_server.load("ziel.glb#Scene0"),
            transform: Transform::from_xyz(x, y, z),
            ..default()
        },
        Ziel,
    ));
}