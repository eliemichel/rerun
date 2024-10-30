use rerun::{ChunkStore, ChunkStoreConfig, Component as _, ComponentDescriptor, VersionPolicy};

#[allow(clippy::unwrap_used)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    const APP_ID: &str = "rerun_example_descriptors_builtin_component_vanilla";

    let rec = rerun::RecordingStreamBuilder::new(APP_ID).spawn()?;

    rec.log_component_batches(
        "data",
        true,
        [&rerun::components::Position3D::new(1.0, 2.0, 3.0) as &dyn rerun::ComponentBatch],
    )?;

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
            ComponentDescriptor {
                archetype_name: None,
                archetype_field_name: None,
                component_name: rerun::components::Position3D::name(),
            }, //
        ];

        similar_asserts::assert_eq!(expected, descriptors);
    }

    Ok(())
}
