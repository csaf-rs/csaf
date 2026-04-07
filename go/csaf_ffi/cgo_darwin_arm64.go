//go:build darwin && arm64

package csaf_ffi

// #cgo LDFLAGS: -lcsaf_ffi -framework Security -framework CoreFoundation
import "C"
