// DO NOT EDIT!: This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs:56.
// Based on "crates/re_types/definitions/rerun/datatypes/utf8.fbs".

#include "utf8.hpp"

#include <arrow/builder.h>
#include <arrow/type_fwd.h>

namespace rerun {
    namespace datatypes {
        const std::shared_ptr<arrow::DataType>& Utf8::arrow_datatype() {
            static const auto datatype = arrow::utf8();
            return datatype;
        }

        Result<std::shared_ptr<arrow::StringBuilder>> Utf8::new_arrow_array_builder(
            arrow::MemoryPool* memory_pool
        ) {
            if (!memory_pool) {
                return Error(ErrorCode::UnexpectedNullArgument, "Memory pool is null.");
            }

            return Result(std::make_shared<arrow::StringBuilder>(memory_pool));
        }

        Error Utf8::fill_arrow_array_builder(
            arrow::StringBuilder* builder, const Utf8* elements, size_t num_elements
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

            ARROW_RETURN_NOT_OK(builder->Reserve(static_cast<int64_t>(num_elements)));
            for (size_t elem_idx = 0; elem_idx < num_elements; elem_idx += 1) {
                ARROW_RETURN_NOT_OK(builder->Append(elements[elem_idx].value));
            }

            return Error::ok();
        }
    } // namespace datatypes
} // namespace rerun
