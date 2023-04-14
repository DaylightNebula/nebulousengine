use bevy::prelude::*;
use bevy_egui::EguiContexts;
use egui::*;

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

pub fn render_editor(mut contexts: EguiContexts, tabs: &mut EditorTabs) {
    
    egui::TopBottomPanel::top("tab_buttons_container").show(contexts.ctx_mut(), |ui| {
        render_editor_tabs(ui, tabs);
    });
    
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        render_editor_main(ui, tabs);
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

fn render_editor_main(ui: &mut Ui, tabs: &mut EditorTabs) {
}