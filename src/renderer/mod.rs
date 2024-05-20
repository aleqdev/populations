use bevy::{prelude::*, render::Render};


pub struct RendererPlugin;


impl Plugin for RendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    mut asset_server: ResMut<AssetServer>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }).with_children(|parent| {
        parent.spawn(PointLightBundle {
            point_light: PointLight {
                color: Color::WHITE,
                intensity: 2000.0,
                range: 30.0,
                radius: 2.0,
                shadows_enabled: false,
                ..default()
            },
            ..default()
        });
    });

    commands.spawn(SceneBundle {
        scene: asset_server.load("earth.glb#Scene0"),
        ..default()
    });
}