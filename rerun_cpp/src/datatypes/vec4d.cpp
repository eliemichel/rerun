// NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.
// Based on "crates/re_types/definitions/rerun/datatypes/vec4d.fbs"

#include <arrow/api.h>

#include "vec4d.hpp"

namespace rr {
    namespace datatypes {
        std::shared_ptr<arrow::DataType> Vec4D::to_arrow_datatype() {
            return arrow::fixed_size_list(arrow::field("item", arrow::float32(), false, nullptr),
                                          4);
        }
    } // namespace datatypes
} // namespace rr
