#include "component_type.hpp"
#include "c/rerun.h"
#include "string_utils.hpp"

#include <arrow/c/bridge.h>

namespace rerun {
    Result<ComponentTypeHandle> ComponentType::register_component() const {
        rr_component_type type;
        // TODO: is this fine? do i need to manually check for nulls or smth?
        type.descriptor.archetype_name = detail::to_rr_string(archetype_name);
        type.descriptor.archetype_field_name = detail::to_rr_string(archetype_field_name);
        type.descriptor.component_name = detail::to_rr_string(component_name);
        ARROW_RETURN_NOT_OK(arrow::ExportType(*arrow_datatype, &type.schema));

        rr_error error = {};
        auto handle = rr_register_component_type(type, &error);
        if (error.code != RR_ERROR_CODE_OK) {
            return Error(error);
        }

        return handle;
    }
} // namespace rerun
