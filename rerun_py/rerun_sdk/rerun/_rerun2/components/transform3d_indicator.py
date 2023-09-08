# DO NOT EDIT!: This file was auto-generated by crates/re_types_builder/src/codegen/python.rs:277.

from __future__ import annotations

from typing import Sequence, Union

import pyarrow as pa
from attrs import define, field

from .._baseclasses import (
    BaseExtensionArray,
    BaseExtensionType,
)

__all__ = [
    "Transform3DIndicator",
    "Transform3DIndicatorArray",
    "Transform3DIndicatorArrayLike",
    "Transform3DIndicatorLike",
    "Transform3DIndicatorType",
]


@define
class Transform3DIndicator:
    """
    Indicator component for the `rerun.components.Transform3DIndicator` archetype.

    Indicator components are data-less components used to give some extra context.
    The Rerun Viewer can make use of them to provide better heuristics and even improve performance
    in some cases.
    """

    Transform3DIndicator: () = field(converter=())


Transform3DIndicatorLike = Transform3DIndicator
Transform3DIndicatorArrayLike = Union[
    Transform3DIndicator,
    Sequence[Transform3DIndicatorLike],
]


# --- Arrow support ---


class Transform3DIndicatorType(BaseExtensionType):
    def __init__(self) -> None:
        pa.ExtensionType.__init__(self, pa.null(), "rerun.components.Transform3DIndicator")


class Transform3DIndicatorArray(BaseExtensionArray[Transform3DIndicatorArrayLike]):
    _EXTENSION_NAME = "rerun.components.Transform3DIndicator"
    _EXTENSION_TYPE = Transform3DIndicatorType

    @staticmethod
    def _native_to_pa_array(data: Transform3DIndicatorArrayLike, data_type: pa.DataType) -> pa.Array:
        raise NotImplementedError  # You need to implement "transform3dindicator_native_to_pa_array" in rerun_py/rerun_sdk/rerun/_rerun2/components/_overrides/transform3d_indicator.py


Transform3DIndicatorType._ARRAY_TYPE = Transform3DIndicatorArray

# TODO(cmc): bring back registration to pyarrow once legacy types are gone
# pa.register_extension_type(Transform3DIndicatorType())
