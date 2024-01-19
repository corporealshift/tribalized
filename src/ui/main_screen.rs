use crate::Error;
use eframe::egui;

pub fn main_screen() -> Result<(), Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1200.0, 1100.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Tribalized",
        options,
        Box::new(|cc| Box::new(MainScreen::new(cc))),
    )
    .map_err(|_e| Error {})
}

struct MainScreen {
    label: String,
}

impl eframe::App for MainScreen {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.add_space(16.0);
                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Tribalized");
        });
    }
}

impl Default for MainScreen {
    fn default() -> Self {
        Self {
            label: "hi".to_owned(),
        }
    }
}

impl MainScreen {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}
