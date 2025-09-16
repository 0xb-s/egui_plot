use eframe::{App, Frame, egui};
use egui::{Color32, Context};
use egui_plot::{Band, Line, Plot};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "egui_plot: Band (Variance) Demo",
        options,
        Box::new(|_cc| Ok(Box::new(VarianceDemoApp::new()))),
    )
}


struct VarianceDemoApp {
    xs: Vec<f64>,
    mean: Vec<f64>,
    y_min: Vec<f64>,
    y_max: Vec<f64>,
}

impl VarianceDemoApp {
    /// demo time series: mean = sin(0.9 t), variance width oscillates with cos(0.7 t).
    fn new() -> Self {
        let n = 200usize;
        let dt = 0.05_f64;

        let xs: Vec<f64> = (0..n).map(|i| i as f64 * dt).collect();
        let mean: Vec<f64> = xs.iter().map(|&t| (t * 0.9).sin()).collect();
        let spread: Vec<f64> = xs
            .iter()
            .map(|&t| 0.15 + 0.05 * (t * 0.7).cos().abs())
            .collect();

        let (y_min, y_max): (Vec<f64>, Vec<f64>) = mean
            .iter()
            .zip(spread.iter())
            .map(|(&m, &s)| (m - s, m + s))
            .unzip();

        Self {
            xs,
            mean,
            y_min,
            y_max,
        }
    }
}


impl App for VarianceDemoApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Variance band demo ");
            ui.label("Shaded area shows uncertainty (y_min..y_max); white line is the mean.");

            Plot::new("variance_band_plot")
                .show(ui, |plot_ui| {
                    let band = Band::new("± variance")
                        .with_color(Color32::from_rgb(64, 160, 255)) 
                        .with_alpha(80) 
                        .with_series(&self.xs, &self.y_min, &self.y_max);
                    plot_ui.band(band);

    
                    let center_series: Vec<[f64; 2]> = self
                        .xs
                        .iter()
                        .zip(self.mean.iter())
                        .map(|(&x, &y)| [x, y])
                        .collect();

                    plot_ui.line(
                        Line::new("mean", center_series)
                            .color(Color32::WHITE)
                            .width(2.0),
                    );
                });
        });
    }
}


