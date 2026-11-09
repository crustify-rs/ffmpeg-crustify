# libavutil/mem.c is foundational only without `av_memcpy_backptr`

Filed by planner run 10.

## What the earlier plan decided

Runs 3 to 8 made `libavutil/mem.c` (module 0) foundational whole, the TU and `mem.h` listed with
empty `items`:

```json
{"index": 0, "name": "libavutil/mem.c", "foundational": true,
 "translation_unit": {"path": "libavutil/mem.c", "items": {"included": [], "excluded": []}},
 "api_headers": [{"path": "libavutil/mem.h", "items": {"included": [], "excluded": []}}]}
```

## Evidence against it

A foundational module depends only on other foundational modules, but for a few dependencies on
its failure paths such as logging (decomposition spec, Foundational modules). `av_memcpy_backptr`
(`libavutil/mem.c:445`) and the helpers it calls, `fill16`, `fill24` and `fill32`
(`mem.c:359`, `377`, `417`), read and write through `AV_RN16`, `AV_RN32`, `AV_WN32` and `AV_WN64`,
whose expansions access the unaligned-access unions `unaligned_16`, `unaligned_32` and
`unaligned_64` of `libavutil/intreadwrite.h`, a header module of its own (module 34). Those are
dependencies of their main path, not of a failure path, on a module that is not foundational:

```console
$ cd c && wavefront . --config ../crustify/wavefront/configs/wavefront-config.json \
    query dag --name av_memcpy_backptr --depth 1
"types": [{"id": "unaligned_16", "kind": "struct", "defined_in": "libavutil/intreadwrite.h", "depth": 1},
          {"id": "unaligned_32", "kind": "struct", "defined_in": "libavutil/intreadwrite.h", "depth": 1}],
"functions": [memcpy, memset (glibc), fill16, fill24, fill32 (libavutil/mem.c)], ...
```

In the whole-closure body-deep schedule of this run (`crustify/tmp/planner10/run.sh`, output
`work/records.schedule.json`), these four are the only units of `mem.c` that depend on a module
other than `mem.c` and the logging of `fast_malloc`'s failure path (`av_log`, `libavutil/log.c`).
`av_memcpy_backptr` is no allocator, releaser, disposer or cloner: it copies an LZ77-style back
reference within one buffer. The API-graph records of the earlier runs had no body edges, which
is why it went unseen.

## The edit

`mem.c` is split as the spec splits a TU that also holds items depending on other modules: the
foundational module 0 lists `mem.c` with every other unit `included` and `mem.h` with
`av_memcpy_backptr` `excluded`; a new module 101, `libavutil/mem.c` (not foundational), lists
the same four as `excluded` and `mem.h` with `av_memcpy_backptr` `included`. Both home in
`src/libavutil/libavutil/mem.rs`. Module 0 keeps its `wrap` objective and its done wave 0, whose
`av_memcpy_backptr` wrap stays landed; module 101 is `port`, and its waves 260 and 285 port
`fill16`, `fill24`, `fill32` and then `av_memcpy_backptr`. Its record depends on
`libavutil/intreadwrite.h` (7 edges); module 0's no longer does.

## Questions

None.
