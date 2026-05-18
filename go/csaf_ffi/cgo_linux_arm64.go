//go:build linux && arm64

package csaf_ffi

// #cgo LDFLAGS: -L${SRCDIR}/lib/linux_arm64 -lcsaf_ffi -lm
import "C"
