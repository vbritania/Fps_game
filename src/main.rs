mod welt;
mod spieler;
mod ziel;
mod schiessen;

use welt::WeltPlugin;
use spieler::SpielerPlugin;
use schiessen::SchiessenPlugin;
use ziel::ZielPlugin;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[derive(Reflect)]
pub struct MyRaycastSet;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WorldInspectorPlugin::new(),
            WeltPlugin,
            SpielerPlugin,
            ZielPlugin,
            SchiessenPlugin,

            ))
    
        .run();
}