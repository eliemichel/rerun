// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/blueprint/archetypes/line_grid_3d.fbs".

#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::cloned_instead_of_copied)]
#![allow(clippy::map_flatten)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]

use ::re_types_core::external::arrow2;
use ::re_types_core::ComponentName;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, MaybeOwnedComponentBatch};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Archetype**: Configuration for the 3D line grid.
#[derive(Clone, Debug)]
pub struct LineGrid3D {
    /// Whether the grid is visible.
    ///
    /// Defaults to true.
    pub visible: Option<crate::blueprint::components::Visible>,

    /// Space between grid lines spacing of one line to the next in scene units.
    pub spacing: Option<crate::blueprint::components::GridSpacing>,

    /// How thick the lines should be in ui units.
    ///
    /// Default is 0.5 ui unit.
    pub line_radius: Option<crate::blueprint::components::UiRadius>,

    /// Color used for the grid.
    ///
    /// Transparency via alpha channel is supported.
    /// Defaults to a slightly transparent light gray.
    pub color: Option<crate::components::Color>,

    /// How the grid is oriented.
    ///
    /// Defaults to whatever plane is determined as the down plane by view coordinates if present.
    pub orientation: Option<crate::blueprint::components::PlaneOrientation>,
}

impl ::re_types_core::SizeBytes for LineGrid3D {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.visible.heap_size_bytes()
            + self.spacing.heap_size_bytes()
            + self.line_radius.heap_size_bytes()
            + self.color.heap_size_bytes()
            + self.orientation.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <Option<crate::blueprint::components::Visible>>::is_pod()
            && <Option<crate::blueprint::components::GridSpacing>>::is_pod()
            && <Option<crate::blueprint::components::UiRadius>>::is_pod()
            && <Option<crate::components::Color>>::is_pod()
            && <Option<crate::blueprint::components::PlaneOrientation>>::is_pod()
    }
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 0usize]> =
    once_cell::sync::Lazy::new(|| []);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(|| ["rerun.blueprint.components.LineGrid3DIndicator".into()]);

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 5usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.blueprint.components.Visible".into(),
            "rerun.blueprint.components.GridSpacing".into(),
            "rerun.blueprint.components.UiRadius".into(),
            "rerun.components.Color".into(),
            "rerun.blueprint.components.PlaneOrientation".into(),
        ]
    });

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 6usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.blueprint.components.LineGrid3DIndicator".into(),
            "rerun.blueprint.components.Visible".into(),
            "rerun.blueprint.components.GridSpacing".into(),
            "rerun.blueprint.components.UiRadius".into(),
            "rerun.components.Color".into(),
            "rerun.blueprint.components.PlaneOrientation".into(),
        ]
    });

impl LineGrid3D {
    /// The total number of components in the archetype: 0 required, 1 recommended, 5 optional
    pub const NUM_COMPONENTS: usize = 6usize;
}

/// Indicator component for the [`LineGrid3D`] [`::re_types_core::Archetype`]
pub type LineGrid3DIndicator = ::re_types_core::GenericIndicatorComponent<LineGrid3D>;

impl ::re_types_core::Archetype for LineGrid3D {
    type Indicator = LineGrid3DIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.blueprint.archetypes.LineGrid3D".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "Line grid 3D"
    }

    #[inline]
    fn indicator() -> MaybeOwnedComponentBatch<'static> {
        static INDICATOR: LineGrid3DIndicator = LineGrid3DIndicator::DEFAULT;
        MaybeOwnedComponentBatch::Ref(&INDICATOR)
    }

    #[inline]
    fn required_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        REQUIRED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn recommended_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        RECOMMENDED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn optional_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        OPTIONAL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn all_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        ALL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn from_arrow2_components(
        arrow_data: impl IntoIterator<Item = (ComponentName, Box<dyn arrow2::array::Array>)>,
    ) -> DeserializationResult<Self> {
        re_tracing::profile_function!();
        use ::re_types_core::{Loggable as _, ResultExt as _};
        let arrays_by_name: ::std::collections::HashMap<_, _> = arrow_data
            .into_iter()
            .map(|(name, array)| (name.full_name(), array))
            .collect();
        let visible = if let Some(array) = arrays_by_name.get("rerun.blueprint.components.Visible")
        {
            <crate::blueprint::components::Visible>::from_arrow2_opt(&**array)
                .with_context("rerun.blueprint.archetypes.LineGrid3D#visible")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        let spacing =
            if let Some(array) = arrays_by_name.get("rerun.blueprint.components.GridSpacing") {
                <crate::blueprint::components::GridSpacing>::from_arrow2_opt(&**array)
                    .with_context("rerun.blueprint.archetypes.LineGrid3D#spacing")?
                    .into_iter()
                    .next()
                    .flatten()
            } else {
                None
            };
        let line_radius =
            if let Some(array) = arrays_by_name.get("rerun.blueprint.components.UiRadius") {
                <crate::blueprint::components::UiRadius>::from_arrow2_opt(&**array)
                    .with_context("rerun.blueprint.archetypes.LineGrid3D#line_radius")?
                    .into_iter()
                    .next()
                    .flatten()
            } else {
                None
            };
        let color = if let Some(array) = arrays_by_name.get("rerun.components.Color") {
            <crate::components::Color>::from_arrow2_opt(&**array)
                .with_context("rerun.blueprint.archetypes.LineGrid3D#color")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        let orientation = if let Some(array) =
            arrays_by_name.get("rerun.blueprint.components.PlaneOrientation")
        {
            <crate::blueprint::components::PlaneOrientation>::from_arrow2_opt(&**array)
                .with_context("rerun.blueprint.archetypes.LineGrid3D#orientation")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        Ok(Self {
            visible,
            spacing,
            line_radius,
            color,
            orientation,
        })
    }
}

impl ::re_types_core::AsComponents for LineGrid3D {
    fn as_component_batches(&self) -> Vec<MaybeOwnedComponentBatch<'_>> {
        re_tracing::profile_function!();
        use ::re_types_core::Archetype as _;
        [
            Some(Self::indicator()),
            self.visible
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
            self.spacing
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
            self.line_radius
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
            self.color
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
            self.orientation
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl ::re_types_core::ArchetypeReflectionMarker for LineGrid3D {}

impl LineGrid3D {
    /// Create a new `LineGrid3D`.
    #[inline]
    pub fn new() -> Self {
        Self {
            visible: None,
            spacing: None,
            line_radius: None,
            color: None,
            orientation: None,
        }
    }

    /// Whether the grid is visible.
    ///
    /// Defaults to true.
    #[inline]
    pub fn with_visible(
        mut self,
        visible: impl Into<crate::blueprint::components::Visible>,
    ) -> Self {
        self.visible = Some(visible.into());
        self
    }

    /// Space between grid lines spacing of one line to the next in scene units.
    #[inline]
    pub fn with_spacing(
        mut self,
        spacing: impl Into<crate::blueprint::components::GridSpacing>,
    ) -> Self {
        self.spacing = Some(spacing.into());
        self
    }

    /// How thick the lines should be in ui units.
    ///
    /// Default is 0.5 ui unit.
    #[inline]
    pub fn with_line_radius(
        mut self,
        line_radius: impl Into<crate::blueprint::components::UiRadius>,
    ) -> Self {
        self.line_radius = Some(line_radius.into());
        self
    }

    /// Color used for the grid.
    ///
    /// Transparency via alpha channel is supported.
    /// Defaults to a slightly transparent light gray.
    #[inline]
    pub fn with_color(mut self, color: impl Into<crate::components::Color>) -> Self {
        self.color = Some(color.into());
        self
    }

    /// How the grid is oriented.
    ///
    /// Defaults to whatever plane is determined as the down plane by view coordinates if present.
    #[inline]
    pub fn with_orientation(
        mut self,
        orientation: impl Into<crate::blueprint::components::PlaneOrientation>,
    ) -> Self {
        self.orientation = Some(orientation.into());
        self
    }
}
