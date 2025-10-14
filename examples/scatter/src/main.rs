#![allow(rustdoc::missing_crate_level_docs)]
use eframe::{App, Frame, egui};
use egui::Color32;
use egui_plot::MarkerShape;
use egui_plot::Plot;
use egui_plot::Scatter;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Scatter (columnar, minimal)",
        eframe::NativeOptions::default(),
        Box::new(|_| Ok(Box::new(Demo))),
    )
}

struct Demo;

impl App for Demo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let xs: Vec<f64> = (0..20).map(|i| i as f64 * 0.5).collect();
            let ys0: Vec<f64> = xs.iter().map(|&x| (x * 0.5).sin()).collect();

            Plot::new("scatter_min").show(ui, |plot_ui| {
            
                let s0 = egui_plot::ColumnarSeries::new(xs.as_slice(), ys0.as_slice());
                plot_ui.add(
                    Scatter::from_series("sin", s0)
                        .marker_shape(MarkerShape::Asterisk)
                        .color(Color32::from_rgb(220, 80, 80))
                        .radius(5.0)
                        .filled(true),
                );
            });
        });
    }
}
