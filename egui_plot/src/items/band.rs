//! Shaded band item: fills the area between  ``y_min(x) `` and  ``y_max(x) ``.
//!
//! visualize variance around a time series.
//!
//! # Example
//! ```
//! let x: Vec<f64> = (0..100).map(|i| i as f64 * 0.1).collect();
//! let y: Vec<f64> = x.iter().map(|&t| t.sin()).collect();
//! let var: f64 = 0.2;
//! let y_min: Vec<f64> = y.iter().map(|&v| v - var).collect();
//! let y_max: Vec<f64> = y.iter().map(|&v| v + var).collect();
//!
//! let band = Band::new("variance")
//!     .with_color(Color32::from_rgb(64, 160, 255)) // optional;
//!     .with_alpha(96)                               // 0..=255
//!     .with_series(&x, &y_min, &y_max);
//!
//! plot_ui.band(band);
//! ```

use std::ops::RangeInclusive;

use egui::{Color32, Mesh, Shape, Ui};

use super::{PlotGeometry, PlotItem, PlotItemBase, PlotPoint};
use crate::{PlotBounds, PlotTransform};

/// Default alpha used for the fill color (0..=255).
const DEFAULT_ALPHA: u8 = 96;

/// A shaded area between two curves  ``y_min(x) `` and  ``y_max(x) ``.
#[derive(Clone, Debug)]
pub struct Band {
    ///  plot-item metadata (name, id, highlight, hover).
    base: PlotItemBase,

    /// Base color for the fill
    color: Color32,

    /// Alpha channel for the fill, 0..=255.
    alpha: u8,

    /// Sampled x-coordinates.
    xs: Vec<f64>,
    /// Lower envelope `` y_min(x) ``.
    y_min: Vec<f64>,
    /// Upper envelope  ``y_max(x) ``.
    y_max: Vec<f64>,
}

impl Band {
    /// Create an empty band with a legend/display `name`.
    ///
    /// Use [`Self::with_series`] to provide data and optionally [`Self::with_color`]
    /// / [`Self::with_alpha`] to style it.
    pub fn new(name: &str) -> Self {
        Self {
            base: PlotItemBase::new(name.into()),
            color: Color32::TRANSPARENT,
            alpha: DEFAULT_ALPHA,
            xs: Vec::new(),
            y_min: Vec::new(),
            y_max: Vec::new(),
        }
    }

    /// Override the item's stable id.
    #[inline]
    pub fn with_id(mut self, id: impl Into<egui::Id>) -> Self {
        self.base.id = id.into();
        self
    }

    /// Set the base RGB color of the band (alpha is taken from [`Self::with_alpha`]).
    #[inline]
    pub fn with_color(mut self, color: Color32) -> Self {
        self.color = color;
        self
    }

    /// Set the alpha component (0..=255) for the band fill.
    #[inline]
    pub fn with_alpha(mut self, alpha: u8) -> Self {
        self.alpha = alpha;
        self
    }

    /// Provide series data. All inputs must have identical length.
    ///
    /// NaN/non-finite samples are skipped segment-wise during tessellation.
    pub fn with_series(mut self, xs: &[f64], y_min: &[f64], y_max: &[f64]) -> Self {
        assert_eq!(
            xs.len(),
            y_min.len(),
            "Band: xs and y_min must have the same length"
        );
        assert_eq!(
            xs.len(),
            y_max.len(),
            "Band: xs and y_max must have the same length"
        );

        self.xs.clear();
        self.y_min.clear();
        self.y_max.clear();

        self.xs.extend_from_slice(xs);
        self.y_min.extend_from_slice(y_min);
        self.y_max.extend_from_slice(y_max);
        self
    }

    /// Compute data bounds for auto-scaling.
    fn compute_bounds(&self) -> Option<PlotBounds> {
        if self.xs.is_empty() {
            return None;
        }

        let mut min_x = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_y = f64::NEG_INFINITY;

        for i in 0..self.xs.len() {
            let x = self.xs[i];
            let yl = self.y_min[i];
            let yu = self.y_max[i];

            if !(x.is_finite() && yl.is_finite() && yu.is_finite()) {
                continue;
            }

            min_x = min_x.min(x);
            max_x = max_x.max(x);

            let lo = yl.min(yu);
            let hi = yl.max(yu);

            min_y = min_y.min(lo);
            max_y = max_y.max(hi);
        }

        if !(min_x.is_finite() && max_x.is_finite() && min_y.is_finite() && max_y.is_finite()) {
            return None;
        }

        Some(PlotBounds::from_min_max([min_x, min_y], [max_x, max_y]))
    }

    /// Build a filled triangle mesh for the band in screen space.
    fn build_mesh(&self, transform: &PlotTransform) -> Mesh {
        let mut mesh = Mesh::default();

      
        let base = self.color;
        let fill = Color32::from_rgba_unmultiplied(base.r(), base.g(), base.b(), self.alpha);

       
        for i in 0..self.xs.len().saturating_sub(1) {
            let x0 = self.xs[i];
            let x1 = self.xs[i + 1];
            let yl0 = self.y_min[i];
            let yl1 = self.y_min[i + 1];
            let yu0 = self.y_max[i];
            let yu1 = self.y_max[i + 1];

        
            if !(x0.is_finite()
                && x1.is_finite()
                && yl0.is_finite()
                && yl1.is_finite()
                && yu0.is_finite()
                && yu1.is_finite())
            {
                continue;
            }

         
            let (a0, b0) = if yl0 <= yu0 { (yl0, yu0) } else { (yu0, yl0) };
            let (a1, b1) = if yl1 <= yu1 { (yl1, yu1) } else { (yu1, yl1) };

       
            let p_ll = PlotPoint::new(x0, a0);
            let p_lr = PlotPoint::new(x1, a1);
            let p_ur = PlotPoint::new(x1, b1);
            let p_ul = PlotPoint::new(x0, b0);

    
            let ll = transform.position_from_point(&p_ll);
            let lr = transform.position_from_point(&p_lr);
            let ur = transform.position_from_point(&p_ur);
            let ul = transform.position_from_point(&p_ul);

          
            let i0 = mesh.vertices.len() as u32;
            mesh.colored_vertex(ll, fill);
            let i1 = mesh.vertices.len() as u32;
            mesh.colored_vertex(lr, fill);
            let i2 = mesh.vertices.len() as u32;
            mesh.colored_vertex(ur, fill);
            let i3 = mesh.vertices.len() as u32;
            mesh.colored_vertex(ul, fill);

            mesh.add_triangle(i0, i1, i2);
            mesh.add_triangle(i0, i2, i3);
        }

        mesh
    }
}

impl PlotItem for Band {
    fn shapes(&self, _ui: &Ui, transform: &PlotTransform, shapes: &mut Vec<Shape>) {
        if self.xs.len() < 2 {
            return;
        }
        let mesh = self.build_mesh(transform);
        if !mesh.indices.is_empty() {
            shapes.push(Shape::Mesh(std::sync::Arc::new(mesh)));
        }
    }

    fn initialize(&mut self, _x_range: RangeInclusive<f64>) {}

    fn color(&self) -> Color32 {
        self.color
    }

    fn geometry(&self) -> PlotGeometry<'_> {
        PlotGeometry::None
    }

    fn bounds(&self) -> PlotBounds {
        self.compute_bounds().unwrap_or(PlotBounds::NOTHING)
    }

    fn base(&self) -> &PlotItemBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut PlotItemBase {
        &mut self.base
    }
}
