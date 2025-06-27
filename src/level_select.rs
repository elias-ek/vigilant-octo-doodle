use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

const PROJECT: &str = "test.ldtk";
const LEVELS: &[u8] = &[0, 1, 2, 3, 4, 5];

pub fn select_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    handle: Res<LdtkHandle>,
    assets: Res<Assets<LdtkProject>>,
) {
    let btn_font: Handle<Font> = asset_server.load("font/joystix.otf");

    commands
        .spawn(Node {
            display: Display::Grid,
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            // grid_template_rows: vec![RepeatedGridTrack::],
            ..default()
        })
        .with_children(|p| {
            if let Some(project) = assets.get(&handle.0) {
                for level in project.iter_root_levels() {
                    p.spawn((
                        Button,
                        Node {
                            padding: UiRect::all(Val::Px(10.)),
                            margin: UiRect::all(Val::Px(10.)),
                            border: UiRect::all(Val::Px(5.)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BorderColor(Color::WHITE),
                        BackgroundColor(Color::srgb(0., 0., 128.)),
                    ))
                    .with_children(|b| {
                        b.spawn((
                            Text::new(level.identifier.clone()),
                            TextFont {
                                font: btn_font.clone(),
                                font_size: 30.,
                                ..default()
                            },
                            TextColor::WHITE,
                        ));
                    });
                }
            }
        });
}

#[derive(Resource)]
pub struct LdtkHandle(Handle<LdtkProject>);

pub fn load_project(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle = asset_server.load(PROJECT);
    commands.insert_resource(LdtkHandle(handle));
}
