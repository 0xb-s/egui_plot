use eframe::egui::{self, RichText};
use eframe::{App, Frame};
use egui::{Color32, Context};
use egui_plot::{BandTooltipOptions, HitRow};
use egui_plot::{Line, PinnedRow, Plot};
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

                                    #[allow(clippy::excessive_nesting)]
  plot_ui.show_band_tooltip_across_series_with(
        12.0,
        &BandTooltipOptions::default(),
        |ui, _hits: &[HitRow], pins: &[PinnedRow]| {
            ui.strong("Pinned snapshots");
            if pins.is_empty() {
                ui.weak("No pins yet. Hover and press P to pin, U to unpin last, Delete to clear.");
                return;
            }

            for (k, snap) in pins.iter().enumerate() {
                egui::CollapsingHeader::new(format!("Pin #{k}"))
                    .default_open(false)
                    .show(ui, |ui| {
                        egui::Grid::new(format!("pin_grid_{k}"))
                            .num_columns(4)
                            .spacing([8.0, 2.0])
                            .striped(true)
                            .show(ui, |ui| {
                                ui.weak(""); ui.weak("series"); ui.weak("x"); ui.weak("y"); ui.end_row();
                                for h in &snap.hits {
                                    ui.label(RichText::new("●").color(h.color));
                                    ui.monospace(&h.series_name);
                                    ui.monospace(format!("{:.6}", h.value.x));
                                    ui.monospace(format!("{:.6}", h.value.y));
                                    ui.end_row();
                                }
                            });
                    });
            }

            ui.add_space(6.0);
            ui.weak("Hotkeys: P = pin current, U = unpin last, Delete = clear all");
        },
    );
      });
        });
    }
}
