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
    "LineStrips2DIndicator",
    "LineStrips2DIndicatorArray",
    "LineStrips2DIndicatorArrayLike",
    "LineStrips2DIndicatorLike",
    "LineStrips2DIndicatorType",
]


@define
class LineStrips2DIndicator:
    """
    Indicator component for the `rerun.components.LineStrips2DIndicator` archetype.

    Indicator components are data-less components used to give some extra context.
    The Rerun Viewer can make use of them to provide better heuristics and even improve performance
    in some cases.
    """

    LineStrips2DIndicator: () = field(converter=())


LineStrips2DIndicatorLike = LineStrips2DIndicator
LineStrips2DIndicatorArrayLike = Union[
    LineStrips2DIndicator,
    Sequence[LineStrips2DIndicatorLike],
]


# --- Arrow support ---


class LineStrips2DIndicatorType(BaseExtensionType):
    def __init__(self) -> None:
        pa.ExtensionType.__init__(self, pa.null(), "rerun.components.LineStrips2DIndicator")


class LineStrips2DIndicatorArray(BaseExtensionArray[LineStrips2DIndicatorArrayLike]):
    _EXTENSION_NAME = "rerun.components.LineStrips2DIndicator"
    _EXTENSION_TYPE = LineStrips2DIndicatorType

    @staticmethod
    def _native_to_pa_array(data: LineStrips2DIndicatorArrayLike, data_type: pa.DataType) -> pa.Array:
        raise NotImplementedError  # You need to implement "linestrips2dindicator_native_to_pa_array" in rerun_py/rerun_sdk/rerun/_rerun2/components/_overrides/line_strips2d_indicator.py


LineStrips2DIndicatorType._ARRAY_TYPE = LineStrips2DIndicatorArray

# TODO(cmc): bring back registration to pyarrow once legacy types are gone
# pa.register_extension_type(LineStrips2DIndicatorType())
