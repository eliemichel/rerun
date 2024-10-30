#!/usr/bin/env python3

from __future__ import annotations

from typing import Any

import pyarrow as pa
import rerun as rr  # pip install rerun-sdk


class CustomPosition3DBatch(rr.ComponentBatchLike):
    def __init__(self: Any, position: rr.components.Position3DBatch) -> None:
        self.position = position

    def component_name(self) -> str:
        return "user.CustomPosition3D"

    def as_arrow_array(self) -> pa.Array:
        return self.position.as_arrow_array()


rr.init("rerun_example_descriptors_custom_component_vanilla")
rr.spawn()

position = CustomPosition3DBatch(rr.components.Position3DBatch([1, 2, 3]))
rr.log_components("data", [position], static=True)

# The tags are indirectly checked by the Rust version (have a look over there for more info).
