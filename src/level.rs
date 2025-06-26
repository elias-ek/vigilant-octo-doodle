use bevy::{prelude::*, window::PrimaryWindow};
use bevy_ecs_ldtk::prelude::*;

const LEVEL_WIDTH: f32 = 960.0;
const LEVEL_HEIGHT: f32 = 640.0;

pub fn level_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2d,
        Transform::from_xyz(LEVEL_WIDTH / 2.0, LEVEL_HEIGHT / 2.0, 0.0),
    ));

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("test.ldtk").into(),
        ..default()
    });
}

pub fn scale_cam(
    mut cam_query: Query<&mut Projection, With<Camera2d>>,
    win_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = win_query.single().unwrap();
    let mut proj = cam_query.single_mut().unwrap();

    let win_aspect = window.width() / window.height();
    let lvl_aspect = LEVEL_WIDTH / LEVEL_HEIGHT;

    let scale = if win_aspect > lvl_aspect {
        LEVEL_HEIGHT / window.height()
    } else {
        LEVEL_WIDTH / window.width()
    };

    if let Projection::Orthographic(ref mut ortho) = *proj {
        ortho.scale = scale;
    }
}
