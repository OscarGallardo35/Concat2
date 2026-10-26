<p align="center">
  <img src="https://cdn.jsdelivr.net/gh/OscarGallardo35/VibeCut@main/assets/vibecut-mark-dark.png" alt="VibeCut" width="100" />
</p>

<h1 align="center">VibeCut</h1>
<h3 align="center">El reemplazo de CapCut gratuito, de código abierto y multiplataforma.</h3>

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

[🇬🇧 English](../README.md) · [🇩🇪 Deutsch](README.de.md) · **🇪🇸 Español** · [🇫🇷 Français](README.fr.md) · [🇮🇹 Italiano](README.it.md) · [🇳🇱 Nederlands](README.nl.md) · [🇵🇱 Polski](README.pl.md) · [🇧🇷 Português (Brasil)](README.pt-BR.md) · [🇷🇺 Русский](README.ru.md) · [🇺🇦 Українська](README.uk.md) · [🇹🇷 Türkçe](README.tr.md) · [🇦🇿 Azərbaycanca](README.az.md) · [🇭🇷 Hrvatski](README.hr.md) · [🇮🇩 Bahasa Indonesia](README.id.md) · [🇻🇳 Tiếng Việt](README.vi.md) · [🇹🇭 ไทย](README.th.md) · [🇯🇵 日本語](README.ja.md) · [🇰🇷 한국어](README.ko.md) · [🇨🇳 简体中文](README.zh-Hans.md) · [🇹🇼 繁體中文](README.zh-TW.md) · [🇸🇦 العربية](README.ar.md) · [🇮🇱 עברית](README.he.md) · [🇮🇷 فارسی](README.fa.md) · [🇮🇳 हिन्दी](README.hi.md) · [🇵🇰 اردو](README.ur.md) · [🇧🇩 বাংলা](README.bn.md)

> Traducido del README en inglés el 11 de octubre de 2026. La versión en inglés es la que se mantiene al día.

</div>


## Acerca de

VibeCut es un editor de vídeo gratuito y de código abierto, una alternativa a CapCut para macOS, Windows, Linux y Android. Cubre aquello para lo que la gente realmente abre CapCut: subtítulos automáticos, texto a voz, eliminación de fondo, animación por fotogramas clave, efectos y títulos, corte multipista, exportación 4K. Sin trampas: sin marca de agua, sin cuenta, sin suscripción, sin subir nada.

Todo corre en local sobre un motor nativo en Rust con un compositor por GPU. Instálalo, suelta el material, corta. Los modelos de IA para subtítulos, voces y recorte se descargan una vez desde Ajustes y después funcionan sin conexión. Tu material nunca sale de tu disco.

**Bueno para:** TikTok, Reels y Shorts, vídeos de YouTube, tutoriales y grabaciones de pantalla, clips de pódcast, memes.

**También para máquinas:** una API JSON-RPC, para que los scripts y los agentes de IA también puedan cortar vídeo con él.

## Lo más destacado

- 🚫 **Sin marcas de agua. Sin cuenta. Sin muro de pago.** Nunca.
- 🔒 **100 % local.** Nada se sube. Funciona sin conexión.
- 💬 **Subtítulos automáticos.** Whisper en local. Elige un tamaño de modelo y obtén subtítulos con estilo en la línea de tiempo.
- 🗣️ **Texto a voz + clonación de voz.** Voces locales gratuitas, o cualquier voz a partir de unos segundos de grabación.
- 🧍 **Eliminación de fondo.** Personas, objetos, o pinta la máscara tú mismo.
- 🎞️ **Fotogramas clave.** Posición, escala, rotación, opacidad, volumen, parámetros de efectos. Editor de curvas integrado.
- ✨ **100 efectos, filtros y transiciones.** Renderizados por GPU, en vivo en la vista previa.
- ✂️ **Corta rápido.** Dividir, recortar, ripple, unir, congelar fotograma, velocidad. Línea de tiempo magnética si la quieres.
- 🎚️ **Multipista, multilínea de tiempo.** Varios montajes en un proyecto. Modos de fusión, recorte, volteos.
- 📝 **Títulos.** Fuentes, trazo, sombra, placa de fondo. Preajustes para empezar.
- 🎙️ **Limpieza de voz con un interruptor.** Reducir ruido, realzar la voz, nivelar el volumen. Más ardilla, robot, teléfono y compañía.
- 📤 **Exportación.** H.264, HEVC, AV1. Hasta 4K 60, color de 10 bits.
- 🦀 **Motor nativo en Rust.** Compositor por GPU, proxies, decodificación por hardware. El 4K se desplaza con fluidez.
- 🤖 **Programable.** Una API JSON-RPC con la que los scripts y los agentes de IA pueden cortar vídeo. Compilando desde el código fuente se añaden gRPC y una herramienta de línea de comandos.
- 🖥️ **macOS, Windows, Linux, Android.** 26 idiomas. La misma app, los mismos archivos de proyecto.

## Descarga

Dos formas de entrar:

1. **[El sitio web](https://github.com/OscarGallardo35/VibeCut/#download)** te da la compilación adecuada para tu máquina. Empieza aquí.
2. **[GitHub Releases](https://github.com/OscarGallardo35/VibeCut/releases)** tiene todas las compilaciones para todas las plataformas, con instaladores, paquetes y sumas de verificación. Para cuando quieras elegir.

VibeCut está en **beta**: funciona, y aún tiene aristas. [Cuéntalo](https://github.com/OscarGallardo35/VibeCut/issues) cuando encuentres una.

**Plataformas**

- ✅ **Windows** · x86_64 (las compilaciones para ARM están en pausa, ver [#326](https://github.com/OscarGallardo35/VibeCut/issues/326)). Un instalador y un `.msi`. Si SmartScreen detiene una compilación sin firmar: **Más información** › **Ejecutar de todas formas**
- ✅ **macOS** · Intel y Apple Silicon. Si macOS se niega a abrir una compilación sin firmar: `xattr -dr com.apple.quarantine /Applications/VibeCut.app`
- ✅ **Linux** · x86_64 y ARM. `.deb`, `.rpm`, `.AppImage` y un paquete para Arch. También un Flatpak en **[Flatpark](https://flatpark.org/apps/pro.mercadodigital.vibecut/)**, un remoto Flatpak comunitario que envuelve el `.deb` x86_64 de cada versión y se actualiza con ella
- ✅ **Android** · teléfonos y tabletas
- ✅ **iOS / iPadOS** · iPhone y iPad, por sideload

✅ Compatible · 🚧 En desarrollo · 🧪 Por probar

Los **requisitos del sistema** y los tamaños de los modelos opcionales están en [el sitio web](https://github.com/OscarGallardo35/VibeCut/tree/main/docs/system-requirements).

**[Code signing policy](../CODE_SIGNING.md)**

## Cómo contribuir

> [!IMPORTANT]
> La mejor forma de contribuir es descargar una compilación de la página de [Releases](https://github.com/OscarGallardo35/VibeCut/releases) y usarla: encuentra dónde falla y di dónde podría ser mejor.
>
> ¿Listo para escribir código? [CONTRIBUTING.md](../CONTRIBUTING.md) cubre la configuración, la disposición del árbol, las comprobaciones que ejecutar y cómo se licencian las contribuciones. ¿Controlas VibeCut desde un script, un servicio o un agente? [La documentación para desarrolladores](https://github.com/OscarGallardo35/VibeCut/tree/main/docs) cubre la API de VibeCut y sus transportes: JSON-RPC y gRPC. [Esta Discussion](https://github.com/OscarGallardo35/VibeCut/discussions/3) es donde se anunció el proyecto.

## Colaboradores

<a href="https://github.com/OscarGallardo35/VibeCut/graphs/contributors">
  <img alt="Contributors" src="https://contrib.rocks/image?repo=OscarGallardo35/VibeCut">
</a>
