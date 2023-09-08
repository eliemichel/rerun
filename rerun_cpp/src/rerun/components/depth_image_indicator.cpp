// DO NOT EDIT!: This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs:56.
// Based on "crates/re_types/definitions/rerun/archetypes/depth_image.fbs".

#include "depth_image_indicator.hpp"

#include "../arrow.hpp"

#include <arrow/builder.h>
#include <arrow/table.h>
#include <arrow/type_fwd.h>

namespace rerun {
    namespace components {
        const char* DepthImageIndicator::NAME = "rerun.components.DepthImageIndicator";

        const std::shared_ptr<arrow::DataType>& DepthImageIndicator::arrow_datatype() {
            static const auto datatype = arrow::null();
            return datatype;
        }

        Result<std::shared_ptr<arrow::NullBuilder>> DepthImageIndicator::new_arrow_array_builder(
            arrow::MemoryPool* memory_pool
        ) {
            if (!memory_pool) {
                return Error(ErrorCode::UnexpectedNullArgument, "Memory pool is null.");
            }

            return Result(std::make_shared<arrow::NullBuilder>(memory_pool));
        }

        Error DepthImageIndicator::fill_arrow_array_builder(
            arrow::NullBuilder* builder, const DepthImageIndicator* elements, size_t num_elements
        ) {
            if (!builder) {
                return Error(ErrorCode::UnexpectedNullArgument, "Passed array builder is null.");
            }
            if (!elements) {
                return Error(
                    ErrorCode::UnexpectedNullArgument,
                    "Cannot serialize null pointer to arrow array."
                );
            }

            ARROW_RETURN_NOT_OK(builder->AppendNulls(static_cast<int64_t>(num_elements)));

            return Error::ok();
        }

        Result<rerun::DataCell> DepthImageIndicator::to_data_cell(
            const DepthImageIndicator* instances, size_t num_instances
        ) {
            // TODO(andreas): Allow configuring the memory pool.
            arrow::MemoryPool* pool = arrow::default_memory_pool();

            auto builder_result = DepthImageIndicator::new_arrow_array_builder(pool);
            RR_RETURN_NOT_OK(builder_result.error);
            auto builder = std::move(builder_result.value);
            if (instances && num_instances > 0) {
                RR_RETURN_NOT_OK(DepthImageIndicator::fill_arrow_array_builder(
                    builder.get(),
                    instances,
                    num_instances
                ));
            }
            std::shared_ptr<arrow::Array> array;
            ARROW_RETURN_NOT_OK(builder->Finish(&array));

            auto schema = arrow::schema({arrow::field(
                DepthImageIndicator::NAME,
                DepthImageIndicator::arrow_datatype(),
                false
            )});

            rerun::DataCell cell;
            cell.component_name = DepthImageIndicator::NAME;
            const auto ipc_result = rerun::ipc_from_table(*arrow::Table::Make(schema, {array}));
            RR_RETURN_NOT_OK(ipc_result.error);
            cell.buffer = std::move(ipc_result.value);

            return cell;
        }
    } // namespace components
} // namespace rerun
