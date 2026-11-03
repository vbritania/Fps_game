use bevy::prelude::*;
use bevy::ui::TargetCamera;
use crate::movement::SourceMovement;
use crate::spieler::{spawn_transform, Spieler};

pub const MAX_LEBEN: i32 = 100;
pub const KOERPER_SCHADEN: i32 = 25; 
pub const KOPF_SCHADEN: i32 = 100;   

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum HitboxArt {
    Kopf,
    Koerper,
}

/// Wird auf die unsichtbaren Kopf/Körper-Collider (Kinder des Spielers) gesetzt
#[derive(Component)]
pub struct Hitbox {
    pub art: HitboxArt,
    pub besitzer: Entity, // der Spieler zu dem diese Hitbox gehört
}

#[derive(Component)]
pub struct Leben(pub i32);

#[derive(Component, Default)]
pub struct Kills(pub u32);

/// Kurze Meldung im HUD 
#[derive(Component, Default)]
pub struct Nachricht {
    pub text: String,
    pub zeit: f32,
}

/// Wird von schiessen.rs gesendet wenn ein Schuss einen Spieler trifft
#[derive(Event)]
pub struct Treffer {
    pub opfer: Entity,
    pub schuetze: Entity,
    pub art: HitboxArt,
}

#[derive(Component)]
pub struct HudText {
    pub spieler: Entity,
}

pub struct KampfPlugin;

impl Plugin for KampfPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Treffer>();
        app.add_systems(
            Update,
            (verarbeite_treffer, nachricht_timer, aktualisiere_hud).chain(),
        );
    }
}

pub fn verarbeite_treffer(
    mut treffer_events: EventReader<Treffer>,
    mut opfer_query: Query<(&Spieler, &mut Leben, &mut Transform, &mut SourceMovement)>,
    mut schuetzen_query: Query<(&mut Kills, &mut Nachricht)>,
) {
    for treffer in treffer_events.read() {
        let (schaden, text) = match treffer.art {
            HitboxArt::Kopf => (KOPF_SCHADEN, "KOPFSCHUSS!"),
            HitboxArt::Koerper => (KOERPER_SCHADEN, "Treffer"),
        };

        //Opfer bekommt Schaden
        let mut getoetet = false;
        if let Ok((spieler, mut leben, mut transform, mut bewegung)) =
            opfer_query.get_mut(treffer.opfer)
        {
            leben.0 -= schaden;
            if leben.0 <= 0 {
                getoetet = true;
                leben.0 = MAX_LEBEN;
                *transform = spawn_transform(spieler.nummer); // Respawn
                bewegung.velocity = Vec3::ZERO;
            }
        }

        //Schütze bekommt Meldung
        if let Ok((mut kills, mut nachricht)) = schuetzen_query.get_mut(treffer.schuetze) {
            if getoetet {
                kills.0 += 1;
                nachricht.text = format!("{} KILL!", text);
            } else {
                nachricht.text = text.to_string();
            }
            nachricht.zeit = 1.5;
        }
    }
}

fn nachricht_timer(zeit: Res<Time>, mut nachrichten: Query<&mut Nachricht>) {
    for mut n in &mut nachrichten {
        if n.zeit > 0.0 {
            n.zeit -= zeit.delta_seconds();
        }
    }
}

fn aktualisiere_hud(
    spieler: Query<(&Leben, &Kills, &Nachricht)>,
    mut texte: Query<(&HudText, &mut Text)>,
) {
    for (hud, mut text) in &mut texte {
        if let Ok((leben, kills, nachricht)) = spieler.get(hud.spieler) {
            let meldung = if nachricht.zeit > 0.0 { nachricht.text.as_str() } else { "" };
            text.sections[0].value =
                format!("Leben: {}   Kills: {}   {}", leben.0.max(0), kills.0, meldung);
        }
    }
}

/// Fadenkreuz + Text fur eine Kamera
pub fn spawne_hud(commands: &mut Commands, kamera: Entity, spieler: Entity) {
    // Fadenkreuz in der Mitte der Bildschirmhalfte
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            TargetCamera(kamera), 
        ))
        .with_children(|root| {
            root.spawn(NodeBundle {
                style: Style {
                    width: Val::Px(6.0),
                    height: Val::Px(6.0),
                    ..default()
                },
                background_color: BackgroundColor(Color::WHITE),
                ..default()
            });
        });

    // Leben / Kills unten links
    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font_size: 28.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
        TargetCamera(kamera),
        HudText { spieler },
    ));
}