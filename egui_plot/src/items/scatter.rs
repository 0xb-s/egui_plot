//! scatter.rs – Zero-copy scatter plot API

use crate::{
    MarkerShape, PlotBounds, PlotPoint, PlotTransform,
    items::{
        ColumnarSeries, PlotGeometry, PlotItem, PlotItemBase,
        geom_helpers::{push_polygon_at, regular_ngon, star_ngon},
    },
};
use egui::{Color32, Pos2, Shape, Stroke, StrokeKind, Ui, Vec2, epaint::CircleShape, pos2, vec2};

/// Per-series uniform marker style (presentation only).
#[derive(Clone, Debug)]
pub struct Marker {
    pub shape: MarkerShape,
    pub filled: bool,
    pub radius: f32,
    pub stroke: Stroke,
    /// None = auto color from Plot palette.
    pub color: Option<Color32>,
}

impl Default for Marker {
    fn default() -> Self {
        Self {
            shape: MarkerShape::Circle,
            filled: true,
            radius: 2.5,
            stroke: Stroke::new(1.0, Color32::TRANSPARENT),
            color: None,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ScatterEncodings<'a> {
    pub per_point_colors: Option<&'a [Color32]>,
    pub per_point_radii: Option<&'a [f32]>,
}

pub struct Scatter<'a> {
    base: PlotItemBase,
    series: ColumnarSeries<'a>,
    marker: Marker,
    enc: ScatterEncodings<'a>,
    stems_y: Option<f32>,
}

impl<'a> Scatter<'a> {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            base: PlotItemBase::new(name.into()),
            series: ColumnarSeries::EMPTY,
            marker: Marker::default(),
            enc: ScatterEncodings::default(),
            stems_y: None,
        }
    }

    #[inline]
    pub fn from_series(name: impl Into<String>, series: ColumnarSeries<'a>) -> Self {
        Self::new(name).series(series)
    }

    #[inline]
    pub fn series(mut self, series: ColumnarSeries<'a>) -> Self {
        self.series = series;
        self
    }

    #[inline]
    pub fn marker(mut self, marker: Marker) -> Self {
        self.marker = marker;
        self
    }

    #[inline]
    pub fn marker_shape(mut self, s: MarkerShape) -> Self {
        self.marker.shape = s;
        self
    }
    #[inline]
    pub fn color(mut self, c: Color32) -> Self {
        self.marker.color = Some(c);
        self
    }
    #[inline]
    pub fn radius(mut self, r: f32) -> Self {
        self.marker.radius = r;
        self
    }
    #[inline]
    pub fn filled(mut self, yes: bool) -> Self {
        self.marker.filled = yes;
        self
    }
    #[inline]
    pub fn stroke(mut self, s: Stroke) -> Self {
        self.marker.stroke = s;
        self
    }

    #[inline]
    pub fn encodings(mut self, enc: ScatterEncodings<'a>) -> Self {
        self.enc = enc;
        self
    }

    #[inline]
    pub fn per_point_colors(mut self, colors: &'a [Color32]) -> Self {
        self.enc.per_point_colors = Some(colors);
        self
    }
    #[inline]
    pub fn per_point_radii(mut self, radii: &'a [f32]) -> Self {
        self.enc.per_point_radii = Some(radii);
        self
    }

    #[inline]
    pub fn stems(mut self, y_reference: f32) -> Self {
        self.stems_y = Some(y_reference);
        self
    }

    #[inline]
    fn resolve_color(&self, idx: usize, auto: Color32) -> Color32 {
        if let Some(colors) = self.enc.per_point_colors {
            if idx < colors.len() {
                return colors[idx];
            }
        }
        self.marker.color.unwrap_or(auto)
    }

    #[inline]
    fn resolve_radius(&self, idx: usize) -> f32 {
        if let Some(r) = self.enc.per_point_radii {
            if idx < r.len() {
                return r[idx];
            }
        }
        self.marker.radius
    }
}

impl PlotItem for Scatter<'_> {
    #[allow(clippy::too_many_lines)]
    fn shapes(&self, ui: &Ui, transform: &PlotTransform, out: &mut Vec<Shape>) {
        let n = self.series.len();
        if n == 0 {
            return;
        }

        let auto_color = self
            .marker
            .color
            .unwrap_or_else(|| ui.visuals().text_color());

        let stems_y_screen = self
            .stems_y
            .map(|y| transform.position_from_point(&PlotPoint::new(0.0, y)).y);

        for i in 0..n {
            let (x, y) = self.series.get(i).unwrap_or_default();
            let pos = transform.position_from_point(&PlotPoint::new(x, y));

            if let Some(y_screen) = stems_y_screen {
                out.push(Shape::line_segment(
                    [Pos2::new(pos.x, y_screen), pos],
                    self.marker.stroke,
                ));
            }

            let color = self.resolve_color(i, auto_color);
            let radius = self.resolve_radius(i);
            let stroke = self.marker.stroke;

            match self.marker.shape {
                MarkerShape::Circle => {
                    out.push(Shape::Circle(CircleShape {
                        center: pos,
                        radius,
                        fill: if self.marker.filled {
                            color
                        } else {
                            Color32::TRANSPARENT
                        },
                        stroke: if self.marker.filled {
                            stroke
                        } else {
                            Stroke::new(stroke.width, color)
                        },
                    }));
                }

                MarkerShape::Point => {
                    out.push(Shape::circle_filled(pos, (radius * 0.4).max(0.5), color));
                }
                MarkerShape::Pixel => {
                    let r = (radius * 0.25).max(0.5);
                    let rect = egui::Rect::from_center_size(pos, Vec2::splat(2.0 * r));
                    out.push(Shape::rect_filled(rect, 0.0, color));
                }
                MarkerShape::PlusFilled => {
                    let w = radius * 0.6;
                    let t = stroke.width.max(1.0).max(radius * 0.6);
                    let rect_h = egui::Rect::from_center_size(pos, Vec2::new(2.0 * w, t));
                    let rect_v = egui::Rect::from_center_size(pos, Vec2::new(t, 2.0 * w));
                    out.push(Shape::rect_filled(rect_h, 0.0, color));
                    out.push(Shape::rect_filled(rect_v, 0.0, color));
                }

                MarkerShape::XFilled => {
                    let r = radius * 0.9;
                    let w = stroke.width.max(1.0);
                    out.push(Shape::line_segment(
                        [pos + vec2(-r, -r), pos + vec2(r, r)],
                        Stroke::new(w, color),
                    ));
                    out.push(Shape::line_segment(
                        [pos + vec2(r, -r), pos + vec2(-r, r)],
                        Stroke::new(w, color),
                    ));
                }
                MarkerShape::RegularPolygon { n, angle_deg } => {
                    let angle_rad = (angle_deg as f32).to_radians();
                    let pts_local: Vec<egui::Vec2> =
                        regular_ngon(n.max(3) as usize, radius, angle_rad)
                            .into_iter()
                            .map(|p: egui::Pos2| p - egui::pos2(0.0, 0.0)) // Pos2 -> Vec2
                            .collect();
                    push_polygon_at(out, pos, pts_local, color, stroke, self.marker.filled);
                }
                MarkerShape::StarPolygon {
                    n,
                    inner_r_ppm,
                    angle_deg,
                } => {
                    let angle_rad = (angle_deg as f32).to_radians();
                    let inner_r = (inner_r_ppm as f32) / 1_000_000.0;
                    let pts = star_ngon(n.max(2) as usize, radius, radius * inner_r, angle_rad);

                    let path: Vec<egui::Pos2> =
                        pts.into_iter().map(|v| pos + v.to_vec2()).collect();
                    if self.marker.filled {
                        out.push(egui::Shape::closed_line(
                            path.clone(),
                            egui::Stroke::new(1.0, color),
                        ));
                    }
                    out.push(egui::Shape::closed_line(
                        path,
                        egui::Stroke::new(stroke.width, color),
                    ));
                }

                MarkerShape::Square => {
                    let r = radius / std::f32::consts::SQRT_2;
                    let rect = egui::Rect::from_center_size(pos, Vec2::splat(2.0 * r));
                    out.push(Shape::rect_filled(
                        rect,
                        0.0,
                        if self.marker.filled {
                            color
                        } else {
                            Color32::TRANSPARENT
                        },
                    ));
                    if !self.marker.filled {
                        out.push(Shape::rect_stroke(
                            rect,
                            0.0,
                            Stroke::new(stroke.width, color),
                            StrokeKind::Outside,
                        ));
                    }
                }
                MarkerShape::Diamond => {
                    let r = radius;
                    let pts = vec![
                        pos2(pos.x, pos.y - r),
                        pos2(pos.x - r, pos.y),
                        pos2(pos.x, pos.y + r),
                        pos2(pos.x + r, pos.y),
                    ];
                    out.push(Shape::convex_polygon(
                        pts.clone(),
                        if self.marker.filled {
                            color
                        } else {
                            Color32::TRANSPARENT
                        },
                        if self.marker.filled {
                            Stroke::NONE
                        } else {
                            Stroke::new(stroke.width, color)
                        },
                    ));
                }
                MarkerShape::Cross => {
                    let r = radius;
                    out.push(Shape::line_segment(
                        [pos2(pos.x - r, pos.y - r), pos2(pos.x + r, pos.y + r)],
                        Stroke::new(stroke.width, color),
                    ));
                    out.push(Shape::line_segment(
                        [pos2(pos.x + r, pos.y - r), pos2(pos.x - r, pos.y + r)],
                        Stroke::new(stroke.width, color),
                    ));
                }
                MarkerShape::Asterisk => {
                    let s3_2 = (3f32.sqrt() / 2.0) * radius;
                    let half = 0.5 * radius;
                    let st = Stroke::new(stroke.width.max(1.0), color);

                    out.push(Shape::line_segment(
                        [pos2(pos.x, pos.y - radius), pos2(pos.x, pos.y + radius)],
                        st,
                    ));

                    out.push(Shape::line_segment(
                        [
                            pos2(pos.x - s3_2, pos.y - half),
                            pos2(pos.x + s3_2, pos.y + half),
                        ],
                        st,
                    ));

                    out.push(Shape::line_segment(
                        [
                            pos2(pos.x - s3_2, pos.y + half),
                            pos2(pos.x + s3_2, pos.y - half),
                        ],
                        st,
                    ));
                }
                MarkerShape::Left => {
                    let s3 = 3f32.sqrt();
                    let pts = vec![
                        Vec2::new(-radius, 0.0),
                        Vec2::new(0.5 * radius, -0.5 * s3 * radius),
                        Vec2::new(0.5 * radius, 0.5 * s3 * radius),
                    ];
                    push_polygon_at(out, pos, pts, color, stroke, self.marker.filled);
                }
                MarkerShape::Down => {
                    let s3 = 3f32.sqrt();
                    let pts = vec![
                        Vec2::new(0.0, radius),
                        Vec2::new(-0.5 * s3 * radius, -0.5 * radius),
                        Vec2::new(0.5 * s3 * radius, -0.5 * radius),
                    ];
                    push_polygon_at(out, pos, pts, color, stroke, self.marker.filled);
                }
                MarkerShape::Up => {
                    let s3 = 3f32.sqrt();
                    let pts = vec![
                        Vec2::new(0.0, -radius),
                        Vec2::new(0.5 * s3 * radius, 0.5 * radius),
                        Vec2::new(-0.5 * s3 * radius, 0.5 * radius),
                    ];
                    push_polygon_at(out, pos, pts, color, stroke, self.marker.filled);
                }
                MarkerShape::Plus => {
                    let r = radius;
                    out.push(Shape::line_segment(
                        [pos2(pos.x - r, pos.y), pos2(pos.x + r, pos.y)],
                        Stroke::new(stroke.width, color),
                    ));
                    out.push(Shape::line_segment(
                        [pos2(pos.x, pos.y - r), pos2(pos.x, pos.y + r)],
                        Stroke::new(stroke.width, color),
                    ));
                }

                _ => {
                    // todo here
                    out.push(Shape::Circle(CircleShape {
                        center: pos,
                        radius,
                        fill: if self.marker.filled {
                            color
                        } else {
                            Color32::TRANSPARENT
                        },
                        stroke: if self.marker.filled {
                            stroke
                        } else {
                            Stroke::new(stroke.width, color)
                        },
                    }));
                }
            }
        }
    }

    fn initialize(&mut self, _x_range: std::ops::RangeInclusive<f64>) {}

    fn color(&self) -> Color32 {
        self.marker.color.unwrap_or(Color32::TRANSPARENT)
    }

    fn geometry(&self) -> PlotGeometry<'_> {
        PlotGeometry::PointsXY {
            xs: self.series.xs(),
            ys: self.series.ys(),
        }
    }

    fn bounds(&self) -> PlotBounds {
        self.series.bounds()
    }

    fn base(&self) -> &PlotItemBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut PlotItemBase {
        &mut self.base
    }
}
