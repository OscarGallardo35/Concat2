<p align="center">
  <img src="https://cdn.jsdelivr.net/gh/OscarGallardo35/VibeCut@main/assets/vibecut-mark-dark.png" alt="VibeCut" width="100" />
</p>

<h1 align="center">VibeCut</h1>
<h3 align="center">免费、开源、跨平台的 CapCut 替代品。</h3>

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

[🇬🇧 English](../README.md) · [🇩🇪 Deutsch](README.de.md) · [🇪🇸 Español](README.es.md) · [🇫🇷 Français](README.fr.md) · [🇮🇹 Italiano](README.it.md) · [🇳🇱 Nederlands](README.nl.md) · [🇵🇱 Polski](README.pl.md) · [🇧🇷 Português (Brasil)](README.pt-BR.md) · [🇷🇺 Русский](README.ru.md) · [🇺🇦 Українська](README.uk.md) · [🇹🇷 Türkçe](README.tr.md) · [🇦🇿 Azərbaycanca](README.az.md) · [🇭🇷 Hrvatski](README.hr.md) · [🇮🇩 Bahasa Indonesia](README.id.md) · [🇻🇳 Tiếng Việt](README.vi.md) · [🇹🇭 ไทย](README.th.md) · [🇯🇵 日本語](README.ja.md) · [🇰🇷 한국어](README.ko.md) · **🇨🇳 简体中文** · [🇹🇼 繁體中文](README.zh-TW.md) · [🇸🇦 العربية](README.ar.md) · [🇮🇱 עברית](README.he.md) · [🇮🇷 فارسی](README.fa.md) · [🇮🇳 हिन्दी](README.hi.md) · [🇵🇰 اردو](README.ur.md) · [🇧🇩 বাংলা](README.bn.md)

> 译自 2026 年 10 月 11 日的英文 README。以英文版为准，它才是持续更新的版本。

</div>


## 关于

VibeCut 是一款免费、开源的视频编辑器，是 macOS、Windows、Linux 和 Android 上的 CapCut 替代品。它覆盖了人们真正打开 CapCut 要做的事：自动字幕、文字转语音、背景移除、关键帧动画、特效与标题、多轨剪辑、4K 导出。而且没有任何套路：无水印、无需账号、无需订阅、不上传。

一切都在本地运行，基于带 GPU 合成器的原生 Rust 引擎。安装、拖入素材、开剪。字幕、语音和抠像所用的 AI 模型只需在设置中下载一次，之后离线可用。您的素材永远不会离开您的硬盘。 AI 聊天是可选的：它只把文本发送给你配置的服务商，绝不会发送你的视频、图片或音频。

**适合：** TikTok、Reels 和 Shorts，YouTube 视频，教程与屏幕录制，播客片段，表情包。

**也适合机器：** 提供 JSON-RPC API，脚本和 AI 代理同样可以用它剪视频。

## 亮点

- 🚫 **无水印。无账号。无付费墙。** 永远如此。
- 🔒 **素材不会离开你的硬盘。** 可选的 AI 聊天只发送文本。
- 💬 **自动字幕。** 本地 Whisper。选一个模型大小，带样式的字幕就排进时间线。
- 🗣️ **文字转语音 + 声音克隆。** 免费的本地声音，或用几秒钟录音克隆任何声音。
- 🧍 **背景移除。** 人物、物体，或者自己涂抹遮罩。
- 🎞️ **关键帧。** 位置、缩放、旋转、不透明度、音量、特效参数。内置曲线编辑器。
- ✨ **100 种特效、滤镜和转场。** GPU 渲染，预览中实时呈现。
- ✂️ **快剪。** 分割、修剪、涟漪、合并、定格、变速。想要的话还有磁性时间线。
- 🎚️ **多轨道、多时间线。** 一个项目里放多个剪辑版本。混合模式、裁剪、翻转。
- 📝 **标题。** 字体、描边、阴影、背景底板。有预设可作起点。
- 🎙️ **一键人声净化。** 降噪、增强人声、响度均衡。还有花栗鼠、机器人、电话等效果。
- 📤 **导出。** H.264、HEVC、AV1。最高 4K 60，10 位色彩。
- 🦀 **原生 Rust 引擎。** GPU 合成器、代理、硬件解码。4K 拖动也顺滑。
- 🤖 **可脚本化。** 提供 JSON-RPC API，脚本和 AI 代理可以用它剪视频。从源码构建还会加上 gRPC 和命令行工具。
- 🖥️ **macOS、Windows、Linux、Android。** 26 种语言。同一个应用，同样的项目文件。

## 下载

两个入口：

1. **[官网](https://github.com/OscarGallardo35/VibeCut/#download)** 会给您适合您机器的版本。从这里开始。
2. **[GitHub Releases](https://github.com/OscarGallardo35/VibeCut/releases)** 有每个平台的每个版本，附带安装程序、软件包和校验和。想自己挑选时用它。

VibeCut 处于 **测试版**：能用，但还有毛边。遇到了请 [告诉我们](https://github.com/OscarGallardo35/VibeCut/issues)。

**平台**

- ✅ **Windows** · x86_64（ARM 版本已暂停，见 [#326](https://github.com/OscarGallardo35/VibeCut/issues/326)）。安装程序和 `.msi`。如果 SmartScreen 拦截未签名版本：**更多信息** › **仍要运行**
- ✅ **macOS** · Intel 和 Apple 芯片。如果 macOS 拒绝打开未签名版本：`xattr -dr com.apple.quarantine /Applications/VibeCut.app`
- ✅ **Linux** · x86_64 和 ARM。`.deb`、`.rpm`、`.AppImage` 和 Arch 软件包。另有 **[Flatpark](https://flatpark.org/apps/pro.mercadodigital.vibecut/)** 上的 Flatpak，这是一个社区 Flatpak 仓库，封装每个版本的 x86_64 `.deb` 并随之更新
- ✅ **Android** · 手机和平板
- ✅ **iOS / iPadOS** · iPhone 和 iPad，需侧载

✅ 已支持 · 🚧 开发中 · 🧪 待测试

**系统要求** 和可选模型的大小见 [官网](https://github.com/OscarGallardo35/VibeCut/tree/main/docs/system-requirements)。

**[Code signing policy](../CODE_SIGNING.md)**

## 如何贡献

> [!IMPORTANT]
> 最好的贡献方式是从 [Releases](https://github.com/OscarGallardo35/VibeCut/releases) 页面拿一个版本来用：找到它哪里会坏，说出它哪里可以更好。
>
> 准备写代码了？[CONTRIBUTING.md](../CONTRIBUTING.md) 介绍了环境搭建、目录结构、要运行的检查以及贡献的许可方式。要从脚本、服务或代理驱动 VibeCut？[开发者文档](https://github.com/OscarGallardo35/VibeCut/tree/main/docs) 介绍了 VibeCut API 及其传输方式：JSON-RPC 和 gRPC。项目在 [这个 Discussion](https://github.com/OscarGallardo35/VibeCut/discussions/3) 里发布。

## 贡献者

<a href="https://github.com/OscarGallardo35/VibeCut/graphs/contributors">
  <img alt="Contributors" src="https://contrib.rocks/image?repo=OscarGallardo35/VibeCut">
</a>
