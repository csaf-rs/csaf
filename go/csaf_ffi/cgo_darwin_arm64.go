//go:build darwin && arm64

package csaf_ffi

// #cgo LDFLAGS: -L${SRCDIR}/lib/darwin_arm64 -lcsaf_ffi -framework Security -framework CoreFoundation
import "C"
