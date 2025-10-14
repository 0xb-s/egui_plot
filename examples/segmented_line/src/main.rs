#![allow(rustdoc::missing_crate_level_docs)]
use eframe::{egui, egui::Color32};
use egui_plot::{Line, Plot};
use std::ops::Range;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "egui_plot • segmented lines",
        eframe::NativeOptions::default(),
        Box::new(|_| Ok(Box::new(Demo::default()))),
    )
}

struct Demo {
    xs: Vec<f64>,
    y_gap_sine: Vec<f64>,


    y_block_cos: Vec<f64>,
    blocks: Vec<Range<usize>>,
}

impl Default for Demo {
    fn default() -> Self {
        let n = 200usize;
        let xs: Vec<f64> = (0..n).map(|i| i as f64 * 0.05).collect();

 
        let mut y_gap_sine = vec![f64::NAN; n];
    
        for i in 10..50 {
            y_gap_sine[i] = (xs[i]).sin();
        }
        for i in 80..120 {
            y_gap_sine[i] = (xs[i]).sin();
        }
        for i in 150..190 {
            y_gap_sine[i] = (xs[i]).sin();
        }

   
        let mut y_block_cos = vec![0.0; n];
        for i in 0..n {
      
            y_block_cos[i] = (xs[i]).cos() - 1.5;
        }
    
        let blocks = vec![0..30, 60..90, 120..150, 180..200];

        Self {
            xs,
            y_gap_sine,
            y_block_cos,
            blocks,
        }
    }
}

impl eframe::App for Demo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Top (blue)");
            ui.label("Bottom (red)");

            Plot::new("segmented_demo")
                .legend(egui_plot::Legend::default())
                .show_x(true)
                .show_y(true)
                .show(ui, |plot_ui| {
            
                    plot_ui.line(
                        Line::new_xy("sine (gap-split)", &self.xs, &self.y_gap_sine)
                            .segment_on_gaps(true)
                            .color(Color32::from_rgb(70, 140, 255))
                            .width(2.0),
                    );

               
                    plot_ui.line(
                        Line::new_xy("cosine (explicit blocks)", &self.xs, &self.y_block_cos)
                            .with_blocks(&self.blocks)
                            .color(Color32::from_rgb(230, 80, 80))
                            .width(2.0),
                    );
                });
        });
    }
}
