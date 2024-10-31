// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/blueprint/archetypes/dataframe_query.fbs".

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

/// **Archetype**: The query for the dataframe view.
#[derive(Clone, Debug)]
pub struct DataframeQuery {
    /// The timeline for this query.
    ///
    /// If unset, the timeline currently active on the time panel is used.
    pub timeline: Option<crate::blueprint::components::TimelineName>,

    /// If provided, only rows whose timestamp is within this range will be shown.
    ///
    /// Note: will be unset as soon as `timeline` is changed.
    pub filter_by_range: Option<crate::blueprint::components::FilterByRange>,

    /// If provided, only show rows which contains a logged event for the specified component.
    pub filter_is_not_null: Option<crate::blueprint::components::FilterIsNotNull>,

    /// Should empty cells be filled with latest-at queries?
    pub apply_latest_at: Option<crate::blueprint::components::ApplyLatestAt>,

    /// Selected columns. If unset, all columns are selected.
    pub select: Option<crate::blueprint::components::SelectedColumns>,
}

impl ::re_types_core::SizeBytes for DataframeQuery {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.timeline.heap_size_bytes()
            + self.filter_by_range.heap_size_bytes()
            + self.filter_is_not_null.heap_size_bytes()
            + self.apply_latest_at.heap_size_bytes()
            + self.select.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <Option<crate::blueprint::components::TimelineName>>::is_pod()
            && <Option<crate::blueprint::components::FilterByRange>>::is_pod()
            && <Option<crate::blueprint::components::FilterIsNotNull>>::is_pod()
            && <Option<crate::blueprint::components::ApplyLatestAt>>::is_pod()
            && <Option<crate::blueprint::components::SelectedColumns>>::is_pod()
    }
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 0usize]> =
    once_cell::sync::Lazy::new(|| []);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(|| ["rerun.blueprint.components.DataframeQueryIndicator".into()]);

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 5usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.blueprint.components.TimelineName".into(),
            "rerun.blueprint.components.FilterByRange".into(),
            "rerun.blueprint.components.FilterIsNotNull".into(),
            "rerun.blueprint.components.ApplyLatestAt".into(),
            "rerun.blueprint.components.SelectedColumns".into(),
        ]
    });

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 6usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.blueprint.components.DataframeQueryIndicator".into(),
            "rerun.blueprint.components.TimelineName".into(),
            "rerun.blueprint.components.FilterByRange".into(),
            "rerun.blueprint.components.FilterIsNotNull".into(),
            "rerun.blueprint.components.ApplyLatestAt".into(),
            "rerun.blueprint.components.SelectedColumns".into(),
        ]
    });

impl DataframeQuery {
    /// The total number of components in the archetype: 0 required, 1 recommended, 5 optional
    pub const NUM_COMPONENTS: usize = 6usize;
}

/// Indicator component for the [`DataframeQuery`] [`::re_types_core::Archetype`]
pub type DataframeQueryIndicator = ::re_types_core::GenericIndicatorComponent<DataframeQuery>;

impl ::re_types_core::Archetype for DataframeQuery {
    type Indicator = DataframeQueryIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.blueprint.archetypes.DataframeQuery".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "Dataframe query"
    }

    #[inline]
    fn indicator() -> MaybeOwnedComponentBatch<'static> {
        static INDICATOR: DataframeQueryIndicator = DataframeQueryIndicator::DEFAULT;
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
    #[cfg(test)]
    #[inline]
    fn from_arrow_components(
        arrow_data: impl IntoIterator<Item = (ComponentName, Box<dyn arrow2::array::Array>)>,
    ) -> DeserializationResult<Self> {
        re_tracing::profile_function!();
        use ::re_types_core::{Loggable as _, ResultExt as _};
        let arrays_by_name: ::std::collections::HashMap<_, _> = arrow_data
            .into_iter()
            .map(|(name, array)| (name.full_name(), array))
            .collect();
        let timeline =
            if let Some(array) = arrays_by_name.get("rerun.blueprint.components.TimelineName") {
                <crate::blueprint::components::TimelineName>::from_arrow_opt(&**array)
                    .with_context("rerun.blueprint.archetypes.DataframeQuery#timeline")?
                    .into_iter()
                    .next()
                    .flatten()
            } else {
                None
            };
        let filter_by_range =
            if let Some(array) = arrays_by_name.get("rerun.blueprint.components.FilterByRange") {
                <crate::blueprint::components::FilterByRange>::from_arrow_opt(&**array)
                    .with_context("rerun.blueprint.archetypes.DataframeQuery#filter_by_range")?
                    .into_iter()
                    .next()
                    .flatten()
            } else {
                None
            };
        let filter_is_not_null =
            if let Some(array) = arrays_by_name.get("rerun.blueprint.components.FilterIsNotNull") {
                <crate::blueprint::components::FilterIsNotNull>::from_arrow_opt(&**array)
                    .with_context("rerun.blueprint.archetypes.DataframeQuery#filter_is_not_null")?
                    .into_iter()
                    .next()
                    .flatten()
            } else {
                None
            };
        let apply_latest_at =
            if let Some(array) = arrays_by_name.get("rerun.blueprint.components.ApplyLatestAt") {
                <crate::blueprint::components::ApplyLatestAt>::from_arrow_opt(&**array)
                    .with_context("rerun.blueprint.archetypes.DataframeQuery#apply_latest_at")?
                    .into_iter()
                    .next()
                    .flatten()
            } else {
                None
            };
        let select =
            if let Some(array) = arrays_by_name.get("rerun.blueprint.components.SelectedColumns") {
                <crate::blueprint::components::SelectedColumns>::from_arrow_opt(&**array)
                    .with_context("rerun.blueprint.archetypes.DataframeQuery#select")?
                    .into_iter()
                    .next()
                    .flatten()
            } else {
                None
            };
        Ok(Self {
            timeline,
            filter_by_range,
            filter_is_not_null,
            apply_latest_at,
            select,
        })
    }
}

impl ::re_types_core::AsComponents for DataframeQuery {
    fn as_component_batches(&self) -> Vec<MaybeOwnedComponentBatch<'_>> {
        re_tracing::profile_function!();
        use ::re_types_core::Archetype as _;
        [
            Some(Self::indicator()),
            self.timeline
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
            self.filter_by_range
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
            self.filter_is_not_null
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
            self.apply_latest_at
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
            self.select
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl ::re_types_core::ArchetypeReflectionMarker for DataframeQuery {}

impl DataframeQuery {
    /// Create a new `DataframeQuery`.
    #[inline]
    pub fn new() -> Self {
        Self {
            timeline: None,
            filter_by_range: None,
            filter_is_not_null: None,
            apply_latest_at: None,
            select: None,
        }
    }

    /// The timeline for this query.
    ///
    /// If unset, the timeline currently active on the time panel is used.
    #[inline]
    pub fn with_timeline(
        mut self,
        timeline: impl Into<crate::blueprint::components::TimelineName>,
    ) -> Self {
        self.timeline = Some(timeline.into());
        self
    }

    /// If provided, only rows whose timestamp is within this range will be shown.
    ///
    /// Note: will be unset as soon as `timeline` is changed.
    #[inline]
    pub fn with_filter_by_range(
        mut self,
        filter_by_range: impl Into<crate::blueprint::components::FilterByRange>,
    ) -> Self {
        self.filter_by_range = Some(filter_by_range.into());
        self
    }

    /// If provided, only show rows which contains a logged event for the specified component.
    #[inline]
    pub fn with_filter_is_not_null(
        mut self,
        filter_is_not_null: impl Into<crate::blueprint::components::FilterIsNotNull>,
    ) -> Self {
        self.filter_is_not_null = Some(filter_is_not_null.into());
        self
    }

    /// Should empty cells be filled with latest-at queries?
    #[inline]
    pub fn with_apply_latest_at(
        mut self,
        apply_latest_at: impl Into<crate::blueprint::components::ApplyLatestAt>,
    ) -> Self {
        self.apply_latest_at = Some(apply_latest_at.into());
        self
    }

    /// Selected columns. If unset, all columns are selected.
    #[inline]
    pub fn with_select(
        mut self,
        select: impl Into<crate::blueprint::components::SelectedColumns>,
    ) -> Self {
        self.select = Some(select.into());
        self
    }
}
