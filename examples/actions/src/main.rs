use eframe::{App, Frame, egui};
use egui::{Align2, Color32};
use egui_plot::{Line, Plot, PlotEvent, TooltipOptions};

const TWO_PI: f64 = std::f64::consts::TAU;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "recompute demo • t ∈ [0, 2π], n=10",
        eframe::NativeOptions::default(),
        Box::new(|_| Ok(Box::new(Demo::new()))),
    )
}

struct Demo {
    n: usize,
    xs: Vec<f64>,
    f1: Vec<f64>,
    f2: Vec<f64>,
    last_event: String,
}

impl Demo {
    #[inline]
    fn f1_of(theta: f64) -> f64 {
        theta.sin()
    }

    #[inline]
    fn f2_of(theta: f64) -> f64 {
        (theta * 0.6 + 0.8).sin() * 0.8 + 0.2
    }

    fn new() -> Self {
        let n = 10;
        let xs: Vec<f64> = (0..n).map(|i| (i as f64) * TWO_PI / (n as f64)).collect();
        let f1: Vec<f64> = xs.iter().copied().map(Self::f1_of).collect();
        let f2: Vec<f64> = xs.iter().copied().map(Self::f2_of).collect();
        Self {
            n,
            xs,
            f1,
            f2,
            last_event: String::new(),
        }
    }
}

impl App for Demo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {


            let xs = self.xs.clone();
            let f1 = self.f1.clone();
            let f2 = self.f2.clone();

            let (_resp, events) = Plot::new("plot")
                .allow_double_click_reset(true)
                .show_x(true)
                .show_y(true)
                .show_actions(ui, |plot_ui| {
                    plot_ui.line(
                        Line::new_xy("f1(t)", xs.as_slice(), f1.as_slice())
                            .color(Color32::from_rgb(200, 100, 100))
                            .width(2.0),
                    );
                    plot_ui.line(
                        Line::new_xy("f2(t)", xs.as_slice(), f2.as_slice())
                            .color(Color32::from_rgb(100, 160, 240))
                            .width(2.0),
                    );
                    plot_ui.show_tooltip_with_options(&TooltipOptions::default());
                });


            for ev in &events {
                match ev {
                    PlotEvent::BoundsChanged { old, new, cause } => {
                        self.last_event = format!(
                            "BoundsChanged cause={:?}\nold: x=[{:.3},{:.3}] y=[{:.3},{:.3}]\nnew: x=[{:.3},{:.3}] y=[{:.3},{:.3}]",
                            cause,
                            old.min()[0], old.max()[0], old.min()[1], old.max()[1],
                            new.min()[0], new.max()[0], new.min()[1], new.max()[1],
                        );


                        let x_min = new.min()[0];
                        let x_max = new.max()[0];

                        let t_min = x_min / TWO_PI;
                        let t_max = x_max / TWO_PI;
                        let delta = t_max - t_min;

                        let n = self.n;

                        for i in 0..n {
                            let t = (i as f64) / (n as f64) * delta + t_min;
                            let theta = TWO_PI * t;
                            self.xs[i] = theta;
                            self.f1[i] = Self::f1_of(theta);
                            self.f2[i] = Self::f2_of(theta);
                        }
                        ctx.request_repaint();
                    }
                    #[allow(deprecated)]
                    PlotEvent::Hover { pos } => {

                        self.last_event = format!("Hover: x≈{:.3}, y≈{:.3}", pos.x, pos.y);
                    }
                    PlotEvent::BoxZoomStarted { .. } => {
                        self.last_event = "BoxZoomStarted".into();
                    }
                    PlotEvent::BoxZoomFinished { new_x, new_y, .. } => {
                        self.last_event = format!(
                            "BoxZoomFinished: X [{:.3},{:.3}], Y [{:.3},{:.3}]",
                            new_x.start(), new_x.end(), new_y.start(), new_y.end()
                        );
                    }
                    PlotEvent::Activate { hovered_item } => {
                        self.last_event = format!("Activate on {hovered_item:?}");
                    }
                    PlotEvent::ContextMenuRequested { screen_pos, item } => {
                        self.last_event = format!(
                            "ContextMenu at ({:.1},{:.1}) on {:?}", screen_pos.x, screen_pos.y, item
                        );
                    }
                    PlotEvent::PinAdded { snapshot } => {
                        self.last_event = format!(
                            "PinAdded at x={:.6} ({} series)", snapshot.plot_x, snapshot.rows.len()
                        );
                    }
                    PlotEvent::PinRemoved { index } => {
                        self.last_event = format!("PinRemoved (index={index})");
                    }
                    PlotEvent::PinsCleared => {
                        self.last_event = "PinsCleared".to_owned();
                    }
                    PlotEvent::KeyPressed { key, modifiers } => {
                        self.last_event = format!("KeyPressed: {key:?} with {modifiers:?}");
                    }
                    PlotEvent::KeyReleased { key, modifiers } => {
                        self.last_event = format!("KeyReleased: {key:?} with {modifiers:?}");
                    }
                    _ => {}
                }
            }

            if !self.last_event.is_empty() {
                egui::Window::new("Events")
                    .anchor(Align2::RIGHT_BOTTOM, (-12.0, -12.0))
                    .collapsible(true)
                    .resizable(true)
                    .default_open(true)
                    .show(ctx, |ui| {
                        ui.monospace(&self.last_event);
                    });
            }
        });
    }
}
