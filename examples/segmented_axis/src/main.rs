use eframe::egui;
use egui::{Color32, Slider};
use egui_plot::{BrokenXAxis, Interval, Line, Plot, TooltipOptions};

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Broken X Axis Demo (with +/-∞ style segments)",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

struct MyApp {
    xs: Vec<f64>,
    ys: Vec<f64>,

    mode: usize, 
    gap_px: f32,
}

impl Default for MyApp {
    fn default() -> Self {

        let xs: Vec<f64> = (0..=2100).map(|i| i as f64).collect();


        let ys: Vec<f64> = xs
            .iter()
            .map(|&x| {
                if x < 200.0 {
                    (x * 0.05).sin() * 5.0 + 10.0
                } else if x < 800.0 {
                    12.0
                } else if x < 1500.0 {
                    ((x - 800.0) * 0.2).cos() * 20.0 + 30.0
                } else if x < 1700.0 {
                    15.0
                } else {
                    ((x - 1700.0) * 0.1).sin() * 8.0 + 22.0
                }
            })
            .collect();

        Self {
            xs,
            ys,
            mode: 2,
            gap_px: 12.0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("controls").show(ctx, |ui| {
            ui.heading("Controls");

            ui.label("Broken axis mode:");
            ui.radio_value(&mut self.mode, 0, "0: no break");
            ui.radio_value(&mut self.mode, 1, "1: [0..200] ⊔ [800..1000]");
            ui.radio_value(
                &mut self.mode,
                2,
                "2: [0..200] ⊔ [800..1500] ⊔ [1700..2000]",
            );
            ui.radio_value(
                &mut self.mode,
                3,
                "3: (~-inf..200] ⊔ [800..820] ⊔ [2000..+inf)  (simulated)",
            );

            ui.add(Slider::new(&mut self.gap_px, 3.0..=40.0).text("gap_px (cut width)"));

            ui.separator();
            match self.mode {
                0 => {
                    ui.label("Mode 0: continuous axis (no breaks).");
                    ui.label("Full data: x ∈ [0, 2100]");
                }
                1 => {
                    ui.label("Mode 1: two windows:");
                    ui.label("[0,200] and [800,1000]");
                    ui.label("Skips (200,800) and (1000,∞).");
                }
                2 => {
                    ui.label("Mode 2: three windows:");
                    ui.label("[0,200], [800,1500], [1700,2000]");
                    ui.label("Skips big boring gaps between them.");
                }
                3 => {
                    ui.label("Mode 3: conceptually (-∞,200], [800,820], [2000,+∞)");
                    ui.label("Here we fake that with finite ranges:");
                    ui.label("[0,200], [800,820], [2000,2100]");
                }
                _ => {}
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
       
            let broken_cfg = match self.mode {
                1 => {
               
                    Some(BrokenXAxis::new(
                        vec![Interval::new(0.0, 200.0), Interval::new(800.0, 1000.0)],
                        self.gap_px,
                    ))
                }
                2 => {
                 
                    Some(BrokenXAxis::new(
                        vec![
                            Interval::new(0.0, 200.0),
                            Interval::new(800.0, 1500.0),
                            Interval::new(1700.0, 2000.0),
                        ],
                        self.gap_px,
                    ))
                }
                3 => {
                
                    Some(BrokenXAxis::new(
                        vec![
                            Interval::new(0.0, 200.0),
                            Interval::new(800.0, 820.0),
                            Interval::new(2000.0, 2100.0),
                        ],
                        self.gap_px,
                    ))
                }
                _ => None,
            };

            Plot::new("plot_broken_x_multi")
                .allow_zoom(true)
                .allow_scroll(true)
                .broken_x_axis(broken_cfg)
                .show(ui, |plot_ui| {
                    plot_ui.line(
                        Line::new_xy("data", &self.xs, &self.ys)
                            .color(Color32::LIGHT_GREEN)
                            .width(2.0),
                    );
                    plot_ui.show_tooltip_with_options(&TooltipOptions::default());
                });
        });
    }
}
