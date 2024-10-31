// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/asset3d.fbs".

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

/// **Archetype**: A prepacked 3D asset (`.gltf`, `.glb`, `.obj`, `.stl`, etc.).
///
/// See also [`archetypes::Mesh3D`][crate::archetypes::Mesh3D].
///
/// If there are multiple [`archetypes::InstancePoses3D`][crate::archetypes::InstancePoses3D] instances logged to the same entity as a mesh,
/// an instance of the mesh will be drawn for each transform.
///
/// ## Example
///
/// ### Simple 3D asset
/// ```ignore
/// use rerun::external::anyhow;
///
/// fn main() -> anyhow::Result<()> {
///     let args = std::env::args().collect::<Vec<_>>();
///     let Some(path) = args.get(1) else {
///         anyhow::bail!("Usage: {} <path_to_asset.[gltf|glb|obj|stl]>", args[0]);
///     };
///
///     let rec = rerun::RecordingStreamBuilder::new("rerun_example_asset3d").spawn()?;
///
///     rec.log_static("world", &rerun::ViewCoordinates::RIGHT_HAND_Z_UP)?; // Set an up-axis
///     rec.log("world/asset", &rerun::Asset3D::from_file(path)?)?;
///
///     Ok(())
/// }
/// ```
/// <center>
/// <picture>
///   <source media="(max-width: 480px)" srcset="https://static.rerun.io/asset3d_simple/af238578188d3fd0de3e330212120e2842a8ddb2/480w.png">
///   <source media="(max-width: 768px)" srcset="https://static.rerun.io/asset3d_simple/af238578188d3fd0de3e330212120e2842a8ddb2/768w.png">
///   <source media="(max-width: 1024px)" srcset="https://static.rerun.io/asset3d_simple/af238578188d3fd0de3e330212120e2842a8ddb2/1024w.png">
///   <source media="(max-width: 1200px)" srcset="https://static.rerun.io/asset3d_simple/af238578188d3fd0de3e330212120e2842a8ddb2/1200w.png">
///   <img src="https://static.rerun.io/asset3d_simple/af238578188d3fd0de3e330212120e2842a8ddb2/full.png" width="640">
/// </picture>
/// </center>
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Asset3D {
    /// The asset's bytes.
    pub blob: crate::components::Blob,

    /// The Media Type of the asset.
    ///
    /// Supported values:
    /// * `model/gltf-binary`
    /// * `model/gltf+json`
    /// * `model/obj` (.mtl material files are not supported yet, references are silently ignored)
    /// * `model/stl`
    ///
    /// If omitted, the viewer will try to guess from the data blob.
    /// If it cannot guess, it won't be able to render the asset.
    pub media_type: Option<crate::components::MediaType>,

    /// A color multiplier applied to the whole asset.
    ///
    /// For mesh who already have `albedo_factor` in materials,
    /// it will be overwritten by actual `albedo_factor` of [`archetypes::Asset3D`][crate::archetypes::Asset3D] (if specified).
    pub albedo_factor: Option<crate::components::AlbedoFactor>,
}

impl ::re_types_core::SizeBytes for Asset3D {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.blob.heap_size_bytes()
            + self.media_type.heap_size_bytes()
            + self.albedo_factor.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <crate::components::Blob>::is_pod()
            && <Option<crate::components::MediaType>>::is_pod()
            && <Option<crate::components::AlbedoFactor>>::is_pod()
    }
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(|| ["rerun.components.Blob".into()]);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 2usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.components.MediaType".into(),
            "rerun.components.Asset3DIndicator".into(),
        ]
    });

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(|| ["rerun.components.AlbedoFactor".into()]);

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 4usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.components.Blob".into(),
            "rerun.components.MediaType".into(),
            "rerun.components.Asset3DIndicator".into(),
            "rerun.components.AlbedoFactor".into(),
        ]
    });

impl Asset3D {
    /// The total number of components in the archetype: 1 required, 2 recommended, 1 optional
    pub const NUM_COMPONENTS: usize = 4usize;
}

/// Indicator component for the [`Asset3D`] [`::re_types_core::Archetype`]
pub type Asset3DIndicator = ::re_types_core::GenericIndicatorComponent<Asset3D>;

impl ::re_types_core::Archetype for Asset3D {
    type Indicator = Asset3DIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.archetypes.Asset3D".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "Asset 3D"
    }

    #[inline]
    fn indicator() -> MaybeOwnedComponentBatch<'static> {
        static INDICATOR: Asset3DIndicator = Asset3DIndicator::DEFAULT;
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
                .with_context("rerun.archetypes.Asset3D#blob")?;
            <crate::components::Blob>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.Asset3D#blob")?
                .into_iter()
                .next()
                .flatten()
                .ok_or_else(DeserializationError::missing_data)
                .with_context("rerun.archetypes.Asset3D#blob")?
        };
        let media_type = if let Some(array) = arrays_by_name.get("rerun.components.MediaType") {
            <crate::components::MediaType>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.Asset3D#media_type")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        let albedo_factor = if let Some(array) = arrays_by_name.get("rerun.components.AlbedoFactor")
        {
            <crate::components::AlbedoFactor>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.Asset3D#albedo_factor")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        Ok(Self {
            blob,
            media_type,
            albedo_factor,
        })
    }
}

impl ::re_types_core::AsComponents for Asset3D {
    fn as_component_batches(&self) -> Vec<MaybeOwnedComponentBatch<'_>> {
        re_tracing::profile_function!();
        use ::re_types_core::Archetype as _;
        [
            Some(Self::indicator()),
            Some((&self.blob as &dyn ComponentBatch).into()),
            self.media_type
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
            self.albedo_factor
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl ::re_types_core::ArchetypeReflectionMarker for Asset3D {}

impl Asset3D {
    /// Create a new `Asset3D`.
    #[inline]
    pub fn new(blob: impl Into<crate::components::Blob>) -> Self {
        Self {
            blob: blob.into(),
            media_type: None,
            albedo_factor: None,
        }
    }

    /// The Media Type of the asset.
    ///
    /// Supported values:
    /// * `model/gltf-binary`
    /// * `model/gltf+json`
    /// * `model/obj` (.mtl material files are not supported yet, references are silently ignored)
    /// * `model/stl`
    ///
    /// If omitted, the viewer will try to guess from the data blob.
    /// If it cannot guess, it won't be able to render the asset.
    #[inline]
    pub fn with_media_type(mut self, media_type: impl Into<crate::components::MediaType>) -> Self {
        self.media_type = Some(media_type.into());
        self
    }

    /// A color multiplier applied to the whole asset.
    ///
    /// For mesh who already have `albedo_factor` in materials,
    /// it will be overwritten by actual `albedo_factor` of [`archetypes::Asset3D`][crate::archetypes::Asset3D] (if specified).
    #[inline]
    pub fn with_albedo_factor(
        mut self,
        albedo_factor: impl Into<crate::components::AlbedoFactor>,
    ) -> Self {
        self.albedo_factor = Some(albedo_factor.into());
        self
    }
}
