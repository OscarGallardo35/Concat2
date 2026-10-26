<p align="center">
  <img src="https://cdn.jsdelivr.net/gh/OscarGallardo35/VibeCut@main/assets/vibecut-mark-dark.png" alt="VibeCut" width="100" />
</p>

<h1 align="center">VibeCut</h1>
<h3 align="center">免費、開源、跨平台的 CapCut 替代方案。</h3>

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

[🇬🇧 English](../README.md) · [🇩🇪 Deutsch](README.de.md) · [🇪🇸 Español](README.es.md) · [🇫🇷 Français](README.fr.md) · [🇮🇹 Italiano](README.it.md) · [🇳🇱 Nederlands](README.nl.md) · [🇵🇱 Polski](README.pl.md) · [🇧🇷 Português (Brasil)](README.pt-BR.md) · [🇷🇺 Русский](README.ru.md) · [🇺🇦 Українська](README.uk.md) · [🇹🇷 Türkçe](README.tr.md) · [🇦🇿 Azərbaycanca](README.az.md) · [🇭🇷 Hrvatski](README.hr.md) · [🇮🇩 Bahasa Indonesia](README.id.md) · [🇻🇳 Tiếng Việt](README.vi.md) · [🇹🇭 ไทย](README.th.md) · [🇯🇵 日本語](README.ja.md) · [🇰🇷 한국어](README.ko.md) · [🇨🇳 简体中文](README.zh-Hans.md) · **🇹🇼 繁體中文** · [🇸🇦 العربية](README.ar.md) · [🇮🇱 עברית](README.he.md) · [🇮🇷 فارسی](README.fa.md) · [🇮🇳 हिन्दी](README.hi.md) · [🇵🇰 اردو](README.ur.md) · [🇧🇩 বাংলা](README.bn.md)

> 譯自 2026 年 10 月 11 日的英文 README。以英文版為準，它才是持續更新的版本。

</div>


## 關於

VibeCut 是一款免費、開源的影片編輯器，是 macOS、Windows、Linux 與 Android 上的 CapCut 替代方案。它涵蓋了人們真正打開 CapCut 要做的事：自動字幕、文字轉語音、背景移除、關鍵影格動畫、特效與標題、多軌剪輯、4K 輸出。而且沒有任何陷阱：無浮水印、不需帳號、不需訂閱、不上傳。

一切都在本機執行，基於帶 GPU 合成器的原生 Rust 引擎。安裝、拖入素材、開剪。字幕、語音與去背所用的 AI 模型只需在設定中下載一次，之後離線可用。您的素材永遠不會離開您的硬碟。

**適合：** TikTok、Reels 與 Shorts，YouTube 影片，教學與螢幕錄影，Podcast 片段，迷因。

**也適合機器：** 提供 JSON-RPC API，指令碼與 AI 代理同樣可以用它剪影片。

## 亮點

- 🚫 **無浮水印。無帳號。無付費牆。** 永遠如此。
- 🔒 **100 % 本機。** 不上傳任何內容。離線可用。
- 💬 **自動字幕。** 本機 Whisper。選一個模型大小，帶樣式的字幕就排進時間軸。
- 🗣️ **文字轉語音 + 聲音複製。** 免費的本機聲音，或用幾秒鐘錄音複製任何聲音。
- 🧍 **背景移除。** 人物、物體，或者自己塗抹遮罩。
- 🎞️ **關鍵影格。** 位置、縮放、旋轉、不透明度、音量、特效參數。內建曲線編輯器。
- ✨ **100 種特效、濾鏡與轉場。** GPU 算繪，預覽中即時呈現。
- ✂️ **快剪。** 分割、修剪、漣漪、合併、定格、變速。想要的話還有磁性時間軸。
- 🎚️ **多軌、多時間軸。** 一個專案裡放多個剪輯版本。混合模式、裁切、翻轉。
- 📝 **標題。** 字型、描邊、陰影、背景底板。有預設可作起點。
- 🎙️ **一鍵人聲淨化。** 降噪、增強人聲、響度均衡。還有花栗鼠、機器人、電話等效果。
- 📤 **輸出。** H.264、HEVC、AV1。最高 4K 60，10 位元色彩。
- 🦀 **原生 Rust 引擎。** GPU 合成器、代理、硬體解碼。4K 拖曳也順暢。
- 🤖 **可指令碼化。** 提供 JSON-RPC API，指令碼與 AI 代理可以用它剪影片。從原始碼建置還會加上 gRPC 與命令列工具。
- 🖥️ **macOS、Windows、Linux、Android。** 26 種語言。同一個應用程式，同樣的專案檔。

## 下載

兩個入口：

1. **[官網](https://github.com/OscarGallardo35/VibeCut/#download)** 會給您適合您機器的版本。從這裡開始。
2. **[GitHub Releases](https://github.com/OscarGallardo35/VibeCut/releases)** 有每個平台的每個版本，附帶安裝程式、套件與校驗和。想自己挑選時用它。

VibeCut 處於 **測試版**：能用，但還有毛邊。遇到了請 [告訴我們](https://github.com/OscarGallardo35/VibeCut/issues)。

**平台**

- ✅ **Windows** · x86_64（ARM 版本已暫停，見 [#326](https://github.com/OscarGallardo35/VibeCut/issues/326)）。安裝程式與 `.msi`。如果 SmartScreen 攔截未簽署版本：**其他資訊** › **仍要執行**
- ✅ **macOS** · Intel 與 Apple 晶片。如果 macOS 拒絕開啟未簽署版本：`xattr -dr com.apple.quarantine /Applications/VibeCut.app`
- ✅ **Linux** · x86_64 與 ARM。`.deb`、`.rpm`、`.AppImage` 與 Arch 套件。另有 **[Flatpark](https://flatpark.org/apps/pro.mercadodigital.vibecut/)** 上的 Flatpak，這是一個社群 Flatpak 儲存庫，封裝每個版本的 x86_64 `.deb` 並隨之更新
- ✅ **Android** · 手機與平板
- ✅ **iOS / iPadOS** · iPhone 與 iPad，需側載

✅ 已支援 · 🚧 開發中 · 🧪 待測試

**系統需求** 與可選模型的大小見 [官網](https://github.com/OscarGallardo35/VibeCut/tree/main/docs/system-requirements)。

**[Code signing policy](../CODE_SIGNING.md)**

## 如何貢獻

> [!IMPORTANT]
> 最好的貢獻方式是從 [Releases](https://github.com/OscarGallardo35/VibeCut/releases) 頁面拿一個版本來用：找到它哪裡會壞，說出它哪裡可以更好。
>
> 準備寫程式了？[CONTRIBUTING.md](../CONTRIBUTING.md) 介紹了環境建置、目錄結構、要執行的檢查以及貢獻的授權方式。要從指令碼、服務或代理驅動 VibeCut？[開發者文件](https://github.com/OscarGallardo35/VibeCut/tree/main/docs) 介紹了 VibeCut API 及其傳輸方式：JSON-RPC 與 gRPC。專案在 [這個 Discussion](https://github.com/OscarGallardo35/VibeCut/discussions/3) 中發布。

## 貢獻者

<a href="https://github.com/OscarGallardo35/VibeCut/graphs/contributors">
  <img alt="Contributors" src="https://contrib.rocks/image?repo=OscarGallardo35/VibeCut">
</a>
