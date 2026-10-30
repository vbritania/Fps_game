mod welt;
mod spieler;
mod ziel;
mod schiessen;

use bevy::prelude::*;
use bevy_mod_raycast::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;


#[derive(Reflect)]
pub struct MyRaycastSet;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            CursorRayPlugin,
            WorldInspectorPlugin::new(),
        ))
        .add_systems(Startup, (
            welt::setup_welt,
            spieler::setup_spieler,
            ziel::setup_ziele
            ))
        .add_systems(Update, (
            spieler::bewege_kamera,
            schiessen::schiessen
            
            ))
        .run();
}