//! Equivalence of `libavutil/pixelutils.h`'s safe API with the C library:
//! [`SadFn::new`] against `av_pixelutils_get_sad_fn`, by what each selection
//! is: a function or NULL, and NULL for every size in the configured build,
//! which compiles pixelutils out.
#![expect(
    clippy::cast_sign_loss,
    reason = "test inputs are chosen within range, and the casts mirror the C conversions under comparison"
)]

use core::ffi::c_int;

use ffmpeg_bindings::libavutil::libavutil::ffi;
use ffmpeg_bindings::libavutil::libavutil::pixelutils::{SadAlignment, SadFn};

const ALIGNMENTS: [(SadAlignment, c_int); 3] = [
    (SadAlignment::Unaligned, 0),
    (SadAlignment::First, 1),
    (SadAlignment::Both, 2),
];

/// `av_pixelutils_get_sad_fn(bits, bits, aligned, NULL)`.
fn c_sad_fn(
    bits: c_int,
    aligned: c_int,
) -> Option<unsafe extern "C" fn(*const u8, isize, *const u8, isize) -> c_int> {
    // SAFETY: integers and a NULL log context, which C only reads without
    // pixelutils.
    unsafe { ffi::av_pixelutils_get_sad_fn(bits, bits, aligned, core::ptr::null_mut()) }
}

#[test]
#[cfg_attr(miri, ignore = "calls C")]
fn selection_matches_c_for_every_size() {
    for (a, c) in ALIGNMENTS {
        for bits in 0..=8 {
            let rust = SadFn::new(bits as u32, a);
            assert_eq!(rust.is_some(), c_sad_fn(bits, c).is_some(), "{bits} {a:?}");
            if let Some(f) = rust {
                assert_eq!((f.size(), f.alignment()), (1 << bits, a));
            }
        }
        for bits in [c_int::MAX as u32, c_int::MAX as u32 + 1, u32::MAX] {
            assert!(SadFn::new(bits, a).is_none());
        }
        assert!(c_sad_fn(c_int::MAX, c).is_none());
        // C rejects a non-square size, which `SadFn::new` cannot ask for.
        // SAFETY: as in `c_sad_fn`.
        let rect = unsafe { ffi::av_pixelutils_get_sad_fn(3, 4, c, core::ptr::null_mut()) };
        assert!(rect.is_none());
    }
}

/// The configured build compiles pixelutils out (`CONFIG_PIXELUTILS` 0):
/// C selects no function for any size, and neither does `SadFn::new`.
#[test]
#[cfg_attr(miri, ignore = "calls C")]
fn without_pixelutils_neither_selects_a_function() {
    for (a, c) in ALIGNMENTS {
        for bits in 1..=5 {
            assert!(c_sad_fn(bits, c).is_none(), "{bits} {a:?}");
            assert!(SadFn::new(bits as u32, a).is_none(), "{bits} {a:?}");
        }
    }
}
