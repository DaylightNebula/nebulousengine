use bevy::prelude::*;
use belly::prelude::*;
use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
};

use crate::components::{MainCamera, Viewport};

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(BellyPlugin)
            .add_startup_systems((setup_viewport, setup_ui))
            .add_system(rotator_system);
    }
}

// Marks the first pass (rendered to a texture.)
#[derive(Component)]
struct FirstPassViewport;

fn setup_viewport(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut viewport: ResMut<Viewport>,
) {
    let size = viewport.size;

    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            view_formats: &[],
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
        },
        ..default()
    };

    // fill image.data with zeroes
    image.resize(size);

    let image_handle = images.add(image);
    viewport.image_handle = Some(image_handle.clone());
    viewport.setup = true;

    // let cube_handle = meshes.add(Mesh::from(shape::Cube { size: 4.0 }));
    // let cube_material_handle = materials.add(StandardMaterial {
    //     base_color: Color::rgb(0.8, 0.7, 0.6),
    //     reflectance: 0.02,
    //     unlit: false,
    //     ..default()
    // });

    // // This specifies the layer used for the first pass, which will be attached to the first pass camera and cube.
    let viewport_pass_layer = RenderLayers::layer(1);

    // // The cube that will be rendered to the texture.
    // commands.spawn((
    //     PbrBundle {
    //         mesh: cube_handle,
    //         material: cube_material_handle,
    //         transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
    //         ..default()
    //     },
    //     FirstPassViewport,
    //     viewport_pass_layer,
    // ));

    // // Light
    // // NOTE: Currently lights are shared between passes - see https://github.com/bevyengine/bevy/issues/3462
    // commands.spawn(PointLightBundle {
    //     transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
    //     ..default()
    // });

    // commands
    //     .spawn(Camera3dBundle {
    //         camera_3d: Camera3d {
    //             clear_color: ClearColorConfig::Custom(Color::WHITE),
    //             ..default()
    //         },
    //         camera: Camera {
    //             // render before the "main pass" camera
    //             order: -1,
    //             target: RenderTarget::Image(image_handle.clone()),
    //             ..default()
    //         },
    //         transform: Transform::from_translation(Vec3::new(0.0, 0.0, 15.0))
    //             .looking_at(Vec3::default(), Vec3::Y),
    //         ..default()
    //     })
        // .insert(viewport_pass_layer)
        /*.insert(UiCameraConfig { show_ui: false }) ;*/
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>, viewport: ResMut<Viewport>) {
    // Could be `None` if the viewport is not created yet
    let img_viewport = match viewport.image_handle.clone() {
        Some(handle) => handle,
        //if setup_viewport failed we show an image instead
        None => asset_server.load("icon.png"),
    };

    // commands.spawn(Camera2dBundle::default());
    //use some css for the viewport if you want to
    commands.add(StyleSheet::parse(
        "
            body { padding: 50px; }
            #viewport {
                margin: 50px;
                width: 50%; 
                height: 50%;
                background: red;
            }
        ",
    ));

    commands.add(eml! {
        <body>
            <img  id="viewport" src=img_viewport></img>
        </body>
    });
}

/// Rotates the inner cube (FirstPassViewport)
fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<FirstPassViewport>>) {
    for mut transform in &mut query {
        transform.rotate_x(1.5 * time.delta_seconds());
        transform.rotate_z(1.3 * time.delta_seconds());
    }
}