//! Navigation module.

use egui::{Key, Modifiers, PointerButton, Vec2b};

/// A reset operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResetBehavior {
    /// Restore the original bounds from the first frame the plot was shown.
    OriginalBounds,
}

/// Per-axis enable flags.
#[derive(Clone, Copy, Debug)]
pub struct AxisToggle {
    /// Master flag. If `false`, the feature is disabled even if individual axes are `true`.
    pub enabled: bool,
    /// Which axes are affected (`x` and/or `y`).
    pub axis: Vec2b,
}

impl AxisToggle {
    #[inline]
    pub const fn new(enabled: bool, axis: Vec2b) -> Self {
        Self { enabled, axis }
    }
}

/// Zoom configuration.
#[derive(Clone, Copy, Debug)]
pub struct ZoomConfig {
    /// Master enable.
    pub enabled: bool,
    /// Axes to zoom (`x` and/or `y`).
    pub axis: Vec2b,
    /// If `true`, zoom centers at the mouse position; otherwise at plot center.
    pub zoom_to_mouse: bool,
    /// Exponent applied to `egui` zoom delta (1.0 = unchanged).
    /// Values >1.0 make zoom more aggressive; <1.0 make it gentler.
    pub wheel_factor_exp: f32,
}

impl ZoomConfig {
    #[inline]
    pub const fn new(enabled: bool, axis: Vec2b) -> Self {
        Self {
            enabled,
            axis,
            zoom_to_mouse: true,
            wheel_factor_exp: 1.0,
        }
    }

    #[inline]
    pub fn zoom_to_mouse(mut self, v: bool) -> Self {
        self.zoom_to_mouse = v;
        self
    }

    #[inline]
    pub fn wheel_factor_exp(mut self, exp: f32) -> Self {
        self.wheel_factor_exp = exp;
        self
    }
}

/// Box (rubber-band) zoom settings.
#[derive(Clone, Copy, Debug)]
pub struct BoxZoomConfig {
    /// Enable boxed zoom.
    pub enabled: bool,
    /// Which pointer button starts the box.
    pub button: PointerButton,
    /// Which modifiers must be down. Any `true` field here must be pressed at runtime.
    pub required_mods: Modifiers,
}

impl BoxZoomConfig {
    #[inline]
    pub const fn new(enabled: bool, button: PointerButton, required_mods: Modifiers) -> Self {
        Self {
            enabled,
            button,
            required_mods,
        }
    }
}

/// All navigation & shortcut controls in one place.
#[derive(Clone, Copy, Debug)]
pub struct NavigationConfig {
    /// Dragging (per axis).
    pub drag: AxisToggle,
    /// Scrolling/panning with mouse wheel/touchpad (per axis).
    pub scroll: AxisToggle,
    /// Axis-zoom-drag (drag on axis strips).
    pub axis_zoom_drag: Vec2b,
    /// Wheel/pinch zoom.
    pub zoom: ZoomConfig,
    /// Box zoom.
    pub box_zoom: BoxZoomConfig,
    /// What double-click reset does.
    pub reset_behavior: ResetBehavior,
    /// Allow double-click reset.
    pub double_click_reset: bool,
    /// Enable pinning (P/U/Delete by default).
    pub pinning_enabled: bool,
    /// Shortcut: fit to view (e.g., `Key::F`). `None` disables shortcut.
    pub fit_to_view_key: Option<Key>,

    /// Pin shortcuts.
    pub pin_add_key: Option<Key>,
    pub pin_remove_key: Option<Key>,
    pub pins_clear_key: Option<Key>,
}

impl Default for NavigationConfig {
    fn default() -> Self {
        Self {
            drag: AxisToggle::new(true, Vec2b::new(true, true)),
            scroll: AxisToggle::new(true, Vec2b::new(true, true)),
            axis_zoom_drag: Vec2b::new(false, false),
            zoom: ZoomConfig::new(true, Vec2b::new(true, true))
                .zoom_to_mouse(true)
                .wheel_factor_exp(1.0),
            box_zoom: BoxZoomConfig::new(false, PointerButton::Secondary, Modifiers::NONE),
            reset_behavior: ResetBehavior::OriginalBounds,
            double_click_reset: true,
            pinning_enabled: true,
            fit_to_view_key: Some(Key::F),

            pin_add_key: Some(Key::P),
            pin_remove_key: Some(Key::U),
            pins_clear_key: Some(Key::Delete),
        }
    }
}
impl NavigationConfig {
    #[allow(clippy::fn_params_excessive_bools)]
    /// Build a `NavigationConfig`.
    pub fn from_legacy_flags(
        allow_drag: Vec2b,
        allow_zoom: Vec2b,
        allow_scroll: Vec2b,
        allow_axis_zoom_drag: Vec2b,
        allow_double_click_reset: bool,
        allow_boxed_zoom: bool,
        boxed_zoom_button: PointerButton,
    ) -> Self {
        Self {
            drag: AxisToggle::new(allow_drag.any(), allow_drag),
            scroll: AxisToggle::new(allow_scroll.any(), allow_scroll),
            axis_zoom_drag: allow_axis_zoom_drag,
            zoom: ZoomConfig::new(allow_zoom.any(), allow_zoom)
                .zoom_to_mouse(true)
                .wheel_factor_exp(1.0),
            box_zoom: BoxZoomConfig::new(allow_boxed_zoom, boxed_zoom_button, Modifiers::NONE),

            ..Self::default().reset_controls(
                ResetBehavior::OriginalBounds,
                allow_double_click_reset,
                Some(Key::R),
            )
        }
    }

    /// Configure drag behavior for the given axes.
    ///
    /// The `axes` parameter uses `(x, y)` ordering:
    /// - `Some(Vec2b::new(true, true))`  → drag on both X and Y
    /// - `Some(Vec2b::new(true, false))` → drag on X only
    /// - `Some(Vec2b::new(false, true))` → drag on Y only
    /// - `None`                          → dragging completely disabled
    #[inline]
    pub fn drag(mut self, axes: Option<Vec2b>) -> Self {
        match axes {
            Some(axis) => {
                self.drag = AxisToggle::new(true, axis);
            }
            None => {
                self.drag = AxisToggle::new(false, Vec2b::new(false, false));
            }
        }
        self
    }

    /// Configure scrolling/panning with the mouse wheel or touchpad.
    ///
    /// Same `(x, y)` ordering as `drag`:
    /// - `Some(Vec2b::new(true, false))` → scroll horizontally only
    /// - `None`                          → disable scroll-based navigation
    #[inline]
    pub fn scroll(mut self, axes: Option<Vec2b>) -> Self {
        match axes {
            Some(axis) => {
                self.scroll = AxisToggle::new(true, axis);
            }
            None => {
                self.scroll = AxisToggle::new(false, Vec2b::new(false, false));
            }
        }
        self
    }

    /// Configure zoom-drag on the axis strips.
    ///
    /// `axis` selects which axes can be zoomed by dragging on their axis strips.
    #[inline]
    pub fn axis_zoom(mut self, axis: Vec2b) -> Self {
        self.axis_zoom_drag = axis;
        self
    }

    /// Set the full zoom configuration.
    #[inline]
    pub fn scroll_zoom(mut self, cfg: ZoomConfig) -> Self {
        self.zoom = cfg;
        self
    }

    /// Set the box-zoom configuration.
    #[inline]
    pub fn box_zoom(mut self, cfg: BoxZoomConfig) -> Self {
        self.box_zoom = cfg;
        self
    }

    /// Configure all reset-related controls in a single place.
    ///
    /// `behavior` defines how reset behaves, `double_click` toggles double-click
    /// reset, and `fit_key` / `restore_key` configure keyboard shortcuts.
    #[inline]
    pub fn reset_controls(
        mut self,
        behavior: ResetBehavior,
        double_click: bool,
        fit_key: Option<Key>,
    ) -> Self {
        self.reset_behavior = behavior;
        self.double_click_reset = double_click;
        self.fit_to_view_key = fit_key;

        self
    }

    /// Set the reset behavior
    ///
    /// This keeps other reset-related fields (double click, shortcuts) unchanged.
    #[inline]
    pub fn reset_behavior(self, behavior: ResetBehavior) -> Self {
        self.reset_controls(behavior, self.double_click_reset, self.fit_to_view_key)
    }

    /// Enable or disable double-click reset.
    ///
    /// This keeps the reset behavior and shortcuts unchanged.
    #[inline]
    pub fn double_click_reset(self, on: bool) -> Self {
        self.reset_controls(self.reset_behavior, on, self.fit_to_view_key)
    }

    /// Configure keyboard shortcuts for "fit to view" and "restore original".
    ///
    /// Pass `None` to disable a shortcut.
    #[inline]
    pub fn shortcuts_fit_restore(self, fit: Option<Key>) -> Self {
        self.reset_controls(self.reset_behavior, self.double_click_reset, fit)
    }

    /// Enable or disable pinning (tooltip pin add/remove/clear).
    ///
    /// This affects keyboard shortcuts for pins and any pin-related UI.
    #[inline]
    pub fn pinning(mut self, on: bool) -> Self {
        self.pinning_enabled = on;
        self
    }

    /// Configure keyboard shortcuts for pin management.
    ///
    /// `add`, `remove`, and `clear` control pin creation and deletion.
    #[inline]
    pub fn shortcuts_pin(
        mut self,
        add: Option<Key>,
        remove: Option<Key>,
        clear: Option<Key>,
    ) -> Self {
        self.pin_add_key = add;
        self.pin_remove_key = remove;
        self.pins_clear_key = clear;
        self
    }
}
