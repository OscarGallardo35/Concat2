<p align="center">
  <img src="https://cdn.jsdelivr.net/gh/OscarGallardo35/VibeCut@main/assets/vibecut-mark-dark.png" alt="VibeCut" width="100" />
</p>

<h1 align="center">VibeCut</h1>
<h3 align="center">無料でオープンソース、クロスプラットフォームの CapCut 代替。</h3>

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

[🇬🇧 English](../README.md) · [🇩🇪 Deutsch](README.de.md) · [🇪🇸 Español](README.es.md) · [🇫🇷 Français](README.fr.md) · [🇮🇹 Italiano](README.it.md) · [🇳🇱 Nederlands](README.nl.md) · [🇵🇱 Polski](README.pl.md) · [🇧🇷 Português (Brasil)](README.pt-BR.md) · [🇷🇺 Русский](README.ru.md) · [🇺🇦 Українська](README.uk.md) · [🇹🇷 Türkçe](README.tr.md) · [🇦🇿 Azərbaycanca](README.az.md) · [🇭🇷 Hrvatski](README.hr.md) · [🇮🇩 Bahasa Indonesia](README.id.md) · [🇻🇳 Tiếng Việt](README.vi.md) · [🇹🇭 ไทย](README.th.md) · **🇯🇵 日本語** · [🇰🇷 한국어](README.ko.md) · [🇨🇳 简体中文](README.zh-Hans.md) · [🇹🇼 繁體中文](README.zh-TW.md) · [🇸🇦 العربية](README.ar.md) · [🇮🇱 עברית](README.he.md) · [🇮🇷 فارسی](README.fa.md) · [🇮🇳 हिन्दी](README.hi.md) · [🇵🇰 اردو](README.ur.md) · [🇧🇩 বাংলা](README.bn.md)

> 2026 年 10 月 11 日時点の英語版 README からの翻訳です。最新の内容は英語版が保持します。

</div>


## 概要

VibeCut は macOS・Windows・Linux・Android 向けの、無料でオープンソースの動画エディタであり、CapCut の代替です。人々が実際に CapCut を開く目的をカバーします。自動字幕、テキスト読み上げ、背景除去、キーフレームアニメーション、エフェクトとタイトル、マルチトラック編集、4K 書き出し。しかも落とし穴なし。透かしなし、アカウント不要、サブスクリプション不要、アップロードなし。

すべてが GPU コンポジターを備えたネイティブ Rust エンジン上でローカルに動きます。インストールして、素材を放り込んで、切る。字幕・音声・切り抜き用の AI モデルは設定から一度だけダウンロードし、以降はオフラインで動作します。あなたの素材がディスクの外へ出ることはありません。 AIチャットは任意です。設定したプロバイダーにテキストのみを送信し、動画・画像・音声は決して送信しません。

**向いている用途:** TikTok・Reels・Shorts、YouTube 動画、チュートリアルや画面収録、ポッドキャストの切り抜き、ミーム。

**機械にも:** JSON-RPC の API があるので、スクリプトや AI エージェントも動画を編集できます。

## 特長

- 🚫 **透かしなし。アカウント不要。有料の壁なし。** 永久に。
- 🔒 **素材がディスクから出ることはありません。** 任意のAIチャットはテキストのみ送信します。
- 💬 **自動字幕。** ローカルの Whisper。モデルサイズを選べば、スタイル付きの字幕がタイムラインに並びます。
- 🗣️ **テキスト読み上げ + 音声クローン。** 無料のローカル音声、または数秒の録音から任意の声を。
- 🧍 **背景除去。** 人物、物体、あるいは自分でマスクを描く。
- 🎞️ **キーフレーム。** 位置、拡大率、回転、不透明度、音量、エフェクトのパラメーター。カーブエディター内蔵。
- ✨ **100 種のエフェクト、フィルター、トランジション。** GPU で描画、プレビューにリアルタイム反映。
- ✂️ **素早く切る。** 分割、トリム、リップル、結合、フリーズフレーム、速度。お好みでマグネットタイムライン。
- 🎚️ **マルチトラック、マルチタイムライン。** 1 つのプロジェクトに複数の編集。ブレンドモード、クロップ、反転。
- 📝 **タイトル。** フォント、縁取り、影、背景プレート。出発点になるプリセット。
- 🎙️ **スイッチ 1 つで音声クリーンアップ。** ノイズ除去、声の強調、ラウドネスの均一化。さらにチップマンク、ロボット、電話などの声も。
- 📤 **書き出し。** H.264、HEVC、AV1。最大 4K 60、10 ビットカラー。
- 🦀 **ネイティブ Rust エンジン。** GPU コンポジター、プロキシ、ハードウェアデコード。4K もなめらかにスクラブ。
- 🤖 **スクリプト対応。** スクリプトや AI エージェントが動画を編集できる JSON-RPC の API。ソースからビルドすると gRPC とコマンドラインツールも使えます。
- 🖥️ **macOS、Windows、Linux、Android。** 26 言語。同じアプリ、同じプロジェクトファイル。

## ダウンロード

入口は 2 つ:

1. **[ウェブサイト](https://github.com/OscarGallardo35/VibeCut/#download)** があなたのマシンに合ったビルドを渡します。まずはここから。
2. **[GitHub Releases](https://github.com/OscarGallardo35/VibeCut/releases)** にはすべてのプラットフォーム向けのすべてのビルドが、インストーラー、パッケージ、チェックサムとともにあります。自分で選びたいときに。

VibeCut は **ベータ版** です。動きますが、まだ粗い部分があります。見つけたら [教えてください](https://github.com/OscarGallardo35/VibeCut/issues)。

**プラットフォーム**

- ✅ **Windows** · x86_64 (ARM ビルドは一時停止中、[#326](https://github.com/OscarGallardo35/VibeCut/issues/326) を参照)。セットアップと `.msi`。SmartScreen が未署名ビルドを止めたら: **詳細情報** › **実行**
- ✅ **macOS** · Intel と Apple シリコン。macOS が未署名ビルドを開かないときは: `xattr -dr com.apple.quarantine /Applications/VibeCut.app`
- ✅ **Linux** · x86_64 と ARM。`.deb`、`.rpm`、`.AppImage`、Arch パッケージ。さらに **[Flatpark](https://flatpark.org/apps/pro.mercadodigital.vibecut/)** 上の Flatpak も。各リリースの x86_64 `.deb` を包み、リリースに合わせて更新されるコミュニティ運営の Flatpak リモートです
- ✅ **Android** · スマートフォンとタブレット
- ✅ **iOS / iPadOS** · iPhone と iPad、サイドロード

✅ 対応 · 🚧 作業中 · 🧪 検証待ち

**システム要件** と任意モデルのサイズは [ウェブサイト](https://github.com/OscarGallardo35/VibeCut/tree/main/docs/system-requirements) にあります。

**[Code signing policy](../CODE_SIGNING.md)**

## 貢献するには

> [!IMPORTANT]
> いちばんの貢献は、[Releases](https://github.com/OscarGallardo35/VibeCut/releases) ページからビルドを取って使うことです。どこで壊れるかを見つけ、どこがもっと良くなるかを伝えてください。
>
> コードを書く準備ができたら、[CONTRIBUTING.md](../CONTRIBUTING.md) にセットアップ、ツリーの構成、実行すべきチェック、貢献のライセンスがまとまっています。スクリプトやサービス、エージェントから VibeCut を動かすなら、[開発者向けドキュメント](https://github.com/OscarGallardo35/VibeCut/tree/main/docs) に VibeCut API とそのトランスポート (JSON-RPC、gRPC) があります。プロジェクトは [この Discussion](https://github.com/OscarGallardo35/VibeCut/discussions/3) で発表されました。

## 貢献者

<a href="https://github.com/OscarGallardo35/VibeCut/graphs/contributors">
  <img alt="Contributors" src="https://contrib.rocks/image?repo=OscarGallardo35/VibeCut">
</a>
