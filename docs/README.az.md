<p align="center">
  <img src="https://cdn.jsdelivr.net/gh/OscarGallardo35/VibeCut@main/assets/vibecut-mark-dark.png" alt="VibeCut" width="100" />
</p>

<h1 align="center">VibeCut</h1>
<h3 align="center">Pulsuz, açıq mənbəli və bütün platformalarda işləyən CapCut alternativi.</h3>

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

[🇬🇧 English](../README.md) · [🇩🇪 Deutsch](README.de.md) · [🇪🇸 Español](README.es.md) · [🇫🇷 Français](README.fr.md) · [🇮🇹 Italiano](README.it.md) · [🇳🇱 Nederlands](README.nl.md) · [🇵🇱 Polski](README.pl.md) · [🇧🇷 Português (Brasil)](README.pt-BR.md) · [🇷🇺 Русский](README.ru.md) · [🇺🇦 Українська](README.uk.md) · [🇹🇷 Türkçe](README.tr.md) · **🇦🇿 Azərbaycanca** · [🇭🇷 Hrvatski](README.hr.md) · [🇮🇩 Bahasa Indonesia](README.id.md) · [🇻🇳 Tiếng Việt](README.vi.md) · [🇹🇭 ไทย](README.th.md) · [🇯🇵 日本語](README.ja.md) · [🇰🇷 한국어](README.ko.md) · [🇨🇳 简体中文](README.zh-Hans.md) · [🇹🇼 繁體中文](README.zh-TW.md) · [🇸🇦 العربية](README.ar.md) · [🇮🇱 עברית](README.he.md) · [🇮🇷 فارسی](README.fa.md) · [🇮🇳 हिन्दी](README.hi.md) · [🇵🇰 اردو](README.ur.md) · [🇧🇩 বাংলা](README.bn.md)

> Bu mətn 18 oktyabr 2026-cı ildə ingiliscə README-dən tərcümə olunub. Ən aktual versiya ingiliscə README-dir.

</div>


## Haqqında

VibeCut macOS, Windows, Linux və Android üçün pulsuz, açıq mənbəli video redaktorudur və CapCut-a alternativdir. İnsanların CapCut-dan ən çox istifadə etdiyi imkanların hamısı burada var: avtomatik subtitrlər, mətnin səsləndirilməsi, fonun silinməsi, açar kadrlarla animasiya, effektlər və yazılar, bir neçə trekdə montaj, 4K ixrac. Üstəlik heç bir gizli şərt yoxdur: nə su nişanı, nə hesab, nə abunəlik, nə də faylların serverə yüklənməsi.

Hər şey Rust ilə yazılmış öz mühərriki üzərində, birbaşa kompüterinizdə işləyir; görüntünün birləşdirilməsi (kompozitinq) GPU-da aparılır. Proqramı quraşdırın, videonuzu əlavə edin və montaja başlayın. Subtitr, səsləndirmə və fon silmə üçün süni intellekt modelləri Parametrlər bölməsindən bir dəfə cihazınıza endirilir, sonra internetsiz işləyir. Videolarınız heç vaxt kompüterinizdən kənara çıxmır. AI söhbəti istəyə bağlıdır: yalnız mətni seçdiyiniz provayderə göndərir — videonuzu, şəkillərinizi və ya səsinizi heç vaxt.

**Nə üçün uyğundur:** TikTok, Reels və Shorts, YouTube videoları, dərslər və ekran yazıları, podkastlardan kəsiklər, memlər.

**Avtomatlaşdırma üçün də:** süni intellekt agentləri üçün daxili MCP serveri (`vibecut --mcp`, [quraşdırma](MCP.md)) və skriptlər üçün JSON-RPC API. Belə ki, onlar da VibeCut ilə video montaj edə bilər.

## Əsas imkanlar

- 🚫 **Su nişanı yoxdur. Hesab tələb olunmur. Pullu məhdudiyyət yoxdur.** Heç vaxt.
- 🔒 **Materialınız diskinizdən heç vaxt çıxmır.** İstəyə bağlı AI söhbəti yalnız mətn göndərir.
- 💬 **Avtomatik subtitrlər.** Kompüterinizdə işləyən Whisper. Model ölçüsünü seçin, zaman xəttində hazır üslubu olan subtitrlər alın.
- 🗣️ **Mətnin səsləndirilməsi və səs klonlama.** Pulsuz lokal səslər, ya da cəmi bir neçə saniyəlik səs yazısından klonlanmış istənilən səs.
- 🧍 **Fonun silinməsi.** İnsanları və obyektləri avtomatik ayırır, istəsəniz maskanı özünüz çəkə bilərsiniz.
- 🎞️ **Açar kadrlar.** Mövqe, miqyas, fırlanma, görünürlük, səs səviyyəsi və effekt parametrləri. Daxili əyri redaktoru ilə.
- ✨ **100 effekt, filtr və keçid.** GPU-da render olunur, önizləmədə dərhal görünür.
- ✂️ **Sürətli montaj.** Bölmə, kəsib qısaltma, boşluğu avtomatik bağlayan ripple redaktə, birləşdirmə, kadrı dondurma, sürət. İstəsəniz, maqnitli zaman xətti də var.
- 🎚️ **Bir neçə trek, bir neçə zaman xətti.** Bir layihədə bir neçə montaj variantı. Qarışdırma rejimləri, kəsib çərçivələmə, çevirmə.
- 📝 **Yazılar.** Şriftlər, kontur, kölgə, fon lövhəsi. Başlamaq üçün hazır şablonlar.
- 🎙️ **Bir düymə ilə səs təmizləmə.** Küyü azaldır, nitqi aydınlaşdırır, səs gücünü bərabərləşdirir. Üstəlik sincab, robot, telefon və digər səs effektləri.
- 📤 **İxrac.** H.264, HEVC, AV1. 4K 60 kadr/san-a qədər, 10-bit rəng.
- 🦀 **Rust ilə yazılmış öz mühərriki.** GPU-da kompozitinq, proxy fayllar, aparat səviyyəsində dekodlama. 4K videolarda da zaman xətti boyunca axıcı hərəkət edin.
- 🤖 **Skriptlərlə idarə oluna bilər.** Daxili MCP serveri sayəsində Claude və digər süni intellekt agentləri VibeCut ilə montaj edə bilər ([bir sətirlə quraşdırılır](MCP.md)), skriptlər üçün isə JSON-RPC API var. Mənbə kodundan yığılan versiyaya əlavə olaraq gRPC və komanda sətri aləti daxildir.
- 🖥️ **macOS, Windows, Linux, Android.** 26 dil. Eyni proqram, eyni layihə faylları.

## Yükləmə

VibeCut-ı iki yolla əldə edə bilərsiniz:

1. **[Veb sayt](https://github.com/OscarGallardo35/VibeCut/#download)** cihazınıza uyğun versiyanı avtomatik təklif edir. Buradan başlamağınız tövsiyə olunur.
2. **[GitHub Releases](https://github.com/OscarGallardo35/VibeCut/releases)** bütün platformalar üçün bütün versiyaları quraşdırıcılar, paketlər və yoxlama cəmləri (checksum) ilə birlikdə təqdim edir. Versiyanı özünüz seçmək istəyirsinizsə, bura baxın.

VibeCut hələ **beta** mərhələsindədir: işləyir, amma hələ də bəzi çatışmazlıqları var. Problemlə qarşılaşsanız, [bizə bildirin](https://github.com/OscarGallardo35/VibeCut/issues).

**Platformalar**

- ✅ **Windows** · x86_64 (ARM versiyalarının buraxılışı müvəqqəti dayandırılıb, bax: [#326](https://github.com/OscarGallardo35/VibeCut/issues/326)). Quraşdırıcı və `.msi` paketi. SmartScreen imzasız versiyanın açılmasının qarşısını alsa: **More info** › **Run anyway**
- ✅ **macOS** · Intel və Apple silicon. macOS imzasız versiyanı açmaq istəməsə: `xattr -dr com.apple.quarantine /Applications/VibeCut.app`
- ✅ **Linux** · x86_64 və ARM. `.deb`, `.rpm`, `.AppImage` və Arch paketi. Bundan əlavə, Flatpak versiyası da var: o, **[Flatpark](https://flatpark.org/apps/pro.mercadodigital.vibecut/)** icma deposundadır. Bu depo hər buraxılışın x86_64 `.deb` paketini Flatpak formatına salır və onunla birlikdə yenilənir
- ✅ **Android** · telefonlar və planşetlər
- ✅ **iOS / iPadOS** · iPhone və iPad, App Store-dan kənar quraşdırma (sideload) ilə

✅ Dəstəklənir · 🚧 Üzərində işlənir · 🧪 Test olunmalıdır

**Sistem tələbləri** və əlavə model ölçüləri haqqında məlumat [veb saytdadır](https://github.com/OscarGallardo35/VibeCut/tree/main/docs/system-requirements).

**[Code signing policy](../CODE_SIGNING.md)**

## Layihəyə necə töhfə vermək olar

> [!IMPORTANT]
> Ən faydalı töhfə [Releases](https://github.com/OscarGallardo35/VibeCut/releases) səhifəsindən son versiyanı yükləyib istifadə etməkdir: rastlaşdığınız xətaları bildirin və nəyin daha yaxşı ola biləcəyini təklif edin.
>
> Kod yazmağa hazırsınız? [CONTRIBUTING.md](../CONTRIBUTING.md) mühitin qurulmasını, layihənin fayl strukturunu, işə salınmalı olan yoxlamaları və töhfələrin hansı lisenziya ilə qəbul edildiyini izah edir. VibeCut-ı skriptdən, servisdən və ya agentdən idarə etmək istəyirsiniz? [Developer sənədləri](https://github.com/OscarGallardo35/VibeCut/tree/main/docs) VibeCut API-ni və onun əlaqə protokollarını, yəni JSON-RPC və gRPC-ni izah edir; [docs/MCP.md](MCP.md) isə süni intellekt agentini MCP vasitəsilə necə qoşmağı göstərir. Layihə ilk dəfə [bu müzakirədə](https://github.com/OscarGallardo35/VibeCut/discussions/3) elan olunub.

## Töhfə verənlər

<a href="https://github.com/OscarGallardo35/VibeCut/graphs/contributors">
  <img alt="Contributors" src="https://contrib.rocks/image?repo=OscarGallardo35/VibeCut">
</a>
