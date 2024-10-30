// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/components/transform_relation.fbs".

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
#![allow(non_camel_case_types)]

use ::re_types_core::external::arrow2;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, MaybeOwnedComponentBatch};
use ::re_types_core::{ComponentDescriptor, ComponentName};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Component**: Specifies relation a spatial transform describes.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum TransformRelation {
    /// The transform describes how to transform into the parent entity's space.
    ///
    /// E.g. a translation of (0, 1, 0) with this [`components::TransformRelation`][crate::components::TransformRelation] logged at `parent/child` means
    /// that from the point of view of `parent`, `parent/child` is translated 1 unit along `parent`'s Y axis.
    /// From perspective of `parent/child`, the `parent` entity is translated -1 unit along `parent/child`'s Y axis.
    #[default]
    ParentFromChild = 1,

    /// The transform describes how to transform into the child entity's space.
    ///
    /// E.g. a translation of (0, 1, 0) with this [`components::TransformRelation`][crate::components::TransformRelation] logged at `parent/child` means
    /// that from the point of view of `parent`, `parent/child` is translated -1 unit along `parent`'s Y axis.
    /// From perspective of `parent/child`, the `parent` entity is translated 1 unit along `parent/child`'s Y axis.
    ChildFromParent = 2,
}

impl ::re_types_core::Component for TransformRelation {
    #[inline]
    fn descriptor() -> ComponentDescriptor {
        ComponentDescriptor::new("rerun.components.TransformRelation")
    }
}

::re_types_core::macros::impl_into_cow!(TransformRelation);

impl ::re_types_core::Loggable for TransformRelation {
    #[inline]
    fn arrow_datatype() -> arrow2::datatypes::DataType {
        #![allow(clippy::wildcard_imports)]
        use arrow2::datatypes::*;
        DataType::UInt8
    }

    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
    ) -> SerializationResult<Box<dyn arrow2::array::Array>>
    where
        Self: Clone + 'a,
    {
        #![allow(clippy::wildcard_imports)]
        #![allow(clippy::manual_is_variant_and)]
        use ::re_types_core::{Loggable as _, ResultExt as _};
        use arrow2::{array::*, datatypes::*};
        Ok({
            let (somes, data0): (Vec<_>, Vec<_>) = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    let datum = datum.map(|datum| *datum as u8);
                    (datum.is_some(), datum)
                })
                .unzip();
            let data0_bitmap: Option<arrow2::bitmap::Bitmap> = {
                let any_nones = somes.iter().any(|some| !*some);
                any_nones.then(|| somes.into())
            };
            PrimitiveArray::new(
                Self::arrow_datatype(),
                data0.into_iter().map(|v| v.unwrap_or_default()).collect(),
                data0_bitmap,
            )
            .boxed()
        })
    }

    fn from_arrow_opt(
        arrow_data: &dyn arrow2::array::Array,
    ) -> DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        #![allow(clippy::wildcard_imports)]
        use ::re_types_core::{Loggable as _, ResultExt as _};
        use arrow2::{array::*, buffer::*, datatypes::*};
        Ok(arrow_data
            .as_any()
            .downcast_ref::<UInt8Array>()
            .ok_or_else(|| {
                let expected = Self::arrow_datatype();
                let actual = arrow_data.data_type().clone();
                DeserializationError::datatype_mismatch(expected, actual)
            })
            .with_context("rerun.components.TransformRelation#enum")?
            .into_iter()
            .map(|opt| opt.copied())
            .map(|typ| match typ {
                Some(1) => Ok(Some(Self::ParentFromChild)),
                Some(2) => Ok(Some(Self::ChildFromParent)),
                None => Ok(None),
                Some(invalid) => Err(DeserializationError::missing_union_arm(
                    Self::arrow_datatype(),
                    "<invalid>",
                    invalid as _,
                )),
            })
            .collect::<DeserializationResult<Vec<Option<_>>>>()
            .with_context("rerun.components.TransformRelation")?)
    }
}

impl std::fmt::Display for TransformRelation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParentFromChild => write!(f, "ParentFromChild"),
            Self::ChildFromParent => write!(f, "ChildFromParent"),
        }
    }
}

impl ::re_types_core::reflection::Enum for TransformRelation {
    #[inline]
    fn variants() -> &'static [Self] {
        &[Self::ParentFromChild, Self::ChildFromParent]
    }

    #[inline]
    fn docstring_md(self) -> &'static str {
        match self {
            Self::ParentFromChild => {
                "The transform describes how to transform into the parent entity's space.\n\nE.g. a translation of (0, 1, 0) with this [`components::TransformRelation`][crate::components::TransformRelation] logged at `parent/child` means\nthat from the point of view of `parent`, `parent/child` is translated 1 unit along `parent`'s Y axis.\nFrom perspective of `parent/child`, the `parent` entity is translated -1 unit along `parent/child`'s Y axis."
            }
            Self::ChildFromParent => {
                "The transform describes how to transform into the child entity's space.\n\nE.g. a translation of (0, 1, 0) with this [`components::TransformRelation`][crate::components::TransformRelation] logged at `parent/child` means\nthat from the point of view of `parent`, `parent/child` is translated -1 unit along `parent`'s Y axis.\nFrom perspective of `parent/child`, the `parent` entity is translated 1 unit along `parent/child`'s Y axis."
            }
        }
    }
}

impl ::re_types_core::SizeBytes for TransformRelation {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        0
    }

    #[inline]
    fn is_pod() -> bool {
        true
    }
}
