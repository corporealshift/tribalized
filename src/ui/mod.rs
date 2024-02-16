pub mod error;
pub mod main_screen;
pub mod world;

use eframe::egui;

pub use error::Error;
pub use main_screen::main_screen;

pub trait View {
    fn ui(&self, ui: &mut egui::Ui);
}
