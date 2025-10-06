#![allow(rustdoc::missing_crate_level_docs)]

use eframe::{App, Frame, egui};
use egui::{Align2, Color32, Context, RichText};
use egui_plot::{Line, Plot, PlotEvent, TooltipOptions};

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Plot + actions + pins demo",
        eframe::NativeOptions::default(),
        Box::new(|_| Ok(Box::new(Demo::new()))),
    )
}

struct Demo {
    xs: Vec<f64>,
    f1: Vec<f64>,
    f2: Vec<f64>,
    last_event: Option<String>,
}

impl Demo {
    fn new() -> Self {
        let n = 500;
        let xs: Vec<f64> = (0..n).map(|i| i as f64 * 0.02).collect();
        let f1: Vec<f64> = xs.iter().copied().map(f64::sin).collect();
        let f2: Vec<f64> = xs
            .iter()
            .copied()
            .map(|t| (t * 0.6 + 0.8).sin() * 0.8 + 0.2)
            .collect();
        Self {
            xs,
            f1,
            f2,
            last_event: None,
        }
    }
}

impl App for Demo {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Actions + Pins + Tooltip demo");
            ui.add_space(6.0);

            if let Some(s) = &self.last_event {
                ui.label(RichText::new(s).size(16.0).monospace());
            }


            let (_resp, events) = Plot::new("demo_plot")
                .allow_zoom(true)
                .allow_scroll(true)
                .allow_boxed_zoom(true)
                .show_actions(ui, |plot_ui| {

                 let pts1: Vec<[f64; 2]> = self
                                                .xs
    .iter()
    .copied()
    .zip(self.f1.iter().copied())
    .map(<[f64; 2]>::from)
    .collect();

let pts2: Vec<[f64; 2]> = self
    .xs
    .iter()
    .copied()
    .zip(self.f2.iter().copied())
    .map(<[f64; 2]>::from)
    .collect();

                    plot_ui.line(
                        Line::new("f1", pts1)
                            .color(Color32::from_rgb(200, 100, 100))
                            .width(2.0),
                    );

                    plot_ui.line(
                        Line::new("f2", pts2)
                            .color(Color32::from_rgb(100, 160, 240))
                            .width(2.0),
                    );


                    plot_ui.show_tooltip_with_options(&TooltipOptions::default());
                });


            for ev in events {
                match ev {
                    PlotEvent::BoundsChanged { old, new, cause } => {
                        self.last_event = Some(format!(
                            "BoundsChanged ({:?})\nold: x=[{:.2},{:.2}] y=[{:.2},{:.2}]\nnew: x=[{:.2},{:.2}] y=[{:.2},{:.2}]",
                            cause,
                            old.min()[0], old.max()[0], old.min()[1], old.max()[1],
                            new.min()[0], new.max()[0], new.min()[1], new.max()[1],
                        ));
                    }
                    PlotEvent::Hover { pos } => {
                        egui::Window::new("Hover")
                            .anchor(Align2::RIGHT_TOP, (-10.0, 10.0))
                            .collapsible(true)
                            .show(ctx, |ui| {
                                #[allow(deprecated)]
                                ui.monospace(format!("x={:.3}, y={:.3}", pos.x, pos.y));
                            });
                    }
                    PlotEvent::BoxZoomStarted { .. } => {
                        self.last_event = Some("BoxZoomStarted".into());
                    }
                    PlotEvent::BoxZoomFinished { new_x, new_y, .. } => {
                        self.last_event = Some(format!(
                            "BoxZoomFinished: X [{:.2},{:.2}], Y [{:.2},{:.2}]",
                            new_x.start(),
                            new_x.end(),
                            new_y.start(),
                            new_y.end()
                        ));
                    }
                    PlotEvent::Activate { hovered_item } => {
                    self.last_event = Some(format!("Activate on {hovered_item:?}"));
                    }
                    PlotEvent::ContextMenuRequested { screen_pos, item } => {
                        self.last_event = Some(format!(
                            "ContextMenu at ({:.1},{:.1}) on {:?}",
                            screen_pos.x, screen_pos.y, item
                        ));
                    }


                    PlotEvent::PinAdded { snapshot } => {
                        self.last_event = Some(format!(
                            "PinAdded at x={:.6} ({} series)",
                            snapshot.plot_x,
                            snapshot.rows.len()
                        ));
                    }
                    PlotEvent::PinRemoved { index } => {
                        self.last_event = Some(format!("PinRemoved (index={index})"));
                    }
                    PlotEvent::PinsCleared => {
                        self.last_event = Some("PinsCleared".to_owned());
                    }
                    PlotEvent::KeyPressed { key, modifiers } => {
                        self.last_event =
                           Some(format!("KeyPressed: {key:?} mods={modifiers:?}"));
                    }
                    PlotEvent::KeyReleased { key, .. } => {
                          self.last_event = Some(format!("KeyReleased: {key:?}"));
                    }
                    _ => {}
                }
            }
        });
    }
}
