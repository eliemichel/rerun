// NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.
// Based on "crates/re_types/definitions/rerun/testing/components/fuzzy.fbs"

#include <arrow/api.h>

#include "affix_fuzzer8.hpp"

namespace rr {
    namespace components {
        std::shared_ptr<arrow::DataType> AffixFuzzer8::to_arrow_datatype() {
            return arrow::float32();
        }
    } // namespace components
} // namespace rr
