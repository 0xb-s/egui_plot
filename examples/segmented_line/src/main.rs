#![allow(rustdoc::missing_crate_level_docs)]
use eframe::{egui, egui::Color32};
use egui_plot::{Legend, Line, Plot, TooltipOptions};

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "egui_plot • line_xy_blocks demo",
        eframe::NativeOptions::default(),
        Box::new(|_| Ok(Box::new(Demo::default()))),
    )
}

struct Demo {
    x_blocks: Vec<Vec<f64>>,
    y_blocks: Vec<Vec<f64>>,
    xs_full: Vec<f64>,
    ys_full: Vec<f64>,
}

impl Default for Demo {
    fn default() -> Self {
        let mk_block = |start: i32, len: usize, f: fn(f64) -> f64| -> (Vec<f64>, Vec<f64>) {
            let xs: Vec<f64> = (0..len).map(|i| (start + i as i32) as f64).collect();
            let ys: Vec<f64> = xs.iter().map(|&x| f(x * 0.2)).collect();
            (xs, ys)
        };

        let (bx0, by0) = mk_block(0, 10, |t| t.sin());
        let (bx1, by1) = mk_block(50, 10, |t| (t + 0.8).cos() * 0.8);
        let (bx2, by2) = mk_block(120, 15, |t| (t * 0.7).sin() + 0.5);

        let xs_full: Vec<f64> = (0..135).map(|i| i as f64).collect();
        let ys_full: Vec<f64> = xs_full.iter().map(|&x| (x * 0.2).sin()).collect();

        Self {
            x_blocks: vec![bx0, bx1, bx2],
            y_blocks: vec![by0, by1, by2],
            xs_full,
            ys_full,
        }
    }
}

impl eframe::App for Demo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Red = segmented (blocks), Blue = continuous (for comparison)");

            let plot = Plot::new("seg_blocks")
                .show_x(true)
                .show_y(true)
                .legend(Legend::default());

            plot.show(ui, |plot_ui| {
                let xs_blocks: Vec<&[f64]> = self.x_blocks.iter().map(|v| v.as_slice()).collect();
                let ys_blocks: Vec<&[f64]> = self.y_blocks.iter().map(|v| v.as_slice()).collect();

                plot_ui.line(
                    Line::new_xy_blocks("segmented blocks", xs_blocks, ys_blocks)
                        .color(Color32::from_rgb(220, 80, 80))
                        .width(2.0),
                );

                plot_ui.line(
                    Line::new_xy("continuous (reference)", &self.xs_full, &self.ys_full)
                        .color(Color32::from_rgb(90, 140, 255))
                        .width(1.0),
                );

                let opts = TooltipOptions::default();
                plot_ui.show_tooltip_across_series_with(&opts, |ui, hits, pins| {
                    ui.strong("Nearest per series");
                    ui.separator();
                    #[allow(deprecated)]
                    for h in hits {
                        ui.label(format!(
                            "{}:  x={:.3}  y={:.3}",
                            h.series_name, h.value.x, h.value.y
                        ));
                    }
                    if !pins.is_empty() {
                        ui.separator();
                        ui.weak(format!("Pins: {}", pins.len()));
                    }
                });
            });
        });
    }
}
