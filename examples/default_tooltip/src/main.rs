#![allow(rustdoc::missing_crate_level_docs)]
use eframe::egui;
use eframe::{App, Frame};
use egui::{Color32, Context};
use egui_plot::{Line, Plot, TooltipOptions};
fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Band tooltip across series",
        eframe::NativeOptions::default(),
        Box::new(|_| Ok(Box::new(Demo::new()))),
    )
}

struct Demo {
    x: Vec<f64>,
    f1: Vec<f64>,
    f2: Vec<f64>,
}

impl Demo {
    fn new() -> Self {
        let n = 400;
        let x: Vec<f64> = (0..n).map(|i| i as f64 * 0.03).collect();
        let f1: Vec<f64> = x.iter().map(|&t| (t).sin()).collect();
        let f2: Vec<f64> = x
            .iter()
            .map(|&t| (t * 0.6 + 0.8).sin() * 0.8 + 0.2)
            .collect();
        Self { x, f1, f2 }
    }
}

impl App for Demo {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            ui.heading("Nearest-by-X band tooltip");
            ui.label("Move the mouse; we select nearest x-samples of each series within the vertical band.");

            Plot::new("band_tooltip").show(ui, |plot_ui| {
                let s1: Vec<[f64; 2]> = self.x.iter().zip(self.f1.iter()).map(|(&x,&y)| [x,y]).collect();
                let s2: Vec<[f64; 2]> = self.x.iter().zip(self.f2.iter()).map(|(&x,&y)| [x,y]).collect();

                plot_ui.line(Line::new("f1", s1).color(Color32::from_rgb(120, 220, 120)).width(2.0));
                plot_ui.line(Line::new("f2", s2).color(Color32::from_rgb(120, 160, 255)).width(2.0));

    plot_ui.show_tooltip_with_options(&TooltipOptions::default());

      });
        });
    }
}
