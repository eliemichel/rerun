use egui::Rect;
use re_format::format_f32;
use re_types::blueprint::components::VisualBounds2D;
use re_ui::UiExt;
use re_viewer_context::SpaceViewState;

use crate::{
    graph::Graph,
    layout::{ForceLayout, Layout},
};

/// Space view state for the custom space view.
///
/// This state is preserved between frames, but not across Viewer sessions.
#[derive(Default)]
pub struct GraphSpaceViewState {
    pub layout_state: LayoutState,

    pub show_debug: bool,

    pub world_bounds: Option<VisualBounds2D>,
}

impl GraphSpaceViewState {
    pub fn layout_ui(&self, ui: &mut egui::Ui) {
        let Some(rect) = self.layout_state.bounding_rect() else {
            return;
        };
        ui.grid_left_hand_label("Layout")
            .on_hover_text("The bounding box encompassing all entities in the view right now");
        ui.vertical(|ui| {
            ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
            let egui::Rect { min, max } = rect;
            ui.label(format!("x [{} - {}]", format_f32(min.x), format_f32(max.x),));
            ui.label(format!("y [{} - {}]", format_f32(min.y), format_f32(max.y),));
        });
        ui.end_row();
    }

    pub fn debug_ui(&mut self, ui: &mut egui::Ui) {
        ui.re_checkbox(&mut self.show_debug, "Show debug information")
            .on_hover_text("Shows debug information for the current graph");
        ui.end_row();
    }

    pub fn simulation_ui(&mut self, ui: &mut egui::Ui) {
        if ui.button("Reset simulation").clicked() {
            self.layout_state.reset();
        }
    }
}

impl SpaceViewState for GraphSpaceViewState {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// Used to determine if a layout is up-to-date or outdated. For now we use a
/// hash of the data the comes from the visualizers.
#[derive(Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Discriminator(u64);

impl Discriminator {
    pub fn new(hash: u64) -> Self {
        Self(hash)
    }
}

/// The following is a simple state machine that keeps track of the different
/// layouts and if they need to be recomputed. It also holds the state of the
/// force-based simulation.
#[derive(Default)]
pub enum LayoutState {
    #[default]
    None,
    InProgress {
        discriminator: Discriminator,
        layout: Layout,
        provider: ForceLayout,
    },
    Finished {
        discriminator: Discriminator,
        layout: Layout,
        _provider: ForceLayout,
    },
}

impl LayoutState {
    pub fn bounding_rect(&self) -> Option<Rect> {
        match self {
            Self::None => None,
            Self::Finished { layout, .. } | Self::InProgress { layout, .. } => {
                Some(layout.bounding_rect())
            }
        }
    }

    pub fn reset(&mut self) {
        *self = Self::None;
    }

    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    pub fn is_in_progress(&self) -> bool {
        matches!(self, Self::InProgress { .. })
    }

    /// A simple state machine that keeps track of the different stages and if the layout needs to be recomputed.
    fn update<'a>(
        self,
        requested: Discriminator,
        graphs: impl Iterator<Item = &'a Graph<'a>> + Clone,
    ) -> Self {
        match self {
            // Layout is up to date, nothing to do here.
            Self::Finished {
                ref discriminator, ..
            } if discriminator == &requested => {
                self // no op
            }
            // We need to recompute the layout.
            Self::None | Self::Finished { .. } => {
                let provider = ForceLayout::new(graphs);
                let layout = provider.init();

                Self::InProgress {
                    discriminator: requested,
                    layout,
                    provider,
                }
            }
            Self::InProgress {
                ref discriminator, ..
            } if discriminator != &requested => {
                let provider = ForceLayout::new(graphs);
                let layout = provider.init();

                Self::InProgress {
                    discriminator: requested,
                    layout,
                    provider,
                }
            }
            // We keep iterating on the layout until it is stable.
            Self::InProgress {
                discriminator,
                mut layout,
                mut provider,
            } => match provider.tick(&mut layout) {
                true => Self::Finished {
                    discriminator,
                    layout,
                    _provider: provider,
                },
                false => Self::InProgress {
                    discriminator,
                    layout,
                    provider,
                },
            },
        }
    }

    /// This method is lazy. A new layout is only computed if the current timestamp requires it.
    pub fn get<'a>(
        &'a mut self,
        hash: Discriminator,
        graphs: impl Iterator<Item = &'a Graph<'a>> + Clone,
    ) -> &'a mut Layout {
        *self = std::mem::take(self).update(hash, graphs);

        match self {
            Self::Finished { layout, .. } | Self::InProgress { layout, .. } => layout,
            Self::None => unreachable!(), // We just set the state to `Self::Current` above.
        }
    }
}
