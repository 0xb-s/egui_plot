//! Demo: Sine envelopes as a Band with translucent fill + min/mean/max lines.
use eframe::{App, Frame, egui};
use egui::{Color32, Context};
use egui_plot::{Band, Legend, Line, Plot};

fn main() -> eframe::Result<()> {
    let opts = eframe::NativeOptions::default();
    eframe::run_native(
        "egui_plot: Sine Band (min/mean/max)",
        opts,
        Box::new(|_| Ok(Box::new(AppBand::new()))),
    )
}

struct AppBand {
    xs: Vec<f64>,
    y_min: Vec<f64>,
    _y_mean: Vec<f64>,
    y_max: Vec<f64>,
    min_line: Vec<[f64; 2]>,
    mean_line: Vec<[f64; 2]>,
    max_line: Vec<[f64; 2]>,
}

impl AppBand {
    fn new() -> Self {
        let n = 1000usize;
        let x0 = 0.0;
        let x1 = 6.0 * std::f64::consts::PI;
        let dx = (x1 - x0) / (n.saturating_sub(1) as f64);

        let p3 = std::f64::consts::PI / 3.0;
        let p5 = std::f64::consts::PI / 5.0;

        let mut xs = Vec::with_capacity(n);
        let mut y1 = Vec::with_capacity(n); 
        let mut _y_mean = Vec::with_capacity(n); 
        let mut y3 = Vec::with_capacity(n); 

        for i in 0..n {
            let x = x0 + (i as f64) * dx;
            xs.push(x);
            y1.push((x).sin());
            _y_mean.push(0.8 * (2.0 * x + p3).sin());
            y3.push(1.5 * (3.0 * x + p5).sin());
        }

        let (y_min, y_max): (Vec<f64>, Vec<f64>) = y1
            .into_iter()
            .zip(y3.iter().copied())
            .map(|(a, b)| if a <= b { (a, b) } else { (b, a) })
            .unzip();

        let min_line: Vec<[f64; 2]> = xs
            .iter()
            .copied()
            .zip(y_min.iter().copied())
            .map(|t| t.into())
            .collect();
        let mean_line: Vec<[f64; 2]> = xs
            .iter()
            .copied()
            .zip(_y_mean.iter().copied())
            .map(|t| t.into())
            .collect();
        let max_line: Vec<[f64; 2]> = xs
            .iter()
            .copied()
            .zip(y_max.iter().copied())
            .map(|t| t.into())
            .collect();

        Self {
            xs,
            y_min,
            _y_mean,
            y_max,
            min_line,
            mean_line,
            max_line,
        }
    }
}

impl App for AppBand {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Band with min/mean/max");
            ui.label("Translucent band between y_min and y_max; white line is y_mean. Toggle lines via legend.");
            ui.add_space(6.0);

            Plot::new("sine_band_plot")
                .legend(Legend::default())
                .allow_zoom(true)
                .allow_drag(true)
                .show(ui, |plot_ui| {
                    let band_color = Color32::from_rgba_unmultiplied(64, 160, 255, 96);
                    plot_ui.band(
                        Band::new()
                            .with_color(band_color)
                            .with_series(&self.xs, &self.y_min, &self.y_max)
                    );

                    plot_ui.line(
                        Line::new("y_min", self.min_line.clone())
                            .color(Color32::from_rgb(64, 160, 255))
                            .width(1.5),
                    );
                    plot_ui.line(
                        Line::new("mean", self.mean_line.clone())
                            .color(Color32::WHITE)
                            .width(2.0),
                    );
                    plot_ui.line(
                        Line::new("y_max", self.max_line.clone())
                            .color(Color32::from_rgb(64, 160, 255))
                            .width(1.5),
                    );
                });
        });
    }
}
