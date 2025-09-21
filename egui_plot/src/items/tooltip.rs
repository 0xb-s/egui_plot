use egui::{Color32, Grid, PopupAnchor, RichText, Shape, Tooltip, Ui};

use crate::{Cursor, PlotConfig, PlotPoint, PlotTransform};

/// A point that the tooltip refers to (and that we mark on the plot).
#[derive(Debug, Clone, Copy)]
pub struct ToolTipAnchor {
    /// Value in plot coordinates.
    pub value: PlotPoint,
    /// Marker fill color.
    pub color: Color32,
    /// Marker radius in points.
    pub radius: f32,
}

/// Draw small round markers for tooltip anchors 
pub(crate) fn draw_tooltip_anchors(
    shapes: &mut Vec<Shape>,
    transform: &PlotTransform,
    anchors: &[ToolTipAnchor],
    ui: &Ui,
) {
    let outline = ui.visuals().noninteractive().fg_stroke;
    for a in anchors {
        let p = transform.position_from_point(&a.value);
        shapes.push(Shape::circle_filled(p, a.radius, a.color));
        shapes.push(Shape::circle_stroke(p, a.radius, outline));
    }
}

/// Show rulers + a custom multi-anchor tooltip UI.
/// - Vertical ruler at X (if enabled)
/// - Horizontal rulers at each anchor Y (if enabled)
/// - Tooltip content rendered as a compact table: anchor (color), y, Δ from mean, width
pub(crate) fn rulers_and_tooltip_for_anchors(
    plot_area_response: &egui::Response,
    name: &str,
    anchors: &[ToolTipAnchor],
    plot: &PlotConfig<'_>,
    cursors: &mut Vec<Cursor>,
) {
    if anchors.is_empty() {
        return;
    }

    // rulers
    let x = anchors[0].value.x;
    if plot.show_x {
        cursors.push(Cursor::Vertical { x });
    }

    if plot.show_y {
        for a in anchors {
            cursors.push(Cursor::Horizontal { y: a.value.y });
        }
    }

    //tooltip
    let mut tooltip = Tooltip::always_open(
        plot_area_response.ctx.clone(),
        plot_area_response.layer_id,
        plot_area_response.id,
        PopupAnchor::Pointer,
    );

    let tooltip_width = plot_area_response.ctx.style().spacing.tooltip_width;
    tooltip.popup = tooltip.popup.width(tooltip_width);

    let scale = plot.transform.dvalue_dpos();
    let x_decimals = ((-scale[0].abs().log10()).ceil().max(0.0) as usize).clamp(1, 6);
    let y_decimals = ((-scale[1].abs().log10()).ceil().max(0.0) as usize).clamp(1, 6);

    tooltip.gap(12.0).show(|ui| {
        ui.set_max_width(tooltip_width);

        if !name.is_empty() {
            ui.strong(name);
            ui.add_space(4.0);
        }

        // X line
        ui.monospace(format!("x = {x:.x_decimals$}"));
        ui.add_space(4.0);

        // table of anchors
        Grid::new("egui_plot_tooltip_multi")
            .num_columns(4)
            .spacing([8.0, 2.0])
            .show(ui, |ui| {
                ui.weak(""); 
                ui.weak("anchor");
                ui.weak("y");
                ui.weak("Δ from mean");
                ui.end_row();
            });

        // Mean: 3 anchors → center is mean; 2 anchors → average

        let mut mean_y: Option<f64> = None;
        if anchors.len() == 3 {
            mean_y = Some(anchors[1].value.y); // [min, mean, max]
        } else if anchors.len() == 2 {
            mean_y = Some(0.5 * (anchors[0].value.y + anchors[1].value.y));
        }

        for (idx, a) in anchors.iter().enumerate() {
       

            ui.label(RichText::new("●").color(a.color));

            // Name (convention: [min, mean, max])
            let anchor_name = match (anchors.len(), idx) {
                (3, 1) => "mean",
                (3, 2) | (2, 1) => "max",
                (2 | 3, 0) => "min",

                _ => "y",
            };

            ui.monospace(anchor_name);
            // Y value
            ui.monospace(format!("{:.*}", y_decimals, a.value.y));

            ui.monospace(format!("{:.*}", y_decimals, a.value.y));

            // Δ from mean
            if let Some(my) = mean_y {
                let d = a.value.y - my;
                ui.monospace(format!("{d:+.y_decimals$}"));
            } else {
                ui.monospace("-");
            }

            ui.end_row();
        }
        if anchors.len() >= 2 {
            let (lo, hi) = if anchors[0].value.y <= anchors[anchors.len() - 1].value.y {
                (anchors[0].value.y, anchors[anchors.len() - 1].value.y)
            } else {
                (anchors[anchors.len() - 1].value.y, anchors[0].value.y)
            };

            ui.end_row();
            ui.weak("");
            ui.weak("width");
            ui.monospace(format!("{:.*}", y_decimals, hi - lo));
            ui.monospace("-");
            ui.end_row();
        }
    });
}
