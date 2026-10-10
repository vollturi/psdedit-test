//! EditPSD web UI links and branded start-screen controls.

use crate::PhotocraftApp;

/// Retained for internal callers, but upstream links are disabled in EditPSD web UI.
pub const DISCORD: &str = "";
pub const ARTCRAFT_WEBSITE: &str = "";
pub const APP_PAGE: &str = "";
pub const GITHUB: &str = "";
pub const ISSUES: &str = "";

pub const COMMANDS: &[(&str, &str)] = &[];

pub fn url_for(_id: &str) -> Option<&'static str> { None }

pub fn open(_app: &mut PhotocraftApp, _ctx: &egui::Context, _url: &str) -> serde_json::Value {
    serde_json::Value::Null
}

/// No external community or upstream project links are displayed in the EditPSD web build.
pub fn discord_button(_app: &mut PhotocraftApp, ui: &mut egui::Ui, _min_width: f32) -> egui::Response {
    ui.allocate_response(egui::Vec2::ZERO, egui::Sense::hover())
}

/// Keep the start screen focused on the EditPSD product brand with no upstream links.
pub fn link_row(_app: &mut PhotocraftApp, ui: &mut egui::Ui) {
    let t = crate::theme::Tokens::get(ui.ctx());
    ui.vertical_centered(|ui| {
        ui.label(egui::RichText::new("editPSD.net").strong().size(13.0).color(t.text));
        ui.label(egui::RichText::new("Free online PSD editor").size(11.5).color(t.text_faint));
    });
}

