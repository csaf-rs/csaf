package csaf_ffi

// #include <csaf_ffi.h>
import "C"

import (
	"bytes"
	"encoding/binary"
	"fmt"
	"io"
	"math"
	"runtime"
	"sync/atomic"
	"unsafe"
)

// This is needed, because as of go 1.24
// type RustBuffer C.RustBuffer cannot have methods,
// RustBuffer is treated as non-local type
type GoRustBuffer struct {
	inner C.RustBuffer
}

type RustBufferI interface {
	AsReader() *bytes.Reader
	Free()
	ToGoBytes() []byte
	Data() unsafe.Pointer
	Len() uint64
	Capacity() uint64
}

// C.RustBuffer fields exposed as an interface so they can be accessed in different Go packages.
// See https://github.com/golang/go/issues/13467
type ExternalCRustBuffer interface {
	Data() unsafe.Pointer
	Len() uint64
	Capacity() uint64
}

func RustBufferFromC(b C.RustBuffer) ExternalCRustBuffer {
	return GoRustBuffer{
		inner: b,
	}
}

func CFromRustBuffer(b ExternalCRustBuffer) C.RustBuffer {
	return C.RustBuffer{
		capacity: C.uint64_t(b.Capacity()),
		len:      C.uint64_t(b.Len()),
		data:     (*C.uchar)(b.Data()),
	}
}

func RustBufferFromExternal(b ExternalCRustBuffer) GoRustBuffer {
	return GoRustBuffer{
		inner: C.RustBuffer{
			capacity: C.uint64_t(b.Capacity()),
			len:      C.uint64_t(b.Len()),
			data:     (*C.uchar)(b.Data()),
		},
	}
}

func (cb GoRustBuffer) Capacity() uint64 {
	return uint64(cb.inner.capacity)
}

func (cb GoRustBuffer) Len() uint64 {
	return uint64(cb.inner.len)
}

func (cb GoRustBuffer) Data() unsafe.Pointer {
	return unsafe.Pointer(cb.inner.data)
}

func (cb GoRustBuffer) AsReader() *bytes.Reader {
	b := unsafe.Slice((*byte)(cb.inner.data), C.uint64_t(cb.inner.len))
	return bytes.NewReader(b)
}

func (cb GoRustBuffer) Free() {
	rustCall(func(status *C.RustCallStatus) bool {
		C.ffi_csaf_ffi_rustbuffer_free(cb.inner, status)
		return false
	})
}

func (cb GoRustBuffer) ToGoBytes() []byte {
	return C.GoBytes(unsafe.Pointer(cb.inner.data), C.int(cb.inner.len))
}

func stringToRustBuffer(str string) C.RustBuffer {
	return bytesToRustBuffer([]byte(str))
}

func bytesToRustBuffer(b []byte) C.RustBuffer {
	if len(b) == 0 {
		return C.RustBuffer{}
	}
	// We can pass the pointer along here, as it is pinned
	// for the duration of this call
	foreign := C.ForeignBytes{
		len:  C.int(len(b)),
		data: (*C.uchar)(unsafe.Pointer(&b[0])),
	}

	return rustCall(func(status *C.RustCallStatus) C.RustBuffer {
		return C.ffi_csaf_ffi_rustbuffer_from_bytes(foreign, status)
	})
}

type BufLifter[GoType any] interface {
	Lift(value RustBufferI) GoType
}

type BufLowerer[GoType any] interface {
	Lower(value GoType) C.RustBuffer
}

type BufReader[GoType any] interface {
	Read(reader io.Reader) GoType
}

type BufWriter[GoType any] interface {
	Write(writer io.Writer, value GoType)
}

func LowerIntoRustBuffer[GoType any](bufWriter BufWriter[GoType], value GoType) C.RustBuffer {
	// This might be not the most efficient way but it does not require knowing allocation size
	// beforehand
	var buffer bytes.Buffer
	bufWriter.Write(&buffer, value)

	bytes, err := io.ReadAll(&buffer)
	if err != nil {
		panic(fmt.Errorf("reading written data: %w", err))
	}
	return bytesToRustBuffer(bytes)
}

func LiftFromRustBuffer[GoType any](bufReader BufReader[GoType], rbuf RustBufferI) GoType {
	defer rbuf.Free()
	reader := rbuf.AsReader()
	item := bufReader.Read(reader)
	if reader.Len() > 0 {
		// TODO: Remove this
		leftover, _ := io.ReadAll(reader)
		panic(fmt.Errorf("Junk remaining in buffer after lifting: %s", string(leftover)))
	}
	return item
}

func rustCallWithError[E any, U any](converter BufReader[E], callback func(*C.RustCallStatus) U) (U, E) {
	var status C.RustCallStatus
	returnValue := callback(&status)
	err := checkCallStatus(converter, status)
	return returnValue, err
}

func checkCallStatus[E any](converter BufReader[E], status C.RustCallStatus) E {
	switch status.code {
	case 0:
		var zero E
		return zero
	case 1:
		return LiftFromRustBuffer(converter, GoRustBuffer{inner: status.errorBuf})
	case 2:
		// when the rust code sees a panic, it tries to construct a rustBuffer
		// with the message.  but if that code panics, then it just sends back
		// an empty buffer.
		if status.errorBuf.len > 0 {
			panic(fmt.Errorf("%s", FfiConverterStringINSTANCE.Lift(GoRustBuffer{inner: status.errorBuf})))
		} else {
			panic(fmt.Errorf("Rust panicked while handling Rust panic"))
		}
	default:
		panic(fmt.Errorf("unknown status code: %d", status.code))
	}
}

func checkCallStatusUnknown(status C.RustCallStatus) error {
	switch status.code {
	case 0:
		return nil
	case 1:
		panic(fmt.Errorf("function not returning an error returned an error"))
	case 2:
		// when the rust code sees a panic, it tries to construct a C.RustBuffer
		// with the message.  but if that code panics, then it just sends back
		// an empty buffer.
		if status.errorBuf.len > 0 {
			panic(fmt.Errorf("%s", FfiConverterStringINSTANCE.Lift(GoRustBuffer{
				inner: status.errorBuf,
			})))
		} else {
			panic(fmt.Errorf("Rust panicked while handling Rust panic"))
		}
	default:
		return fmt.Errorf("unknown status code: %d", status.code)
	}
}

func rustCall[U any](callback func(*C.RustCallStatus) U) U {
	returnValue, err := rustCallWithError[error](nil, callback)
	if err != nil {
		panic(err)
	}
	return returnValue
}

type NativeError interface {
	AsError() error
}

func writeInt8(writer io.Writer, value int8) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func writeUint8(writer io.Writer, value uint8) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func writeInt16(writer io.Writer, value int16) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func writeUint16(writer io.Writer, value uint16) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func writeInt32(writer io.Writer, value int32) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func writeUint32(writer io.Writer, value uint32) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func writeInt64(writer io.Writer, value int64) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func writeUint64(writer io.Writer, value uint64) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func writeFloat32(writer io.Writer, value float32) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func writeFloat64(writer io.Writer, value float64) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func readInt8(reader io.Reader) int8 {
	var result int8
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func readUint8(reader io.Reader) uint8 {
	var result uint8
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func readInt16(reader io.Reader) int16 {
	var result int16
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func readUint16(reader io.Reader) uint16 {
	var result uint16
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func readInt32(reader io.Reader) int32 {
	var result int32
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func readUint32(reader io.Reader) uint32 {
	var result uint32
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func readInt64(reader io.Reader) int64 {
	var result int64
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func readUint64(reader io.Reader) uint64 {
	var result uint64
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func readFloat32(reader io.Reader) float32 {
	var result float32
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func readFloat64(reader io.Reader) float64 {
	var result float64
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func init() {

	uniffiCheckChecksums()
}

func uniffiCheckChecksums() {
	// Get the bindings contract version from our ComponentInterface
	bindingsContractVersion := 30
	// Get the scaffolding contract version by calling the into the dylib
	scaffoldingContractVersion := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint32_t {
		return C.ffi_csaf_ffi_uniffi_contract_version()
	})
	if bindingsContractVersion != int(scaffoldingContractVersion) {
		// If this happens try cleaning and rebuilding your project
		panic("csaf_ffi: UniFFI contract version mismatch")
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_csaf_ffi_checksum_func_validate_csaf()
		})
		if checksum != 6588 {
			// If this happens try cleaning and rebuilding your project
			panic("csaf_ffi: uniffi_csaf_ffi_checksum_func_validate_csaf: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_csaf_ffi_checksum_func_validate_csaf_2_0()
		})
		if checksum != 6842 {
			// If this happens try cleaning and rebuilding your project
			panic("csaf_ffi: uniffi_csaf_ffi_checksum_func_validate_csaf_2_0: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_csaf_ffi_checksum_func_validate_csaf_2_1()
		})
		if checksum != 22579 {
			// If this happens try cleaning and rebuilding your project
			panic("csaf_ffi: uniffi_csaf_ffi_checksum_func_validate_csaf_2_1: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_csaf_ffi_checksum_method_csafdocument_get_all_group_references()
		})
		if checksum != 46338 {
			// If this happens try cleaning and rebuilding your project
			panic("csaf_ffi: uniffi_csaf_ffi_checksum_method_csafdocument_get_all_group_references: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_csaf_ffi_checksum_method_csafdocument_get_all_product_ids()
		})
		if checksum != 12458 {
			// If this happens try cleaning and rebuilding your project
			panic("csaf_ffi: uniffi_csaf_ffi_checksum_method_csafdocument_get_all_product_ids: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_csaf_ffi_checksum_method_csafdocument_get_all_product_references()
		})
		if checksum != 58411 {
			// If this happens try cleaning and rebuilding your project
			panic("csaf_ffi: uniffi_csaf_ffi_checksum_method_csafdocument_get_all_product_references: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_csaf_ffi_checksum_method_csafdocument_get_category()
		})
		if checksum != 48677 {
			// If this happens try cleaning and rebuilding your project
			panic("csaf_ffi: uniffi_csaf_ffi_checksum_method_csafdocument_get_category: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_csaf_ffi_checksum_method_csafdocument_get_current_release_date()
		})
		if checksum != 9273 {
			// If this happens try cleaning and rebuilding your project
			panic("csaf_ffi: uniffi_csaf_ffi_checksum_method_csafdocument_get_current_release_date: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_csaf_ffi_checksum_method_csafdocument_get_initial_release_date()
		})
		if checksum != 38168 {
			// If this happens try cleaning and rebuilding your project
			panic("csaf_ffi: uniffi_csaf_ffi_checksum_method_csafdocument_get_initial_release_date: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_csaf_ffi_checksum_method_csafdocument_get_lang()
		})
		if checksum != 5286 {
			// If this happens try cleaning and rebuilding your project
			panic("csaf_ffi: uniffi_csaf_ffi_checksum_method_csafdocument_get_lang: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_csaf_ffi_checksum_method_csafdocument_get_publisher_category()
		})
		if checksum != 19665 {
			// If this happens try cleaning and rebuilding your project
			panic("csaf_ffi: uniffi_csaf_ffi_checksum_method_csafdocument_get_publisher_category: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_csaf_ffi_checksum_method_csafdocument_get_title()
		})
		if checksum != 49061 {
			// If this happens try cleaning and rebuilding your project
			panic("csaf_ffi: uniffi_csaf_ffi_checksum_method_csafdocument_get_title: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_csaf_ffi_checksum_method_csafdocument_get_tracking_id()
		})
		if checksum != 57812 {
			// If this happens try cleaning and rebuilding your project
			panic("csaf_ffi: uniffi_csaf_ffi_checksum_method_csafdocument_get_tracking_id: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_csaf_ffi_checksum_method_csafdocument_get_version()
		})
		if checksum != 11556 {
			// If this happens try cleaning and rebuilding your project
			panic("csaf_ffi: uniffi_csaf_ffi_checksum_method_csafdocument_get_version: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_csaf_ffi_checksum_method_csafdocument_get_version_string()
		})
		if checksum != 1927 {
			// If this happens try cleaning and rebuilding your project
			panic("csaf_ffi: uniffi_csaf_ffi_checksum_method_csafdocument_get_version_string: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_csaf_ffi_checksum_method_csafdocument_get_vulnerability_count()
		})
		if checksum != 26430 {
			// If this happens try cleaning and rebuilding your project
			panic("csaf_ffi: uniffi_csaf_ffi_checksum_method_csafdocument_get_vulnerability_count: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_csaf_ffi_checksum_method_csafdocument_get_vulnerability_cve()
		})
		if checksum != 1333 {
			// If this happens try cleaning and rebuilding your project
			panic("csaf_ffi: uniffi_csaf_ffi_checksum_method_csafdocument_get_vulnerability_cve: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_csaf_ffi_checksum_method_csafdocument_get_vulnerability_cwes()
		})
		if checksum != 23236 {
			// If this happens try cleaning and rebuilding your project
			panic("csaf_ffi: uniffi_csaf_ffi_checksum_method_csafdocument_get_vulnerability_cwes: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_csaf_ffi_checksum_method_csafdocument_get_vulnerability_disclosure_date()
		})
		if checksum != 34567 {
			// If this happens try cleaning and rebuilding your project
			panic("csaf_ffi: uniffi_csaf_ffi_checksum_method_csafdocument_get_vulnerability_disclosure_date: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_csaf_ffi_checksum_method_csafdocument_get_vulnerability_ids()
		})
		if checksum != 22867 {
			// If this happens try cleaning and rebuilding your project
			panic("csaf_ffi: uniffi_csaf_ffi_checksum_method_csafdocument_get_vulnerability_ids: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_csaf_ffi_checksum_method_csafdocument_has_product_tree()
		})
		if checksum != 40983 {
			// If this happens try cleaning and rebuilding your project
			panic("csaf_ffi: uniffi_csaf_ffi_checksum_method_csafdocument_has_product_tree: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_csaf_ffi_checksum_method_csafdocument_run_test()
		})
		if checksum != 55414 {
			// If this happens try cleaning and rebuilding your project
			panic("csaf_ffi: uniffi_csaf_ffi_checksum_method_csafdocument_run_test: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_csaf_ffi_checksum_method_csafdocument_run_tests()
		})
		if checksum != 11781 {
			// If this happens try cleaning and rebuilding your project
			panic("csaf_ffi: uniffi_csaf_ffi_checksum_method_csafdocument_run_tests: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_csaf_ffi_checksum_method_csafdocument_to_json()
		})
		if checksum != 30606 {
			// If this happens try cleaning and rebuilding your project
			panic("csaf_ffi: uniffi_csaf_ffi_checksum_method_csafdocument_to_json: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_csaf_ffi_checksum_method_csafdocument_validate()
		})
		if checksum != 48953 {
			// If this happens try cleaning and rebuilding your project
			panic("csaf_ffi: uniffi_csaf_ffi_checksum_method_csafdocument_validate: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_csaf_ffi_checksum_constructor_csafdocument_from_json()
		})
		if checksum != 4893 {
			// If this happens try cleaning and rebuilding your project
			panic("csaf_ffi: uniffi_csaf_ffi_checksum_constructor_csafdocument_from_json: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_csaf_ffi_checksum_constructor_csafdocument_from_json_2_0()
		})
		if checksum != 6417 {
			// If this happens try cleaning and rebuilding your project
			panic("csaf_ffi: uniffi_csaf_ffi_checksum_constructor_csafdocument_from_json_2_0: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_csaf_ffi_checksum_constructor_csafdocument_from_json_2_1()
		})
		if checksum != 11298 {
			// If this happens try cleaning and rebuilding your project
			panic("csaf_ffi: uniffi_csaf_ffi_checksum_constructor_csafdocument_from_json_2_1: UniFFI API checksum mismatch")
		}
	}
}

type FfiConverterUint64 struct{}

var FfiConverterUint64INSTANCE = FfiConverterUint64{}

func (FfiConverterUint64) Lower(value uint64) C.uint64_t {
	return C.uint64_t(value)
}

func (FfiConverterUint64) Write(writer io.Writer, value uint64) {
	writeUint64(writer, value)
}

func (FfiConverterUint64) Lift(value C.uint64_t) uint64 {
	return uint64(value)
}

func (FfiConverterUint64) Read(reader io.Reader) uint64 {
	return readUint64(reader)
}

type FfiDestroyerUint64 struct{}

func (FfiDestroyerUint64) Destroy(_ uint64) {}

type FfiConverterBool struct{}

var FfiConverterBoolINSTANCE = FfiConverterBool{}

func (FfiConverterBool) Lower(value bool) C.int8_t {
	if value {
		return C.int8_t(1)
	}
	return C.int8_t(0)
}

func (FfiConverterBool) Write(writer io.Writer, value bool) {
	if value {
		writeInt8(writer, 1)
	} else {
		writeInt8(writer, 0)
	}
}

func (FfiConverterBool) Lift(value C.int8_t) bool {
	return value != 0
}

func (FfiConverterBool) Read(reader io.Reader) bool {
	return readInt8(reader) != 0
}

type FfiDestroyerBool struct{}

func (FfiDestroyerBool) Destroy(_ bool) {}

type FfiConverterString struct{}

var FfiConverterStringINSTANCE = FfiConverterString{}

func (FfiConverterString) Lift(rb RustBufferI) string {
	defer rb.Free()
	reader := rb.AsReader()
	b, err := io.ReadAll(reader)
	if err != nil {
		panic(fmt.Errorf("reading reader: %w", err))
	}
	return string(b)
}

func (FfiConverterString) Read(reader io.Reader) string {
	length := readInt32(reader)
	buffer := make([]byte, length)
	read_length, err := reader.Read(buffer)
	if err != nil && err != io.EOF {
		panic(err)
	}
	if read_length != int(length) {
		panic(fmt.Errorf("bad read length when reading string, expected %d, read %d", length, read_length))
	}
	return string(buffer)
}

func (FfiConverterString) Lower(value string) C.RustBuffer {
	return stringToRustBuffer(value)
}

func (c FfiConverterString) LowerExternal(value string) ExternalCRustBuffer {
	return RustBufferFromC(stringToRustBuffer(value))
}

func (FfiConverterString) Write(writer io.Writer, value string) {
	if len(value) > math.MaxInt32 {
		panic("String is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	write_length, err := io.WriteString(writer, value)
	if err != nil {
		panic(err)
	}
	if write_length != len(value) {
		panic(fmt.Errorf("bad write length when writing string, expected %d, written %d", len(value), write_length))
	}
}

type FfiDestroyerString struct{}

func (FfiDestroyerString) Destroy(_ string) {}

// Below is an implementation of synchronization requirements outlined in the link.
// https://github.com/mozilla/uniffi-rs/blob/0dc031132d9493ca812c3af6e7dd60ad2ea95bf0/uniffi_bindgen/src/bindings/kotlin/templates/ObjectRuntime.kt#L31

type FfiObject struct {
	handle        C.uint64_t
	callCounter   atomic.Int64
	cloneFunction func(C.uint64_t, *C.RustCallStatus) C.uint64_t
	freeFunction  func(C.uint64_t, *C.RustCallStatus)
	destroyed     atomic.Bool
}

func newFfiObject(
	handle C.uint64_t,
	cloneFunction func(C.uint64_t, *C.RustCallStatus) C.uint64_t,
	freeFunction func(C.uint64_t, *C.RustCallStatus),
) FfiObject {
	return FfiObject{
		handle:        handle,
		cloneFunction: cloneFunction,
		freeFunction:  freeFunction,
	}
}

func (ffiObject *FfiObject) incrementPointer(debugName string) C.uint64_t {
	for {
		counter := ffiObject.callCounter.Load()
		if counter <= -1 {
			panic(fmt.Errorf("%v object has already been destroyed", debugName))
		}
		if counter == math.MaxInt64 {
			panic(fmt.Errorf("%v object call counter would overflow", debugName))
		}
		if ffiObject.callCounter.CompareAndSwap(counter, counter+1) {
			break
		}
	}

	return rustCall(func(status *C.RustCallStatus) C.uint64_t {
		return ffiObject.cloneFunction(ffiObject.handle, status)
	})
}

func (ffiObject *FfiObject) decrementPointer() {
	if ffiObject.callCounter.Add(-1) == -1 {
		ffiObject.freeRustArcPtr()
	}
}

func (ffiObject *FfiObject) destroy() {
	if ffiObject.destroyed.CompareAndSwap(false, true) {
		if ffiObject.callCounter.Add(-1) == -1 {
			ffiObject.freeRustArcPtr()
		}
	}
}

func (ffiObject *FfiObject) freeRustArcPtr() {
	if ffiObject.handle == 0 {
		return
	}
	rustCall(func(status *C.RustCallStatus) int32 {
		ffiObject.freeFunction(ffiObject.handle, status)
		return 0
	})
}

// A parsed CSAF document (2.0 or 2.1).
//
// Create via [`CsafDocument::from_json`] (auto-detect) or
// [`CsafDocument::from_json_2_0`] / [`CsafDocument::from_json_2_1`].
type CsafDocumentInterface interface {
	// Get all group references across all vulnerabilities.
	GetAllGroupReferences() ([]ProductReference, error)
	// Get all product IDs referenced in the document.
	GetAllProductIds() ([]string, error)
	// Get all product references across all vulnerabilities.
	GetAllProductReferences() ([]ProductReference, error)
	// Document category (e.g., csaf_vex, csaf_security_advisory).
	GetCategory() (DocumentCategory, error)
	// Current release date.
	GetCurrentReleaseDate() (CsafDateTime, error)
	// Initial release date.
	GetInitialReleaseDate() (CsafDateTime, error)
	// Document language tag, if present.
	GetLang() (*CsafLanguage, error)
	// Publisher category as a string.
	GetPublisherCategory() (string, error)
	// Document title.
	GetTitle() (string, error)
	// Tracking ID.
	GetTrackingId() (string, error)
	// The CSAF version of this document.
	GetVersion() CsafVersion
	// The version string ("2.0" or "2.1").
	GetVersionString() string
	// Number of vulnerabilities in the document.
	GetVulnerabilityCount() (uint64, error)
	// Get the CVE identifier for a vulnerability at the given index.
	GetVulnerabilityCve(index uint64) (*string, error)
	// Get CWE entries for a vulnerability at the given index.
	GetVulnerabilityCwes(index uint64) ([]Cwe, error)
	// Get disclosure date for a vulnerability at the given index.
	GetVulnerabilityDisclosureDate(index uint64) (*CsafDateTime, error)
	// Get vulnerability IDs (non-CVE) for a vulnerability at the given index.
	GetVulnerabilityIds(index uint64) ([]VulnerabilityId, error)
	// Whether the document has a product tree.
	HasProductTree() (bool, error)
	// Run a single validation test by ID.
	RunTest(testId string) (TestResult, error)
	// Run specific validation tests by their IDs.
	RunTests(testIds []string) (ValidationResult, error)
	// The original JSON string used to create this document.
	ToJson() string
	// Run validation with the given preset ("basic", "extended", "full").
	Validate(preset string) (ValidationResult, error)
}

// A parsed CSAF document (2.0 or 2.1).
//
// Create via [`CsafDocument::from_json`] (auto-detect) or
// [`CsafDocument::from_json_2_0`] / [`CsafDocument::from_json_2_1`].
type CsafDocument struct {
	ffiObject FfiObject
}

// Parse a CSAF document from JSON, auto-detecting the version.
func CsafDocumentFromJson(jsonStr string) (*CsafDocument, error) {
	_uniffiRV, _uniffiErr := rustCallWithError[*CsafError](FfiConverterCsafError{}, func(_uniffiStatus *C.RustCallStatus) C.uint64_t {
		return C.uniffi_csaf_ffi_fn_constructor_csafdocument_from_json(FfiConverterStringINSTANCE.Lower(jsonStr), _uniffiStatus)
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue *CsafDocument
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterCsafDocumentINSTANCE.Lift(_uniffiRV), nil
	}
}

// Parse a CSAF 2.0 document from JSON.
func CsafDocumentFromJson20(jsonStr string) (*CsafDocument, error) {
	_uniffiRV, _uniffiErr := rustCallWithError[*CsafError](FfiConverterCsafError{}, func(_uniffiStatus *C.RustCallStatus) C.uint64_t {
		return C.uniffi_csaf_ffi_fn_constructor_csafdocument_from_json_2_0(FfiConverterStringINSTANCE.Lower(jsonStr), _uniffiStatus)
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue *CsafDocument
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterCsafDocumentINSTANCE.Lift(_uniffiRV), nil
	}
}

// Parse a CSAF 2.1 document from JSON.
func CsafDocumentFromJson21(jsonStr string) (*CsafDocument, error) {
	_uniffiRV, _uniffiErr := rustCallWithError[*CsafError](FfiConverterCsafError{}, func(_uniffiStatus *C.RustCallStatus) C.uint64_t {
		return C.uniffi_csaf_ffi_fn_constructor_csafdocument_from_json_2_1(FfiConverterStringINSTANCE.Lower(jsonStr), _uniffiStatus)
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue *CsafDocument
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterCsafDocumentINSTANCE.Lift(_uniffiRV), nil
	}
}

// Get all group references across all vulnerabilities.
func (_self *CsafDocument) GetAllGroupReferences() ([]ProductReference, error) {
	_pointer := _self.ffiObject.incrementPointer("*CsafDocument")
	defer _self.ffiObject.decrementPointer()
	_uniffiRV, _uniffiErr := rustCallWithError[*CsafError](FfiConverterCsafError{}, func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return GoRustBuffer{
			inner: C.uniffi_csaf_ffi_fn_method_csafdocument_get_all_group_references(
				_pointer, _uniffiStatus),
		}
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue []ProductReference
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterSequenceProductReferenceINSTANCE.Lift(_uniffiRV), nil
	}
}

// Get all product IDs referenced in the document.
func (_self *CsafDocument) GetAllProductIds() ([]string, error) {
	_pointer := _self.ffiObject.incrementPointer("*CsafDocument")
	defer _self.ffiObject.decrementPointer()
	_uniffiRV, _uniffiErr := rustCallWithError[*CsafError](FfiConverterCsafError{}, func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return GoRustBuffer{
			inner: C.uniffi_csaf_ffi_fn_method_csafdocument_get_all_product_ids(
				_pointer, _uniffiStatus),
		}
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue []string
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterSequenceStringINSTANCE.Lift(_uniffiRV), nil
	}
}

// Get all product references across all vulnerabilities.
func (_self *CsafDocument) GetAllProductReferences() ([]ProductReference, error) {
	_pointer := _self.ffiObject.incrementPointer("*CsafDocument")
	defer _self.ffiObject.decrementPointer()
	_uniffiRV, _uniffiErr := rustCallWithError[*CsafError](FfiConverterCsafError{}, func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return GoRustBuffer{
			inner: C.uniffi_csaf_ffi_fn_method_csafdocument_get_all_product_references(
				_pointer, _uniffiStatus),
		}
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue []ProductReference
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterSequenceProductReferenceINSTANCE.Lift(_uniffiRV), nil
	}
}

// Document category (e.g., csaf_vex, csaf_security_advisory).
func (_self *CsafDocument) GetCategory() (DocumentCategory, error) {
	_pointer := _self.ffiObject.incrementPointer("*CsafDocument")
	defer _self.ffiObject.decrementPointer()
	_uniffiRV, _uniffiErr := rustCallWithError[*CsafError](FfiConverterCsafError{}, func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return GoRustBuffer{
			inner: C.uniffi_csaf_ffi_fn_method_csafdocument_get_category(
				_pointer, _uniffiStatus),
		}
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue DocumentCategory
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterDocumentCategoryINSTANCE.Lift(_uniffiRV), nil
	}
}

// Current release date.
func (_self *CsafDocument) GetCurrentReleaseDate() (CsafDateTime, error) {
	_pointer := _self.ffiObject.incrementPointer("*CsafDocument")
	defer _self.ffiObject.decrementPointer()
	_uniffiRV, _uniffiErr := rustCallWithError[*CsafError](FfiConverterCsafError{}, func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return GoRustBuffer{
			inner: C.uniffi_csaf_ffi_fn_method_csafdocument_get_current_release_date(
				_pointer, _uniffiStatus),
		}
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue CsafDateTime
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterCsafDateTimeINSTANCE.Lift(_uniffiRV), nil
	}
}

// Initial release date.
func (_self *CsafDocument) GetInitialReleaseDate() (CsafDateTime, error) {
	_pointer := _self.ffiObject.incrementPointer("*CsafDocument")
	defer _self.ffiObject.decrementPointer()
	_uniffiRV, _uniffiErr := rustCallWithError[*CsafError](FfiConverterCsafError{}, func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return GoRustBuffer{
			inner: C.uniffi_csaf_ffi_fn_method_csafdocument_get_initial_release_date(
				_pointer, _uniffiStatus),
		}
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue CsafDateTime
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterCsafDateTimeINSTANCE.Lift(_uniffiRV), nil
	}
}

// Document language tag, if present.
func (_self *CsafDocument) GetLang() (*CsafLanguage, error) {
	_pointer := _self.ffiObject.incrementPointer("*CsafDocument")
	defer _self.ffiObject.decrementPointer()
	_uniffiRV, _uniffiErr := rustCallWithError[*CsafError](FfiConverterCsafError{}, func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return GoRustBuffer{
			inner: C.uniffi_csaf_ffi_fn_method_csafdocument_get_lang(
				_pointer, _uniffiStatus),
		}
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue *CsafLanguage
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterOptionalCsafLanguageINSTANCE.Lift(_uniffiRV), nil
	}
}

// Publisher category as a string.
func (_self *CsafDocument) GetPublisherCategory() (string, error) {
	_pointer := _self.ffiObject.incrementPointer("*CsafDocument")
	defer _self.ffiObject.decrementPointer()
	_uniffiRV, _uniffiErr := rustCallWithError[*CsafError](FfiConverterCsafError{}, func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return GoRustBuffer{
			inner: C.uniffi_csaf_ffi_fn_method_csafdocument_get_publisher_category(
				_pointer, _uniffiStatus),
		}
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue string
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterStringINSTANCE.Lift(_uniffiRV), nil
	}
}

// Document title.
func (_self *CsafDocument) GetTitle() (string, error) {
	_pointer := _self.ffiObject.incrementPointer("*CsafDocument")
	defer _self.ffiObject.decrementPointer()
	_uniffiRV, _uniffiErr := rustCallWithError[*CsafError](FfiConverterCsafError{}, func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return GoRustBuffer{
			inner: C.uniffi_csaf_ffi_fn_method_csafdocument_get_title(
				_pointer, _uniffiStatus),
		}
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue string
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterStringINSTANCE.Lift(_uniffiRV), nil
	}
}

// Tracking ID.
func (_self *CsafDocument) GetTrackingId() (string, error) {
	_pointer := _self.ffiObject.incrementPointer("*CsafDocument")
	defer _self.ffiObject.decrementPointer()
	_uniffiRV, _uniffiErr := rustCallWithError[*CsafError](FfiConverterCsafError{}, func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return GoRustBuffer{
			inner: C.uniffi_csaf_ffi_fn_method_csafdocument_get_tracking_id(
				_pointer, _uniffiStatus),
		}
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue string
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterStringINSTANCE.Lift(_uniffiRV), nil
	}
}

// The CSAF version of this document.
func (_self *CsafDocument) GetVersion() CsafVersion {
	_pointer := _self.ffiObject.incrementPointer("*CsafDocument")
	defer _self.ffiObject.decrementPointer()
	return FfiConverterCsafVersionINSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return GoRustBuffer{
			inner: C.uniffi_csaf_ffi_fn_method_csafdocument_get_version(
				_pointer, _uniffiStatus),
		}
	}))
}

// The version string ("2.0" or "2.1").
func (_self *CsafDocument) GetVersionString() string {
	_pointer := _self.ffiObject.incrementPointer("*CsafDocument")
	defer _self.ffiObject.decrementPointer()
	return FfiConverterStringINSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return GoRustBuffer{
			inner: C.uniffi_csaf_ffi_fn_method_csafdocument_get_version_string(
				_pointer, _uniffiStatus),
		}
	}))
}

// Number of vulnerabilities in the document.
func (_self *CsafDocument) GetVulnerabilityCount() (uint64, error) {
	_pointer := _self.ffiObject.incrementPointer("*CsafDocument")
	defer _self.ffiObject.decrementPointer()
	_uniffiRV, _uniffiErr := rustCallWithError[*CsafError](FfiConverterCsafError{}, func(_uniffiStatus *C.RustCallStatus) C.uint64_t {
		return C.uniffi_csaf_ffi_fn_method_csafdocument_get_vulnerability_count(
			_pointer, _uniffiStatus)
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue uint64
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterUint64INSTANCE.Lift(_uniffiRV), nil
	}
}

// Get the CVE identifier for a vulnerability at the given index.
func (_self *CsafDocument) GetVulnerabilityCve(index uint64) (*string, error) {
	_pointer := _self.ffiObject.incrementPointer("*CsafDocument")
	defer _self.ffiObject.decrementPointer()
	_uniffiRV, _uniffiErr := rustCallWithError[*CsafError](FfiConverterCsafError{}, func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return GoRustBuffer{
			inner: C.uniffi_csaf_ffi_fn_method_csafdocument_get_vulnerability_cve(
				_pointer, FfiConverterUint64INSTANCE.Lower(index), _uniffiStatus),
		}
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue *string
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterOptionalStringINSTANCE.Lift(_uniffiRV), nil
	}
}

// Get CWE entries for a vulnerability at the given index.
func (_self *CsafDocument) GetVulnerabilityCwes(index uint64) ([]Cwe, error) {
	_pointer := _self.ffiObject.incrementPointer("*CsafDocument")
	defer _self.ffiObject.decrementPointer()
	_uniffiRV, _uniffiErr := rustCallWithError[*CsafError](FfiConverterCsafError{}, func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return GoRustBuffer{
			inner: C.uniffi_csaf_ffi_fn_method_csafdocument_get_vulnerability_cwes(
				_pointer, FfiConverterUint64INSTANCE.Lower(index), _uniffiStatus),
		}
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue []Cwe
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterSequenceCweINSTANCE.Lift(_uniffiRV), nil
	}
}

// Get disclosure date for a vulnerability at the given index.
func (_self *CsafDocument) GetVulnerabilityDisclosureDate(index uint64) (*CsafDateTime, error) {
	_pointer := _self.ffiObject.incrementPointer("*CsafDocument")
	defer _self.ffiObject.decrementPointer()
	_uniffiRV, _uniffiErr := rustCallWithError[*CsafError](FfiConverterCsafError{}, func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return GoRustBuffer{
			inner: C.uniffi_csaf_ffi_fn_method_csafdocument_get_vulnerability_disclosure_date(
				_pointer, FfiConverterUint64INSTANCE.Lower(index), _uniffiStatus),
		}
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue *CsafDateTime
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterOptionalCsafDateTimeINSTANCE.Lift(_uniffiRV), nil
	}
}

// Get vulnerability IDs (non-CVE) for a vulnerability at the given index.
func (_self *CsafDocument) GetVulnerabilityIds(index uint64) ([]VulnerabilityId, error) {
	_pointer := _self.ffiObject.incrementPointer("*CsafDocument")
	defer _self.ffiObject.decrementPointer()
	_uniffiRV, _uniffiErr := rustCallWithError[*CsafError](FfiConverterCsafError{}, func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return GoRustBuffer{
			inner: C.uniffi_csaf_ffi_fn_method_csafdocument_get_vulnerability_ids(
				_pointer, FfiConverterUint64INSTANCE.Lower(index), _uniffiStatus),
		}
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue []VulnerabilityId
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterSequenceVulnerabilityIdINSTANCE.Lift(_uniffiRV), nil
	}
}

// Whether the document has a product tree.
func (_self *CsafDocument) HasProductTree() (bool, error) {
	_pointer := _self.ffiObject.incrementPointer("*CsafDocument")
	defer _self.ffiObject.decrementPointer()
	_uniffiRV, _uniffiErr := rustCallWithError[*CsafError](FfiConverterCsafError{}, func(_uniffiStatus *C.RustCallStatus) C.int8_t {
		return C.uniffi_csaf_ffi_fn_method_csafdocument_has_product_tree(
			_pointer, _uniffiStatus)
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue bool
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterBoolINSTANCE.Lift(_uniffiRV), nil
	}
}

// Run a single validation test by ID.
func (_self *CsafDocument) RunTest(testId string) (TestResult, error) {
	_pointer := _self.ffiObject.incrementPointer("*CsafDocument")
	defer _self.ffiObject.decrementPointer()
	_uniffiRV, _uniffiErr := rustCallWithError[*CsafError](FfiConverterCsafError{}, func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return GoRustBuffer{
			inner: C.uniffi_csaf_ffi_fn_method_csafdocument_run_test(
				_pointer, FfiConverterStringINSTANCE.Lower(testId), _uniffiStatus),
		}
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue TestResult
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterTestResultINSTANCE.Lift(_uniffiRV), nil
	}
}

// Run specific validation tests by their IDs.
func (_self *CsafDocument) RunTests(testIds []string) (ValidationResult, error) {
	_pointer := _self.ffiObject.incrementPointer("*CsafDocument")
	defer _self.ffiObject.decrementPointer()
	_uniffiRV, _uniffiErr := rustCallWithError[*CsafError](FfiConverterCsafError{}, func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return GoRustBuffer{
			inner: C.uniffi_csaf_ffi_fn_method_csafdocument_run_tests(
				_pointer, FfiConverterSequenceStringINSTANCE.Lower(testIds), _uniffiStatus),
		}
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue ValidationResult
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterValidationResultINSTANCE.Lift(_uniffiRV), nil
	}
}

// The original JSON string used to create this document.
func (_self *CsafDocument) ToJson() string {
	_pointer := _self.ffiObject.incrementPointer("*CsafDocument")
	defer _self.ffiObject.decrementPointer()
	return FfiConverterStringINSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return GoRustBuffer{
			inner: C.uniffi_csaf_ffi_fn_method_csafdocument_to_json(
				_pointer, _uniffiStatus),
		}
	}))
}

// Run validation with the given preset ("basic", "extended", "full").
func (_self *CsafDocument) Validate(preset string) (ValidationResult, error) {
	_pointer := _self.ffiObject.incrementPointer("*CsafDocument")
	defer _self.ffiObject.decrementPointer()
	_uniffiRV, _uniffiErr := rustCallWithError[*CsafError](FfiConverterCsafError{}, func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return GoRustBuffer{
			inner: C.uniffi_csaf_ffi_fn_method_csafdocument_validate(
				_pointer, FfiConverterStringINSTANCE.Lower(preset), _uniffiStatus),
		}
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue ValidationResult
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterValidationResultINSTANCE.Lift(_uniffiRV), nil
	}
}
func (object *CsafDocument) Destroy() {
	runtime.SetFinalizer(object, nil)
	object.ffiObject.destroy()
}

type FfiConverterCsafDocument struct{}

var FfiConverterCsafDocumentINSTANCE = FfiConverterCsafDocument{}

func (c FfiConverterCsafDocument) Lift(handle C.uint64_t) *CsafDocument {
	result := &CsafDocument{
		newFfiObject(
			handle,
			func(handle C.uint64_t, status *C.RustCallStatus) C.uint64_t {
				return C.uniffi_csaf_ffi_fn_clone_csafdocument(handle, status)
			},
			func(handle C.uint64_t, status *C.RustCallStatus) {
				C.uniffi_csaf_ffi_fn_free_csafdocument(handle, status)
			},
		),
	}
	runtime.SetFinalizer(result, (*CsafDocument).Destroy)
	return result
}

func (c FfiConverterCsafDocument) Read(reader io.Reader) *CsafDocument {
	return c.Lift(C.uint64_t(readUint64(reader)))
}

func (c FfiConverterCsafDocument) Lower(value *CsafDocument) C.uint64_t {
	// TODO: this is bad - all synchronization from ObjectRuntime.go is discarded here,
	// because the handle will be decremented immediately after this function returns,
	// and someone will be left holding onto a non-locked handle.
	handle := value.ffiObject.incrementPointer("*CsafDocument")
	defer value.ffiObject.decrementPointer()
	return handle
}

func (c FfiConverterCsafDocument) Write(writer io.Writer, value *CsafDocument) {
	writeUint64(writer, uint64(c.Lower(value)))
}

func LiftFromExternalCsafDocument(handle uint64) *CsafDocument {
	return FfiConverterCsafDocumentINSTANCE.Lift(C.uint64_t(handle))
}

func LowerToExternalCsafDocument(value *CsafDocument) uint64 {
	return uint64(FfiConverterCsafDocumentINSTANCE.Lower(value))
}

type FfiDestroyerCsafDocument struct{}

func (_ FfiDestroyerCsafDocument) Destroy(value *CsafDocument) {
	value.Destroy()
}

type Cwe struct {
	Id      string
	Name    string
	Version *string
}

func (r *Cwe) Destroy() {
	FfiDestroyerString{}.Destroy(r.Id)
	FfiDestroyerString{}.Destroy(r.Name)
	FfiDestroyerOptionalString{}.Destroy(r.Version)
}

type FfiConverterCwe struct{}

var FfiConverterCweINSTANCE = FfiConverterCwe{}

func (c FfiConverterCwe) Lift(rb RustBufferI) Cwe {
	return LiftFromRustBuffer[Cwe](c, rb)
}

func (c FfiConverterCwe) Read(reader io.Reader) Cwe {
	return Cwe{
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterOptionalStringINSTANCE.Read(reader),
	}
}

func (c FfiConverterCwe) Lower(value Cwe) C.RustBuffer {
	return LowerIntoRustBuffer[Cwe](c, value)
}

func (c FfiConverterCwe) LowerExternal(value Cwe) ExternalCRustBuffer {
	return RustBufferFromC(LowerIntoRustBuffer[Cwe](c, value))
}

func (c FfiConverterCwe) Write(writer io.Writer, value Cwe) {
	FfiConverterStringINSTANCE.Write(writer, value.Id)
	FfiConverterStringINSTANCE.Write(writer, value.Name)
	FfiConverterOptionalStringINSTANCE.Write(writer, value.Version)
}

type FfiDestroyerCwe struct{}

func (_ FfiDestroyerCwe) Destroy(value Cwe) {
	value.Destroy()
}

type Note struct {
	Category string
	Title    *string
}

func (r *Note) Destroy() {
	FfiDestroyerString{}.Destroy(r.Category)
	FfiDestroyerOptionalString{}.Destroy(r.Title)
}

type FfiConverterNote struct{}

var FfiConverterNoteINSTANCE = FfiConverterNote{}

func (c FfiConverterNote) Lift(rb RustBufferI) Note {
	return LiftFromRustBuffer[Note](c, rb)
}

func (c FfiConverterNote) Read(reader io.Reader) Note {
	return Note{
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterOptionalStringINSTANCE.Read(reader),
	}
}

func (c FfiConverterNote) Lower(value Note) C.RustBuffer {
	return LowerIntoRustBuffer[Note](c, value)
}

func (c FfiConverterNote) LowerExternal(value Note) ExternalCRustBuffer {
	return RustBufferFromC(LowerIntoRustBuffer[Note](c, value))
}

func (c FfiConverterNote) Write(writer io.Writer, value Note) {
	FfiConverterStringINSTANCE.Write(writer, value.Category)
	FfiConverterOptionalStringINSTANCE.Write(writer, value.Title)
}

type FfiDestroyerNote struct{}

func (_ FfiDestroyerNote) Destroy(value Note) {
	value.Destroy()
}

type ProductReference struct {
	ProductId string
	JsonPath  string
}

func (r *ProductReference) Destroy() {
	FfiDestroyerString{}.Destroy(r.ProductId)
	FfiDestroyerString{}.Destroy(r.JsonPath)
}

type FfiConverterProductReference struct{}

var FfiConverterProductReferenceINSTANCE = FfiConverterProductReference{}

func (c FfiConverterProductReference) Lift(rb RustBufferI) ProductReference {
	return LiftFromRustBuffer[ProductReference](c, rb)
}

func (c FfiConverterProductReference) Read(reader io.Reader) ProductReference {
	return ProductReference{
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterStringINSTANCE.Read(reader),
	}
}

func (c FfiConverterProductReference) Lower(value ProductReference) C.RustBuffer {
	return LowerIntoRustBuffer[ProductReference](c, value)
}

func (c FfiConverterProductReference) LowerExternal(value ProductReference) ExternalCRustBuffer {
	return RustBufferFromC(LowerIntoRustBuffer[ProductReference](c, value))
}

func (c FfiConverterProductReference) Write(writer io.Writer, value ProductReference) {
	FfiConverterStringINSTANCE.Write(writer, value.ProductId)
	FfiConverterStringINSTANCE.Write(writer, value.JsonPath)
}

type FfiDestroyerProductReference struct{}

func (_ FfiDestroyerProductReference) Destroy(value ProductReference) {
	value.Destroy()
}

// Result of a single validation test.
type TestResult struct {
	TestId string
	Status TestResultStatus
}

func (r *TestResult) Destroy() {
	FfiDestroyerString{}.Destroy(r.TestId)
	FfiDestroyerTestResultStatus{}.Destroy(r.Status)
}

type FfiConverterTestResult struct{}

var FfiConverterTestResultINSTANCE = FfiConverterTestResult{}

func (c FfiConverterTestResult) Lift(rb RustBufferI) TestResult {
	return LiftFromRustBuffer[TestResult](c, rb)
}

func (c FfiConverterTestResult) Read(reader io.Reader) TestResult {
	return TestResult{
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterTestResultStatusINSTANCE.Read(reader),
	}
}

func (c FfiConverterTestResult) Lower(value TestResult) C.RustBuffer {
	return LowerIntoRustBuffer[TestResult](c, value)
}

func (c FfiConverterTestResult) LowerExternal(value TestResult) ExternalCRustBuffer {
	return RustBufferFromC(LowerIntoRustBuffer[TestResult](c, value))
}

func (c FfiConverterTestResult) Write(writer io.Writer, value TestResult) {
	FfiConverterStringINSTANCE.Write(writer, value.TestId)
	FfiConverterTestResultStatusINSTANCE.Write(writer, value.Status)
}

type FfiDestroyerTestResult struct{}

func (_ FfiDestroyerTestResult) Destroy(value TestResult) {
	value.Destroy()
}

// A single validation error with a message and the JSON path where it occurred.
type ValidationError struct {
	Message      string
	InstancePath string
}

func (r *ValidationError) Destroy() {
	FfiDestroyerString{}.Destroy(r.Message)
	FfiDestroyerString{}.Destroy(r.InstancePath)
}

type FfiConverterValidationError struct{}

var FfiConverterValidationErrorINSTANCE = FfiConverterValidationError{}

func (c FfiConverterValidationError) Lift(rb RustBufferI) ValidationError {
	return LiftFromRustBuffer[ValidationError](c, rb)
}

func (c FfiConverterValidationError) Read(reader io.Reader) ValidationError {
	return ValidationError{
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterStringINSTANCE.Read(reader),
	}
}

func (c FfiConverterValidationError) Lower(value ValidationError) C.RustBuffer {
	return LowerIntoRustBuffer[ValidationError](c, value)
}

func (c FfiConverterValidationError) LowerExternal(value ValidationError) ExternalCRustBuffer {
	return RustBufferFromC(LowerIntoRustBuffer[ValidationError](c, value))
}

func (c FfiConverterValidationError) Write(writer io.Writer, value ValidationError) {
	FfiConverterStringINSTANCE.Write(writer, value.Message)
	FfiConverterStringINSTANCE.Write(writer, value.InstancePath)
}

type FfiDestroyerValidationError struct{}

func (_ FfiDestroyerValidationError) Destroy(value ValidationError) {
	value.Destroy()
}

// Overall result of a CSAF validation run.
type ValidationResult struct {
	// Whether the validation was successful (no errors).
	Success bool
	// The detected CSAF version.
	Version string
	// Individual test results.
	TestResults []TestResult
	// Total number of errors.
	NumErrors uint64
	// Total number of warnings.
	NumWarnings uint64
	// Total number of informational findings.
	NumInfos uint64
	// Total number of tests not found.
	NumNotFound uint64
}

func (r *ValidationResult) Destroy() {
	FfiDestroyerBool{}.Destroy(r.Success)
	FfiDestroyerString{}.Destroy(r.Version)
	FfiDestroyerSequenceTestResult{}.Destroy(r.TestResults)
	FfiDestroyerUint64{}.Destroy(r.NumErrors)
	FfiDestroyerUint64{}.Destroy(r.NumWarnings)
	FfiDestroyerUint64{}.Destroy(r.NumInfos)
	FfiDestroyerUint64{}.Destroy(r.NumNotFound)
}

type FfiConverterValidationResult struct{}

var FfiConverterValidationResultINSTANCE = FfiConverterValidationResult{}

func (c FfiConverterValidationResult) Lift(rb RustBufferI) ValidationResult {
	return LiftFromRustBuffer[ValidationResult](c, rb)
}

func (c FfiConverterValidationResult) Read(reader io.Reader) ValidationResult {
	return ValidationResult{
		FfiConverterBoolINSTANCE.Read(reader),
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterSequenceTestResultINSTANCE.Read(reader),
		FfiConverterUint64INSTANCE.Read(reader),
		FfiConverterUint64INSTANCE.Read(reader),
		FfiConverterUint64INSTANCE.Read(reader),
		FfiConverterUint64INSTANCE.Read(reader),
	}
}

func (c FfiConverterValidationResult) Lower(value ValidationResult) C.RustBuffer {
	return LowerIntoRustBuffer[ValidationResult](c, value)
}

func (c FfiConverterValidationResult) LowerExternal(value ValidationResult) ExternalCRustBuffer {
	return RustBufferFromC(LowerIntoRustBuffer[ValidationResult](c, value))
}

func (c FfiConverterValidationResult) Write(writer io.Writer, value ValidationResult) {
	FfiConverterBoolINSTANCE.Write(writer, value.Success)
	FfiConverterStringINSTANCE.Write(writer, value.Version)
	FfiConverterSequenceTestResultINSTANCE.Write(writer, value.TestResults)
	FfiConverterUint64INSTANCE.Write(writer, value.NumErrors)
	FfiConverterUint64INSTANCE.Write(writer, value.NumWarnings)
	FfiConverterUint64INSTANCE.Write(writer, value.NumInfos)
	FfiConverterUint64INSTANCE.Write(writer, value.NumNotFound)
}

type FfiDestroyerValidationResult struct{}

func (_ FfiDestroyerValidationResult) Destroy(value ValidationResult) {
	value.Destroy()
}

type VulnerabilityId struct {
	SystemName string
	Text       string
}

func (r *VulnerabilityId) Destroy() {
	FfiDestroyerString{}.Destroy(r.SystemName)
	FfiDestroyerString{}.Destroy(r.Text)
}

type FfiConverterVulnerabilityId struct{}

var FfiConverterVulnerabilityIdINSTANCE = FfiConverterVulnerabilityId{}

func (c FfiConverterVulnerabilityId) Lift(rb RustBufferI) VulnerabilityId {
	return LiftFromRustBuffer[VulnerabilityId](c, rb)
}

func (c FfiConverterVulnerabilityId) Read(reader io.Reader) VulnerabilityId {
	return VulnerabilityId{
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterStringINSTANCE.Read(reader),
	}
}

func (c FfiConverterVulnerabilityId) Lower(value VulnerabilityId) C.RustBuffer {
	return LowerIntoRustBuffer[VulnerabilityId](c, value)
}

func (c FfiConverterVulnerabilityId) LowerExternal(value VulnerabilityId) ExternalCRustBuffer {
	return RustBufferFromC(LowerIntoRustBuffer[VulnerabilityId](c, value))
}

func (c FfiConverterVulnerabilityId) Write(writer io.Writer, value VulnerabilityId) {
	FfiConverterStringINSTANCE.Write(writer, value.SystemName)
	FfiConverterStringINSTANCE.Write(writer, value.Text)
}

type FfiDestroyerVulnerabilityId struct{}

func (_ FfiDestroyerVulnerabilityId) Destroy(value VulnerabilityId) {
	value.Destroy()
}

type CategoryOfTheBranch uint

const (
	CategoryOfTheBranchArchitecture        CategoryOfTheBranch = 1
	CategoryOfTheBranchHostName            CategoryOfTheBranch = 2
	CategoryOfTheBranchLanguage            CategoryOfTheBranch = 3
	CategoryOfTheBranchLegacy              CategoryOfTheBranch = 4
	CategoryOfTheBranchPatchLevel          CategoryOfTheBranch = 5
	CategoryOfTheBranchPlatform            CategoryOfTheBranch = 6
	CategoryOfTheBranchProductFamily       CategoryOfTheBranch = 7
	CategoryOfTheBranchProductName         CategoryOfTheBranch = 8
	CategoryOfTheBranchProductVersion      CategoryOfTheBranch = 9
	CategoryOfTheBranchProductVersionRange CategoryOfTheBranch = 10
	CategoryOfTheBranchServicePack         CategoryOfTheBranch = 11
	CategoryOfTheBranchSpecification       CategoryOfTheBranch = 12
	CategoryOfTheBranchVendor              CategoryOfTheBranch = 13
)

type FfiConverterCategoryOfTheBranch struct{}

var FfiConverterCategoryOfTheBranchINSTANCE = FfiConverterCategoryOfTheBranch{}

func (c FfiConverterCategoryOfTheBranch) Lift(rb RustBufferI) CategoryOfTheBranch {
	return LiftFromRustBuffer[CategoryOfTheBranch](c, rb)
}

func (c FfiConverterCategoryOfTheBranch) Lower(value CategoryOfTheBranch) C.RustBuffer {
	return LowerIntoRustBuffer[CategoryOfTheBranch](c, value)
}

func (c FfiConverterCategoryOfTheBranch) LowerExternal(value CategoryOfTheBranch) ExternalCRustBuffer {
	return RustBufferFromC(LowerIntoRustBuffer[CategoryOfTheBranch](c, value))
}
func (FfiConverterCategoryOfTheBranch) Read(reader io.Reader) CategoryOfTheBranch {
	id := readInt32(reader)
	return CategoryOfTheBranch(id)
}

func (FfiConverterCategoryOfTheBranch) Write(writer io.Writer, value CategoryOfTheBranch) {
	writeInt32(writer, int32(value))
}

type FfiDestroyerCategoryOfTheBranch struct{}

func (_ FfiDestroyerCategoryOfTheBranch) Destroy(value CategoryOfTheBranch) {
}

type CsafDateTime interface {
	Destroy()
}
type CsafDateTimeValid struct {
	RawString string
	UtcString string
}

func (e CsafDateTimeValid) Destroy() {
	FfiDestroyerString{}.Destroy(e.RawString)
	FfiDestroyerString{}.Destroy(e.UtcString)
}

type CsafDateTimeInvalid struct {
	RawString string
}

func (e CsafDateTimeInvalid) Destroy() {
	FfiDestroyerString{}.Destroy(e.RawString)
}

type FfiConverterCsafDateTime struct{}

var FfiConverterCsafDateTimeINSTANCE = FfiConverterCsafDateTime{}

func (c FfiConverterCsafDateTime) Lift(rb RustBufferI) CsafDateTime {
	return LiftFromRustBuffer[CsafDateTime](c, rb)
}

func (c FfiConverterCsafDateTime) Lower(value CsafDateTime) C.RustBuffer {
	return LowerIntoRustBuffer[CsafDateTime](c, value)
}

func (c FfiConverterCsafDateTime) LowerExternal(value CsafDateTime) ExternalCRustBuffer {
	return RustBufferFromC(LowerIntoRustBuffer[CsafDateTime](c, value))
}
func (FfiConverterCsafDateTime) Read(reader io.Reader) CsafDateTime {
	id := readInt32(reader)
	switch id {
	case 1:
		return CsafDateTimeValid{
			FfiConverterStringINSTANCE.Read(reader),
			FfiConverterStringINSTANCE.Read(reader),
		}
	case 2:
		return CsafDateTimeInvalid{
			FfiConverterStringINSTANCE.Read(reader),
		}
	default:
		panic(fmt.Sprintf("invalid enum value %v in FfiConverterCsafDateTime.Read()", id))
	}
}

func (FfiConverterCsafDateTime) Write(writer io.Writer, value CsafDateTime) {
	switch variant_value := value.(type) {
	case CsafDateTimeValid:
		writeInt32(writer, 1)
		FfiConverterStringINSTANCE.Write(writer, variant_value.RawString)
		FfiConverterStringINSTANCE.Write(writer, variant_value.UtcString)
	case CsafDateTimeInvalid:
		writeInt32(writer, 2)
		FfiConverterStringINSTANCE.Write(writer, variant_value.RawString)
	default:
		_ = variant_value
		panic(fmt.Sprintf("invalid enum value `%v` in FfiConverterCsafDateTime.Write", value))
	}
}

type FfiDestroyerCsafDateTime struct{}

func (_ FfiDestroyerCsafDateTime) Destroy(value CsafDateTime) {
	value.Destroy()
}

type CsafError struct {
	err error
}

// Convience method to turn *CsafError into error
// Avoiding treating nil pointer as non nil error interface
func (err *CsafError) AsError() error {
	if err == nil {
		return nil
	} else {
		return err
	}
}

func (err CsafError) Error() string {
	return fmt.Sprintf("CsafError: %s", err.err.Error())
}

func (err CsafError) Unwrap() error {
	return err.err
}

// Err* are used for checking error type with `errors.Is`
var ErrCsafErrorInvalidJson = fmt.Errorf("CsafErrorInvalidJson")
var ErrCsafErrorMissingVersion = fmt.Errorf("CsafErrorMissingVersion")
var ErrCsafErrorUnsupportedVersion = fmt.Errorf("CsafErrorUnsupportedVersion")
var ErrCsafErrorLoadError = fmt.Errorf("CsafErrorLoadError")

// Variant structs
type CsafErrorInvalidJson struct {
	Message string
}

func NewCsafErrorInvalidJson(
	message string,
) *CsafError {
	return &CsafError{err: &CsafErrorInvalidJson{
		Message: message}}
}

func (e CsafErrorInvalidJson) destroy() {
	FfiDestroyerString{}.Destroy(e.Message)
}

func (err CsafErrorInvalidJson) Error() string {
	return fmt.Sprint("InvalidJson",
		": ",

		"Message=",
		err.Message,
	)
}

func (self CsafErrorInvalidJson) Is(target error) bool {
	return target == ErrCsafErrorInvalidJson
}

type CsafErrorMissingVersion struct {
	Message string
}

func NewCsafErrorMissingVersion(
	message string,
) *CsafError {
	return &CsafError{err: &CsafErrorMissingVersion{
		Message: message}}
}

func (e CsafErrorMissingVersion) destroy() {
	FfiDestroyerString{}.Destroy(e.Message)
}

func (err CsafErrorMissingVersion) Error() string {
	return fmt.Sprint("MissingVersion",
		": ",

		"Message=",
		err.Message,
	)
}

func (self CsafErrorMissingVersion) Is(target error) bool {
	return target == ErrCsafErrorMissingVersion
}

type CsafErrorUnsupportedVersion struct {
	Version string
}

func NewCsafErrorUnsupportedVersion(
	version string,
) *CsafError {
	return &CsafError{err: &CsafErrorUnsupportedVersion{
		Version: version}}
}

func (e CsafErrorUnsupportedVersion) destroy() {
	FfiDestroyerString{}.Destroy(e.Version)
}

func (err CsafErrorUnsupportedVersion) Error() string {
	return fmt.Sprint("UnsupportedVersion",
		": ",

		"Version=",
		err.Version,
	)
}

func (self CsafErrorUnsupportedVersion) Is(target error) bool {
	return target == ErrCsafErrorUnsupportedVersion
}

type CsafErrorLoadError struct {
	Message string
}

func NewCsafErrorLoadError(
	message string,
) *CsafError {
	return &CsafError{err: &CsafErrorLoadError{
		Message: message}}
}

func (e CsafErrorLoadError) destroy() {
	FfiDestroyerString{}.Destroy(e.Message)
}

func (err CsafErrorLoadError) Error() string {
	return fmt.Sprint("LoadError",
		": ",

		"Message=",
		err.Message,
	)
}

func (self CsafErrorLoadError) Is(target error) bool {
	return target == ErrCsafErrorLoadError
}

type FfiConverterCsafError struct{}

var FfiConverterCsafErrorINSTANCE = FfiConverterCsafError{}

func (c FfiConverterCsafError) Lift(eb RustBufferI) *CsafError {
	return LiftFromRustBuffer[*CsafError](c, eb)
}

func (c FfiConverterCsafError) Lower(value *CsafError) C.RustBuffer {
	return LowerIntoRustBuffer[*CsafError](c, value)
}

func (c FfiConverterCsafError) LowerExternal(value *CsafError) ExternalCRustBuffer {
	return RustBufferFromC(LowerIntoRustBuffer[*CsafError](c, value))
}

func (c FfiConverterCsafError) Read(reader io.Reader) *CsafError {
	errorID := readUint32(reader)

	switch errorID {
	case 1:
		return &CsafError{&CsafErrorInvalidJson{
			Message: FfiConverterStringINSTANCE.Read(reader),
		}}
	case 2:
		return &CsafError{&CsafErrorMissingVersion{
			Message: FfiConverterStringINSTANCE.Read(reader),
		}}
	case 3:
		return &CsafError{&CsafErrorUnsupportedVersion{
			Version: FfiConverterStringINSTANCE.Read(reader),
		}}
	case 4:
		return &CsafError{&CsafErrorLoadError{
			Message: FfiConverterStringINSTANCE.Read(reader),
		}}
	default:
		panic(fmt.Sprintf("Unknown error code %d in FfiConverterCsafError.Read()", errorID))
	}
}

func (c FfiConverterCsafError) Write(writer io.Writer, value *CsafError) {
	switch variantValue := value.err.(type) {
	case *CsafErrorInvalidJson:
		writeInt32(writer, 1)
		FfiConverterStringINSTANCE.Write(writer, variantValue.Message)
	case *CsafErrorMissingVersion:
		writeInt32(writer, 2)
		FfiConverterStringINSTANCE.Write(writer, variantValue.Message)
	case *CsafErrorUnsupportedVersion:
		writeInt32(writer, 3)
		FfiConverterStringINSTANCE.Write(writer, variantValue.Version)
	case *CsafErrorLoadError:
		writeInt32(writer, 4)
		FfiConverterStringINSTANCE.Write(writer, variantValue.Message)
	default:
		_ = variantValue
		panic(fmt.Sprintf("invalid error value `%v` in FfiConverterCsafError.Write", value))
	}
}

type FfiDestroyerCsafError struct{}

func (_ FfiDestroyerCsafError) Destroy(value *CsafError) {
	switch variantValue := value.err.(type) {
	case CsafErrorInvalidJson:
		variantValue.destroy()
	case CsafErrorMissingVersion:
		variantValue.destroy()
	case CsafErrorUnsupportedVersion:
		variantValue.destroy()
	case CsafErrorLoadError:
		variantValue.destroy()
	default:
		_ = variantValue
		panic(fmt.Sprintf("invalid error value `%v` in FfiDestroyerCsafError.Destroy", value))
	}
}

type CsafLanguage interface {
	Destroy()
}
type CsafLanguageValid struct {
	Value string
}

func (e CsafLanguageValid) Destroy() {
	FfiDestroyerString{}.Destroy(e.Value)
}

type CsafLanguageInvalid struct {
	Value string
}

func (e CsafLanguageInvalid) Destroy() {
	FfiDestroyerString{}.Destroy(e.Value)
}

type FfiConverterCsafLanguage struct{}

var FfiConverterCsafLanguageINSTANCE = FfiConverterCsafLanguage{}

func (c FfiConverterCsafLanguage) Lift(rb RustBufferI) CsafLanguage {
	return LiftFromRustBuffer[CsafLanguage](c, rb)
}

func (c FfiConverterCsafLanguage) Lower(value CsafLanguage) C.RustBuffer {
	return LowerIntoRustBuffer[CsafLanguage](c, value)
}

func (c FfiConverterCsafLanguage) LowerExternal(value CsafLanguage) ExternalCRustBuffer {
	return RustBufferFromC(LowerIntoRustBuffer[CsafLanguage](c, value))
}
func (FfiConverterCsafLanguage) Read(reader io.Reader) CsafLanguage {
	id := readInt32(reader)
	switch id {
	case 1:
		return CsafLanguageValid{
			FfiConverterStringINSTANCE.Read(reader),
		}
	case 2:
		return CsafLanguageInvalid{
			FfiConverterStringINSTANCE.Read(reader),
		}
	default:
		panic(fmt.Sprintf("invalid enum value %v in FfiConverterCsafLanguage.Read()", id))
	}
}

func (FfiConverterCsafLanguage) Write(writer io.Writer, value CsafLanguage) {
	switch variant_value := value.(type) {
	case CsafLanguageValid:
		writeInt32(writer, 1)
		FfiConverterStringINSTANCE.Write(writer, variant_value.Value)
	case CsafLanguageInvalid:
		writeInt32(writer, 2)
		FfiConverterStringINSTANCE.Write(writer, variant_value.Value)
	default:
		_ = variant_value
		panic(fmt.Sprintf("invalid enum value `%v` in FfiConverterCsafLanguage.Write", value))
	}
}

type FfiDestroyerCsafLanguage struct{}

func (_ FfiDestroyerCsafLanguage) Destroy(value CsafLanguage) {
	value.Destroy()
}

type CsafVersion uint

const (
	CsafVersionV20 CsafVersion = 1
	CsafVersionV21 CsafVersion = 2
)

type FfiConverterCsafVersion struct{}

var FfiConverterCsafVersionINSTANCE = FfiConverterCsafVersion{}

func (c FfiConverterCsafVersion) Lift(rb RustBufferI) CsafVersion {
	return LiftFromRustBuffer[CsafVersion](c, rb)
}

func (c FfiConverterCsafVersion) Lower(value CsafVersion) C.RustBuffer {
	return LowerIntoRustBuffer[CsafVersion](c, value)
}

func (c FfiConverterCsafVersion) LowerExternal(value CsafVersion) ExternalCRustBuffer {
	return RustBufferFromC(LowerIntoRustBuffer[CsafVersion](c, value))
}
func (FfiConverterCsafVersion) Read(reader io.Reader) CsafVersion {
	id := readInt32(reader)
	return CsafVersion(id)
}

func (FfiConverterCsafVersion) Write(writer io.Writer, value CsafVersion) {
	writeInt32(writer, int32(value))
}

type FfiDestroyerCsafVersion struct{}

func (_ FfiDestroyerCsafVersion) Destroy(value CsafVersion) {
}

type DocumentCategory interface {
	Destroy()
}
type DocumentCategoryCsafBase struct {
}

func (e DocumentCategoryCsafBase) Destroy() {
}

type DocumentCategoryCsafInformationalAdvisory struct {
}

func (e DocumentCategoryCsafInformationalAdvisory) Destroy() {
}

type DocumentCategoryCsafSecurityIncidentResponse struct {
}

func (e DocumentCategoryCsafSecurityIncidentResponse) Destroy() {
}

type DocumentCategoryCsafSecurityAdvisory struct {
}

func (e DocumentCategoryCsafSecurityAdvisory) Destroy() {
}

type DocumentCategoryCsafVex struct {
}

func (e DocumentCategoryCsafVex) Destroy() {
}

type DocumentCategoryCsafWithdrawn struct {
}

func (e DocumentCategoryCsafWithdrawn) Destroy() {
}

type DocumentCategoryCsafSuperseded struct {
}

func (e DocumentCategoryCsafSuperseded) Destroy() {
}

type DocumentCategoryCsafDeprecatedSecurityAdvisory struct {
}

func (e DocumentCategoryCsafDeprecatedSecurityAdvisory) Destroy() {
}

type DocumentCategoryCsafBaseOther struct {
	Value string
}

func (e DocumentCategoryCsafBaseOther) Destroy() {
	FfiDestroyerString{}.Destroy(e.Value)
}

type FfiConverterDocumentCategory struct{}

var FfiConverterDocumentCategoryINSTANCE = FfiConverterDocumentCategory{}

func (c FfiConverterDocumentCategory) Lift(rb RustBufferI) DocumentCategory {
	return LiftFromRustBuffer[DocumentCategory](c, rb)
}

func (c FfiConverterDocumentCategory) Lower(value DocumentCategory) C.RustBuffer {
	return LowerIntoRustBuffer[DocumentCategory](c, value)
}

func (c FfiConverterDocumentCategory) LowerExternal(value DocumentCategory) ExternalCRustBuffer {
	return RustBufferFromC(LowerIntoRustBuffer[DocumentCategory](c, value))
}
func (FfiConverterDocumentCategory) Read(reader io.Reader) DocumentCategory {
	id := readInt32(reader)
	switch id {
	case 1:
		return DocumentCategoryCsafBase{}
	case 2:
		return DocumentCategoryCsafInformationalAdvisory{}
	case 3:
		return DocumentCategoryCsafSecurityIncidentResponse{}
	case 4:
		return DocumentCategoryCsafSecurityAdvisory{}
	case 5:
		return DocumentCategoryCsafVex{}
	case 6:
		return DocumentCategoryCsafWithdrawn{}
	case 7:
		return DocumentCategoryCsafSuperseded{}
	case 8:
		return DocumentCategoryCsafDeprecatedSecurityAdvisory{}
	case 9:
		return DocumentCategoryCsafBaseOther{
			FfiConverterStringINSTANCE.Read(reader),
		}
	default:
		panic(fmt.Sprintf("invalid enum value %v in FfiConverterDocumentCategory.Read()", id))
	}
}

func (FfiConverterDocumentCategory) Write(writer io.Writer, value DocumentCategory) {
	switch variant_value := value.(type) {
	case DocumentCategoryCsafBase:
		writeInt32(writer, 1)
	case DocumentCategoryCsafInformationalAdvisory:
		writeInt32(writer, 2)
	case DocumentCategoryCsafSecurityIncidentResponse:
		writeInt32(writer, 3)
	case DocumentCategoryCsafSecurityAdvisory:
		writeInt32(writer, 4)
	case DocumentCategoryCsafVex:
		writeInt32(writer, 5)
	case DocumentCategoryCsafWithdrawn:
		writeInt32(writer, 6)
	case DocumentCategoryCsafSuperseded:
		writeInt32(writer, 7)
	case DocumentCategoryCsafDeprecatedSecurityAdvisory:
		writeInt32(writer, 8)
	case DocumentCategoryCsafBaseOther:
		writeInt32(writer, 9)
		FfiConverterStringINSTANCE.Write(writer, variant_value.Value)
	default:
		_ = variant_value
		panic(fmt.Sprintf("invalid enum value `%v` in FfiConverterDocumentCategory.Write", value))
	}
}

type FfiDestroyerDocumentCategory struct{}

func (_ FfiDestroyerDocumentCategory) Destroy(value DocumentCategory) {
	value.Destroy()
}

type ProductStatusGroup uint

const (
	ProductStatusGroupAffected           ProductStatusGroup = 1
	ProductStatusGroupNotAffected        ProductStatusGroup = 2
	ProductStatusGroupFixed              ProductStatusGroup = 3
	ProductStatusGroupUnderInvestigation ProductStatusGroup = 4
	ProductStatusGroupUnknown            ProductStatusGroup = 5
	ProductStatusGroupRecommended        ProductStatusGroup = 6
)

type FfiConverterProductStatusGroup struct{}

var FfiConverterProductStatusGroupINSTANCE = FfiConverterProductStatusGroup{}

func (c FfiConverterProductStatusGroup) Lift(rb RustBufferI) ProductStatusGroup {
	return LiftFromRustBuffer[ProductStatusGroup](c, rb)
}

func (c FfiConverterProductStatusGroup) Lower(value ProductStatusGroup) C.RustBuffer {
	return LowerIntoRustBuffer[ProductStatusGroup](c, value)
}

func (c FfiConverterProductStatusGroup) LowerExternal(value ProductStatusGroup) ExternalCRustBuffer {
	return RustBufferFromC(LowerIntoRustBuffer[ProductStatusGroup](c, value))
}
func (FfiConverterProductStatusGroup) Read(reader io.Reader) ProductStatusGroup {
	id := readInt32(reader)
	return ProductStatusGroup(id)
}

func (FfiConverterProductStatusGroup) Write(writer io.Writer, value ProductStatusGroup) {
	writeInt32(writer, int32(value))
}

type FfiDestroyerProductStatusGroup struct{}

func (_ FfiDestroyerProductStatusGroup) Destroy(value ProductStatusGroup) {
}

// Status of a single test execution.
type TestResultStatus interface {
	Destroy()
}
type TestResultStatusSuccess struct {
}

func (e TestResultStatusSuccess) Destroy() {
}

type TestResultStatusFailure struct {
	Errors   []ValidationError
	Warnings []ValidationError
	Infos    []ValidationError
}

func (e TestResultStatusFailure) Destroy() {
	FfiDestroyerSequenceValidationError{}.Destroy(e.Errors)
	FfiDestroyerSequenceValidationError{}.Destroy(e.Warnings)
	FfiDestroyerSequenceValidationError{}.Destroy(e.Infos)
}

type TestResultStatusNotFound struct {
}

func (e TestResultStatusNotFound) Destroy() {
}

type TestResultStatusSkipped struct {
}

func (e TestResultStatusSkipped) Destroy() {
}

type FfiConverterTestResultStatus struct{}

var FfiConverterTestResultStatusINSTANCE = FfiConverterTestResultStatus{}

func (c FfiConverterTestResultStatus) Lift(rb RustBufferI) TestResultStatus {
	return LiftFromRustBuffer[TestResultStatus](c, rb)
}

func (c FfiConverterTestResultStatus) Lower(value TestResultStatus) C.RustBuffer {
	return LowerIntoRustBuffer[TestResultStatus](c, value)
}

func (c FfiConverterTestResultStatus) LowerExternal(value TestResultStatus) ExternalCRustBuffer {
	return RustBufferFromC(LowerIntoRustBuffer[TestResultStatus](c, value))
}
func (FfiConverterTestResultStatus) Read(reader io.Reader) TestResultStatus {
	id := readInt32(reader)
	switch id {
	case 1:
		return TestResultStatusSuccess{}
	case 2:
		return TestResultStatusFailure{
			FfiConverterSequenceValidationErrorINSTANCE.Read(reader),
			FfiConverterSequenceValidationErrorINSTANCE.Read(reader),
			FfiConverterSequenceValidationErrorINSTANCE.Read(reader),
		}
	case 3:
		return TestResultStatusNotFound{}
	case 4:
		return TestResultStatusSkipped{}
	default:
		panic(fmt.Sprintf("invalid enum value %v in FfiConverterTestResultStatus.Read()", id))
	}
}

func (FfiConverterTestResultStatus) Write(writer io.Writer, value TestResultStatus) {
	switch variant_value := value.(type) {
	case TestResultStatusSuccess:
		writeInt32(writer, 1)
	case TestResultStatusFailure:
		writeInt32(writer, 2)
		FfiConverterSequenceValidationErrorINSTANCE.Write(writer, variant_value.Errors)
		FfiConverterSequenceValidationErrorINSTANCE.Write(writer, variant_value.Warnings)
		FfiConverterSequenceValidationErrorINSTANCE.Write(writer, variant_value.Infos)
	case TestResultStatusNotFound:
		writeInt32(writer, 3)
	case TestResultStatusSkipped:
		writeInt32(writer, 4)
	default:
		_ = variant_value
		panic(fmt.Sprintf("invalid enum value `%v` in FfiConverterTestResultStatus.Write", value))
	}
}

type FfiDestroyerTestResultStatus struct{}

func (_ FfiDestroyerTestResultStatus) Destroy(value TestResultStatus) {
	value.Destroy()
}

type VulnerabilityMetric interface {
	Destroy()
}
type VulnerabilityMetricSsvcV1 struct {
}

func (e VulnerabilityMetricSsvcV1) Destroy() {
}

type VulnerabilityMetricCvssV2 struct {
	Version string
}

func (e VulnerabilityMetricCvssV2) Destroy() {
	FfiDestroyerString{}.Destroy(e.Version)
}

type VulnerabilityMetricCvssV3 struct {
	Version string
}

func (e VulnerabilityMetricCvssV3) Destroy() {
	FfiDestroyerString{}.Destroy(e.Version)
}

type VulnerabilityMetricCvssV4 struct {
	Version string
}

func (e VulnerabilityMetricCvssV4) Destroy() {
	FfiDestroyerString{}.Destroy(e.Version)
}

type VulnerabilityMetricEpss struct {
}

func (e VulnerabilityMetricEpss) Destroy() {
}

type VulnerabilityMetricQualitativeSeverityRating struct {
}

func (e VulnerabilityMetricQualitativeSeverityRating) Destroy() {
}

type FfiConverterVulnerabilityMetric struct{}

var FfiConverterVulnerabilityMetricINSTANCE = FfiConverterVulnerabilityMetric{}

func (c FfiConverterVulnerabilityMetric) Lift(rb RustBufferI) VulnerabilityMetric {
	return LiftFromRustBuffer[VulnerabilityMetric](c, rb)
}

func (c FfiConverterVulnerabilityMetric) Lower(value VulnerabilityMetric) C.RustBuffer {
	return LowerIntoRustBuffer[VulnerabilityMetric](c, value)
}

func (c FfiConverterVulnerabilityMetric) LowerExternal(value VulnerabilityMetric) ExternalCRustBuffer {
	return RustBufferFromC(LowerIntoRustBuffer[VulnerabilityMetric](c, value))
}
func (FfiConverterVulnerabilityMetric) Read(reader io.Reader) VulnerabilityMetric {
	id := readInt32(reader)
	switch id {
	case 1:
		return VulnerabilityMetricSsvcV1{}
	case 2:
		return VulnerabilityMetricCvssV2{
			FfiConverterStringINSTANCE.Read(reader),
		}
	case 3:
		return VulnerabilityMetricCvssV3{
			FfiConverterStringINSTANCE.Read(reader),
		}
	case 4:
		return VulnerabilityMetricCvssV4{
			FfiConverterStringINSTANCE.Read(reader),
		}
	case 5:
		return VulnerabilityMetricEpss{}
	case 6:
		return VulnerabilityMetricQualitativeSeverityRating{}
	default:
		panic(fmt.Sprintf("invalid enum value %v in FfiConverterVulnerabilityMetric.Read()", id))
	}
}

func (FfiConverterVulnerabilityMetric) Write(writer io.Writer, value VulnerabilityMetric) {
	switch variant_value := value.(type) {
	case VulnerabilityMetricSsvcV1:
		writeInt32(writer, 1)
	case VulnerabilityMetricCvssV2:
		writeInt32(writer, 2)
		FfiConverterStringINSTANCE.Write(writer, variant_value.Version)
	case VulnerabilityMetricCvssV3:
		writeInt32(writer, 3)
		FfiConverterStringINSTANCE.Write(writer, variant_value.Version)
	case VulnerabilityMetricCvssV4:
		writeInt32(writer, 4)
		FfiConverterStringINSTANCE.Write(writer, variant_value.Version)
	case VulnerabilityMetricEpss:
		writeInt32(writer, 5)
	case VulnerabilityMetricQualitativeSeverityRating:
		writeInt32(writer, 6)
	default:
		_ = variant_value
		panic(fmt.Sprintf("invalid enum value `%v` in FfiConverterVulnerabilityMetric.Write", value))
	}
}

type FfiDestroyerVulnerabilityMetric struct{}

func (_ FfiDestroyerVulnerabilityMetric) Destroy(value VulnerabilityMetric) {
	value.Destroy()
}

type FfiConverterOptionalString struct{}

var FfiConverterOptionalStringINSTANCE = FfiConverterOptionalString{}

func (c FfiConverterOptionalString) Lift(rb RustBufferI) *string {
	return LiftFromRustBuffer[*string](c, rb)
}

func (_ FfiConverterOptionalString) Read(reader io.Reader) *string {
	if readInt8(reader) == 0 {
		return nil
	}
	temp := FfiConverterStringINSTANCE.Read(reader)
	return &temp
}

func (c FfiConverterOptionalString) Lower(value *string) C.RustBuffer {
	return LowerIntoRustBuffer[*string](c, value)
}

func (c FfiConverterOptionalString) LowerExternal(value *string) ExternalCRustBuffer {
	return RustBufferFromC(LowerIntoRustBuffer[*string](c, value))
}

func (_ FfiConverterOptionalString) Write(writer io.Writer, value *string) {
	if value == nil {
		writeInt8(writer, 0)
	} else {
		writeInt8(writer, 1)
		FfiConverterStringINSTANCE.Write(writer, *value)
	}
}

type FfiDestroyerOptionalString struct{}

func (_ FfiDestroyerOptionalString) Destroy(value *string) {
	if value != nil {
		FfiDestroyerString{}.Destroy(*value)
	}
}

type FfiConverterOptionalCsafDateTime struct{}

var FfiConverterOptionalCsafDateTimeINSTANCE = FfiConverterOptionalCsafDateTime{}

func (c FfiConverterOptionalCsafDateTime) Lift(rb RustBufferI) *CsafDateTime {
	return LiftFromRustBuffer[*CsafDateTime](c, rb)
}

func (_ FfiConverterOptionalCsafDateTime) Read(reader io.Reader) *CsafDateTime {
	if readInt8(reader) == 0 {
		return nil
	}
	temp := FfiConverterCsafDateTimeINSTANCE.Read(reader)
	return &temp
}

func (c FfiConverterOptionalCsafDateTime) Lower(value *CsafDateTime) C.RustBuffer {
	return LowerIntoRustBuffer[*CsafDateTime](c, value)
}

func (c FfiConverterOptionalCsafDateTime) LowerExternal(value *CsafDateTime) ExternalCRustBuffer {
	return RustBufferFromC(LowerIntoRustBuffer[*CsafDateTime](c, value))
}

func (_ FfiConverterOptionalCsafDateTime) Write(writer io.Writer, value *CsafDateTime) {
	if value == nil {
		writeInt8(writer, 0)
	} else {
		writeInt8(writer, 1)
		FfiConverterCsafDateTimeINSTANCE.Write(writer, *value)
	}
}

type FfiDestroyerOptionalCsafDateTime struct{}

func (_ FfiDestroyerOptionalCsafDateTime) Destroy(value *CsafDateTime) {
	if value != nil {
		FfiDestroyerCsafDateTime{}.Destroy(*value)
	}
}

type FfiConverterOptionalCsafLanguage struct{}

var FfiConverterOptionalCsafLanguageINSTANCE = FfiConverterOptionalCsafLanguage{}

func (c FfiConverterOptionalCsafLanguage) Lift(rb RustBufferI) *CsafLanguage {
	return LiftFromRustBuffer[*CsafLanguage](c, rb)
}

func (_ FfiConverterOptionalCsafLanguage) Read(reader io.Reader) *CsafLanguage {
	if readInt8(reader) == 0 {
		return nil
	}
	temp := FfiConverterCsafLanguageINSTANCE.Read(reader)
	return &temp
}

func (c FfiConverterOptionalCsafLanguage) Lower(value *CsafLanguage) C.RustBuffer {
	return LowerIntoRustBuffer[*CsafLanguage](c, value)
}

func (c FfiConverterOptionalCsafLanguage) LowerExternal(value *CsafLanguage) ExternalCRustBuffer {
	return RustBufferFromC(LowerIntoRustBuffer[*CsafLanguage](c, value))
}

func (_ FfiConverterOptionalCsafLanguage) Write(writer io.Writer, value *CsafLanguage) {
	if value == nil {
		writeInt8(writer, 0)
	} else {
		writeInt8(writer, 1)
		FfiConverterCsafLanguageINSTANCE.Write(writer, *value)
	}
}

type FfiDestroyerOptionalCsafLanguage struct{}

func (_ FfiDestroyerOptionalCsafLanguage) Destroy(value *CsafLanguage) {
	if value != nil {
		FfiDestroyerCsafLanguage{}.Destroy(*value)
	}
}

type FfiConverterSequenceString struct{}

var FfiConverterSequenceStringINSTANCE = FfiConverterSequenceString{}

func (c FfiConverterSequenceString) Lift(rb RustBufferI) []string {
	return LiftFromRustBuffer[[]string](c, rb)
}

func (c FfiConverterSequenceString) Read(reader io.Reader) []string {
	length := readInt32(reader)
	if length == 0 {
		return nil
	}
	result := make([]string, 0, length)
	for i := int32(0); i < length; i++ {
		result = append(result, FfiConverterStringINSTANCE.Read(reader))
	}
	return result
}

func (c FfiConverterSequenceString) Lower(value []string) C.RustBuffer {
	return LowerIntoRustBuffer[[]string](c, value)
}

func (c FfiConverterSequenceString) LowerExternal(value []string) ExternalCRustBuffer {
	return RustBufferFromC(LowerIntoRustBuffer[[]string](c, value))
}

func (c FfiConverterSequenceString) Write(writer io.Writer, value []string) {
	if len(value) > math.MaxInt32 {
		panic("[]string is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	for _, item := range value {
		FfiConverterStringINSTANCE.Write(writer, item)
	}
}

type FfiDestroyerSequenceString struct{}

func (FfiDestroyerSequenceString) Destroy(sequence []string) {
	for _, value := range sequence {
		FfiDestroyerString{}.Destroy(value)
	}
}

type FfiConverterSequenceCwe struct{}

var FfiConverterSequenceCweINSTANCE = FfiConverterSequenceCwe{}

func (c FfiConverterSequenceCwe) Lift(rb RustBufferI) []Cwe {
	return LiftFromRustBuffer[[]Cwe](c, rb)
}

func (c FfiConverterSequenceCwe) Read(reader io.Reader) []Cwe {
	length := readInt32(reader)
	if length == 0 {
		return nil
	}
	result := make([]Cwe, 0, length)
	for i := int32(0); i < length; i++ {
		result = append(result, FfiConverterCweINSTANCE.Read(reader))
	}
	return result
}

func (c FfiConverterSequenceCwe) Lower(value []Cwe) C.RustBuffer {
	return LowerIntoRustBuffer[[]Cwe](c, value)
}

func (c FfiConverterSequenceCwe) LowerExternal(value []Cwe) ExternalCRustBuffer {
	return RustBufferFromC(LowerIntoRustBuffer[[]Cwe](c, value))
}

func (c FfiConverterSequenceCwe) Write(writer io.Writer, value []Cwe) {
	if len(value) > math.MaxInt32 {
		panic("[]Cwe is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	for _, item := range value {
		FfiConverterCweINSTANCE.Write(writer, item)
	}
}

type FfiDestroyerSequenceCwe struct{}

func (FfiDestroyerSequenceCwe) Destroy(sequence []Cwe) {
	for _, value := range sequence {
		FfiDestroyerCwe{}.Destroy(value)
	}
}

type FfiConverterSequenceProductReference struct{}

var FfiConverterSequenceProductReferenceINSTANCE = FfiConverterSequenceProductReference{}

func (c FfiConverterSequenceProductReference) Lift(rb RustBufferI) []ProductReference {
	return LiftFromRustBuffer[[]ProductReference](c, rb)
}

func (c FfiConverterSequenceProductReference) Read(reader io.Reader) []ProductReference {
	length := readInt32(reader)
	if length == 0 {
		return nil
	}
	result := make([]ProductReference, 0, length)
	for i := int32(0); i < length; i++ {
		result = append(result, FfiConverterProductReferenceINSTANCE.Read(reader))
	}
	return result
}

func (c FfiConverterSequenceProductReference) Lower(value []ProductReference) C.RustBuffer {
	return LowerIntoRustBuffer[[]ProductReference](c, value)
}

func (c FfiConverterSequenceProductReference) LowerExternal(value []ProductReference) ExternalCRustBuffer {
	return RustBufferFromC(LowerIntoRustBuffer[[]ProductReference](c, value))
}

func (c FfiConverterSequenceProductReference) Write(writer io.Writer, value []ProductReference) {
	if len(value) > math.MaxInt32 {
		panic("[]ProductReference is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	for _, item := range value {
		FfiConverterProductReferenceINSTANCE.Write(writer, item)
	}
}

type FfiDestroyerSequenceProductReference struct{}

func (FfiDestroyerSequenceProductReference) Destroy(sequence []ProductReference) {
	for _, value := range sequence {
		FfiDestroyerProductReference{}.Destroy(value)
	}
}

type FfiConverterSequenceTestResult struct{}

var FfiConverterSequenceTestResultINSTANCE = FfiConverterSequenceTestResult{}

func (c FfiConverterSequenceTestResult) Lift(rb RustBufferI) []TestResult {
	return LiftFromRustBuffer[[]TestResult](c, rb)
}

func (c FfiConverterSequenceTestResult) Read(reader io.Reader) []TestResult {
	length := readInt32(reader)
	if length == 0 {
		return nil
	}
	result := make([]TestResult, 0, length)
	for i := int32(0); i < length; i++ {
		result = append(result, FfiConverterTestResultINSTANCE.Read(reader))
	}
	return result
}

func (c FfiConverterSequenceTestResult) Lower(value []TestResult) C.RustBuffer {
	return LowerIntoRustBuffer[[]TestResult](c, value)
}

func (c FfiConverterSequenceTestResult) LowerExternal(value []TestResult) ExternalCRustBuffer {
	return RustBufferFromC(LowerIntoRustBuffer[[]TestResult](c, value))
}

func (c FfiConverterSequenceTestResult) Write(writer io.Writer, value []TestResult) {
	if len(value) > math.MaxInt32 {
		panic("[]TestResult is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	for _, item := range value {
		FfiConverterTestResultINSTANCE.Write(writer, item)
	}
}

type FfiDestroyerSequenceTestResult struct{}

func (FfiDestroyerSequenceTestResult) Destroy(sequence []TestResult) {
	for _, value := range sequence {
		FfiDestroyerTestResult{}.Destroy(value)
	}
}

type FfiConverterSequenceValidationError struct{}

var FfiConverterSequenceValidationErrorINSTANCE = FfiConverterSequenceValidationError{}

func (c FfiConverterSequenceValidationError) Lift(rb RustBufferI) []ValidationError {
	return LiftFromRustBuffer[[]ValidationError](c, rb)
}

func (c FfiConverterSequenceValidationError) Read(reader io.Reader) []ValidationError {
	length := readInt32(reader)
	if length == 0 {
		return nil
	}
	result := make([]ValidationError, 0, length)
	for i := int32(0); i < length; i++ {
		result = append(result, FfiConverterValidationErrorINSTANCE.Read(reader))
	}
	return result
}

func (c FfiConverterSequenceValidationError) Lower(value []ValidationError) C.RustBuffer {
	return LowerIntoRustBuffer[[]ValidationError](c, value)
}

func (c FfiConverterSequenceValidationError) LowerExternal(value []ValidationError) ExternalCRustBuffer {
	return RustBufferFromC(LowerIntoRustBuffer[[]ValidationError](c, value))
}

func (c FfiConverterSequenceValidationError) Write(writer io.Writer, value []ValidationError) {
	if len(value) > math.MaxInt32 {
		panic("[]ValidationError is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	for _, item := range value {
		FfiConverterValidationErrorINSTANCE.Write(writer, item)
	}
}

type FfiDestroyerSequenceValidationError struct{}

func (FfiDestroyerSequenceValidationError) Destroy(sequence []ValidationError) {
	for _, value := range sequence {
		FfiDestroyerValidationError{}.Destroy(value)
	}
}

type FfiConverterSequenceVulnerabilityId struct{}

var FfiConverterSequenceVulnerabilityIdINSTANCE = FfiConverterSequenceVulnerabilityId{}

func (c FfiConverterSequenceVulnerabilityId) Lift(rb RustBufferI) []VulnerabilityId {
	return LiftFromRustBuffer[[]VulnerabilityId](c, rb)
}

func (c FfiConverterSequenceVulnerabilityId) Read(reader io.Reader) []VulnerabilityId {
	length := readInt32(reader)
	if length == 0 {
		return nil
	}
	result := make([]VulnerabilityId, 0, length)
	for i := int32(0); i < length; i++ {
		result = append(result, FfiConverterVulnerabilityIdINSTANCE.Read(reader))
	}
	return result
}

func (c FfiConverterSequenceVulnerabilityId) Lower(value []VulnerabilityId) C.RustBuffer {
	return LowerIntoRustBuffer[[]VulnerabilityId](c, value)
}

func (c FfiConverterSequenceVulnerabilityId) LowerExternal(value []VulnerabilityId) ExternalCRustBuffer {
	return RustBufferFromC(LowerIntoRustBuffer[[]VulnerabilityId](c, value))
}

func (c FfiConverterSequenceVulnerabilityId) Write(writer io.Writer, value []VulnerabilityId) {
	if len(value) > math.MaxInt32 {
		panic("[]VulnerabilityId is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	for _, item := range value {
		FfiConverterVulnerabilityIdINSTANCE.Write(writer, item)
	}
}

type FfiDestroyerSequenceVulnerabilityId struct{}

func (FfiDestroyerSequenceVulnerabilityId) Destroy(sequence []VulnerabilityId) {
	for _, value := range sequence {
		FfiDestroyerVulnerabilityId{}.Destroy(value)
	}
}

// Validate a CSAF document from a JSON string.
//
// Auto-detects the CSAF version from the document's `document.csaf_version`
// field and validates it according to the specified preset.
//
// # Arguments
//
// * `json_str` - The CSAF document as a JSON string.
// * `preset`   - The validation preset: `"basic"`, `"extended"`, or `"full"`.
//
// # Returns
//
// An `ValidationResult` containing the validation outcome and any findings.
func ValidateCsaf(jsonStr string, preset string) (ValidationResult, error) {
	_uniffiRV, _uniffiErr := rustCallWithError[*CsafError](FfiConverterCsafError{}, func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return GoRustBuffer{
			inner: C.uniffi_csaf_ffi_fn_func_validate_csaf(FfiConverterStringINSTANCE.Lower(jsonStr), FfiConverterStringINSTANCE.Lower(preset), _uniffiStatus),
		}
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue ValidationResult
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterValidationResultINSTANCE.Lift(_uniffiRV), nil
	}
}

// Validate a CSAF 2.0 document from a JSON string.
//
// # Arguments
//
// * `json_str` - The CSAF 2.0 document as a JSON string.
// * `preset`   - The validation preset: `"basic"`, `"extended"`, or `"full"`.
func ValidateCsaf20(jsonStr string, preset string) (ValidationResult, error) {
	_uniffiRV, _uniffiErr := rustCallWithError[*CsafError](FfiConverterCsafError{}, func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return GoRustBuffer{
			inner: C.uniffi_csaf_ffi_fn_func_validate_csaf_2_0(FfiConverterStringINSTANCE.Lower(jsonStr), FfiConverterStringINSTANCE.Lower(preset), _uniffiStatus),
		}
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue ValidationResult
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterValidationResultINSTANCE.Lift(_uniffiRV), nil
	}
}

// Validate a CSAF 2.1 document from a JSON string.
//
// # Arguments
//
// * `json_str` - The CSAF 2.1 document as a JSON string.
// * `preset`   - The validation preset: `"basic"`, `"extended"`, or `"full"`.
func ValidateCsaf21(jsonStr string, preset string) (ValidationResult, error) {
	_uniffiRV, _uniffiErr := rustCallWithError[*CsafError](FfiConverterCsafError{}, func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return GoRustBuffer{
			inner: C.uniffi_csaf_ffi_fn_func_validate_csaf_2_1(FfiConverterStringINSTANCE.Lower(jsonStr), FfiConverterStringINSTANCE.Lower(preset), _uniffiStatus),
		}
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue ValidationResult
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterValidationResultINSTANCE.Lift(_uniffiRV), nil
	}
}
