#![allow(rustdoc::missing_crate_level_docs)]
use eframe::{App, Frame, egui};
use egui::Color32;
use egui_plot::{ColumnarSeries, Legend, MarkerShape, Plot, Scatter, TooltipOptions};

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Scatter • all marker shapes",
        eframe::NativeOptions::default(),
        Box::new(|_| Ok(Box::new(Demo::default()))),
    )
}

struct Demo {
    xs: Vec<f64>,

    waves: Vec<Vec<f64>>,

    shapes: Vec<(&'static str, MarkerShape)>,
}

impl Default for Demo {
    fn default() -> Self {
        let xs: Vec<f64> = (0..240).map(|i| i as f64 * 0.04).collect();

        let shapes: Vec<(&'static str, MarkerShape)> = vec![
            ("Circle", MarkerShape::Circle),
            ("Point", MarkerShape::Point),
            ("Pixel", MarkerShape::Pixel),
            ("PlusFilled", MarkerShape::PlusFilled),
            ("XFilled", MarkerShape::XFilled),
            (
                "RegularPolygon(6)",
                MarkerShape::RegularPolygon { n: 6, angle_deg: 0 },
            ),
            (
                "StarPolygon(5)",
                MarkerShape::StarPolygon {
                    n: 5,
                    inner_r_ppm: 40_000,
                    angle_deg: -18,
                },
            ),
            ("Square", MarkerShape::Square),
            ("Diamond", MarkerShape::Diamond),
            ("Cross", MarkerShape::Cross),
            ("Asterisk", MarkerShape::Asterisk),
            ("Left", MarkerShape::Left),
            ("Down", MarkerShape::Down),
            ("Up", MarkerShape::Up),
            ("Plus", MarkerShape::Plus),
        ];

        let waves: Vec<Vec<f64>> = shapes
            .iter()
            .enumerate()
            .map(|(i, _)| {
                let phase = i as f64 * 0.35;
                xs.iter().map(|&x| (x + phase).sin()).collect()
            })
            .collect();

        Self { xs, waves, shapes }
    }
}

impl App for Demo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let palette = [
                Color32::from_rgb(220, 80, 80),
                Color32::from_rgb(90, 140, 255),
                Color32::from_rgb(60, 180, 120),
                Color32::from_rgb(240, 180, 60),
                Color32::from_rgb(160, 90, 210),
                Color32::from_rgb(70, 170, 200),
                Color32::from_rgb(210, 110, 130),
                Color32::from_rgb(140, 140, 140),
            ];

            Plot::new("all_marker_shapes")
                .legend(Legend::default())
                .show(ui, |plot_ui| {
                    for (i, (label, shape)) in self.shapes.iter().enumerate() {
                        let ys = &self.waves[i];
                        let series = ColumnarSeries::new(&self.xs, ys);
                        let color = palette[i % palette.len()];

                        // balance size per glyph
                        let radius = match shape {
                            MarkerShape::Point => 3.0,
                            MarkerShape::Pixel => 2.2,
                            MarkerShape::PlusFilled
                            | MarkerShape::XFilled
                            | MarkerShape::Asterisk
                            | MarkerShape::Cross
                            | MarkerShape::Plus => 5.0,
                            _ => 4.0,
                        };

                        plot_ui.add(
                            Scatter::from_series(*label, series)
                                .marker_shape(*shape)
                                .color(color)
                                .radius(radius)
                                .filled(true),
                        );
                    }

                    let tip = TooltipOptions {
                        radius_px: 18.0,
                        ..Default::default()
                    };
                    plot_ui.show_tooltip_with_options(&tip);
                });
        });
    }
}
