# Contributing to VibeCut

Thanks for being here. VibeCut is a video editor that runs entirely on the
user's machine, and it gets better mostly through people using it hard and
reporting what broke.

## The most valuable thing you can do

Grab a build from the [Releases](https://github.com/OscarGallardo35/VibeCut/releases) page
and edit a real video with it. Real footage finds what no reading of the
source does. A good bug report (what you did, what happened, your
OS, the media you used) is worth more than most patches.

Longer-form discussion lives in
[the contribution discussion](https://github.com/OscarGallardo35/VibeCut/discussions/3)
and on [Discord](https://discord.gg/DVuPfpXfqP).

## Before you write code

Open an issue or drop into Discord first for anything beyond a small fix. Large
areas are already in progress or intentionally deferred, and it is
no fun to review a big PR that has to be turned down for reasons that were
invisible from outside.

## Setting up

You will need Rust (see `rust-version` in `src/Cargo.toml`), the FFmpeg
7+ development libraries (`brew install ffmpeg`; see `src/README.md` for
Windows and Linux), cmake and a C++ compiler. Then:

```sh
cd src && cargo run -p vibecut
```

That is the editor window. Everything - the engine, the host layer and the
Slint UI - is one Cargo workspace under `src/`.

## Layout

| Path | What lives there |
|---|---|
| `src/crates/` | The engine (core, media, render, effects, export, project), the host layer (`vibecut-host`, `vibecut-speech`), the API (`vibecut-api`, `vibecut-server`), the CLI, and `vibecut`, the Slint editor window |

[`src/README.md`](src/README.md) explains how the crates fit together and
where the sharp edges are. Read it before touching the engine.

## Checks

Run these before opening a PR:

```sh
cd src && cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test
```

If you added or changed text the interface shows, also run
`python3 scripts/locales.py` so the string inventory follows; CI checks it.
Translations live in one JSON file per language; see
[`TRANSLATING.md`](TRANSLATING.md).

If you added or changed a downloadable model, put it in
[`models/manifest.toml`](models/manifest.toml) as well as the engine table it
belongs to, and run `python3 scripts/models.py --check`; CI checks that too.
A maintainer then runs the *Mirror models* workflow, which fetches the model,
records its digest in both places and publishes it to the mirror the app
downloads from.

Write what happens through the `log` facade (`log::info!`, `log::warn!`,
`log::error!`) rather than to standard output or standard error. Every run
writes `<app data>/logs/vibecut-<when>.log`, and a packaged build has no
terminal for anything that goes anywhere else; `VIBECUT_LOG=debug` turns the
level up. See `src/crates/vibecut-host/src/logs.rs`.

New source files need a licence header (see below). Match the style of the code
around you; the engine avoids cleverness on purpose.

## Licensing your contribution

VibeCut is **AGPL-3.0-or-later** ([`LICENSE`](LICENSE)). There is no separate
contributor agreement: by opening a pull request you agree to license your
contribution under the same terms, and you keep the copyright in your own work.

Sign off your commits to indicate that:

```sh
git commit -s -m "your message"
```

Typo and documentation fixes do not need a sign-off.

### File headers

Every source file starts with:

```rust
// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Jareer Abdullah and Concat contributors
```

Use this header as it is in a file that came from Concat, and don't add a
copyright line of your own: the line records who wrote the code that is there,
and it is part of the licence it arrived under. A file you create from scratch
carries the same two lines, with the second naming you or your project; do not
paste the upstream line onto code that is not theirs.

### Third-party code

If your patch brings in code you did not write, say so in the PR: what it is,
where it came from, and its licence. Anything incompatible with
AGPL-3.0-or-later cannot be merged, and licence problems are much cheaper to
catch before a merge than after a release.

## Trademarks

The code is free to fork. The VibeCut name and logo are not covered by the AGPL
grant; see [`TRADEMARK.md`](TRADEMARK.md). Forks are welcome; please ship them
under your own name.

## Conduct

[`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md) applies to the repo, the Discussions
and Discord.
