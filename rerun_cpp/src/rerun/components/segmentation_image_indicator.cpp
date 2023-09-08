// DO NOT EDIT!: This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs:56.
// Based on "crates/re_types/definitions/rerun/archetypes/segmentation_image.fbs".

#include "segmentation_image_indicator.hpp"

#include "../arrow.hpp"

#include <arrow/builder.h>
#include <arrow/table.h>
#include <arrow/type_fwd.h>

namespace rerun {
    namespace components {
        const char* SegmentationImageIndicator::NAME =
            "rerun.components.SegmentationImageIndicator";

        const std::shared_ptr<arrow::DataType>& SegmentationImageIndicator::arrow_datatype() {
            static const auto datatype = arrow::null();
            return datatype;
        }

        Result<std::shared_ptr<arrow::NullBuilder>>
            SegmentationImageIndicator::new_arrow_array_builder(arrow::MemoryPool* memory_pool) {
            if (!memory_pool) {
                return Error(ErrorCode::UnexpectedNullArgument, "Memory pool is null.");
            }

            return Result(std::make_shared<arrow::NullBuilder>(memory_pool));
        }

        Error SegmentationImageIndicator::fill_arrow_array_builder(
            arrow::NullBuilder* builder, const SegmentationImageIndicator* elements,
            size_t num_elements
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

        Result<rerun::DataCell> SegmentationImageIndicator::to_data_cell(
            const SegmentationImageIndicator* instances, size_t num_instances
        ) {
            // TODO(andreas): Allow configuring the memory pool.
            arrow::MemoryPool* pool = arrow::default_memory_pool();

            auto builder_result = SegmentationImageIndicator::new_arrow_array_builder(pool);
            RR_RETURN_NOT_OK(builder_result.error);
            auto builder = std::move(builder_result.value);
            if (instances && num_instances > 0) {
                RR_RETURN_NOT_OK(SegmentationImageIndicator::fill_arrow_array_builder(
                    builder.get(),
                    instances,
                    num_instances
                ));
            }
            std::shared_ptr<arrow::Array> array;
            ARROW_RETURN_NOT_OK(builder->Finish(&array));

            auto schema = arrow::schema({arrow::field(
                SegmentationImageIndicator::NAME,
                SegmentationImageIndicator::arrow_datatype(),
                false
            )});

            rerun::DataCell cell;
            cell.component_name = SegmentationImageIndicator::NAME;
            const auto ipc_result = rerun::ipc_from_table(*arrow::Table::Make(schema, {array}));
            RR_RETURN_NOT_OK(ipc_result.error);
            cell.buffer = std::move(ipc_result.value);

            return cell;
        }
    } // namespace components
} // namespace rerun
