//! Navigation module.

use egui::{Key, Modifiers, PointerButton, Vec2b};

/// A reset operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResetBehavior {
    /// Reset by auto-fitting bounds to visible content.
    AutoFit,
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
    /// Shortcut: restore original bounds (e.g., `Key::R`). `None` disables shortcut.
    pub restore_original_key: Option<Key>,
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
            reset_behavior: ResetBehavior::AutoFit,
            double_click_reset: true,
            pinning_enabled: true,
            fit_to_view_key: Some(Key::F),
            restore_original_key: Some(Key::R),
            pin_add_key: Some(Key::P),
            pin_remove_key: Some(Key::U),
            pins_clear_key: Some(Key::Delete),
        }
    }
}
impl NavigationConfig {
    #[allow(clippy::fn_params_excessive_bools)]
    /// Helper used to migrate legacy per-field flags into a `NavigationConfig`.
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
            reset_behavior: ResetBehavior::AutoFit,
            double_click_reset: allow_double_click_reset,
            ..Default::default()
        }
    }

    /// Builders for convenience.
    #[inline]
    pub fn drag(mut self, axis: Vec2b, enabled: bool) -> Self {
        self.drag = AxisToggle::new(enabled, axis);
        self
    }

    #[inline]
    pub fn scroll(mut self, axis: Vec2b, enabled: bool) -> Self {
        self.scroll = AxisToggle::new(enabled, axis);
        self
    }

    #[inline]
    pub fn axis_zoom_drag(mut self, axis: Vec2b) -> Self {
        self.axis_zoom_drag = axis;
        self
    }

    #[inline]
    pub fn zoom(mut self, cfg: ZoomConfig) -> Self {
        self.zoom = cfg;
        self
    }

    #[inline]
    pub fn box_zoom(mut self, cfg: BoxZoomConfig) -> Self {
        self.box_zoom = cfg;
        self
    }

    #[inline]
    pub fn reset_behavior(mut self, behavior: ResetBehavior) -> Self {
        self.reset_behavior = behavior;
        self
    }

    #[inline]
    pub fn double_click_reset(mut self, on: bool) -> Self {
        self.double_click_reset = on;
        self
    }

    #[inline]
    pub fn pinning(mut self, on: bool) -> Self {
        self.pinning_enabled = on;
        self
    }

    #[inline]
    pub fn shortcuts_fit_restore(mut self, fit: Option<Key>, restore: Option<Key>) -> Self {
        self.fit_to_view_key = fit;
        self.restore_original_key = restore;
        self
    }

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
