// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/encoded_image.fbs".

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

/// **Archetype**: An image encoded as e.g. a JPEG or PNG.
///
/// Rerun also supports uncompressed images with the [`archetypes::Image`][crate::archetypes::Image].
/// For images that refer to video frames see [`archetypes::VideoFrameReference`][crate::archetypes::VideoFrameReference].
///
/// ## Example
///
/// ### `encoded_image`:
/// ```ignore
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let rec = rerun::RecordingStreamBuilder::new("rerun_example_encoded_image").spawn()?;
///
///     let image = include_bytes!("ferris.png");
///
///     rec.log(
///         "image",
///         &rerun::EncodedImage::from_file_contents(image.to_vec()),
///     )?;
///
///     Ok(())
/// }
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct EncodedImage {
    /// The encoded content of some image file, e.g. a PNG or JPEG.
    pub blob: crate::components::Blob,

    /// The Media Type of the asset.
    ///
    /// Supported values:
    /// * `image/jpeg`
    /// * `image/png`
    ///
    /// If omitted, the viewer will try to guess from the data blob.
    /// If it cannot guess, it won't be able to render the asset.
    pub media_type: Option<crate::components::MediaType>,

    /// Opacity of the image, useful for layering several images.
    ///
    /// Defaults to 1.0 (fully opaque).
    pub opacity: Option<crate::components::Opacity>,

    /// An optional floating point value that specifies the 2D drawing order.
    ///
    /// Objects with higher values are drawn on top of those with lower values.
    pub draw_order: Option<crate::components::DrawOrder>,
}

impl ::re_types_core::SizeBytes for EncodedImage {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.blob.heap_size_bytes()
            + self.media_type.heap_size_bytes()
            + self.opacity.heap_size_bytes()
            + self.draw_order.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <crate::components::Blob>::is_pod()
            && <Option<crate::components::MediaType>>::is_pod()
            && <Option<crate::components::Opacity>>::is_pod()
            && <Option<crate::components::DrawOrder>>::is_pod()
    }
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(|| ["rerun.components.Blob".into()]);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 2usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.components.MediaType".into(),
            "rerun.components.EncodedImageIndicator".into(),
        ]
    });

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 2usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.components.Opacity".into(),
            "rerun.components.DrawOrder".into(),
        ]
    });

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 5usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.components.Blob".into(),
            "rerun.components.MediaType".into(),
            "rerun.components.EncodedImageIndicator".into(),
            "rerun.components.Opacity".into(),
            "rerun.components.DrawOrder".into(),
        ]
    });

impl EncodedImage {
    /// The total number of components in the archetype: 1 required, 2 recommended, 2 optional
    pub const NUM_COMPONENTS: usize = 5usize;
}

/// Indicator component for the [`EncodedImage`] [`::re_types_core::Archetype`]
pub type EncodedImageIndicator = ::re_types_core::GenericIndicatorComponent<EncodedImage>;

impl ::re_types_core::Archetype for EncodedImage {
    type Indicator = EncodedImageIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.archetypes.EncodedImage".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "Encoded image"
    }

    #[inline]
    fn indicator() -> MaybeOwnedComponentBatch<'static> {
        static INDICATOR: EncodedImageIndicator = EncodedImageIndicator::DEFAULT;
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
        let blob = {
            let array = arrays_by_name
                .get("rerun.components.Blob")
                .ok_or_else(DeserializationError::missing_data)
                .with_context("rerun.archetypes.EncodedImage#blob")?;
            <crate::components::Blob>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.EncodedImage#blob")?
                .into_iter()
                .next()
                .flatten()
                .ok_or_else(DeserializationError::missing_data)
                .with_context("rerun.archetypes.EncodedImage#blob")?
        };
        let media_type = if let Some(array) = arrays_by_name.get("rerun.components.MediaType") {
            <crate::components::MediaType>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.EncodedImage#media_type")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        let opacity = if let Some(array) = arrays_by_name.get("rerun.components.Opacity") {
            <crate::components::Opacity>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.EncodedImage#opacity")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        let draw_order = if let Some(array) = arrays_by_name.get("rerun.components.DrawOrder") {
            <crate::components::DrawOrder>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.EncodedImage#draw_order")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        Ok(Self {
            blob,
            media_type,
            opacity,
            draw_order,
        })
    }
}

impl ::re_types_core::AsComponents for EncodedImage {
    fn as_component_batches(&self) -> Vec<MaybeOwnedComponentBatch<'_>> {
        re_tracing::profile_function!();
        use ::re_types_core::Archetype as _;
        [
            Some(Self::indicator()),
            Some((&self.blob as &dyn ComponentBatch).into()),
            self.media_type
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
            self.opacity
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
            self.draw_order
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl ::re_types_core::ArchetypeReflectionMarker for EncodedImage {}

impl EncodedImage {
    /// Create a new `EncodedImage`.
    #[inline]
    pub fn new(blob: impl Into<crate::components::Blob>) -> Self {
        Self {
            blob: blob.into(),
            media_type: None,
            opacity: None,
            draw_order: None,
        }
    }

    /// The Media Type of the asset.
    ///
    /// Supported values:
    /// * `image/jpeg`
    /// * `image/png`
    ///
    /// If omitted, the viewer will try to guess from the data blob.
    /// If it cannot guess, it won't be able to render the asset.
    #[inline]
    pub fn with_media_type(mut self, media_type: impl Into<crate::components::MediaType>) -> Self {
        self.media_type = Some(media_type.into());
        self
    }

    /// Opacity of the image, useful for layering several images.
    ///
    /// Defaults to 1.0 (fully opaque).
    #[inline]
    pub fn with_opacity(mut self, opacity: impl Into<crate::components::Opacity>) -> Self {
        self.opacity = Some(opacity.into());
        self
    }

    /// An optional floating point value that specifies the 2D drawing order.
    ///
    /// Objects with higher values are drawn on top of those with lower values.
    #[inline]
    pub fn with_draw_order(mut self, draw_order: impl Into<crate::components::DrawOrder>) -> Self {
        self.draw_order = Some(draw_order.into());
        self
    }
}
