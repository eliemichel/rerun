#pragma once

#include <memory>
#include <string_view>

#include "result.hpp"

namespace arrow {
    class DataType;
} // namespace arrow

namespace rerun {
    /// Handle to a registered component types.
    using ComponentTypeHandle = uint32_t;

    /// A type of component that can be registered.
    ///
    /// All built-in components automatically register their types lazily upon first serialization.
    struct ComponentType {
        std::string_view archetype_name;       // optional
        std::string_view archetype_field_name; // optional
        std::string_view component_name;
        const std::shared_ptr<arrow::DataType>& arrow_datatype;

        ComponentType(
            std::string_view archetype_name_, std::string_view archetype_field_name_,
            std::string_view component_name_,
            const std::shared_ptr<arrow::DataType>& arrow_datatype_
        )
            : archetype_name(archetype_name_),
              archetype_field_name(archetype_field_name_),
              component_name(component_name_),
              arrow_datatype(arrow_datatype_) {}

        // TODO: I'm really not a fan of having that around though, ngl
        ComponentType(
            std::string_view component_name_,
            const std::shared_ptr<arrow::DataType>& arrow_datatype_
        )
            : component_name(component_name_), arrow_datatype(arrow_datatype_) {}

        /// Registers a component type with the SDK.
        ///
        /// There is currently no deregistration mechanism.
        /// Ideally, this method is only ever called once per component type.
        Result<ComponentTypeHandle> register_component() const;
    };
} // namespace rerun
