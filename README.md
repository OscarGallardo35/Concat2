<p align="center">
  <img src="https://cdn.jsdelivr.net/gh/OscarGallardo35/VibeCut@main/assets/vibecut-mark-dark.png" alt="VibeCut" width="100" />
</p>

<h1 align="center">VibeCut</h1>
<h3 align="center">The free, open-source, cross-platform CapCut replacement.</h3>

<div align="center">

<img src="https://cdn.jsdelivr.net/gh/OscarGallardo35/VibeCut@main/assets/hero.png" alt="VibeCut" width="100%" />

<p align="center">
  <a href="https://github.com/OscarGallardo35/VibeCut/releases"><img src="https://img.shields.io/github/downloads/OscarGallardo35/VibeCut/total?style=flat&logo=github&logoColor=F8F8F8&label=Downloads&labelColor=212123&color=0568FD" alt="Total Downloads" /></a>
  <a href="https://github.com/OscarGallardo35/VibeCut/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/OscarGallardo35/VibeCut/ci.yml?style=flat&logo=githubactions&logoColor=F8F8F8&label=Build&labelColor=000000" alt="Build Status" /></a>
  <a href="https://github.com/OscarGallardo35/VibeCut/releases"><img src="https://img.shields.io/badge/Version-0.2.8-0568FD?style=flat&logo=semver&logoColor=F8F8F8&labelColor=212123" alt="VibeCut Version 0.2.8" /></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-AGPL%20v3-0568FD?style=flat&logo=gnu&logoColor=F8F8F8&labelColor=212123" alt="License: AGPL-3.0-or-later" /></a>
</p>

<p align="center">
  <a href="https://github.com/OscarGallardo35/VibeCut/#download"><img src="assets/download_button.svg" alt="Download VibeCut" width="251" /></a>
</p>

**🇬🇧 English** · [🇩🇪 Deutsch](docs/README.de.md) · [🇪🇸 Español](docs/README.es.md) · [🇫🇷 Français](docs/README.fr.md) · [🇮🇹 Italiano](docs/README.it.md) · [🇳🇱 Nederlands](docs/README.nl.md) · [🇵🇱 Polski](docs/README.pl.md) · [🇧🇷 Português (Brasil)](docs/README.pt-BR.md) · [🇷🇺 Русский](docs/README.ru.md) · [🇺🇦 Українська](docs/README.uk.md) · [🇹🇷 Türkçe](docs/README.tr.md) · [🇦🇿 Azərbaycanca](docs/README.az.md) · [🇭🇷 Hrvatski](docs/README.hr.md) · [🇮🇩 Bahasa Indonesia](docs/README.id.md) · [🇻🇳 Tiếng Việt](docs/README.vi.md) · [🇹🇭 ไทย](docs/README.th.md) · [🇯🇵 日本語](docs/README.ja.md) · [🇰🇷 한국어](docs/README.ko.md) · [🇨🇳 简体中文](docs/README.zh-Hans.md) · [🇹🇼 繁體中文](docs/README.zh-TW.md) · [🇸🇦 العربية](docs/README.ar.md) · [🇮🇱 עברית](docs/README.he.md) · [🇮🇷 فارسی](docs/README.fa.md) · [🇮🇳 हिन्दी](docs/README.hi.md) · [🇵🇰 اردو](docs/README.ur.md) · [🇧🇩 বাংলা](docs/README.bn.md)

</div>


## About

VibeCut is a free, open-source video editor and a CapCut alternative for macOS, Windows, Linux and Android. It covers what people actually open CapCut for: auto-captions, text-to-speech, background removal, keyframe animation, effects and titles, multi-track cutting, 4K export. With none of the catches: no watermark, no account, no subscription, and your footage is never uploaded.

Everything runs locally on a native Rust engine with a GPU compositor. Install it, drop in footage, cut. The AI models for captions, voices and cutout download once from Settings and work offline after that. Your footage never leaves your disk.

The AI chat is optional: when you use it, it sends your message, the conversation and a summary of your timeline to the provider you configure — never your video, images or audio. With the chat closed, nothing goes out. See [PRIVACY.md](PRIVACY.md).

**Good for:** TikTok, Reels and Shorts, YouTube videos, tutorials and screen recordings, podcast clips, memes.

**Also for machines:** a built-in MCP server for AI agents (`vibecut --mcp`, [set up](docs/MCP.md)) and a JSON-RPC API for scripts, so they can cut video with it too.

## Highlights

- 🚫 **No watermarks. No account. No paywall.** Ever.
- 🔒 **Your footage never leaves your disk.** The optional AI chat sends text only.
- 💬 **Auto-captions.** Local Whisper. Pick a model size, get styled captions on the timeline.
- 🗣️ **Text-to-speech + voice cloning.** Free local voices, or any voice from a few seconds of a recording.
- 🧍 **Background removal.** People, objects, or paint the mask yourself.
- 🎞️ **Keyframes.** Position, scale, rotation, opacity, volume, effect parameters. Curve editor built in.
- ✨ **100 effects, filters and transitions.** GPU-rendered, live in the preview.
- ✂️ **Cut fast.** Split, trim, ripple, merge, freeze frame, speed. Magnetic timeline if you want it.
- 🎚️ **Multi-track, multi-timeline.** Overlays and picture-in-picture: stack video, stills and titles on as many tracks as you like. Several cuts in one project. Blend modes (screen, add, multiply…), crop, flips.
- 📝 **Titles.** Fonts, stroke, shadow, background plate. Presets to start from.
- 🎙️ **One-switch voice cleanup.** Denoise, enhance voice, level the loudness. Plus chipmunk, robot, telephone and friends.
- 📤 **Export.** H.264, HEVC, AV1. Up to 4K 60, 10-bit colour.
- 🦀 **Native Rust engine.** GPU compositor, proxies, hardware decode. 4K scrubs smoothly.
- 🤖 **Scriptable.** An MCP server built in, so Claude and other AI agents can edit with it ([one line to set up](docs/MCP.md)), and a JSON-RPC API for scripts. A build from source adds gRPC and a command-line tool.
- 🖥️ **macOS, Windows, Linux, Android.** 26 languages. Same app, same project files.

## Download

Two ways in:

1. **[The website](https://github.com/OscarGallardo35/VibeCut/#download)** hands you the right build for your machine. Start here.
2. **[GitHub Releases](https://github.com/OscarGallardo35/VibeCut/releases)** has every build for every platform, with installers, packages and checksums. For when you want to pick.

VibeCut is in **beta**: it works, and it still has edges. [Say so](https://github.com/OscarGallardo35/VibeCut/issues) when you find one.

**Platforms**

- ✅ **Windows** · x86_64 (ARM builds are paused, see [#326](https://github.com/OscarGallardo35/VibeCut/issues/326)). A setup and an `.msi`. If SmartScreen stops an unsigned build: **More info** › **Run anyway**
- ✅ **macOS** · Intel and Apple silicon. If macOS refuses to open an unsigned build: `xattr -dr com.apple.quarantine /Applications/VibeCut.app`
- ✅ **Linux** · x86_64 and ARM. `.deb`, `.rpm`, `.AppImage` and an Arch package. Also a Flatpak on **[Flatpark](https://flatpark.org/apps/pro.mercadodigital.vibecut/)**, a community Flatpak remote that wraps the x86_64 `.deb` of each release and updates with it
- ✅ **Android** · phones and tablets
- ✅ **iOS / iPadOS** · iPhone and iPad, sideloaded

✅ Supported · 🚧 Work in progress · 🧪 To be tested

**System requirements** and the optional model sizes are on [the website](https://github.com/OscarGallardo35/VibeCut/tree/main/docs/system-requirements).

**[Code signing policy](CODE_SIGNING.md)**: why the Windows builds are not signed yet, and what VibeCut sends over the network (nothing you did not ask for).

## Build from source

You need Rust 1.93 or newer (see `rust-version` in `src/Cargo.toml`), the FFmpeg 7+ development libraries, cmake and a C++ compiler.

- **macOS:** `brew install ffmpeg`
- **Windows:** from [FFmpeg's Windows downloads](https://ffmpeg.org/download.html#build-windows), take BtbN's `win64-gpl-shared` build (the one CI uses), unpack it and point `FFMPEG_DIR` at it
- **Linux:** use your distro's FFmpeg if it is version 7 or newer, otherwise use a BtbN `shared` build with `FFMPEG_DIR` like on Windows

Then:

```sh
cd src
cargo run -p vibecut          # the editor window
```

Other useful commands:

```sh
cargo build --profile app -p vibecut   # the shipping binary (fat LTO, stripped)
cargo test                            # run the test suite
cargo fmt --check && cargo clippy --all-targets -- -D warnings   # what CI checks
```

If you change text the interface shows, run `python3 scripts/locales.py` from the repo root so the string inventory stays in sync. Full details live in [CONTRIBUTING.md](CONTRIBUTING.md).

## How to Contribute

> [!IMPORTANT]
> The best way to contribute is to grab a build from the [Releases](https://github.com/OscarGallardo35/VibeCut/releases) page and use it: find where it breaks, and say where it could be better.
>
> Ready to write code? [CONTRIBUTING.md](./CONTRIBUTING.md) covers setup, the layout of the tree, the checks to run, and how contributions are licensed. Driving VibeCut from a script, a service or an agent? [The developer docs](https://github.com/OscarGallardo35/VibeCut/tree/main/docs) cover the VibeCut API and its transports: JSON-RPC and gRPC; [docs/MCP.md](docs/MCP.md) connects an AI agent over MCP. [This Discussion](https://github.com/OscarGallardo35/VibeCut/discussions/3) is where the project was announced.

## Contributors

<a href="https://github.com/OscarGallardo35/VibeCut/graphs/contributors">
  <img alt="Contributors" src="https://contrib.rocks/image?repo=OscarGallardo35/VibeCut">
</a>
