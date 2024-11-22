use crate::{result::_Backtrace, DeserializationResult, SerializationResult, SizeBytes};

#[allow(unused_imports)] // used in docstrings
use crate::{Archetype, ComponentBatch, LoggableBatch};

// ---

/// A [`Loggable`] represents a single instance in an array of loggable data.
///
/// Internally, Arrow, and by extension Rerun, only deal with arrays of data.
/// We refer to individual entries in these arrays as instances.
///
/// A [`Loggable`] has no semantics (such as a name, for example): it's just data.
/// If you want to encode semantics, then you're looking for a [`Component`], which extends [`Loggable`].
///
/// Implementing the [`Loggable`] trait automatically derives the [`LoggableBatch`] implementation,
/// which makes it possible to work with lists' worth of data in a generic fashion.
pub trait Loggable: 'static + Send + Sync + Clone + Sized + SizeBytes {
    /// The underlying [`arrow::datatypes::DataType`], excluding datatype extensions.
    fn arrow_datatype() -> arrow::datatypes::DataType {
        Self::arrow2_datatype().into()
    }

    /// The underlying [`arrow2::datatypes::DataType`], excluding datatype extensions.
    fn arrow2_datatype() -> arrow2::datatypes::DataType;

    /// Given an iterator of options of owned or reference values to the current
    /// [`Loggable`], serializes them into an Arrow array.
    ///
    /// When using Rerun's builtin components & datatypes, this can only fail if the data
    /// exceeds the maximum number of entries in an Arrow array (2^31 for standard arrays,
    /// 2^63 for large arrays).
    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<std::borrow::Cow<'a, Self>>>>,
    ) -> SerializationResult<arrow::array::ArrayData>
    where
        Self: 'a,
    {
        Self::to_arrow2_opt(data).map(|array| arrow2::array::to_data(array.as_ref()))
    }

    /// Given an iterator of options of owned or reference values to the current
    /// [`Loggable`], serializes them into an Arrow2 array.
    ///
    /// When using Rerun's builtin components & datatypes, this can only fail if the data
    /// exceeds the maximum number of entries in an Arrow2 array (2^31 for standard arrays,
    /// 2^63 for large arrays).
    fn to_arrow2_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<std::borrow::Cow<'a, Self>>>>,
    ) -> SerializationResult<Box<dyn arrow2::array::Array>>
    where
        Self: 'a;

    // --- Optional serialization methods ---

    /// Given an iterator of owned or reference values to the current [`Loggable`], serializes
    /// them into an Arrow array.
    ///
    /// When using Rerun's builtin components & datatypes, this can only fail if the data
    /// exceeds the maximum number of entries in an Arrow array (2^31 for standard arrays,
    /// 2^63 for large arrays).
    #[inline]
    fn to_arrow<'a>(
        data: impl IntoIterator<Item = impl Into<std::borrow::Cow<'a, Self>>>,
    ) -> SerializationResult<arrow::array::ArrayData>
    where
        Self: 'a,
    {
        re_tracing::profile_function!();
        Self::to_arrow_opt(data.into_iter().map(Some))
    }

    /// Given an iterator of owned or reference values to the current [`Loggable`], serializes
    /// them into an Arrow2 array.
    ///
    /// When using Rerun's builtin components & datatypes, this can only fail if the data
    /// exceeds the maximum number of entries in an Arrow2 array (2^31 for standard arrays,
    /// 2^63 for large arrays).
    #[inline]
    fn to_arrow2<'a>(
        data: impl IntoIterator<Item = impl Into<std::borrow::Cow<'a, Self>>>,
    ) -> SerializationResult<Box<dyn arrow2::array::Array>>
    where
        Self: 'a,
    {
        re_tracing::profile_function!();
        Self::to_arrow2_opt(data.into_iter().map(Some))
    }

    // --- Optional deserialization methods ---

    /// Given an Arrow array, deserializes it into a collection of [`Loggable`]s.
    #[inline]
    fn from_arrow(data: &arrow::array::ArrayData) -> DeserializationResult<Vec<Self>> {
        re_tracing::profile_function!();
        Self::from_arrow_opt(data)?
            .into_iter()
            .map(|opt| {
                opt.ok_or_else(|| crate::DeserializationError::MissingData {
                    backtrace: _Backtrace::new_unresolved(),
                })
            })
            .collect::<DeserializationResult<Vec<_>>>()
    }

    /// Given an Arrow2 array, deserializes it into a collection of [`Loggable`]s.
    #[inline]
    fn from_arrow2(data: &dyn arrow2::array::Array) -> DeserializationResult<Vec<Self>> {
        re_tracing::profile_function!();
        Self::from_arrow2_opt(data)?
            .into_iter()
            .map(|opt| {
                opt.ok_or_else(|| crate::DeserializationError::MissingData {
                    backtrace: _Backtrace::new_unresolved(),
                })
            })
            .collect::<DeserializationResult<Vec<_>>>()
    }

    /// Given an Arrow array, deserializes it into a collection of optional [`Loggable`]s.
    fn from_arrow_opt(data: &arrow::array::ArrayData) -> DeserializationResult<Vec<Option<Self>>> {
        Self::from_arrow2_opt(arrow2::array::from_data(data).as_ref())
    }

    /// Given an Arrow2 array, deserializes it into a collection of optional [`Loggable`]s.
    fn from_arrow2_opt(
        data: &dyn arrow2::array::Array,
    ) -> DeserializationResult<Vec<Option<Self>>> {
        _ = data; // NOTE: do this here to avoid breaking users' autocomplete snippets
        Err(crate::DeserializationError::NotImplemented {
            fqname: "<unknown>".to_owned(),
            backtrace: _Backtrace::new_unresolved(),
        })
    }
}

/// A [`Component`] describes semantic data that can be used by any number of [`Archetype`]s.
///
/// Implementing the [`Component`] trait automatically derives the [`ComponentBatch`] implementation,
/// which makes it possible to work with lists' worth of data in a generic fashion.
pub trait Component: Loggable {
    /// The fully-qualified name of this component, e.g. `rerun.components.Position2D`.
    fn name() -> ComponentName;
}

// ---

pub type ComponentNameSet = std::collections::BTreeSet<ComponentName>;

re_string_interner::declare_new_type!(
    /// The fully-qualified name of a [`Component`], e.g. `rerun.components.Position2D`.
    pub struct ComponentName;
);

impl ComponentName {
    /// Returns the fully-qualified name, e.g. `rerun.components.Position2D`.
    ///
    /// This is the default `Display` implementation for [`ComponentName`].
    #[inline]
    pub fn full_name(&self) -> &'static str {
        self.0.as_str()
    }

    /// Returns the unqualified name, e.g. `Position2D`.
    ///
    /// Used for most UI elements.
    ///
    /// ```
    /// # use re_types_core::ComponentName;
    /// assert_eq!(ComponentName::from("rerun.components.Position2D").short_name(), "Position2D");
    /// ```
    #[inline]
    pub fn short_name(&self) -> &'static str {
        let full_name = self.0.as_str();
        if let Some(short_name) = full_name.strip_prefix("rerun.blueprint.components.") {
            short_name
        } else if let Some(short_name) = full_name.strip_prefix("rerun.components.") {
            short_name
        } else if let Some(short_name) = full_name.strip_prefix("rerun.controls.") {
            short_name
        } else if let Some(short_name) = full_name.strip_prefix("rerun.") {
            short_name
        } else {
            full_name
        }
    }

    /// Is this an indicator component for an archetype?
    pub fn is_indicator_component(&self) -> bool {
        (self.starts_with("rerun.components.") || self.starts_with("rerun.blueprint.components."))
            && self.ends_with("Indicator")
    }

    /// If this is an indicator component, for which archetype?
    pub fn indicator_component_archetype(&self) -> Option<String> {
        if let Some(name) = self.strip_prefix("rerun.components.") {
            if let Some(name) = name.strip_suffix("Indicator") {
                return Some(name.to_owned());
            }
        }
        None
    }

    /// Web URL to the Rerun documentation for this component.
    pub fn doc_url(&self) -> Option<String> {
        if let Some(archetype_name_pascal_case) = self.indicator_component_archetype() {
            // Link indicator components to their archetype.
            // This code should be correct as long as this url passes our link checker:
            // https://rerun.io/docs/reference/types/archetypes/line_strips3d

            let archetype_name_snake_case = re_case::to_snake_case(&archetype_name_pascal_case);
            let base_url = "https://rerun.io/docs/reference/types/archetypes";
            Some(format!("{base_url}/{archetype_name_snake_case}"))
        } else if let Some(component_name_pascal_case) =
            self.full_name().strip_prefix("rerun.components.")
        {
            // This code should be correct as long as this url passes our link checker:
            // https://rerun.io/docs/reference/types/components/line_strip2d

            let component_name_snake_case = re_case::to_snake_case(component_name_pascal_case);
            let base_url = "https://rerun.io/docs/reference/types/components";
            Some(format!("{base_url}/{component_name_snake_case}"))
        } else {
            None // A user component
        }
    }

    /// Determine if component matches a string
    ///
    /// Valid matches are case invariant matches of either the full name or the short name.
    pub fn matches(&self, other: &str) -> bool {
        self.0.as_str() == other
            || self.full_name().to_lowercase() == other.to_lowercase()
            || self.short_name().to_lowercase() == other.to_lowercase()
    }
}

// ---

impl crate::SizeBytes for ComponentName {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        0
    }
}

re_string_interner::declare_new_type!(
    /// The fully-qualified name of a [`Datatype`], e.g. `rerun.datatypes.Vec2D`.
    pub struct DatatypeName;
);

impl DatatypeName {
    /// Returns the fully-qualified name, e.g. `rerun.datatypes.Vec2D`.
    ///
    /// This is the default `Display` implementation for [`DatatypeName`].
    #[inline]
    pub fn full_name(&self) -> &'static str {
        self.0.as_str()
    }

    /// Returns the unqualified name, e.g. `Vec2D`.
    ///
    /// Used for most UI elements.
    ///
    /// ```
    /// # use re_types_core::DatatypeName;
    /// assert_eq!(DatatypeName::from("rerun.datatypes.Vec2D").short_name(), "Vec2D");
    /// ```
    #[inline]
    pub fn short_name(&self) -> &'static str {
        let full_name = self.0.as_str();
        if let Some(short_name) = full_name.strip_prefix("rerun.datatypes.") {
            short_name
        } else if let Some(short_name) = full_name.strip_prefix("rerun.") {
            short_name
        } else {
            full_name
        }
    }
}

impl crate::SizeBytes for DatatypeName {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        0
    }
}
