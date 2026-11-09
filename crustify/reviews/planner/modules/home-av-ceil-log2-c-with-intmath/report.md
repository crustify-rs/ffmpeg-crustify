# `av_ceil_log2_c` implements intmath.c's logic, not common.h's

Filed by planner run 10.

## What the earlier plan decided

Runs 3 to 8 homed every `static inline` of the shared header `libavutil/common.h` in its header
module (module 17) but for the two functions `libavutil/intmath.c` defines, which module 33 lists:

```json
{"index": 17, "name": "libavutil/common.h",
 "api_headers": [{"path": "libavutil/common.h", "items": {"included": [], "excluded": ["av_log2", "av_log2_16bit"]}}]}
{"index": 33, "name": "libavutil/intmath.c",
 "api_headers": [{"path": "libavutil/common.h", "items": {"included": ["av_log2", "av_log2_16bit"], "excluded": []}}]}
```

## Evidence against it

A shared header's `static inline` function goes to the module whose logic it implements, as the
functions it calls and the types it accesses show (decomposition spec, Bundling).
`av_ceil_log2_c` (`libavutil/common.h:425`) is `return av_log2((x - 1U) << 1);`: it calls
`av_log2`, defined in `libavutil/intmath.c`, and nothing else, and touches no type. In this run's
seeded schedule (`crustify/tmp/planner10/work/plan.schedule.json`) it is the only `static inline`
of `common.h` whose every in-tree dependency is another module's:

```console
$ cd c && wavefront . --config ../crustify/wavefront/configs/wavefront-config.json \
    query symbols --name av_ceil_log2_c --callees
"reached": __builtin_clz (builtin), av_log2 (libavutil/intmath.c),
           av_log2 and ff_log2 (libavutil/intmath.h, the macros av_log2 expands through)
```

Every other function of `common.h` computes on its arguments alone (clipping, saturation, popcount).

## The edit

Module 33 lists `common.h` with `av_ceil_log2_c` added to its `included`, and module 17 adds it
to its `excluded`. The item moves from `common.h`'s waves to `intmath.c`'s wave 231, with
`av_log2_16bit`, after wave 214 ports `av_log2`; its home is `src/libavutil/libavutil/intmath.rs`.
With it, `common.h` depends on no module: its record of `intmath.c`, the only one the body-deep count would give it, goes.

## Questions

None.
