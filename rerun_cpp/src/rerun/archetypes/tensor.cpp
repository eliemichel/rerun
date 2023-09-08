// DO NOT EDIT!: This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs:56.
// Based on "crates/re_types/definitions/rerun/archetypes/tensor.fbs".

#include "tensor.hpp"

#include "../components/tensor_data.hpp"

namespace rerun {
    namespace archetypes {
        Result<std::vector<rerun::DataCell>> Tensor::to_data_cells() const {
            std::vector<rerun::DataCell> cells;
            cells.reserve(1);

            {
                const auto result = rerun::components::TensorData::to_data_cell(&data, 1);
                if (result.is_err()) {
                    return result.error;
                }
                cells.emplace_back(std::move(result.value));
            }

            return cells;
        }
    } // namespace archetypes
} // namespace rerun
