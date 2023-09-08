// DO NOT EDIT!: This file was auto-generated by crates/re_types_builder/src/codegen/rust/api.rs:165.

#![allow(trivial_numeric_casts)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::iter_on_single_items)]
#![allow(clippy::map_flatten)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unnecessary_cast)]

/// Indicator component for the `rerun.components.ImageIndicator` archetype.
///
/// Indicator components are data-less components used to give some extra context.
/// The Rerun Viewer can make use of them to provide better heuristics and even improve performance
/// in some cases.
#[derive(Clone, Debug)]
pub struct ImageIndicator;

impl<'a> From<ImageIndicator> for ::std::borrow::Cow<'a, ImageIndicator> {
    #[inline]
    fn from(value: ImageIndicator) -> Self {
        std::borrow::Cow::Owned(value)
    }
}

impl<'a> From<&'a ImageIndicator> for ::std::borrow::Cow<'a, ImageIndicator> {
    #[inline]
    fn from(value: &'a ImageIndicator) -> Self {
        std::borrow::Cow::Borrowed(value)
    }
}

impl crate::Loggable for ImageIndicator {
    type Name = crate::ComponentName;

    #[inline]
    fn name() -> Self::Name {
        "rerun.components.ImageIndicator".into()
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    #[inline]
    fn arrow_datatype() -> arrow2::datatypes::DataType {
        use ::arrow2::datatypes::*;
        DataType::Null
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    fn try_to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
    ) -> crate::SerializationResult<Box<dyn ::arrow2::array::Array>>
    where
        Self: Clone + 'a,
    {
        use crate::{Loggable as _, ResultExt as _};
        use ::arrow2::{array::*, datatypes::*};
        Ok({
            let data0 = data.into_iter().filter(|datum| datum.is_some()).count();
            NullArray::new(Self::arrow_datatype(), data0).boxed()
        })
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    fn try_from_arrow_opt(
        arrow_data: &dyn ::arrow2::array::Array,
    ) -> crate::DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        use crate::{Loggable as _, ResultExt as _};
        use ::arrow2::{array::*, buffer::*, datatypes::*};
        Ok(std::iter::repeat(Ok(Some(Self)))
            .take(
                arrow_data
                    .as_any()
                    .downcast_ref::<NullArray>()
                    .ok_or_else(|| {
                        crate::DeserializationError::datatype_mismatch(
                            DataType::Null,
                            arrow_data.data_type().clone(),
                        )
                    })
                    .with_context("rerun.components.ImageIndicator")?
                    .null_count(),
            )
            .collect::<crate::DeserializationResult<Vec<Option<_>>>>()
            .with_context("rerun.components.ImageIndicator")
            .with_context("rerun.components.ImageIndicator")?)
    }
}
