// DO NOT EDIT!: This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs:56.
// Based on "crates/re_types/definitions/rerun/components/annotation_context.fbs".

#pragma once

#include "../data_cell.hpp"
#include "../datatypes/class_description_map_elem.hpp"
#include "../result.hpp"

#include <cstdint>
#include <memory>
#include <utility>
#include <vector>

namespace arrow {
    class DataType;
    class ListBuilder;
    class MemoryPool;
} // namespace arrow

namespace rerun {
    namespace components {
        /// The `AnnotationContext` provides additional information on how to display entities.
        ///
        /// Entities can use `ClassId`s and `KeypointId`s to provide annotations, and
        /// the labels and colors will be looked up in the appropriate
        ///`AnnotationContext`. We use the *first* annotation context we find in the
        /// path-hierarchy when searching up through the ancestors of a given entity
        /// path.
        struct AnnotationContext {
            std::vector<rerun::datatypes::ClassDescriptionMapElem> class_map;

            /// Name of the component, used for serialization.
            static const char* NAME;

          public:
            // Extensions to generated type defined in 'annotation_context_ext.cpp'

            AnnotationContext(
                std::initializer_list<rerun::datatypes::ClassDescription> class_descriptions
            ) {
                class_map.reserve(class_descriptions.size());
                for (const auto& class_description : class_descriptions) {
                    class_map.emplace_back(std::move(class_description));
                }
            }

          public:
            AnnotationContext() = default;

            AnnotationContext(std::vector<rerun::datatypes::ClassDescriptionMapElem> _class_map)
                : class_map(std::move(_class_map)) {}

            AnnotationContext& operator=(
                std::vector<rerun::datatypes::ClassDescriptionMapElem> _class_map
            ) {
                class_map = std::move(_class_map);
                return *this;
            }

            /// Returns the arrow data type this type corresponds to.
            static const std::shared_ptr<arrow::DataType>& arrow_datatype();

            /// Creates a new array builder with an array of this type.
            static Result<std::shared_ptr<arrow::ListBuilder>> new_arrow_array_builder(
                arrow::MemoryPool* memory_pool
            );

            /// Fills an arrow array builder with an array of this type.
            static Error fill_arrow_array_builder(
                arrow::ListBuilder* builder, const AnnotationContext* elements, size_t num_elements
            );

            /// Creates a Rerun DataCell from an array of AnnotationContext components.
            static Result<rerun::DataCell> to_data_cell(
                const AnnotationContext* instances, size_t num_instances
            );
        };
    } // namespace components
} // namespace rerun
