#![allow(rustdoc::missing_crate_level_docs)]
use eframe::egui;
use egui::{Color32, Stroke};
use egui_plot::{HSpan, Interval, Line, Plot, VSpan};

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "HSpan/VSpan demo",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

struct MyApp {
    xs: Vec<f64>,
    f1: Vec<f64>,

    show_x_spans: bool,
    show_y_spans: bool,

    alpha: u8,
}

impl Default for MyApp {
    fn default() -> Self {
        let xs: Vec<f64> = (0..600).map(|i| i as f64 * 0.02).collect();
        let f1: Vec<f64> = xs.iter().map(|&x| (x * 1.2).sin()).collect();

        Self {
            xs,
            f1,
            show_x_spans: true,
            show_y_spans: true,
            alpha: 90,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("controls").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.toggle_value(&mut self.show_y_spans, "Show HSpan (Y intervals)");
                ui.toggle_value(&mut self.show_x_spans, "Show VSpan (X intervals)");
                ui.label("Fill alpha");
                ui.add(egui::Slider::new(&mut self.alpha, 20..=160));
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            Plot::new("plot")
                .legend(Default::default())
                .allow_zoom(true)
                .allow_scroll(true)
                .show(ui, |plot_ui| {
                    plot_ui.line(
                        Line::new_xy("f1 = sin(1.2x)", &self.xs, &self.f1)
                            .color(Color32::LIGHT_BLUE)
                            .width(2.0),
                    );

                    if self.show_y_spans {
                        plot_ui.add(
                            HSpan::new("center band", Interval::new(-0.25, 0.25))
                                .color(Color32::from_rgba_unmultiplied(64, 160, 255, self.alpha))
                                .outline(Stroke::new(1.0, Color32::from_rgb(64, 160, 255))),
                        );

                        plot_ui.add(
                            HSpan::new("below -0.6", Interval::new(f64::NEG_INFINITY, -0.6))
                                .color(Color32::from_rgba_unmultiplied(220, 80, 80, self.alpha)),
                        );

                        plot_ui.add(
                            HSpan::new("above 0.8", Interval::new(0.8, f64::INFINITY))
                                .color(Color32::from_rgba_unmultiplied(80, 80, 220, self.alpha))
                                .outline(Stroke::new(1.0, Color32::from_rgb(80, 80, 220))),
                        );
                    }

                    if self.show_x_spans {
                        plot_ui.add(
                            VSpan::new("x ∈ [2, 4]", Interval::new(2.0, 4.0))
                                .color(Color32::from_rgba_unmultiplied(120, 200, 140, self.alpha))
                                .outline(Stroke::new(1.0, Color32::from_rgb(120, 200, 140))),
                        );

                        plot_ui.add(
                            VSpan::new("x ≤ 1.2", Interval::new(f64::NEG_INFINITY, 1.2))
                                .color(Color32::from_rgba_unmultiplied(200, 120, 60, self.alpha)),
                        );

                        plot_ui.add(
                            VSpan::new("x ≥ 8", Interval::new(8.0, f64::INFINITY))
                                .color(Color32::from_rgba_unmultiplied(150, 120, 220, self.alpha))
                                .outline(Stroke::new(1.0, Color32::from_rgb(150, 120, 220))),
                        );
                    }
                });
        });
    }
}
