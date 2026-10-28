<p align="center">
  <img src="https://cdn.jsdelivr.net/gh/OscarGallardo35/VibeCut@main/assets/vibecut-mark-dark.png" alt="VibeCut" width="100" />
</p>

<h1 align="center">VibeCut</h1>
<h3 align="center">Бесплатная кроссплатформенная замена CapCut с открытым исходным кодом.</h3>

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

[🇬🇧 English](../README.md) · [🇩🇪 Deutsch](README.de.md) · [🇪🇸 Español](README.es.md) · [🇫🇷 Français](README.fr.md) · [🇮🇹 Italiano](README.it.md) · [🇳🇱 Nederlands](README.nl.md) · [🇵🇱 Polski](README.pl.md) · [🇧🇷 Português (Brasil)](README.pt-BR.md) · **🇷🇺 Русский** · [🇺🇦 Українська](README.uk.md) · [🇹🇷 Türkçe](README.tr.md) · [🇦🇿 Azərbaycanca](README.az.md) · [🇭🇷 Hrvatski](README.hr.md) · [🇮🇩 Bahasa Indonesia](README.id.md) · [🇻🇳 Tiếng Việt](README.vi.md) · [🇹🇭 ไทย](README.th.md) · [🇯🇵 日本語](README.ja.md) · [🇰🇷 한국어](README.ko.md) · [🇨🇳 简体中文](README.zh-Hans.md) · [🇹🇼 繁體中文](README.zh-TW.md) · [🇸🇦 العربية](README.ar.md) · [🇮🇱 עברית](README.he.md) · [🇮🇷 فارسی](README.fa.md) · [🇮🇳 हिन्दी](README.hi.md) · [🇵🇰 اردو](README.ur.md) · [🇧🇩 বাংলা](README.bn.md)

> Переведено с английского README 11 октября 2026 года. Актуальной поддерживается английская версия.

</div>


## О проекте

VibeCut является бесплатным видеоредактором с открытым исходным кодом и альтернативой CapCut для macOS, Windows, Linux и Android. Он умеет то, ради чего люди на самом деле открывают CapCut: автоматические субтитры, синтез речи, удаление фона, анимацию по ключевым кадрам, эффекты и титры, многодорожечный монтаж, экспорт в 4K. И без подвохов: ни водяного знака, ни аккаунта, ни подписки, ни загрузки на сервер.

Всё работает локально на нативном движке на Rust с GPU-композитором. Установите, перетащите материал, режьте. Модели ИИ для субтитров, голосов и вырезания фона скачиваются один раз из настроек и дальше работают офлайн. Ваш материал никогда не покидает ваш диск. ИИ-чат необязателен: он отправляет только текст выбранному вами провайдеру — никогда видео, изображения или аудио.

**Подходит для:** TikTok, Reels и Shorts, видео на YouTube, обучающих роликов и записей экрана, фрагментов подкастов, мемов.

**И для машин тоже:** API JSON-RPC, чтобы скрипты и ИИ-агенты тоже могли монтировать в нём видео.

## Главное

- 🚫 **Без водяных знаков. Без аккаунта. Без платного доступа.** Никогда.
- 🔒 **Ваш материал не покидает диск.** Необязательный ИИ-чат отправляет только текст.
- 💬 **Автоматические субтитры.** Локальный Whisper. Выберите размер модели, и стилизованные субтитры окажутся на таймлайне.
- 🗣️ **Синтез речи + клонирование голоса.** Бесплатные локальные голоса или любой голос по нескольким секундам записи.
- 🧍 **Удаление фона.** Люди, объекты или нарисуйте маску сами.
- 🎞️ **Ключевые кадры.** Положение, масштаб, поворот, прозрачность, громкость, параметры эффектов. Встроенный редактор кривых.
- ✨ **100 эффектов, фильтров и переходов.** Рендер на GPU, вживую в предпросмотре.
- ✂️ **Быстрый монтаж.** Разрезать, подрезать, ripple, объединить, стоп-кадр, скорость. Магнитный таймлайн по желанию.
- 🎚️ **Много дорожек, много таймлайнов.** Несколько монтажей в одном проекте. Режимы наложения, кадрирование, отражения.
- 📝 **Титры.** Шрифты, обводка, тень, подложка. Пресеты для старта.
- 🎙️ **Чистка голоса одним переключателем.** Шумоподавление, улучшение голоса, выравнивание громкости. Плюс бурундук, робот, телефон и компания.
- 📤 **Экспорт.** H.264, HEVC, AV1. До 4K 60, 10-битный цвет.
- 🦀 **Нативный движок на Rust.** GPU-композитор, прокси, аппаратное декодирование. 4K перематывается плавно.
- 🤖 **Скриптуемый.** API JSON-RPC, через который скрипты и ИИ-агенты могут монтировать видео. Сборка из исходников добавляет gRPC и инструмент командной строки.
- 🖥️ **macOS, Windows, Linux, Android.** 26 языков. Одно приложение, одни и те же файлы проектов.

## Скачать

Два пути:

1. **[Сайт](https://github.com/OscarGallardo35/VibeCut/#download)** подберёт сборку под вашу машину. Начните отсюда.
2. **[GitHub Releases](https://github.com/OscarGallardo35/VibeCut/releases)** содержит каждую сборку для каждой платформы, с установщиками, пакетами и контрольными суммами. Когда хочется выбрать самому.

VibeCut в **бете**: он работает, но шероховатости ещё есть. [Сообщите](https://github.com/OscarGallardo35/VibeCut/issues), если найдёте.

**Платформы**

- ✅ **Windows** · x86_64 (сборки для ARM приостановлены, см. [#326](https://github.com/OscarGallardo35/VibeCut/issues/326)). Установщик и `.msi`. Если SmartScreen останавливает неподписанную сборку: **Подробнее** › **Выполнить в любом случае**
- ✅ **macOS** · Intel и Apple Silicon. Если macOS отказывается открыть неподписанную сборку: `xattr -dr com.apple.quarantine /Applications/VibeCut.app`
- ✅ **Linux** · x86_64 и ARM. `.deb`, `.rpm`, `.AppImage` и пакет для Arch. А ещё Flatpak на **[Flatpark](https://flatpark.org/apps/pro.mercadodigital.vibecut/)**, поддерживаемом сообществом Flatpak-репозитории, который оборачивает `.deb` x86_64 каждого релиза и обновляется вместе с ним
- ✅ **Android** · телефоны и планшеты
- ✅ **iOS / iPadOS** · iPhone и iPad, через sideload

✅ Поддерживается · 🚧 В работе · 🧪 Предстоит проверить

**Системные требования** и размеры необязательных моделей указаны на [сайте](https://github.com/OscarGallardo35/VibeCut/tree/main/docs/system-requirements).

**[Code signing policy](../CODE_SIGNING.md)**

## Как помочь

> [!IMPORTANT]
> Лучше всего вы поможете, если возьмёте сборку со страницы [Releases](https://github.com/OscarGallardo35/VibeCut/releases) и будете ею пользоваться: найдёте, где она ломается, и скажете, где она могла бы быть лучше.
>
> Готовы писать код? В [CONTRIBUTING.md](../CONTRIBUTING.md) описаны настройка, устройство дерева, какие проверки запускать и как лицензируются вклады. Управляете VibeCut из скрипта, сервиса или агента? [Документация для разработчиков](https://github.com/OscarGallardo35/VibeCut/tree/main/docs) описывает API VibeCut и его транспорты: JSON-RPC и gRPC. [В этом обсуждении](https://github.com/OscarGallardo35/VibeCut/discussions/3) проект был анонсирован.

## Участники

<a href="https://github.com/OscarGallardo35/VibeCut/graphs/contributors">
  <img alt="Contributors" src="https://contrib.rocks/image?repo=OscarGallardo35/VibeCut">
</a>
