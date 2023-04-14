use bevy::prelude::*;
use bevy_egui::EguiContexts;
use egui::*;
use nebulousengine_components::Viewport;

#[derive(Resource)]
pub struct EditorTabs {
    tree: Vec<String>
}

impl EditorTabs {
    pub fn new() -> Self {
        let mut tree = Vec::new();
        tree.push("tab1".to_string());
        tree.push("tab2".to_string());

        Self { tree }
    }
}

pub fn render_editor(mut contexts: EguiContexts, viewport: ResMut<Viewport>, mut rendered_texture_id: Local<egui::TextureId>, tabs: &mut EditorTabs) {
    
    egui::TopBottomPanel::top("tab_buttons_container").show(contexts.ctx_mut(), |ui| {
        render_editor_tabs(ui, tabs);
    });
    
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        render_editor_main(ui, viewport, rendered_texture_id, tabs);
    });
}

// function that renders tabs in top menu
fn render_editor_tabs(ui: &mut Ui, tabs: &mut EditorTabs) {
    // create a menu bar for the tabs
    ui.horizontal(|ui| {
        for element in tabs.tree.iter() {
            if ui.button(element).clicked() {
                println!("Element clicked {}", element);
            }
        }
    });
}

fn render_editor_main(ui: &mut Ui, viewport: ResMut<Viewport>, mut rendered_texture_id: Local<egui::TextureId>, tabs: &mut EditorTabs) {
    let img = viewport.image_handle.as_ref().unwrap();
    ui.add(egui::widgets::Image::new(
        *rendered_texture_id,
        [viewport.size.width.to_owned() as f32, viewport.size.height.to_owned() as f32]
    ));
}
// Example how to insert render image
// ui.add(egui::widgets::Image::new(
//     *rendered_texture_id,
//     [512.0, 512.0]
// ));