mod welt;
mod spieler;
mod ziel;
mod schiessen;

use bevy::prelude::*;
use bevy_mod_raycast::prelude::*;

#[derive(Reflect)]
pub struct MyRaycastSet;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CursorRayPlugin)
        .add_systems(Startup, (welt::setup_welt, spieler::setup_spieler, ziel::setup_ziele))
        .add_systems(Update, (spieler::bewege_kamera, schiessen::schiessen))
        .run();
}