use std::{fmt::Debug, ops::RangeInclusive, sync::Arc};

use egui::{
    Pos2, Rangef, Rect, Response, Sense, TextStyle, TextWrapMode, Ui, Vec2, WidgetText,
    emath::{Rot2, remap_clamp},
    epaint::TextShape,
};

use super::{GridMark, transform::PlotTransform};

// Gap between tick labels and axis label in units of the axis label height
const AXIS_LABEL_GAP: f32 = 0.25;

pub(super) type AxisFormatterFn<'a> = dyn Fn(GridMark, &RangeInclusive<f64>) -> String + 'a;

/// X or Y axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    /// Horizontal X-Axis
    X = 0,

    /// Vertical Y-axis
    Y = 1,
}

impl From<Axis> for usize {
    #[inline]
    fn from(value: Axis) -> Self {
        match value {
            Axis::X => 0,
            Axis::Y => 1,
        }
    }
}

/// Placement of the horizontal X-Axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VPlacement {
    Top,
    Bottom,
}

/// Placement of the vertical Y-Axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HPlacement {
    Left,
    Right,
}

/// Placement of an axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Placement {
    /// Bottom for X-axis, or left for Y-axis.
    LeftBottom,

    /// Top for x-axis and right for y-axis.
    RightTop,
}

impl From<HPlacement> for Placement {
    #[inline]
    fn from(placement: HPlacement) -> Self {
        match placement {
            HPlacement::Left => Self::LeftBottom,
            HPlacement::Right => Self::RightTop,
        }
    }
}

impl From<Placement> for HPlacement {
    #[inline]
    fn from(placement: Placement) -> Self {
        match placement {
            Placement::LeftBottom => Self::Left,
            Placement::RightTop => Self::Right,
        }
    }
}

impl From<VPlacement> for Placement {
    #[inline]
    fn from(placement: VPlacement) -> Self {
        match placement {
            VPlacement::Top => Self::RightTop,
            VPlacement::Bottom => Self::LeftBottom,
        }
    }
}

impl From<Placement> for VPlacement {
    #[inline]
    fn from(placement: Placement) -> Self {
        match placement {
            Placement::LeftBottom => Self::Bottom,
            Placement::RightTop => Self::Top,
        }
    }
}

/// Axis configuration.
///
/// Used to configure axis label and ticks.
#[derive(Clone)]
pub struct AxisHints<'a> {
    pub(super) label: WidgetText,
    pub(super) formatter: Arc<AxisFormatterFn<'a>>,
    pub(super) min_thickness: f32,
    pub(super) placement: Placement,
    pub(super) label_spacing: Rangef,
}

impl<'a> AxisHints<'a> {
    /// Initializes a default axis configuration for the X axis.
    pub fn new_x() -> Self {
        Self::new(Axis::X)
    }

    /// Initializes a default axis configuration for the Y axis.
    pub fn new_y() -> Self {
        Self::new(Axis::Y)
    }

    /// Initializes a default axis configuration for the specified axis.
    ///
    /// `label` is empty.
    /// `formatter` is default float to string formatter.
    pub fn new(axis: Axis) -> Self {
        Self {
            label: Default::default(),
            formatter: Arc::new(Self::default_formatter),
            min_thickness: 14.0,
            placement: Placement::LeftBottom,
            label_spacing: match axis {
                Axis::X => Rangef::new(60.0, 80.0), // labels can get pretty wide
                Axis::Y => Rangef::new(20.0, 30.0), // text isn't very high
            },
        }
    }

    /// Specify custom formatter for ticks.
    ///
    /// The first parameter of `formatter` is the raw tick value as `f64`.
    /// The second parameter of `formatter` is the currently shown range on this axis.
    pub fn formatter(
        mut self,
        fmt: impl Fn(GridMark, &RangeInclusive<f64>) -> String + 'a,
    ) -> Self {
        self.formatter = Arc::new(fmt);
        self
    }

    fn default_formatter(mark: GridMark, _range: &RangeInclusive<f64>) -> String {
        // Example: If the step to the next tick is `0.01`, we should use 2 decimals of precision:
        let num_decimals = -mark.step_size.log10().round() as usize;

        emath::format_with_decimals_in_range(mark.value, num_decimals..=num_decimals)
    }

    /// Specify axis label.
    ///
    /// The default is 'x' for x-axes and 'y' for y-axes.
    #[inline]
    pub fn label(mut self, label: impl Into<WidgetText>) -> Self {
        self.label = label.into();
        self
    }

    /// Specify minimum thickness of the axis
    #[inline]
    pub fn min_thickness(mut self, min_thickness: f32) -> Self {
        self.min_thickness = min_thickness;
        self
    }

    /// Specify maximum number of digits for ticks.
    #[inline]
    #[deprecated = "Use `min_thickness` instead"]
    pub fn max_digits(self, digits: usize) -> Self {
        self.min_thickness(12.0 * digits as f32)
    }

    /// Specify the placement of the axis.
    ///
    /// For X-axis, use [`VPlacement`].
    /// For Y-axis, use [`HPlacement`].
    #[inline]
    pub fn placement(mut self, placement: impl Into<Placement>) -> Self {
        self.placement = placement.into();
        self
    }

    /// Set the minimum spacing between labels
    ///
    /// When labels get closer together than the given minimum, then they become invisible.
    /// When they get further apart than the max, they are at full opacity.
    ///
    /// Labels can never be closer together than the [`crate::Plot::grid_spacing`] setting.
    #[inline]
    pub fn label_spacing(mut self, range: impl Into<Rangef>) -> Self {
        self.label_spacing = range.into();
        self
    }
}

#[derive(Clone)]
pub(super) struct AxisWidget<'a> {
    pub range: RangeInclusive<f64>,
    pub hints: AxisHints<'a>,

    /// The region where we draw the axis labels.
    pub rect: Rect,
    pub transform: Option<PlotTransform>,
    pub steps: Arc<Vec<GridMark>>,
}

impl<'a> AxisWidget<'a> {
    /// if `rect` has width or height == 0, it will be automatically calculated from ticks and text.
    pub fn new(hints: AxisHints<'a>, rect: Rect) -> Self {
        Self {
            range: (0.0..=0.0),
            hints,
            rect,
            transform: None,
            steps: Default::default(),
        }
    }

    /// Returns the actual thickness of the axis.
    pub fn ui(self, ui: &mut Ui, axis: Axis) -> (Response, f32) {
        let response = ui.allocate_rect(self.rect, Sense::hover());

        if !ui.is_rect_visible(response.rect) {
            return (response, 0.0);
        }

        let Some(ref transform) = self.transform else {
            return (response, 0.0);
        };
        let tick_labels_thickness = self.add_tick_labels(ui, transform, axis);

        if self.hints.label.is_empty() {
            return (response, tick_labels_thickness);
        }

        let galley = self.hints.label.into_galley(
            ui,
            Some(TextWrapMode::Extend),
            f32::INFINITY,
            TextStyle::Body,
        );

        let text_pos = match self.hints.placement {
            Placement::LeftBottom => match axis {
                Axis::X => {
                    let pos = response.rect.center_bottom();
                    Pos2 {
                        x: pos.x - galley.size().x * 0.5,
                        y: pos.y - galley.size().y * (1.0 + AXIS_LABEL_GAP),
                    }
                }
                Axis::Y => {
                    let pos = response.rect.left_center();
                    Pos2 {
                        x: pos.x - galley.size().y * AXIS_LABEL_GAP,
                        y: pos.y + galley.size().x * 0.5,
                    }
                }
            },
            Placement::RightTop => match axis {
                Axis::X => {
                    let pos = response.rect.center_top();
                    Pos2 {
                        x: pos.x - galley.size().x * 0.5,
                        y: pos.y + galley.size().y * AXIS_LABEL_GAP,
                    }
                }
                Axis::Y => {
                    let pos = response.rect.right_center();
                    Pos2 {
                        x: pos.x - galley.size().y * (1.0 - AXIS_LABEL_GAP),
                        y: pos.y + galley.size().x * 0.5,
                    }
                }
            },
        };
        let axis_label_thickness = galley.size().y * (1.0 + AXIS_LABEL_GAP);
        let angle = match axis {
            Axis::X => 0.0,
            Axis::Y => -std::f32::consts::FRAC_PI_2,
        };

        ui.painter()
            .add(TextShape::new(text_pos, galley, ui.visuals().text_color()).with_angle(angle));

        (response, tick_labels_thickness + axis_label_thickness)
    }

    /// Add tick labels to the axis. Returns the thickness of the axis.
    fn add_tick_labels(&self, ui: &Ui, transform: &PlotTransform, axis: Axis) -> f32 {
        let font_id = TextStyle::Body.resolve(ui.style());
        let label_spacing = self.hints.label_spacing;
        let mut thickness: f32 = 0.0;

        const SIDE_MARGIN: f32 = 4.0; // Add some margin to both sides of the text on the Y axis.
        let painter = ui.painter();
        // Add tick labels:
        if axis == Axis::X {
            if let Some(bx) = transform.segment_xaxis() {
                let text_color = ui.visuals().text_color();

                let step_hint = estimate_step_hint_data_units(transform);

                let raw_ticks = compute_segmented_x_ticks(transform, bx, step_hint);

                const CLUSTER_PX_THRESHOLD: f32 = 6.0;
                let clusters = cluster_overlapping_ticks(raw_ticks, CLUSTER_PX_THRESHOLD);

                let mut last_drawn_center_x: Option<f32> = None;
                let mut thickness: f32 = 0.0;

                for cluster in clusters {
                    if !cluster.has_edge {
                        if let Some(prev_cx) = last_drawn_center_x {
                            if (cluster.screen_x - prev_cx).abs() < self.hints.label_spacing.min {
                                continue;
                            }
                        }
                    }

                    let mut inner = cluster.ticks.clone();
                    inner.sort_by(|a, b| {
                        a.world_x
                            .partial_cmp(&b.world_x)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    });

                    let to_draw: Vec<(ScreenTick, TickSide)> = if inner.len() == 1 {
                        vec![(inner[0], TickSide::Center)]
                    } else {
                        vec![
                            (inner.first().copied().expect(""), TickSide::Left),
                            (inner.last().copied().expect(""), TickSide::Right),
                        ]
                    };

                    for (tick, side) in to_draw {
                        let gm = GridMark {
                            value: tick.world_x,
                            step_size: step_hint,
                        };
                        let txt = (self.hints.formatter)(gm, &self.range);
                        if txt.is_empty() {
                            continue;
                        }

                        let galley = painter.layout_no_wrap(txt, font_id.clone(), text_color);
                        let galley_size = galley.size();

                        let y = match VPlacement::from(self.hints.placement) {
                            VPlacement::Bottom => self.rect.min.y,
                            VPlacement::Top => self.rect.max.y - galley_size.y,
                        };

                        let label_pos_x = match side {
                            TickSide::Center => tick.screen_x - galley_size.x * 0.5,
                            TickSide::Left => tick.screen_x - galley_size.x - 2.0,
                            TickSide::Right => tick.screen_x + 2.0,
                        };

                        let label_pos = Pos2::new(label_pos_x, y);

                        if label_pos.x + galley_size.x < self.rect.min.x {
                            continue;
                        }
                        if label_pos.x > self.rect.max.x {
                            continue;
                        }

                        painter.add(TextShape::new(label_pos, galley, text_color));

                        thickness = thickness.max(galley_size.y);
                    }

                    last_drawn_center_x = Some(cluster.screen_x);
                }

                return thickness;
            }
        }

        for step in self.steps.iter() {
            let text = (self.hints.formatter)(*step, &self.range);
            if !text.is_empty() {
                let spacing_in_points =
                    (transform.dpos_dvalue()[usize::from(axis)] * step.step_size).abs() as f32;

                if spacing_in_points <= label_spacing.min {
                    // Labels are too close together - don't paint them.
                    continue;
                }

                // Fade in labels as they get further apart:
                let strength = remap_clamp(spacing_in_points, label_spacing, 0.0..=1.0);

                let text_color = super::color_from_strength(ui, strength);
                let galley = painter.layout_no_wrap(text, font_id.clone(), text_color);
                let galley_size = match axis {
                    Axis::X => galley.size(),
                    Axis::Y => galley.size() + 2.0 * SIDE_MARGIN * Vec2::X,
                };

                if spacing_in_points < galley_size[axis as usize] {
                    continue; // the galley won't fit (likely too wide on the X axis).
                }

                match axis {
                    Axis::X => {
                        thickness = thickness.max(galley_size.y);

                        let projected_point = super::PlotPoint::new(step.value, 0.0);
                        let center_x = transform.position_from_point(&projected_point).x;
                        let y = match VPlacement::from(self.hints.placement) {
                            VPlacement::Bottom => self.rect.min.y,
                            VPlacement::Top => self.rect.max.y - galley_size.y,
                        };
                        let pos = Pos2::new(center_x - galley_size.x / 2.0, y);
                        painter.add(TextShape::new(pos, galley, text_color));
                    }
                    Axis::Y => {
                        thickness = thickness.max(galley_size.x);

                        let projected_point = super::PlotPoint::new(0.0, step.value);
                        let center_y = transform.position_from_point(&projected_point).y;

                        match HPlacement::from(self.hints.placement) {
                            HPlacement::Left => {
                                let angle = 0.0; // TODO(emilk): allow users to rotate text

                                if angle == 0.0 {
                                    let x = self.rect.max.x - galley_size.x + SIDE_MARGIN;
                                    let pos = Pos2::new(x, center_y - galley_size.y / 2.0);
                                    painter.add(TextShape::new(pos, galley, text_color));
                                } else {
                                    let right =
                                        Pos2::new(self.rect.max.x, center_y - galley_size.y / 2.0);
                                    let width = galley_size.x;
                                    let left =
                                        right - Rot2::from_angle(angle) * Vec2::new(width, 0.0);

                                    painter.add(
                                        TextShape::new(left, galley, text_color).with_angle(angle),
                                    );
                                }
                            }
                            HPlacement::Right => {
                                let x = self.rect.min.x + SIDE_MARGIN;
                                let pos = Pos2::new(x, center_y - galley_size.y / 2.0);
                                painter.add(TextShape::new(pos, galley, text_color));
                            }
                        };
                    }
                };
            }
        }

        thickness
    }
}
fn estimate_step_hint_data_units(transform: &PlotTransform) -> f64 {
    let desired_px_spacing: f32 = 80.0;

    let units_per_px = transform.dvalue_dpos()[0] as f32;
    (units_per_px.abs() * desired_px_spacing) as f64
}
#[derive(Clone, Copy, Debug)]
struct ScreenTick {
    world_x: f64,
    screen_x: f32,
    is_segment_edge: bool,
}

fn compute_segmented_x_ticks(
    tf: &PlotTransform,
    bx: &crate::SegmentedAxis,
    step_hint: f64,
) -> Vec<ScreenTick> {
    let per_seg_ticks = bx.segment_ticks(step_hint);

    let mut out = Vec::new();

    for (seg_idx, ticks_for_seg) in per_seg_ticks.iter().enumerate() {
        let seg = &bx.segments[seg_idx];

        for &world_x in ticks_for_seg {
            if !world_x.is_finite() {
                continue;
            }

            let screen_x = tf.position_from_point_x(world_x);

            if !screen_x.is_finite() {
                continue;
            }

            out.push(ScreenTick {
                world_x,
                screen_x,
                is_segment_edge: (world_x == seg.start) || (world_x == seg.end),
            });
        }
    }

    out.sort_by(|a, b| {
        a.screen_x
            .partial_cmp(&b.screen_x)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    out
}
#[derive(Clone)]
struct TickCluster {
    pub screen_x: f32,
    pub ticks: Vec<ScreenTick>,
    pub has_edge: bool,
}

fn cluster_overlapping_ticks(
    mut ticks: Vec<ScreenTick>,
    px_merge_threshold: f32,
) -> Vec<TickCluster> {
    ticks.sort_by(|a, b| {
        a.screen_x
            .partial_cmp(&b.screen_x)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut clusters: Vec<TickCluster> = Vec::new();
    let mut i = 0;

    while i < ticks.len() {
        let base_x = ticks[i].screen_x;

        let mut group: Vec<ScreenTick> = vec![ticks[i]];
        i += 1;

        while i < ticks.len() && (ticks[i].screen_x - base_x).abs() < px_merge_threshold {
            group.push(ticks[i]);
            i += 1;
        }

        let has_edge = group.iter().any(|t| t.is_segment_edge);

        clusters.push(TickCluster {
            screen_x: base_x,
            ticks: group,
            has_edge,
        });
    }

    clusters
}

#[derive(Clone, Copy, Debug)]
enum TickSide {
    Left,
    Right,
    Center,
}
