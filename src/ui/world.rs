use crate::world::{Terrain, Tile};
use crate::Error;
use crate::World;

use eframe::egui;
use eframe::egui::emath::Pos2;
use eframe::egui::{emath, Rect};
use eframe::epaint::Color32;
use eframe::epaint::{RectShape, Rounding};

pub struct WorldView {
    pub world: World,
}

fn fill_for_tile(tile: &Tile) -> Color32 {
    match tile.terrain {
        Terrain::Mountain => Color32::from_rgb(100, 100, 100),
        Terrain::Water => Color32::from_rgb(150, 190, 250),
        Terrain::Forest => Color32::from_rgb(50, 150, 75),
        Terrain::Grass => Color32::from_rgb(50, 200, 100),
    }
}

fn top_left_for_tile(offset: Pos2, x: u16, y: u16) -> Pos2 {
    Pos2::new(
        offset.x + f32::from(x) * 10.0,
        offset.y + f32::from(y) * 10.0,
    )
}

impl WorldView {
    pub fn map_content(&self, ui: &mut egui::Ui) -> egui::Response {
        let (response, painter) =
            ui.allocate_painter(emath::Vec2::splat(1000.0), egui::Sense::click());
        let rounding = Rounding::ZERO;
        let offset = response.rect.left_top();
        let shapes = self
            .world
            .map
            .iter()
            .map(|(coords, tile)| {
                let color = fill_for_tile(tile);
                let pos1 = top_left_for_tile(offset, coords.0, coords.1);
                let translated = pos1;
                let pos2 = Pos2::new(translated.x + 8.0, translated.y + 8.0);
                //println!("Top Left {:?}, {:?}", coords, translated);
                let rect = Rect::from_two_pos(translated, pos2);
                let rect_shape = RectShape::filled(rect, rounding, color);
                egui::Shape::Rect(rect_shape)
            })
            .collect::<Vec<egui::Shape>>();
        //println!("--------------DRAWING-----------------");

        painter.extend(shapes);

        response
    }
}

impl super::View for WorldView {
    fn ui(&self, ui: &mut egui::Ui) {
        egui::Frame::canvas(ui.style()).show(ui, |ui| {});
    }
}
