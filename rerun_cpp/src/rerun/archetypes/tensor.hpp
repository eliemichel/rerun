// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/archetypes/tensor.fbs".

#pragma once

#include "../component_batch.hpp"
#include "../components/tensor_data.hpp"
#include "../data_cell.hpp"
#include "../error.hpp"
#include "../indicator_component.hpp"
#include "../result.hpp"

#include <cstdint>
#include <utility>
#include <vector>

namespace rerun {
    namespace archetypes {
        /// **Archetype**: A generic n-dimensional Tensor.
        ///
        /// ## Example
        ///
        /// ### Simple Tensor
        /// ```cpp,ignore
        /// #include <rerun.hpp>
        ///
        /// #include <random>
        ///
        /// int main() {
        ///     auto rec = rerun::RecordingStream("rerun_example_tensor_simple");
        ///     rec.spawn().throw_on_failure();
        ///
        ///     std::default_random_engine gen;
        ///     // On MSVC uint8_t distributions are not supported.
        ///     std::uniform_int_distribution<int> dist(0, 255);
        ///
        ///     std::vector<uint8_t> data(8 * 6 * 3 * 5);
        ///     std::generate(data.begin(), data.end(), [&] { return static_cast<uint8_t>(dist(gen)); });
        ///
        ///     rec.log(
        ///         "tensor",
        ///         rerun::Tensor({8, 6, 3, 5}, data).with_dim_names({"batch", "channel", "height", "width"})
        ///     );
        /// }
        /// ```
        struct Tensor {
            /// The tensor data
            rerun::components::TensorData data;

            /// Name of the indicator component, used to identify the archetype when converting to a list of components.
            static const char INDICATOR_COMPONENT_NAME[];
            /// Indicator component, used to identify the archetype when converting to a list of components.
            using IndicatorComponent = components::IndicatorComponent<INDICATOR_COMPONENT_NAME>;

          public:
            // Extensions to generated type defined in 'tensor_ext.cpp'

            /// New Tensor from dimensions and tensor buffer.
            Tensor(
                std::vector<rerun::datatypes::TensorDimension> shape,
                rerun::datatypes::TensorBuffer buffer
            )
                : Tensor(rerun::datatypes::TensorData(std::move(shape), std::move(buffer))) {}

            /// Update the `names` of the contained [`TensorData`] dimensions.
            ///
            /// Any existing Dimension names will be be overwritten.
            ///
            /// If too many, or too few names are provided, this function will call
            /// Error::handle and then proceed to only update the subset of names that it can.
            ///
            /// TODO(#3794): don't use std::vector here.
            Tensor with_dim_names(std::vector<std::string> names) &&;

          public:
            Tensor() = default;
            Tensor(Tensor&& other) = default;

            explicit Tensor(rerun::components::TensorData _data) : data(std::move(_data)) {}

            /// Returns the number of primary instances of this archetype.
            size_t num_instances() const {
                return 1;
            }
        };

    } // namespace archetypes

    template <typename T>
    struct AsComponents;

    template <>
    struct AsComponents<archetypes::Tensor> {
        /// Serialize all set component batches.
        static Result<std::vector<SerializedComponentBatch>> serialize(
            const archetypes::Tensor& archetype
        );
    };
} // namespace rerun
