mod editor;
mod components;

use bevy::prelude::*;
use bevy::render::camera::RenderTarget;

use crate::editor::*;
use crate::components::*;

fn main() {
    App::new()
        .init_resource::<Viewport>()
        .add_plugins(DefaultPlugins)
        .add_plugin(EditorPlugin)
        .add_startup_system(setup_scene)
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    viewport: ResMut<Viewport>,
) {
    let cube_handle = meshes.add(Mesh::from(shape::Cube { size: 4.0 }));
    let cube_material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.8, 0.7, 0.6),
        reflectance: 0.02,
        unlit: false,
        ..default()
    });

    // The cube that will be rendered to the texture.
    commands.spawn(
        PbrBundle {
            mesh: cube_handle,
            material: cube_material_handle,
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
            ..default()
        }
    );

    // Light
    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
        ..default()
    });

    commands
        .spawn(Camera3dBundle {
            camera: Camera {
                target: if viewport.image_handle.is_some() { RenderTarget::Image(viewport.image_handle.clone().expect("hi")) } else { Default::default() },
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 15.0))
                .looking_at(Vec3::default(), Vec3::Y),
            ..default()
        })
        .insert(UiCameraConfig { show_ui: false });
}