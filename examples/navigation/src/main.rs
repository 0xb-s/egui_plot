#![allow(rustdoc::missing_crate_level_docs)]

use eframe::{App, Frame, egui};
use egui::{Color32, Context, Key, Modifiers, PointerButton, Vec2b};
use egui_plot::{
    Line, Plot, TooltipOptions, {BoxZoomConfig, NavigationConfig, ResetBehavior, ZoomConfig},
};

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Line::new_xy + custom navigation",
        eframe::NativeOptions::default(),
        Box::new(|_| Ok(Box::new(Demo::new()))),
    )
}

struct Demo {
    xs: Vec<f64>,
    f1: Vec<f64>,
    f2: Vec<f64>,
}

impl Demo {
    fn new() -> Self {
        let n = 500;
        let xs: Vec<f64> = (0..n).map(|i| i as f64 * 0.02).collect();
        let f1: Vec<f64> = xs.iter().map(|&t| t.sin()).collect();
        let f2: Vec<f64> = xs
            .iter()
            .map(|&t| (t * 0.6 + 0.8).sin() * 0.8 + 0.2)
            .collect();

        Self { xs, f1, f2 }
    }
}

impl App for Demo {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Line::new_xy + completely custom navigation");

            let nav = NavigationConfig::default()
                .drag(Vec2b::new(true, false), true)
                .scroll(Vec2b::new(true, false), true)
                .axis_zoom_drag(Vec2b::new(true, false))
                .zoom(
                    ZoomConfig::new(true, Vec2b::new(true, true))
                        .zoom_to_mouse(true)
                        .wheel_factor_exp(1.15),
                )
                .box_zoom(BoxZoomConfig::new(
                    true,
                    PointerButton::Secondary,
                    Modifiers {
                        shift: true,
                        ..Modifiers::NONE
                    },
                ))
                .reset_behavior(ResetBehavior::OriginalBounds)
                .double_click_reset(true)
                .shortcuts_fit_restore(Some(Key::F), Some(Key::R))
                .shortcuts_pin(Some(Key::D), Some(Key::U), Some(Key::Delete));

            Plot::new("demo_plot").navigation(nav).show(ui, |plot_ui| {
                plot_ui.line(
                    Line::new_xy("f1", &self.xs, &self.f1)
                        .color(Color32::from_rgb(200, 100, 100))
                        .width(2.0),
                );
                plot_ui.line(
                    Line::new_xy("f2", &self.xs, &self.f2)
                        .color(Color32::from_rgb(100, 160, 240))
                        .width(2.0),
                );

                plot_ui.show_tooltip_with_options(&TooltipOptions::default());
            });
        });
    }
}
