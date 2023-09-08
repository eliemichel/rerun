// DO NOT EDIT!: This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs:56.
// Based on "crates/re_types/definitions/rerun/archetypes/depth_image.fbs".

#pragma once

#include "../data_cell.hpp"
#include "../result.hpp"

#include <cstdint>
#include <memory>

namespace arrow {
    class DataType;
    class MemoryPool;
    class NullBuilder;
} // namespace arrow

namespace rerun {
    namespace components {
        /// Indicator component for the `rerun.components.DepthImageIndicator` archetype.
        ///
        /// Indicator components are data-less components used to give some extra context.
        /// The Rerun Viewer can make use of them to provide better heuristics and even improve
        /// performance in some cases.
        struct DepthImageIndicator {
            /// Name of the component, used for serialization.
            static const char* NAME;

          public:
            DepthImageIndicator() = default;

            /// Returns the arrow data type this type corresponds to.
            static const std::shared_ptr<arrow::DataType>& arrow_datatype();

            /// Creates a new array builder with an array of this type.
            static Result<std::shared_ptr<arrow::NullBuilder>> new_arrow_array_builder(
                arrow::MemoryPool* memory_pool
            );

            /// Fills an arrow array builder with an array of this type.
            static Error fill_arrow_array_builder(
                arrow::NullBuilder* builder, const DepthImageIndicator* elements,
                size_t num_elements
            );

            /// Creates a Rerun DataCell from an array of DepthImageIndicator components.
            static Result<rerun::DataCell> to_data_cell(
                const DepthImageIndicator* instances, size_t num_instances
            );
        };
    } // namespace components
} // namespace rerun
