mod welt;
mod spieler;
mod ziel;
mod schiessen;
mod movement;
mod gamepad; 
mod kampf;   

use bevy::prelude::*;
use bevy_rapier3d::prelude::*; 

#[derive(Reflect)]
pub struct MyRaycastSet;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default()) 
        .add_plugins(welt::WeltPlugin)
        .add_plugins(gamepad::EingabePlugin) 
        .add_plugins(spieler::SpielerPlugin)
        .add_plugins(schiessen::SchiessenPlugin)
        .add_plugins(kampf::KampfPlugin) 
        .run();
}


//reload 
//hand auf waffe 
//inspect gun
//menu
//aimlab auch noch



//waffe soll auch auf der kamera sichtbar sein
//kugel kleiner machen
//map unbedingt andern und credits vepasen
