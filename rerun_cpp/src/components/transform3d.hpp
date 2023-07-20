// NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.
// Based on "crates/re_types/definitions/rerun/components/transform3d.fbs"

#pragma once

#include <cstdint>
#include <memory>
#include <utility>

#include "../datatypes/transform3d.hpp"

namespace arrow {
    class DataType;
}

namespace rr {
    namespace components {
        /// An affine transform between two 3D spaces, represented in a given direction.
        struct Transform3D {
            /// Representation of the transform.
            rr::datatypes::Transform3D repr;

          public:
            Transform3D(rr::datatypes::Transform3D repr) : repr(std::move(repr)) {}

            /// Returns the arrow data type this type corresponds to.
            static std::shared_ptr<arrow::DataType> to_arrow_datatype();
        };
    } // namespace components
} // namespace rr
