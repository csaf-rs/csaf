//go:build darwin && amd64

package csaf_ffi

// #cgo LDFLAGS: -L${SRCDIR}/lib/darwin_amd64 -lcsaf_ffi -framework Security -framework CoreFoundation
import "C"
