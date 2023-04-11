use bevy::prelude::*;
use bevy::render::render_resource::Extent3d;

#[derive(Component)]
pub struct MainCamera;

#[derive(Resource)]
pub struct Viewport {
    pub image_handle: Option<Handle<Image>>,
    pub size: Extent3d,
    pub setup: bool
}

impl Default for Viewport {
    fn default() -> Self {
        let size = Extent3d {
            width: 200,
            height: 200,
            ..default()
        };
        Viewport {
            image_handle: None,
            size: size,
            setup: false
        }
    }
}