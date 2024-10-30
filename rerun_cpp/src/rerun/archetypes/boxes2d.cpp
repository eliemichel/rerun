// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/boxes2d.fbs".

#include "boxes2d.hpp"

#include "../collection_adapter_builtins.hpp"

namespace rerun::archetypes {}

namespace rerun {

    Result<std::vector<ComponentBatch>> AsComponents<archetypes::Boxes2D>::serialize(
        const archetypes::Boxes2D& archetype
    ) {
        using namespace archetypes;
        std::vector<ComponentBatch> cells;
        cells.reserve(9);

        {
            auto result = ComponentBatch::from_loggable(
                archetype.half_sizes,
                "rerun.archetypes.Boxes2D",
                "half_sizes"
            );
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        if (archetype.centers.has_value()) {
            auto result = ComponentBatch::from_loggable(
                archetype.centers.value(),
                "rerun.archetypes.Boxes2D",
                "centers"
            );
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        if (archetype.colors.has_value()) {
            auto result = ComponentBatch::from_loggable(
                archetype.colors.value(),
                "rerun.archetypes.Boxes2D",
                "colors"
            );
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        if (archetype.radii.has_value()) {
            auto result = ComponentBatch::from_loggable(
                archetype.radii.value(),
                "rerun.archetypes.Boxes2D",
                "radii"
            );
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        if (archetype.labels.has_value()) {
            auto result = ComponentBatch::from_loggable(
                archetype.labels.value(),
                "rerun.archetypes.Boxes2D",
                "labels"
            );
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        if (archetype.show_labels.has_value()) {
            auto result = ComponentBatch::from_loggable(
                archetype.show_labels.value(),
                "rerun.archetypes.Boxes2D",
                "show_labels"
            );
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        if (archetype.draw_order.has_value()) {
            auto result = ComponentBatch::from_loggable(
                archetype.draw_order.value(),
                "rerun.archetypes.Boxes2D",
                "draw_order"
            );
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        if (archetype.class_ids.has_value()) {
            auto result = ComponentBatch::from_loggable(
                archetype.class_ids.value(),
                "rerun.archetypes.Boxes2D",
                "class_ids"
            );
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        {
            auto indicator = Boxes2D::IndicatorComponent();
            auto result = ComponentBatch::from_loggable(indicator, "rerun.archetypes.Boxes2D");
            RR_RETURN_NOT_OK(result.error);
            cells.emplace_back(std::move(result.value));
        }

        return cells;
    }
} // namespace rerun
