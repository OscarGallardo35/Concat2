<p align="center">
  <img src="https://cdn.jsdelivr.net/gh/OscarGallardo35/VibeCut@main/assets/vibecut-mark-dark.png" alt="VibeCut" width="100" />
</p>

<h1 align="center">VibeCut</h1>
<h3 align="center">Der kostenlose, quelloffene CapCut-Ersatz für alle Plattformen.</h3>

<div align="center">

<img src="https://cdn.jsdelivr.net/gh/OscarGallardo35/VibeCut@main/assets/hero.png" alt="VibeCut" width="100%" />

<p align="center">
  <a href="https://github.com/OscarGallardo35/VibeCut/releases"><img src="https://img.shields.io/github/downloads/OscarGallardo35/VibeCut/total?style=flat&logo=github&logoColor=F8F8F8&label=Downloads&labelColor=212123&color=0568FD" alt="Total Downloads" /></a>
  <a href="https://github.com/OscarGallardo35/VibeCut/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/OscarGallardo35/VibeCut/ci.yml?style=flat&logo=githubactions&logoColor=F8F8F8&label=Build&labelColor=000000" alt="Build Status" /></a>
  <a href="https://github.com/OscarGallardo35/VibeCut/releases"><img src="https://img.shields.io/badge/Version-0.2.8-0568FD?style=flat&logo=semver&logoColor=F8F8F8&labelColor=212123" alt="VibeCut Version 0.2.8" /></a>
  <a href="../LICENSE"><img src="https://img.shields.io/badge/License-AGPL%20v3-0568FD?style=flat&logo=gnu&logoColor=F8F8F8&labelColor=212123" alt="License: AGPL-3.0-or-later" /></a>
</p>

<p align="center">
  <a href="https://github.com/OscarGallardo35/VibeCut/#download"><img src="../assets/download_button.svg" alt="Download VibeCut" width="251" /></a>
</p>

[🇬🇧 English](../README.md) · **🇩🇪 Deutsch** · [🇪🇸 Español](README.es.md) · [🇫🇷 Français](README.fr.md) · [🇮🇹 Italiano](README.it.md) · [🇳🇱 Nederlands](README.nl.md) · [🇵🇱 Polski](README.pl.md) · [🇧🇷 Português (Brasil)](README.pt-BR.md) · [🇷🇺 Русский](README.ru.md) · [🇺🇦 Українська](README.uk.md) · [🇹🇷 Türkçe](README.tr.md) · [🇦🇿 Azərbaycanca](README.az.md) · [🇭🇷 Hrvatski](README.hr.md) · [🇮🇩 Bahasa Indonesia](README.id.md) · [🇻🇳 Tiếng Việt](README.vi.md) · [🇹🇭 ไทย](README.th.md) · [🇯🇵 日本語](README.ja.md) · [🇰🇷 한국어](README.ko.md) · [🇨🇳 简体中文](README.zh-Hans.md) · [🇹🇼 繁體中文](README.zh-TW.md) · [🇸🇦 العربية](README.ar.md) · [🇮🇱 עברית](README.he.md) · [🇮🇷 فارسی](README.fa.md) · [🇮🇳 हिन्दी](README.hi.md) · [🇵🇰 اردو](README.ur.md) · [🇧🇩 বাংলা](README.bn.md)

> Übersetzt aus der englischen README vom 11. Oktober 2026. Die englische Fassung ist die, die aktuell gehalten wird.

</div>


## Über VibeCut

VibeCut ist ein kostenloser, quelloffener Videoeditor und eine CapCut-Alternative für macOS, Windows, Linux und Android. Er kann das, wofür man CapCut tatsächlich öffnet: automatische Untertitel, Text-zu-Sprache, Hintergrundentfernung, Keyframe-Animation, Effekte und Titel, Schnitt auf mehreren Spuren, 4K-Export. Ohne die Haken: kein Wasserzeichen, kein Konto, kein Abo, kein Upload.

Alles läuft lokal auf einer nativen Rust-Engine mit GPU-Compositor. Installieren, Material hineinziehen, schneiden. Die KI-Modelle für Untertitel, Stimmen und Freisteller werden einmal in den Einstellungen geladen und arbeiten danach offline. Dein Material verlässt deine Festplatte nie.

**Gut für:** TikTok, Reels und Shorts, YouTube-Videos, Tutorials und Bildschirmaufnahmen, Podcast-Clips, Memes.

**Auch für Maschinen:** eine JSON-RPC-API, damit auch Skripte und KI-Agenten damit Video schneiden können.

## Highlights

- 🚫 **Keine Wasserzeichen. Kein Konto. Keine Bezahlschranke.** Niemals.
- 🔒 **100 % lokal.** Nichts wird hochgeladen. Funktioniert offline.
- 💬 **Automatische Untertitel.** Lokales Whisper. Modellgröße wählen, gestylte Untertitel auf der Zeitleiste bekommen.
- 🗣️ **Text-zu-Sprache + Stimmklonen.** Kostenlose lokale Stimmen, oder jede Stimme aus ein paar Sekunden Aufnahme.
- 🧍 **Hintergrundentfernung.** Personen, Objekte, oder die Maske selbst malen.
- 🎞️ **Keyframes.** Position, Skalierung, Drehung, Deckkraft, Lautstärke, Effektparameter. Kurveneditor eingebaut.
- ✨ **100 Effekte, Filter und Übergänge.** GPU-gerendert, live in der Vorschau.
- ✂️ **Schnell schneiden.** Teilen, trimmen, Ripple, zusammenführen, Standbild, Geschwindigkeit. Magnetische Zeitleiste, wenn du willst.
- 🎚️ **Mehrere Spuren, mehrere Zeitleisten.** Mehrere Schnitte in einem Projekt. Mischmodi, Zuschnitt, Spiegelungen.
- 📝 **Titel.** Schriften, Kontur, Schatten, Hintergrundplatte. Vorlagen zum Loslegen.
- 🎙️ **Stimmbereinigung mit einem Schalter.** Entrauschen, Stimme verbessern, Lautheit angleichen. Dazu Chipmunk, Roboter, Telefon und Freunde.
- 📤 **Export.** H.264, HEVC, AV1. Bis 4K 60, 10-Bit-Farbe.
- 🦀 **Native Rust-Engine.** GPU-Compositor, Proxys, Hardware-Decoding. 4K scrubbt flüssig.
- 🤖 **Skriptbar.** Eine JSON-RPC-API, mit der Skripte und KI-Agenten Video schneiden können. Ein Build aus dem Quellcode bringt zusätzlich gRPC und ein Kommandozeilenwerkzeug mit.
- 🖥️ **macOS, Windows, Linux, Android.** 26 Sprachen. Dieselbe App, dieselben Projektdateien.

## Download

Zwei Wege hinein:

1. **[Die Website](https://github.com/OscarGallardo35/VibeCut/#download)** gibt dir den passenden Build für deinen Rechner. Fang hier an.
2. **[GitHub Releases](https://github.com/OscarGallardo35/VibeCut/releases)** hat jeden Build für jede Plattform, mit Installern, Paketen und Prüfsummen. Für den Fall, dass du selbst wählen willst.

VibeCut ist in der **Beta**: Es funktioniert, hat aber noch Kanten. [Sag Bescheid](https://github.com/OscarGallardo35/VibeCut/issues), wenn du eine findest.

**Plattformen**

- ✅ **Windows** · x86_64 (ARM-Builds sind pausiert, siehe [#326](https://github.com/OscarGallardo35/VibeCut/issues/326)). Ein Setup und eine `.msi`. Wenn SmartScreen einen unsignierten Build stoppt: **Weitere Informationen** › **Trotzdem ausführen**
- ✅ **macOS** · Intel und Apple Silicon. Wenn macOS einen unsignierten Build nicht öffnen will: `xattr -dr com.apple.quarantine /Applications/VibeCut.app`
- ✅ **Linux** · x86_64 und ARM. `.deb`, `.rpm`, `.AppImage` und ein Arch-Paket. Außerdem ein Flatpak auf **[Flatpark](https://flatpark.org/apps/pro.mercadodigital.vibecut/)**, einem Community-Flatpak-Remote, das die x86_64-`.deb` jeder Version verpackt und mit ihr aktualisiert
- ✅ **Android** · Telefone und Tablets
- ✅ **iOS / iPadOS** · iPhone und iPad, per Sideload

✅ Unterstützt · 🚧 In Arbeit · 🧪 Noch zu testen

**Systemanforderungen** und die Größen der optionalen Modelle stehen auf [der Website](https://github.com/OscarGallardo35/VibeCut/tree/main/docs/system-requirements).

**[Code signing policy](../CODE_SIGNING.md)**

## Mitmachen

> [!IMPORTANT]
> Am besten hilfst du, indem du dir einen Build von der [Releases](https://github.com/OscarGallardo35/VibeCut/releases)-Seite holst und ihn benutzt: finde, wo er bricht, und sag, wo er besser sein könnte.
>
> Bereit, Code zu schreiben? [CONTRIBUTING.md](../CONTRIBUTING.md) beschreibt die Einrichtung, den Aufbau des Baums, die auszuführenden Prüfungen und wie Beiträge lizenziert werden. Du steuerst VibeCut aus einem Skript, einem Dienst oder einem Agenten? [Die Entwicklerdokumentation](https://github.com/OscarGallardo35/VibeCut/tree/main/docs) beschreibt die VibeCut-API und ihre Transporte: JSON-RPC und gRPC. [In dieser Discussion](https://github.com/OscarGallardo35/VibeCut/discussions/3) wurde das Projekt angekündigt.

## Mitwirkende

<a href="https://github.com/OscarGallardo35/VibeCut/graphs/contributors">
  <img alt="Contributors" src="https://contrib.rocks/image?repo=OscarGallardo35/VibeCut">
</a>
