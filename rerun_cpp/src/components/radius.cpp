// NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.
// Based on "crates/re_types/definitions/rerun/components/radius.fbs"

#include <arrow/api.h>

#include "radius.hpp"

namespace rr {
    namespace components {
        std::shared_ptr<arrow::DataType> Radius::to_arrow_datatype() {
            return arrow::float32();
        }
    } // namespace components
} // namespace rr
