use bevy::prelude::*;
use bevy::render::render_resource::*;
use bevy_egui::*;
use nebulousengine_components::Viewport;
use self::files_editor_panel::render_files;

pub mod files_editor_panel;

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(EguiPlugin)
            .add_startup_system(setup_viewport)
            .add_system(render_ui);
    }
}

fn setup_viewport(
    mut images: ResMut<Assets<Image>>,
    mut viewport: ResMut<Viewport>
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
}

fn render_ui(mut contexts: EguiContexts, viewport: ResMut<Viewport>, mut rendered_texture_id: Local<egui::TextureId>, mut is_initialized: Local<bool>) {
    // make sure we have an image handle
    if viewport.image_handle.is_none() { return }

    if !*is_initialized {
        *is_initialized = true;
        *rendered_texture_id = contexts.add_image(viewport.image_handle.clone().expect("why"));
    }

    // add image handle to ui
    egui::TopBottomPanel::top("top_panel").show(contexts.ctx_mut(), |ui| {
        egui::menu::bar(ui, |ui| {
            egui::menu::menu_button(ui, "File", |ui| {
                if ui.button("Open Project...").clicked() { println!("TODO open project") }
                if ui.button("New Project...").clicked() { println!("TODO new project") }
            });
        });
    });
    render_files(contexts.ctx_mut());
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        ui.add(egui::widgets::Image::new(
            *rendered_texture_id,
            [512.0, 512.0]
        ));
    });
}