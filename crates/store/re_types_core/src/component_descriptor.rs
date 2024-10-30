use std::borrow::Cow;

use crate::{ArchetypeFieldName, ArchetypeName, ComponentName, SizeBytes};

// TODO: might want to have a closer look at the hashing situation for this one.
//
/// A [`ComponentDescriptor`] fully describes the semantics of a column of data.
///
/// Every component is uniquely identified by its [`ComponentDescriptor`].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ComponentDescriptor {
    /// Optional name of the `Archetype` associated with this data.
    ///
    /// `None` if the data wasn't logged through an archetype.
    ///
    /// Example: `rerun.archetypes.Points3D`.
    pub archetype_name: Option<ArchetypeName>,

    /// Optional name of the field within `Archetype` associated with this data.
    ///
    /// `None` if the data wasn't logged through an archetype.
    ///
    /// Example: `positions`.
    pub archetype_field_name: Option<ArchetypeFieldName>,

    /// Semantic name associated with this data.
    ///
    /// This is fully implied by `archetype_name` and `archetype_field`, but
    /// included for semantic convenience.
    ///
    /// Example: `rerun.components.Position3D`.
    pub component_name: ComponentName,
}

impl std::fmt::Display for ComponentDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_any_string(false))
    }
}

impl From<ComponentDescriptor> for Cow<'static, ComponentDescriptor> {
    #[inline]
    fn from(descr: ComponentDescriptor) -> Self {
        Cow::Owned(descr)
    }
}

impl<'d> From<&'d ComponentDescriptor> for Cow<'d, ComponentDescriptor> {
    #[inline]
    fn from(descr: &'d ComponentDescriptor) -> Self {
        Cow::Borrowed(descr)
    }
}

impl ComponentDescriptor {
    fn to_any_string(&self, use_short_names: bool) -> String {
        let Self {
            archetype_name,
            archetype_field_name,
            component_name,
        } = self;

        let (archetype_name, component_name) = if use_short_names {
            (
                archetype_name.map(|s| s.short_name()),
                component_name.short_name(),
            )
        } else {
            (archetype_name.map(|s| s.as_str()), component_name.as_str())
        };

        match (archetype_name, component_name, archetype_field_name) {
            (None, component_name, None) => component_name.to_owned(),
            (Some(archetype_name), component_name, None) => {
                format!("{archetype_name}:{component_name}")
            }
            (None, component_name, Some(archetype_field_name)) => {
                format!("{component_name}#{archetype_field_name}")
            }
            (Some(archetype_name), component_name, Some(archetype_field_name)) => {
                format!("{archetype_name}:{component_name}#{archetype_field_name}")
            }
        }
    }

    // TODO: we need to make sure everything is always using full names btw.
    /// Returns the fully-qualified name, e.g. `rerun.archetypes.Points3D:rerun.components.Position3D#positions`.
    ///
    /// This is the default `Display` implementation for [`ComponentDescriptor`].
    #[inline]
    pub fn full_name(&self) -> String {
        self.to_string()
    }

    /// Returns the unqualified name, e.g. `Points3D:Position3D#positions`.
    #[inline]
    pub fn short_name(&self) -> String {
        self.to_any_string(true)
    }
}

impl SizeBytes for ComponentDescriptor {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        let Self {
            archetype_name,
            archetype_field_name,
            component_name,
        } = self;
        archetype_name.heap_size_bytes()
            + component_name.heap_size_bytes()
            + archetype_field_name.heap_size_bytes()
    }
}

impl ComponentDescriptor {
    #[inline]
    pub fn new(component_name: impl Into<ComponentName>) -> Self {
        let component_name = component_name.into();
        Self {
            archetype_name: None,
            archetype_field_name: None,
            component_name,
        }
    }

    #[inline]
    pub fn with_archetype_name(mut self, archetype_name: ArchetypeName) -> Self {
        self.archetype_name = Some(archetype_name);
        self
    }

    #[inline]
    pub fn with_archetype_field_name(mut self, archetype_field_name: ArchetypeFieldName) -> Self {
        self.archetype_field_name = Some(archetype_field_name);
        self
    }
}
