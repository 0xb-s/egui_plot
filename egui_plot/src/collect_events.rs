use egui::{Response, Shape, Vec2b};

use crate::{
    ActionExecutor, ActionQueue, PlotBounds, PlotEvent,
    action::{AppliedActions, BoundsLike, ChangeCause, PlotAction},
};

impl ActionExecutor {
    pub fn apply<I, B>(
        queue: ActionQueue<I>,
        mut bounds: B,
        mut auto_bounds: Vec2b,
        _last_transform: Option<()>,
        _response: Option<&Response>,
    ) -> AppliedActions<I, B>
    where
        B: BoundsLike,
    {
        let mut items: Vec<I> = Vec::new();
        let mut overlays: Vec<Shape> = Vec::new();
        let mut events: Vec<PlotEvent> = Vec::new();

        for action in queue.drain() {
            if let Some(ev) = action.as_event() {
                events.push(ev);
            }

            match action {
                PlotAction::AddItem(item) => items.push(item),

                PlotAction::SetBoundsX(range) => {
                    bounds.set_x_range(range);
                    auto_bounds.x = false;
                }
                PlotAction::SetBoundsY(range) => {
                    bounds.set_y_range(range);
                    auto_bounds.y = false;
                }
                PlotAction::Translate(delta) => {
                    bounds.translate(delta.x as f64, delta.y as f64);
                    auto_bounds = Vec2b::from([false, false]);
                }
                PlotAction::SetAutoBounds(v) => {
                    auto_bounds = v;
                }
                PlotAction::Zoom(factor, center) => {
                    bounds.zoom(factor, center);
                    auto_bounds = Vec2b::from([false, false]);
                }
                PlotAction::AddOverlayShape(shape) => overlays.push(shape),
            }
        }

        AppliedActions {
            items,
            auto_bounds,
            bounds,
            overlays,
            events,
        }
    }
}
// action.rs

impl<I> PlotAction<I> {
    /// Turn action to events.
    pub fn as_event(&self) -> Option<PlotEvent> {
        match self {
            Self::SetBoundsX(range) => Some(PlotEvent::BoundsChanged {
                old: PlotBounds::NOTHING,
                new: PlotBounds {
                    min: [*range.start(), f64::NEG_INFINITY],
                    max: [*range.end(), f64::INFINITY],
                },
                cause: ChangeCause::Programmatic,
            }),

            Self::SetBoundsY(range) => Some(PlotEvent::BoundsChanged {
                old: PlotBounds::NOTHING,
                new: PlotBounds {
                    min: [f64::NEG_INFINITY, *range.start()],
                    max: [f64::INFINITY, *range.end()],
                },
                cause: ChangeCause::Programmatic,
            }),

            Self::Translate(_)
            | Self::Zoom(_, _)
            | Self::SetAutoBounds(_)
            | Self::AddOverlayShape(_)
            | Self::AddItem(_) => None,
        }
    }
}
