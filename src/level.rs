use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub fn level_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        //    Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scale: 0.5,
            ..OrthographicProjection::default_2d()
        }),
        Transform::from_xyz(1280.0 / 4.0, 720.0 / 4.0, 0.0),
    ));

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("test.ldtk").into(),
        ..default()
    });
}
