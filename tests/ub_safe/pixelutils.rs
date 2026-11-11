//! UB tests for `libavutil/pixelutils.h`'s [`SadFn::new`], through the safe
//! API only: every block size and alignment C is asked for, and selection from
//! several threads at once. The configured build compiles pixelutils out, so C
//! returns no function to call; the unit tests call `SadFn::call` on C's plain
//! `sad_wxh`, in Rust, under Miri.

use std::thread;

use ffmpeg_bindings::libavutil::libavutil::pixelutils::{SadAlignment, SadFn};

const ALIGNMENTS: [SadAlignment; 3] = [
    SadAlignment::Unaligned,
    SadAlignment::First,
    SadAlignment::Both,
];

/// Block sizes C has functions for, those around them, and the extremes.
fn sizes() -> impl Iterator<Item = u32> {
    (0..=9).chain([
        31,
        32,
        63,
        64,
        255,
        256,
        i32::MAX as u32,
        i32::MAX as u32 + 1,
        u32::MAX,
    ])
}

/// The configured build compiles pixelutils out (`CONFIG_PIXELUTILS` 0), so C
/// refuses every size, with a NULL log context it does not write through.
#[test]
#[cfg_attr(miri, ignore = "calls C")]
fn every_size_is_refused_without_pixelutils() {
    for a in ALIGNMENTS {
        for bits in sizes() {
            assert!(SadFn::new(bits, a).is_none(), "{bits} {a:?}");
        }
    }
}

#[test]
#[cfg_attr(miri, ignore = "calls C")]
fn selection_from_threads_at_once() {
    thread::scope(|s| {
        for t in 0..8 {
            s.spawn(move || {
                let a = ALIGNMENTS[t % 3];
                assert!(sizes().all(|b| SadFn::new(b, a).is_none()));
            });
        }
    });
}
