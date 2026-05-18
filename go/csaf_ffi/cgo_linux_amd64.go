//go:build linux && amd64

package csaf_ffi

// #cgo LDFLAGS: -L${SRCDIR}/lib/linux_amd64 -lcsaf_ffi -lm
import "C"
