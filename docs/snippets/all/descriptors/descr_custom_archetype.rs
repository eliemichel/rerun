use rerun::{
    external::arrow2, ChunkStore, ChunkStoreConfig, Component, ComponentDescriptor, Loggable,
    VersionPolicy,
};

#[derive(Debug, Clone, Copy)]
struct CustomPosition3D(rerun::components::Position3D);

impl rerun::SizeBytes for CustomPosition3D {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        0
    }
}

impl Loggable for CustomPosition3D {
    #[inline]
    fn arrow_datatype() -> arrow2::datatypes::DataType {
        rerun::components::Position3D::arrow_datatype()
    }

    #[inline]
    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<std::borrow::Cow<'a, Self>>>>,
    ) -> rerun::SerializationResult<Box<dyn arrow2::array::Array>>
    where
        Self: 'a,
    {
        rerun::components::Position3D::to_arrow_opt(
            data.into_iter().map(|opt| opt.map(Into::into).map(|c| c.0)),
        )
    }
}

impl Component for CustomPosition3D {
    #[inline]
    fn descriptor() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("user.CustomArchetype".into()),
            archetype_field_name: Some("user.CustomArchetypeField".into()),
            component_name: "user.CustomPosition3D".into(),
        }
    }
}

// TODO: both a builtin component with an overridden descriptor as well as fully custom component

// TODO: _archetype is the wrong name for this test -- nobody cares about archetypes, it's all about AsComponents.

struct CustomPoints3D {
    positions: Vec<CustomPosition3D>,
    colors: Option<Vec<rerun::components::Color>>,
}

impl CustomPoints3D {
    fn indicator() -> rerun::NamedIndicatorComponent {
        rerun::NamedIndicatorComponent("user.CustomPoints3DIndicator".into())
    }

    fn overridden_color_descriptor() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("user.CustomArchetype".into()),
            archetype_field_name: Some("user.CustomArchetypeField".into()),
            component_name: rerun::components::Color::name(),
        }
    }
}

impl rerun::AsComponents for CustomPoints3D {
    fn as_component_batches(&self) -> Vec<rerun::MaybeOwnedComponentBatch<'_>> {
        [
            Some(Self::indicator().to_batch()),
            Some(rerun::MaybeOwnedComponentBatch::new(
                &self.positions as &dyn rerun::ComponentBatch,
            )),
            self.colors.as_ref().map(|colors| {
                rerun::MaybeOwnedComponentBatch::new(colors as &dyn rerun::ComponentBatch)
                    .with_descriptor_override(Self::overridden_color_descriptor())
            }),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

#[allow(clippy::unwrap_used)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    const APP_ID: &str = "rerun_example_descriptors_custom_component_vanilla";

    let rec = rerun::RecordingStreamBuilder::new(APP_ID).spawn()?;

    let position = CustomPosition3D(rerun::components::Position3D::new(1.0, 2.0, 3.0));
    let color = rerun::components::Color::new(0xFF00FFFF);

    let points = CustomPoints3D {
        positions: vec![position],
        colors: Some(vec![color]),
    };

    rec.log_static("data", &points as _)?;

    // When this snippet runs through the snippet comparison machinery, this environment variable
    // will point to the output RRD.
    // We can thus load this RRD to check that the proper tags were indeed forwarded.
    //
    // Python and C++ are indirectly check by the snippet comparison itself.
    if let Ok(path_to_rrd) = std::env::var("_RERUN_TEST_FORCE_SAVE") {
        rec.flush_blocking();

        let stores = ChunkStore::from_rrd_filepath(
            &ChunkStoreConfig::ALL_DISABLED,
            path_to_rrd,
            VersionPolicy::Warn,
        )?;
        assert_eq!(1, stores.len());

        let store = stores.into_values().next().unwrap();
        let chunks = store.iter_chunks().collect::<Vec<_>>();
        assert_eq!(1, chunks.len());

        let chunk = chunks.into_iter().next().unwrap();

        let descriptors = chunk
            .components()
            .values()
            .flat_map(|per_desc| per_desc.keys())
            .cloned()
            .collect::<Vec<_>>();

        let expected = vec![
            CustomPoints3D::overridden_color_descriptor(),
            rerun::ComponentBatch::descriptor(&CustomPoints3D::indicator()).into_owned(),
            CustomPosition3D::descriptor(),
        ];

        similar_asserts::assert_eq!(expected, descriptors);
    }

    Ok(())
}
