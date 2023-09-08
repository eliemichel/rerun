// DO NOT EDIT!: This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs:56.
// Based on "crates/re_types/definitions/rerun/datatypes/tensor_buffer.fbs".

#pragma once

#include "../result.hpp"

#include <cstdint>
#include <cstring>
#include <memory>
#include <new>
#include <utility>
#include <vector>

namespace arrow {
    class DataType;
    class DenseUnionBuilder;
    class MemoryPool;
} // namespace arrow

namespace rerun {
    namespace datatypes {
        namespace detail {
            enum class TensorBufferTag : uint8_t {
                /// Having a special empty state makes it possible to implement move-semantics. We
                /// need to be able to leave the object in a state which we can run the destructor
                /// on.
                NONE = 0,
                U8,
                U16,
                U32,
                U64,
                I8,
                I16,
                I32,
                I64,
                F32,
                F64,
                JPEG,
            };

            union TensorBufferData {
                std::vector<uint8_t> u8;

                std::vector<uint16_t> u16;

                std::vector<uint32_t> u32;

                std::vector<uint64_t> u64;

                std::vector<int8_t> i8;

                std::vector<int16_t> i16;

                std::vector<int32_t> i32;

                std::vector<int64_t> i64;

                std::vector<float> f32;

                std::vector<double> f64;

                std::vector<uint8_t> jpeg;

                TensorBufferData() {}

                ~TensorBufferData() {}

                void swap(TensorBufferData &other) noexcept {
                    // This bitwise swap would fail for self-referential types, but we don't have
                    // any of those.
                    char temp[sizeof(TensorBufferData)];
                    void *otherbytes = reinterpret_cast<void *>(&other);
                    void *thisbytes = reinterpret_cast<void *>(this);
                    std::memcpy(temp, thisbytes, sizeof(TensorBufferData));
                    std::memcpy(thisbytes, otherbytes, sizeof(TensorBufferData));
                    std::memcpy(otherbytes, temp, sizeof(TensorBufferData));
                }
            };
        } // namespace detail

        /// The underlying storage for a `Tensor`.
        ///
        /// Tensor elements are stored in a contiguous buffer of a single type.
        struct TensorBuffer {
            TensorBuffer() : _tag(detail::TensorBufferTag::NONE) {}

            TensorBuffer(const TensorBuffer &other) : _tag(other._tag) {
                switch (other._tag) {
                    case detail::TensorBufferTag::U8: {
                        _data.u8 = other._data.u8;
                        break;
                    }
                    case detail::TensorBufferTag::U16: {
                        _data.u16 = other._data.u16;
                        break;
                    }
                    case detail::TensorBufferTag::U32: {
                        _data.u32 = other._data.u32;
                        break;
                    }
                    case detail::TensorBufferTag::U64: {
                        _data.u64 = other._data.u64;
                        break;
                    }
                    case detail::TensorBufferTag::I8: {
                        _data.i8 = other._data.i8;
                        break;
                    }
                    case detail::TensorBufferTag::I16: {
                        _data.i16 = other._data.i16;
                        break;
                    }
                    case detail::TensorBufferTag::I32: {
                        _data.i32 = other._data.i32;
                        break;
                    }
                    case detail::TensorBufferTag::I64: {
                        _data.i64 = other._data.i64;
                        break;
                    }
                    case detail::TensorBufferTag::F32: {
                        _data.f32 = other._data.f32;
                        break;
                    }
                    case detail::TensorBufferTag::F64: {
                        _data.f64 = other._data.f64;
                        break;
                    }
                    case detail::TensorBufferTag::JPEG: {
                        _data.jpeg = other._data.jpeg;
                        break;
                    }
                    case detail::TensorBufferTag::NONE:
                        const void *otherbytes = reinterpret_cast<const void *>(&other._data);
                        void *thisbytes = reinterpret_cast<void *>(&this->_data);
                        std::memcpy(thisbytes, otherbytes, sizeof(detail::TensorBufferData));
                        break;
                }
            }

            TensorBuffer &operator=(const TensorBuffer &other) noexcept {
                TensorBuffer tmp(other);
                this->swap(tmp);
                return *this;
            }

            TensorBuffer(TensorBuffer &&other) noexcept : _tag(detail::TensorBufferTag::NONE) {
                this->swap(other);
            }

            TensorBuffer &operator=(TensorBuffer &&other) noexcept {
                this->swap(other);
                return *this;
            }

            ~TensorBuffer() {
                switch (this->_tag) {
                    case detail::TensorBufferTag::NONE: {
                        break; // Nothing to destroy
                    }
                    case detail::TensorBufferTag::U8: {
                        typedef std::vector<uint8_t> TypeAlias;
                        _data.u8.~TypeAlias();
                        break;
                    }
                    case detail::TensorBufferTag::U16: {
                        typedef std::vector<uint16_t> TypeAlias;
                        _data.u16.~TypeAlias();
                        break;
                    }
                    case detail::TensorBufferTag::U32: {
                        typedef std::vector<uint32_t> TypeAlias;
                        _data.u32.~TypeAlias();
                        break;
                    }
                    case detail::TensorBufferTag::U64: {
                        typedef std::vector<uint64_t> TypeAlias;
                        _data.u64.~TypeAlias();
                        break;
                    }
                    case detail::TensorBufferTag::I8: {
                        typedef std::vector<int8_t> TypeAlias;
                        _data.i8.~TypeAlias();
                        break;
                    }
                    case detail::TensorBufferTag::I16: {
                        typedef std::vector<int16_t> TypeAlias;
                        _data.i16.~TypeAlias();
                        break;
                    }
                    case detail::TensorBufferTag::I32: {
                        typedef std::vector<int32_t> TypeAlias;
                        _data.i32.~TypeAlias();
                        break;
                    }
                    case detail::TensorBufferTag::I64: {
                        typedef std::vector<int64_t> TypeAlias;
                        _data.i64.~TypeAlias();
                        break;
                    }
                    case detail::TensorBufferTag::F32: {
                        typedef std::vector<float> TypeAlias;
                        _data.f32.~TypeAlias();
                        break;
                    }
                    case detail::TensorBufferTag::F64: {
                        typedef std::vector<double> TypeAlias;
                        _data.f64.~TypeAlias();
                        break;
                    }
                    case detail::TensorBufferTag::JPEG: {
                        typedef std::vector<uint8_t> TypeAlias;
                        _data.jpeg.~TypeAlias();
                        break;
                    }
                }
            }

            void swap(TensorBuffer &other) noexcept {
                auto tag_temp = this->_tag;
                this->_tag = other._tag;
                other._tag = tag_temp;
                this->_data.swap(other._data);
            }

            static TensorBuffer u8(std::vector<uint8_t> u8) {
                typedef std::vector<uint8_t> TypeAlias;
                TensorBuffer self;
                self._tag = detail::TensorBufferTag::U8;
                new (&self._data.u8) TypeAlias(std::move(u8));
                return self;
            }

            static TensorBuffer u16(std::vector<uint16_t> u16) {
                typedef std::vector<uint16_t> TypeAlias;
                TensorBuffer self;
                self._tag = detail::TensorBufferTag::U16;
                new (&self._data.u16) TypeAlias(std::move(u16));
                return self;
            }

            static TensorBuffer u32(std::vector<uint32_t> u32) {
                typedef std::vector<uint32_t> TypeAlias;
                TensorBuffer self;
                self._tag = detail::TensorBufferTag::U32;
                new (&self._data.u32) TypeAlias(std::move(u32));
                return self;
            }

            static TensorBuffer u64(std::vector<uint64_t> u64) {
                typedef std::vector<uint64_t> TypeAlias;
                TensorBuffer self;
                self._tag = detail::TensorBufferTag::U64;
                new (&self._data.u64) TypeAlias(std::move(u64));
                return self;
            }

            static TensorBuffer i8(std::vector<int8_t> i8) {
                typedef std::vector<int8_t> TypeAlias;
                TensorBuffer self;
                self._tag = detail::TensorBufferTag::I8;
                new (&self._data.i8) TypeAlias(std::move(i8));
                return self;
            }

            static TensorBuffer i16(std::vector<int16_t> i16) {
                typedef std::vector<int16_t> TypeAlias;
                TensorBuffer self;
                self._tag = detail::TensorBufferTag::I16;
                new (&self._data.i16) TypeAlias(std::move(i16));
                return self;
            }

            static TensorBuffer i32(std::vector<int32_t> i32) {
                typedef std::vector<int32_t> TypeAlias;
                TensorBuffer self;
                self._tag = detail::TensorBufferTag::I32;
                new (&self._data.i32) TypeAlias(std::move(i32));
                return self;
            }

            static TensorBuffer i64(std::vector<int64_t> i64) {
                typedef std::vector<int64_t> TypeAlias;
                TensorBuffer self;
                self._tag = detail::TensorBufferTag::I64;
                new (&self._data.i64) TypeAlias(std::move(i64));
                return self;
            }

            static TensorBuffer f32(std::vector<float> f32) {
                typedef std::vector<float> TypeAlias;
                TensorBuffer self;
                self._tag = detail::TensorBufferTag::F32;
                new (&self._data.f32) TypeAlias(std::move(f32));
                return self;
            }

            static TensorBuffer f64(std::vector<double> f64) {
                typedef std::vector<double> TypeAlias;
                TensorBuffer self;
                self._tag = detail::TensorBufferTag::F64;
                new (&self._data.f64) TypeAlias(std::move(f64));
                return self;
            }

            static TensorBuffer jpeg(std::vector<uint8_t> jpeg) {
                typedef std::vector<uint8_t> TypeAlias;
                TensorBuffer self;
                self._tag = detail::TensorBufferTag::JPEG;
                new (&self._data.jpeg) TypeAlias(std::move(jpeg));
                return self;
            }

            /// Returns the arrow data type this type corresponds to.
            static const std::shared_ptr<arrow::DataType> &arrow_datatype();

            /// Creates a new array builder with an array of this type.
            static Result<std::shared_ptr<arrow::DenseUnionBuilder>> new_arrow_array_builder(
                arrow::MemoryPool *memory_pool
            );

            /// Fills an arrow array builder with an array of this type.
            static Error fill_arrow_array_builder(
                arrow::DenseUnionBuilder *builder, const TensorBuffer *elements, size_t num_elements
            );

          private:
            detail::TensorBufferTag _tag;
            detail::TensorBufferData _data;
        };
    } // namespace datatypes
} // namespace rerun
