#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use egui::ViewportBuilder;

mod wpm;

fn main() {
    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_resizable(false)
            .with_decorations(false)
            .with_always_on_top()
            .with_position([0.0, 0.0])
            .with_transparent(true),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))),
    )
    .err();
}

struct MyEguiApp {
    text_size: f32,
    text_color: egui::Color32,
    background_color: egui::Color32,
    padding: i8,
    wpm_tracker: wpm::WpmTracker,
}

impl Default for MyEguiApp {
    fn default() -> Self {
        Self {
            text_size: 20.0,
            text_color: egui::Color32::WHITE,
            background_color: egui::Color32::from_rgba_unmultiplied(10, 10, 10, 220),
            padding: 10,
            wpm_tracker: wpm::WpmTracker::new(),
        }
    }
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
    }
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Sync keystrokes from background listener
        self.wpm_tracker.update();
        ctx.request_repaint();

        let mut visual = egui::Visuals::dark();
        visual.panel_fill = self.background_color;
        ctx.set_visuals(visual);

        egui::CentralPanel::default()
            .frame(
                egui::Frame::NONE
                    .inner_margin(egui::Margin::same(self.padding))
                    .fill(self.background_color),
            )
            .show(ctx, |ui| {
                ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);

                let wpm = self.wpm_tracker.calculate_wpm();
                let text = format!("{:.0} WPM", wpm);
                let response = ui.label(
                    egui::RichText::new(&text)
                        .size(self.text_size)
                        .color(self.text_color),
                );

                let content_size = response.rect.size();
                let total_size = egui::vec2(
                    content_size.x + self.padding as f32 * 2.0,
                    content_size.y + self.padding as f32 * 2.0,
                );
                ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(total_size));
            });
    }
}
