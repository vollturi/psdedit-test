//! EditPSD web UI links and branded start-screen controls.

use crate::PhotocraftApp;

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

