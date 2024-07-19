# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/components/color_model.fbs".

# You can extend this class by creating a "ColorModelExt" class in "color_model_ext.py".

from __future__ import annotations

from typing import Literal, Sequence, Union

import pyarrow as pa

from .._baseclasses import (
    BaseBatch,
    BaseExtensionType,
    ComponentBatchMixin,
)

__all__ = ["ColorModel", "ColorModelArrayLike", "ColorModelBatch", "ColorModelLike", "ColorModelType"]


from enum import Enum


class ColorModel(Enum):
    """
    **Component**: Specified what color components are present in an [`archetypes.Image`][rerun.archetypes.Image].

    This combined with [`components.ChannelDatatype`][rerun.components.ChannelDatatype] determines the pixel format of an image.
    """

    L = 1
    """Grayscale luminance intencity/brightness/value, sometimes called `Y`"""

    RGB = 2
    """Red, Green, Blue"""

    RGBA = 3
    """Red, Green, Blue, Alpha"""

    def __str__(self) -> str:
        """Returns the variant name."""
        if self == ColorModel.L:
            return "L"
        elif self == ColorModel.RGB:
            return "RGB"
        elif self == ColorModel.RGBA:
            return "RGBA"
        else:
            raise ValueError("Unknown enum variant")


ColorModelLike = Union[ColorModel, Literal["L", "RGB", "RGBA", "l", "rgb", "rgba"]]
ColorModelArrayLike = Union[ColorModelLike, Sequence[ColorModelLike]]


class ColorModelType(BaseExtensionType):
    _TYPE_NAME: str = "rerun.components.ColorModel"

    def __init__(self) -> None:
        pa.ExtensionType.__init__(
            self,
            pa.sparse_union([
                pa.field("_null_markers", pa.null(), nullable=True, metadata={}),
                pa.field("L", pa.null(), nullable=True, metadata={}),
                pa.field("RGB", pa.null(), nullable=True, metadata={}),
                pa.field("RGBA", pa.null(), nullable=True, metadata={}),
            ]),
            self._TYPE_NAME,
        )


class ColorModelBatch(BaseBatch[ColorModelArrayLike], ComponentBatchMixin):
    _ARROW_TYPE = ColorModelType()

    @staticmethod
    def _native_to_pa_array(data: ColorModelArrayLike, data_type: pa.DataType) -> pa.Array:
        if isinstance(data, (ColorModel, int, str)):
            data = [data]

        types: list[int] = []

        for value in data:
            if value is None:
                types.append(0)
            elif isinstance(value, ColorModel):
                types.append(value.value)  # Actual enum value
            elif isinstance(value, int):
                types.append(value)  # By number
            elif isinstance(value, str):
                if hasattr(ColorModel, value):
                    types.append(ColorModel[value].value)  # fast path
                elif value.lower() == "l":
                    types.append(ColorModel.L.value)
                elif value.lower() == "rgb":
                    types.append(ColorModel.RGB.value)
                elif value.lower() == "rgba":
                    types.append(ColorModel.RGBA.value)
                else:
                    raise ValueError(f"Unknown ColorModel kind: {value}")
            else:
                raise ValueError(f"Unknown ColorModel kind: {value}")

        buffers = [
            None,
            pa.array(types, type=pa.int8()).buffers()[1],
        ]
        children = (1 + 3) * [pa.nulls(len(data))]

        return pa.UnionArray.from_buffers(
            type=data_type,
            length=len(data),
            buffers=buffers,
            children=children,
        )
