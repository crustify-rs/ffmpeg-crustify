
Fill in as many answers as you want before starting Crustify. It will ask only
for campaign decisions that remain unresolved.

# Mandatory questions

## Campaign

1. **Which repository and revision should this campaign use?** (`id: source`)
   - Answer: `https://github.com/FFmpeg/FFmpeg`, `1019f8f036`: the `c/` submodule. The
     revision the campaign has used from the start, first as the checkout at `/target`

2. **Should this campaign port the C implementation to Rust, or create safe
   Rust wrappers?** (`id: campaign`)
   - Answer: `port`

3. **What should this campaign target: a named subset of modules, or a named
   subset of functions and types, or the whole target repo? You can define them
   now or we can brainstorm them during the live session. You can also let the
   orchestrator decide.** (`id: scope`)
   - Answer: `libavutil`, whole: every translation unit of libavutil the configured build
     compiles, deprecated items excluded. A pilot: libswresample and libswscale, then the
     configured libavcodec and libavformat, follow.

4. **Which model and billing should do the translation work?**
   (`id: translate-agent`)
   - Answer: `openrouter/anthropic/claude-opus-5.5, api`
   - Format: `<provider/model, api | subscription>`, e.g.
     `openrouter/anthropic/claude-opus-5, api`. The provider is the first
     segment and selects both the billing rate table and the backend CLI.
     Supported providers:
     - `anthropic/<model>` — Anthropic direct, driven by Claude Code;
     - `openai/<model>` — OpenAI direct, driven by Codex;
     - `openrouter/<vendor>/<model>` — OpenRouter, which resells every vendor
       behind one key, so the vendor segment picks the CLI: `anthropic/*` is
       driven by Claude Code and everything else by Codex. OpenRouter
       supports `api` billing only.

5. **Do you want agentic review after translated work lands? If so, which
   model and billing should perform each review?** (`id: review-agent`)
   - Answer: `openrouter/anthropic/claude-opus-5.5, api`, after each wave

6. **Should I run fully autonomously end to end?** (`id: autonomy`)
   - Answer: `no`

# Optional questions

Unanswered optional questions use their defaults.

## Campaign execution

7. **Which model and billing should plan the campaign, and how many planner runs
   should it make? Each run emits what the plan lacks and corrects what the runs
   before it committed.** (`id: plan-agent`)
   - Answer: the translation model and billing, 2 runs. The subsystems already
     planned were planned by the orchestrator, before planners existed.
   - By default: the translation model and billing, 2 runs.

8. **Should the campaign use the default batching and parallelism settings, or
   customize them?** (`id: workload`)
   - Answer: `max-types: 2, max-syms: 25, max-loc: 500`, at least 20 fields per type
     batch; parallelism: assistant's choice.

9. **What batch caps should review agents use? We recommend the same caps as
   translation by default.** (`id: review-workload`)
   - Answer: `as many as the reviewed wave requires`

10. **Which objectives should review agents review for? By default, every one the
    reviewer offers.** (`id: review-objectives`)
    - Answer: `all`
    - Without `fix`, reviewers report and reproduce what they find but patch nothing.

11. **Which sanitizers should the campaign build and test under? By default, every one
    the toolchain supports.** (`id: sanitizers`)
    - Answer: `asan-ubsan, tsan, bsan`, the variants `crustify/artifacts/build.json`
      records; MSan is not built.
    - Miri needs no build and is always on.

12. **How should the target be configured: which of its build features should be
    enabled or disabled?** (`id: target-config`)
    - Answer: a media-ingest production profile, on top of the variants' shared
      libraries, autodetection off, pthreads and zlib on, docs off and the FATE samples
      in `crustify/fate-suite`: `--disable-everything --disable-avdevice`,
      `--enable-decoder=h264,hevc,vp9,aac,mp3float,opus,mjpeg,png,subrip,webvtt`,
      `--enable-encoder=aac,mjpeg,png,webvtt,subrip,rawvideo,pcm_s16le,wrapped_avframe`,
      `--enable-parser=h264,hevc,vp9,aac,mpegaudio,opus,mjpeg,png`,
      `--enable-bsf=h264_mp4toannexb,hevc_mp4toannexb,aac_adtstoasc,extract_extradata,vp9_superframe`,
      `--enable-demuxer=mov,matroska,mp3,mpegts,aac,h264,hevc,ivf,image2,image_png_pipe,image_jpeg_pipe,srt,webvtt`,
      `--enable-muxer=mp4,mov,matroska,webm,mpegts,hls,dash,segment,image2,adts,webvtt,srt,framecrc,framemd5,md5,crc,null`,
      `--enable-protocol=file,pipe`, `--enable-filter=format,aformat,scale,aresample,null,anull`.
      The `ffmpeg` and `ffprobe` programs stay built, as FATE's driver; x86 assembly
      stays enabled.

13. **At which points should the hybrid C-Rust program ship? Shipping advances `main` to
    what the campaign has integrated, with your approval each time: the modules shipped
    are translated, and the C still left is reached only through the wrappers on its
    boundary.** (`id: ship`)
    - Answer: `decide after planning`
    - By default: after every integrated module.

## Commits

14. **Which git identity should the campaign's commits carry: an author, and any
    co-authors? By default, Crustify's own, `crustify <crustify@berkeley.edu>` (the
    GitHub account `crustify-ai`), as the author, with no co-author.** (`id: identity`)
    - Answer: `author: Marius Momeu <marius.momeu@berkeley.edu>, co-authors: crustify
      <crustify@berkeley.edu>` (the GitHub account `crustify-ai`)

## Autonomy (if question 6 is answered `no`)

Answer these approval-gate questions only if question 6 is answered `no`.

15. **Should I wait for your approval before starting the setup phase?** (`id: gates.setup`)
    - Answer: `no`
16. **Should I wait for your approval before starting the translation phase?**
    (`id: gates.translation`)
    - Answer: `yes`
17. **Should I wait for your approval before integrating each module into the working
    branch?** (`id: gates.module`)
    - Answer: `no`
18. **Should I wait for your approval before starting review passes?** (`id: gates.review`)
    - Answer: `no`

# Benchmark recording questions

19. **Where and in what format should the dashboard be recorded?** (`id: dashboard`)
    - Answer: `<repo-checkout>/crustify/artifacts/dashboard.md`, `standard` template

# Additional instructions

Unless otherwise stated, preserve the format of the dashboard exactly.

