use std::{collections::VecDeque, ops::RangeInclusive};

use egui::{Id, Key, Modifiers, PointerButton, Pos2, Shape, Vec2, Vec2b};

use crate::{PlotPoint, transform::PlotBounds};

/// Describes what caused the plot’s bounds or transform to change during this frame.
///
/// This single enum is used for all change types (like zooming or panning).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoundsChangeCause {
    /// Code requested a change via input actions (`SetBounds`*/Translate/Zoom).
    Programmatic,
    /// User panned.
    Pan,
    /// User used wheel/touch to zoom.
    Zoom,
    /// User dragged on an axis area to zoom that axis.
    AxisZoomX,
    /// User dragged on an axis area to zoom that axis.
    AxisZoomY,
    /// User performed boxed zoom (drag rectangle to zoom).
    BoxZoom,
    /// Double-click reset to defaults or explicit reset.
    Reset,
    /// Auto-fit to content ran (because auto-bounds was enabled).
    AutoFit,
    /// This plot synced from a linked group.
    LinkSync,
}

/// Optional input telemetry attached to events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct InputInfo {
    /// Latest screen pointer position when the event was generated.
    pub pointer: Option<Pos2>,
    /// Mouse button involved (if any).
    pub button: Option<PointerButton>,
    /// Keyboard modifiers snapshot.
    pub modifiers: Modifiers,
}

/// Public identifier type used in item-related events (hover/click/legend).
pub type PlotItemId = Id;

/// Lightweight snapshot for a "pin".
#[derive(Debug, Clone)]
pub struct PinSnapshot {
    pub plot_x: f64,
    pub rows: Vec<PinRow>,
}

/// One row of a pin snapshot (series/value/color).
#[derive(Debug, Clone)]
pub struct PinRow {
    pub series_name: String,
    pub x: f64,
    pub y: f64,
    pub color_rgba: [u8; 4],
}

/// Adapter trait: executor mutates your bounds type without depending on its API.
///
/// An impl for `crate::transform::PlotBounds` is provided below.
pub trait BoundsLike: Clone {
    /// Replace the X-range with `range` (inclusive).
    fn set_x_range(&mut self, range: RangeInclusive<f64>);
    /// Replace the Y-range with `range` (inclusive).
    fn set_y_range(&mut self, range: RangeInclusive<f64>);
    /// Translate bounds by `(dx, dy)` in plot-space units.
    fn translate(&mut self, dx: f64, dy: f64);
    /// Zoom bounds around `center` by factors per axis.
    ///
    /// Interpretation: visible extent is divided by `factor` (factor>1.0 zooms in).
    fn zoom(&mut self, factor: Vec2, center: PlotPoint);
}

/// Output events produced by the widget during the render/interaction phase.
///
/// These are *non-mutating*; they describe user intent and frame results.
/// Emit them as they happen and also add a single `BoundsChanged` at the end
/// of the frame if the bounds differ from the frame start.
/// # Example
/// ```rs
/// let (_resp, events) = Plot::new("demo")
///     .show_actions(ui, |p| {
///         p.line(Line::new_xy("sin", xs.as_slice(), ys.as_slice());
///     });
///
/// for ev in &events {
///     if let PlotEvent::BoundsChanged { new, .. } = ev {
///
///     }
/// }
/// ```
#[derive(Debug, Clone)]
pub enum PlotEvent {
    /// keyboard
    KeyPressed {
        key: Key,
        modifiers: Modifiers,
    },
    /// keyboard
    KeyReleased {
        key: Key,
        modifiers: Modifiers,
    },

    ///UI
    Activate {
        hovered_item: Option<PlotItemId>,
    },

    /// Cursor
    Hover {
        pos: PlotPoint,
    },

    /// Menu
    ContextMenuRequested {
        screen_pos: Pos2,
        item: Option<PlotItemId>,
    },

    // frame summaries
    BoundsChanged {
        old: PlotBounds,
        new: PlotBounds,
        cause: BoundsChangeCause,
    },

    /// Transform was updated explicitly
    TransformChanged {
        old: PlotBounds,
        new: PlotBounds,
        cause: BoundsChangeCause,
    },

    /// Auto-fit was applied with the new resulting bounds.
    AutoFitApplied {
        new: PlotBounds,
    },

    /// Reset to defaults took place this frame.
    ResetApplied {
        input: InputInfo,
    },

    //  deltas
    PanStarted {
        input: InputInfo,
    },
    PanDelta {
        delta_plot_x: f64,
        delta_plot_y: f64,
        input: InputInfo,
    },
    PanFinished {
        input: InputInfo,
    },

    ZoomStarted {
        input: InputInfo,
    },
    ZoomDelta {
        factor_x: f32,
        factor_y: f32,
        center_plot_x: f64,
        center_plot_y: f64,
        input: InputInfo,
    },
    ZoomFinished {
        input: InputInfo,
    },

    AxisZoomDragStarted {
        axis_x: bool,
        axis_y: bool,
        input: InputInfo,
    },
    AxisZoomDragDelta {
        factor_x: f32,
        factor_y: f32,
        input: InputInfo,
    },
    AxisZoomDragFinished {
        input: InputInfo,
    },

    BoxZoomStarted {
        screen_start: Pos2,
        input: InputInfo,
    },
    BoxZoomFinished {
        new_x: RangeInclusive<f64>,
        new_y: RangeInclusive<f64>,
        input: InputInfo,
    },

    // Items / Legend
    CursorMoved {
        plot_x: f64,
        plot_y: f64,
    },

    ItemHovered {
        item: PlotItemId,
        pos: PlotPoint,
    },

    ItemClicked {
        item: PlotItemId,
        pos: PlotPoint,
        button: PointerButton,
        input: InputInfo,
    },

    LegendItemToggled {
        item: PlotItemId,
        now_visible: bool,
    },

    // Pins
    PinAdded {
        snapshot: PinSnapshot,
    },
    PinRemoved {
        index: usize,
    },
    PinsCleared,
}

/// Input actions recorded during the build phase (`PlotUi`).
///
/// `I` is your item type (e.g., `Box<dyn PlotItem>`).
#[derive(Debug)]
pub enum PlotAction<I> {
    /// Add a renderable item. Your renderer decides how to draw it.
    AddItem(I),

    /// Set the X bounds (inclusive). Disables auto-bounds on X.
    SetBoundsX(RangeInclusive<f64>),

    /// Set the Y bounds (inclusive). Disables auto-bounds on Y.
    SetBoundsY(RangeInclusive<f64>),

    /// Translate bounds by a plot-space delta `(dx, dy)`. Disables auto-bounds.
    Translate(Vec2),

    /// Set auto-bounds per axis (`true` enables auto).
    SetAutoBounds(Vec2b),

    /// Zoom by a per-axis factor around a plot-space `center`. Disables auto-bounds.
    Zoom(Vec2, PlotPoint),

    // ------------------------ Decorations / overlays --------------------------
    /// Add an overlay `Shape` to be painted after items.
    AddOverlayShape(Shape),
}
impl<I> PlotAction<I> {
    pub fn into_event(self) -> Option<PlotEvent> {
        match self {
            Self::SetBoundsX(range) => Some(PlotEvent::BoundsChanged {
                old: PlotBounds::NOTHING,
                new: PlotBounds {
                    min: [*range.start(), f64::NEG_INFINITY],
                    max: [*range.end(), f64::INFINITY],
                },
                cause: BoundsChangeCause::Programmatic,
            }),
            Self::SetBoundsY(range) => Some(PlotEvent::BoundsChanged {
                old: PlotBounds::NOTHING,
                new: PlotBounds {
                    min: [f64::NEG_INFINITY, *range.start()],
                    max: [f64::INFINITY, *range.end()],
                },
                cause: BoundsChangeCause::Programmatic,
            }),
            Self::Translate(_)
            | Self::Zoom(_, _)
            | Self::SetAutoBounds(_)
            | Self::AddOverlayShape(_)
            | Self::AddItem(_) => None,
        }
    }
}

#[derive(Debug)]
pub struct ActionQueue<I> {
    actions: VecDeque<PlotAction<I>>,
}

impl<I> Default for ActionQueue<I> {
    #[inline]
    fn default() -> Self {
        Self {
            actions: VecDeque::new(),
        }
    }
}

impl<I> ActionQueue<I> {
    /// Create an empty queue.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Push a single action (to the back; FIFO).
    #[inline]
    pub fn push(&mut self, action: PlotAction<I>) {
        self.actions.push_back(action);
    }

    /// Extend with a batch of actions (preserves order).
    #[inline]
    pub fn extend<T: IntoIterator<Item = PlotAction<I>>>(&mut self, iter: T) {
        self.actions.extend(iter);
    }

    /// Number of queued actions.
    #[inline]
    pub fn len(&self) -> usize {
        self.actions.len()
    }

    /// Whether the queue is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.actions.is_empty()
    }

    /// Drain the internal action list by value.
    #[inline]
    pub fn drain(self) -> Vec<PlotAction<I>> {
        Vec::from(self.actions) // moves out in order
    }

    #[inline]
    pub fn add_item(&mut self, item: I) {
        self.push(PlotAction::AddItem(item));
    }

    #[inline]
    pub fn set_bounds_x(&mut self, r: RangeInclusive<f64>) {
        self.push(PlotAction::SetBoundsX(r));
    }

    #[inline]
    pub fn set_bounds_y(&mut self, r: RangeInclusive<f64>) {
        self.push(PlotAction::SetBoundsY(r));
    }

    #[inline]
    pub fn translate(&mut self, delta: egui::Vec2) {
        self.push(PlotAction::Translate(delta));
    }

    #[inline]
    pub fn set_auto_bounds(&mut self, auto_bounds: egui::Vec2b) {
        self.push(PlotAction::SetAutoBounds(auto_bounds));
    }

    /// Iterator over actions (not items directly).
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &PlotAction<I>> {
        self.actions.iter()
    }

    /// Iterate over **items** that were added.
    #[inline]
    pub fn iter_items(&self) -> impl Iterator<Item = &I> {
        self.actions.iter().filter_map(|act| {
            if let PlotAction::AddItem(item) = act {
                Some(item)
            } else {
                None
            }
        })
    }

    #[inline]
    pub fn iter_items_mut(&mut self) -> impl Iterator<Item = &mut I> {
        self.actions.iter_mut().filter_map(|act| {
            if let PlotAction::AddItem(item) = act {
                Some(item)
            } else {
                None
            }
        })
    }

    #[inline]
    pub fn zoom(&mut self, zoom_factor: egui::Vec2, center: PlotPoint) {
        self.push(PlotAction::Zoom(zoom_factor, center));
    }
}

/// Result of applying a queue of actions in a given state.
///
/// - `items`: items to render
/// - `auto_bounds`: final auto-bounds flags
/// - `bounds`: final mutated bounds
/// - `overlays`: overlay shapes to paint last
/// - `events`: empty Vec; fill during interaction rendering
#[derive(Debug)]
pub struct AppliedActions<I, B> {
    pub items: Vec<I>,
    pub auto_bounds: Vec2b,
    pub bounds: B,
    pub overlays: Vec<Shape>,
    pub events: Vec<PlotEvent>,
}

impl<I, B> AppliedActions<I, B> {
    pub fn take_events(&mut self) -> Vec<PlotEvent> {
        std::mem::take(&mut self.events)
    }
    /// Access the events buffer to push interaction outputs.
    #[inline]
    pub fn events_mut(&mut self) -> &mut Vec<PlotEvent> {
        &mut self.events
    }
}

/// Deterministic executor: applies input actions in FIFO order.
///
/// Order inside a single frame:
/// 1) Bounds-affecting actions: `SetBounds*`, `Translate`, `SetAutoBounds`, `Zoom`
/// 2) Data actions: `AddItem`
/// 3) Decorations: `AddOverlayShape`
///
/// Auto-fitting to content is **not** performed here.
pub struct ActionExecutor;

impl BoundsLike for PlotBounds {
    #[inline]
    fn set_x_range(&mut self, range: RangeInclusive<f64>) {
        self.min[0] = *range.start();
        self.max[0] = *range.end();
    }

    #[inline]
    fn set_y_range(&mut self, range: RangeInclusive<f64>) {
        self.min[1] = *range.start();
        self.max[1] = *range.end();
    }

    #[inline]
    fn translate(&mut self, dx: f64, dy: f64) {
        Self::translate(self, (dx, dy));
    }

    #[inline]
    fn zoom(&mut self, factor: Vec2, center: PlotPoint) {
        Self::zoom(self, factor, center);
    }
}
