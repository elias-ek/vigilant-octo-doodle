use bevy::{color, prelude::*};

const LEVELS: &[&str] = &["test", "erm", "sick castle"];

pub fn select_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let btn_font: Handle<Font> = asset_server.load("font/joystix.otf");

    commands.spawn(Node { ..default() }).with_children(|p| {
        for level in LEVELS {
            p.spawn(Button).with_children(|b| {
                b.spawn((
                    Text::new(level.to_string()),
                    TextFont {
                        font: btn_font.clone(),
                        font_size: 25.,
                        ..default()
                    },
                ));
            });
        }
    });
}
