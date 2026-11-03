// gamepad.rs  --  NEUE DATEI
//
// Idee: Jeder Spieler hat eine Komponente `SpielerEingabe`.
// Ein einziges System (`lese_eingaben`) füllt sie jeden Frame:
//   - Spieler mit `Eingabequelle::TastaturMaus` bekommt Tastatur + Maus
//   - Spieler mit `Eingabequelle::Controller`   bekommt den ersten Gamepad
// Danach arbeiten bewege_kamera() und schiessen() nur noch mit `SpielerEingabe`.
// Dadurch ist apply_movement() für beide Spieler exakt dasselbe.

use bevy::prelude::*;
use bevy::input::{Axis, InputSystem};
use bevy::input::gamepad::{
    Gamepad, GamepadAxis, GamepadAxisType, GamepadButton, GamepadButtonType,
    GamepadConnection, GamepadConnectionEvent, Gamepads,
};
use bevy::input::mouse::{MouseButton, MouseMotion};

const MAUS_EMPFINDLICHKEIT: f32 = 0.002;
const STICK_TOTZONE: f32 = 0.15;     // kleine Stick-Bewegungen ignorieren (Drift)
const STICK_DREHTEMPO: f32 = 3.5;    // Radiant pro Sekunde bei vollem Ausschlag rechter Stick

/// Woher bekommt dieser Spieler seine Eingabe?
#[derive(Component, Clone, Copy, PartialEq)]
pub enum Eingabequelle {
    TastaturMaus,
    Controller,
}

/// Gemeinsame Eingabe für beide Spieler
#[derive(Component, Default, Clone, Copy)]
pub struct SpielerEingabe {
    pub bewegung: Vec2, // x = rechts/links, y = vorwärts/rückwärts (jeweils -1 bis 1)
    pub blick: Vec2,    // x = Drehung links/rechts (Radiant), y = hoch/runter (Radiant)
    pub springen: bool, // gedrückt gehalten -> Bunnyhop
    pub schiessen: bool,
}

pub struct EingabePlugin;

impl Plugin for EingabePlugin {
    fn build(&self, app: &mut App) {
        // PreUpdate nach InputSystem: dann sind Maus/Tasten/Gamepad schon aktuell
        app.add_systems(PreUpdate, lese_eingaben.after(InputSystem));
        app.add_systems(Update, gamepad_meldungen);
    }
}

fn stick(achsen: &Axis<GamepadAxis>, pad: Gamepad, achse: GamepadAxisType) -> f32 {
    let wert = achsen.get(GamepadAxis::new(pad, achse)).unwrap_or(0.0);
    if wert.abs() < STICK_TOTZONE { 0.0 } else { wert }
}

pub fn lese_eingaben(
    tasten: Res<ButtonInput<KeyCode>>,
    maus: Res<ButtonInput<MouseButton>>,
    mut maus_bewegung: EventReader<MouseMotion>,
    gamepads: Res<Gamepads>,
    achsen: Res<Axis<GamepadAxis>>,
    knoepfe: Res<ButtonInput<GamepadButton>>,
    zeit: Res<Time>,
    mut spieler: Query<(&Eingabequelle, &mut SpielerEingabe)>,
) {
    // Mausbewegung nur EINMAL pro Frame einsammeln
    let mut maus_delta = Vec2::ZERO;
    for b in maus_bewegung.read() {
        maus_delta += b.delta;
    }

    // Der erste verbundene Controller gehört Spieler 2
    let pad = gamepads.iter().next();
    let dt = zeit.delta_seconds();

    for (quelle, mut eingabe) in &mut spieler {
        match quelle {
            Eingabequelle::TastaturMaus => {
                let mut bewegung = Vec2::ZERO;
                if tasten.pressed(KeyCode::KeyW) { bewegung.y += 1.0; }
                if tasten.pressed(KeyCode::KeyS) { bewegung.y -= 1.0; }
                if tasten.pressed(KeyCode::KeyD) { bewegung.x += 1.0; }
                if tasten.pressed(KeyCode::KeyA) { bewegung.x -= 1.0; }

                eingabe.bewegung = bewegung;
                eingabe.blick = Vec2::new(
                    -maus_delta.x * MAUS_EMPFINDLICHKEIT,
                    -maus_delta.y * MAUS_EMPFINDLICHKEIT,
                );
                eingabe.springen = tasten.pressed(KeyCode::Space);
                eingabe.schiessen = maus.just_pressed(MouseButton::Left);
            }
            Eingabequelle::Controller => {
                let Some(pad) = pad else {
                    // kein Controller da -> Spieler 2 steht still
                    *eingabe = SpielerEingabe::default();
                    continue;
                };

                let links_x = stick(&achsen, pad, GamepadAxisType::LeftStickX);
                let links_y = stick(&achsen, pad, GamepadAxisType::LeftStickY);
                let rechts_x = stick(&achsen, pad, GamepadAxisType::RightStickX);
                let rechts_y = stick(&achsen, pad, GamepadAxisType::RightStickY);

                eingabe.bewegung = Vec2::new(links_x, links_y);
                // rechter Stick rechts = nach rechts drehen (negativer Yaw, wie bei der Maus)
                eingabe.blick = Vec2::new(
                    -rechts_x * STICK_DREHTEMPO * dt,
                    rechts_y * STICK_DREHTEMPO * dt,
                );
                // Kreuz (PS) = South = springen (gedrückt halten = Bunnyhop)
                eingabe.springen =
                    knoepfe.pressed(GamepadButton::new(pad, GamepadButtonType::South));
                // R2 oder R1 = schiessen
                eingabe.schiessen =
                    knoepfe.just_pressed(GamepadButton::new(pad, GamepadButtonType::RightTrigger2))
                        || knoepfe.just_pressed(GamepadButton::new(pad, GamepadButtonType::RightTrigger));
            }
        }
    }
}

// Nur eine Info in der Konsole, damit du siehst ob der Controller erkannt wurde
fn gamepad_meldungen(mut events: EventReader<GamepadConnectionEvent>) {
    for e in events.read() {
        match &e.connection {
            GamepadConnection::Connected(info) => println!("Gamepad verbunden: {}", info.name),
            GamepadConnection::Disconnected => println!("Gamepad getrennt"),
        }
    }
}