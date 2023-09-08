// DO NOT EDIT!: This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs:56.
// Based on "crates/re_types/definitions/rerun/testing/components/fuzzy.fbs".

#include "affix_fuzzer7.hpp"

#include "../datatypes/affix_fuzzer1.hpp"

#include <arrow/builder.h>
#include <arrow/table.h>
#include <arrow/type_fwd.h>
#include <rerun/arrow.hpp>

namespace rerun {
    namespace components {
        const char *AffixFuzzer7::NAME = "rerun.testing.components.AffixFuzzer7";

        const std::shared_ptr<arrow::DataType> &AffixFuzzer7::arrow_datatype() {
            static const auto datatype = arrow::list(
                arrow::field("item", rerun::datatypes::AffixFuzzer1::arrow_datatype(), false)
            );
            return datatype;
        }

        Result<std::shared_ptr<arrow::ListBuilder>> AffixFuzzer7::new_arrow_array_builder(
            arrow::MemoryPool *memory_pool
        ) {
            if (!memory_pool) {
                return Error(ErrorCode::UnexpectedNullArgument, "Memory pool is null.");
            }

            return Result(std::make_shared<arrow::ListBuilder>(
                memory_pool,
                rerun::datatypes::AffixFuzzer1::new_arrow_array_builder(memory_pool).value
            ));
        }

        Error AffixFuzzer7::fill_arrow_array_builder(
            arrow::ListBuilder *builder, const AffixFuzzer7 *elements, size_t num_elements
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

            auto value_builder = static_cast<arrow::StructBuilder *>(builder->value_builder());
            ARROW_RETURN_NOT_OK(builder->Reserve(static_cast<int64_t>(num_elements)));
            ARROW_RETURN_NOT_OK(value_builder->Reserve(static_cast<int64_t>(num_elements * 1)));

            for (size_t elem_idx = 0; elem_idx < num_elements; elem_idx += 1) {
                const auto &element = elements[elem_idx];
                if (element.many_optional.has_value()) {
                    ARROW_RETURN_NOT_OK(builder->Append());
                    if (element.many_optional.value().data()) {
                        RR_RETURN_NOT_OK(rerun::datatypes::AffixFuzzer1::fill_arrow_array_builder(
                            value_builder,
                            element.many_optional.value().data(),
                            element.many_optional.value().size()
                        ));
                    }
                } else {
                    ARROW_RETURN_NOT_OK(builder->AppendNull());
                }
            }

            return Error::ok();
        }

        Result<rerun::DataCell> AffixFuzzer7::to_data_cell(
            const AffixFuzzer7 *instances, size_t num_instances
        ) {
            // TODO(andreas): Allow configuring the memory pool.
            arrow::MemoryPool *pool = arrow::default_memory_pool();

            auto builder_result = AffixFuzzer7::new_arrow_array_builder(pool);
            RR_RETURN_NOT_OK(builder_result.error);
            auto builder = std::move(builder_result.value);
            if (instances && num_instances > 0) {
                RR_RETURN_NOT_OK(
                    AffixFuzzer7::fill_arrow_array_builder(builder.get(), instances, num_instances)
                );
            }
            std::shared_ptr<arrow::Array> array;
            ARROW_RETURN_NOT_OK(builder->Finish(&array));

            auto schema = arrow::schema(
                {arrow::field(AffixFuzzer7::NAME, AffixFuzzer7::arrow_datatype(), false)}
            );

            rerun::DataCell cell;
            cell.component_name = AffixFuzzer7::NAME;
            const auto ipc_result = rerun::ipc_from_table(*arrow::Table::Make(schema, {array}));
            RR_RETURN_NOT_OK(ipc_result.error);
            cell.buffer = std::move(ipc_result.value);

            return cell;
        }
    } // namespace components
} // namespace rerun
