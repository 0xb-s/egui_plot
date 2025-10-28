use egui::{Color32, Pos2, Shape, Stroke, Vec2};
use std::f32::consts::PI;

#[inline]
pub fn regular_ngon(n: usize, r: f32, angle_rad: f32) -> Vec<Pos2> {
    let n = n.max(3);
    (0..n)
        .map(|k| {
            let a = angle_rad + 2.0 * PI * (k as f32) / (n as f32);
            Pos2::new(a.cos() * r, a.sin() * r)
        })
        .collect()
}

#[inline]
pub fn star_ngon(n: usize, r_outer: f32, r_inner: f32, angle_rad: f32) -> Vec<Pos2> {
    let n = n.max(2);
    let mut pts = Vec::with_capacity(n * 2);
    for k in 0..n {
        let a_outer = angle_rad + 2.0 * PI * (k as f32) / (n as f32);
        let a_inner = a_outer + PI / (n as f32);
        pts.push(Pos2::new(a_outer.cos() * r_outer, a_outer.sin() * r_outer));
        pts.push(Pos2::new(a_inner.cos() * r_inner, a_inner.sin() * r_inner));
    }
    pts
}

pub fn push_polygon_at(
    out: &mut Vec<Shape>,
    center: Pos2,
    local_pts: Vec<Vec2>,
    color: Color32,
    stroke: Stroke,
    filled: bool,
) {
    let pts: Vec<Pos2> = local_pts.into_iter().map(|v| center + v).collect();
    if filled {
        out.push(Shape::convex_polygon(pts, color, Stroke::NONE));
    } else {
        out.push(Shape::closed_line(pts, Stroke::new(stroke.width, color)));
    }
}
