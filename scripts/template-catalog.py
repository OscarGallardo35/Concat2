#!/usr/bin/env python3
"""Catalogo del pack de plantillas: previsualizaciones + una pagina sola.

    python scripts/template-catalog.py

Lee cada templates/gastronomia/<slug>/template.json, dibuja una tira de frames
con la misma fuente, tamano, color y placa que declara el documento (las fotos
son marcadores: no hay render del motor aca) y escribe:

    templates/gastronomia/previews/<slug>.png
    templates/gastronomia/index.html

La pagina es autonoma -sin JS ni fuentes de CDN- para poder hostearse en
cualquier lado o abrirse desde el disco.
"""
import glob
import html
import json
import os
import sys

try:
    from PIL import Image, ImageDraw, ImageFont
except ImportError:
    sys.exit("hace falta Pillow: python -m pip install pillow")

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
PACK = os.path.join(ROOT, "templates", "gastronomia")
PREVIEWS = os.path.join(PACK, "previews")
FONTS = os.path.join(ROOT, "src", "crates", "vibecut-text", "fonts")

W, H = 1080, 1920
THUMB_W, THUMB_H = 216, 384
PALETTE = ["#2F3A46", "#43362B", "#3B2F45", "#2B3F3A", "#453A2A", "#33384A", "#402F32"]
INK = "#0B0B0F"
AMBER = "#F3A712"


def font(name, size):
    return ImageFont.truetype(os.path.join(FONTS, name), size)


def wrap(draw, text, fnt, max_w):
    lines, line = [], ""
    for word in text.split():
        probe = f"{line} {word}".strip()
        if draw.textlength(probe, font=fnt) <= max_w or not line:
            line = probe
        else:
            lines.append(line)
            line = word
    return lines + ([line] if line else [])


def frame(doc, t):
    img = Image.new("RGB", (W, H), INK)
    d = ImageDraw.Draw(img, "RGBA")
    pics = sorted([c for c in doc["clips"] if c["kind"] in ("image", "video")],
                  key=lambda c: c["start"])
    for i, clip in enumerate(pics):
        if clip["start"] <= t < clip["start"] + clip["duration"]:
            media = next((m for m in doc["media"] if m["id"] == clip["mediaId"]), {})
            d.rectangle([0, 0, W, H], fill=PALETTE[i % len(PALETTE)])
            d.text((W // 2, H // 2 - 22), media.get("name", "slot"),
                   font=font("HankenGrotesk-SemiBold.ttf", 54), fill="#FFFFFF60", anchor="mm")
            what = "clip del local" if media.get("kind") == "video" else "foto del local"
            d.text((W // 2, H // 2 + 46), what,
                   font=font("HankenGrotesk-Regular.ttf", 32), fill="#FFFFFF33", anchor="mm")
            break
    for clip in doc["clips"]:
        if clip["kind"] != "text" or not (clip["start"] <= t < clip["start"] + clip["duration"]):
            continue
        st = clip["text"]
        size = max(12, int(st["fontSize"] * H))
        fnt = font("HankenGrotesk-Bold.ttf" if st["fontWeight"] >= 700 else "HankenGrotesk-Medium.ttf", size)
        lines = wrap(d, st["content"], fnt, int(W * (st.get("maxWidth") or 0.84)))
        lh = int(size * st.get("lineHeight", 1.2))
        total = lh * len(lines)
        y = H / 2 + (clip.get("offsetY") or 0) * H
        plate = (st.get("background") or "").strip()
        if plate:
            box_w = max(d.textlength(l, font=fnt) for l in lines) + W * st.get("backgroundPaddingX", 0.03) * 2
            box_h = total + W * st.get("backgroundPaddingY", 0.018) * 2
            d.rounded_rectangle([W / 2 - box_w / 2, y - box_h / 2, W / 2 + box_w / 2, y + box_h / 2],
                                radius=int(box_h * 0.28), fill=plate)
        ty = y - total / 2
        for line in lines:
            d.text((W / 2, ty + lh / 2), line, font=fnt, fill=st["color"], anchor="mm")
            ty += lh
    return img


def strip(doc, path, frames=4):
    """Una tira horizontal con los primeros frames de la plantilla."""
    pics = sorted([c for c in doc["clips"] if c["kind"] in ("image", "video")],
                  key=lambda c: c["start"])[:frames]
    pad = 10
    out = Image.new("RGB", (len(pics) * THUMB_W + (len(pics) + 1) * pad,
                            THUMB_H + 2 * pad), "#16161C")
    x = pad
    for clip in pics:
        out.paste(frame(doc, clip["start"] + clip["duration"] / 2).resize((THUMB_W, THUMB_H), Image.Resampling.LANCZOS),
                  (x, pad))
        x += THUMB_W + pad
    out.save(path, optimize=True)
    return pics


def human(seconds):
    return f"{seconds:.1f} s".replace(".", ",")


CARDS = """
  <article class="card">
    <div class="shot">
      <img src="previews/{slug}.png" alt="{name}, frame a frame" loading="lazy" />
      <span>maqueta</span>
    </div>
    <div class="body">
      <h2>{name}</h2>
      <p class="meta">{n} slots · {dur}</p>
      <ul class="chips">{chips}</ul>
    </div>
  </article>"""

PAGE = """<!doctype html>
<html lang="es">
<meta charset="utf-8" />
<meta name="viewport" content="width=device-width, initial-scale=1" />
<title>Plantillas gastronómicas · VibeCut</title>
<style>
  :root {{
    --ink: {ink}; --paper: #F6F6FA; --muted: #9A9AAA;
    --violet: #7C3AED; --cyan: #06B6D4; --amber: {amber};
    --card: #16161C; --line: #2A2A33;
  }}
  * {{ box-sizing: border-box; }}
  body {{
    margin: 0; background: var(--ink); color: var(--paper);
    font: 16px/1.55 "Hanken Grotesk", system-ui, -apple-system, Segoe UI, sans-serif;
    -webkit-font-smoothing: antialiased;
  }}
  .hero {{
    background: radial-gradient(60% 120% at 12% 0%, rgba(124,58,237,.28), transparent 70%),
                radial-gradient(50% 100% at 92% 8%, rgba(6,182,212,.18), transparent 70%);
  }}
  header {{ padding: 72px 32px 42px; max-width: 1180px; margin: 0 auto; }}
  .eyebrow {{
    display: inline-block; font-size: 13px; letter-spacing: .14em; text-transform: uppercase;
    color: var(--muted); margin: 0 0 18px;
  }}
  h1 {{ font-size: clamp(34px, 5vw, 56px); line-height: 1.05; margin: 0 0 16px; letter-spacing: -.02em; }}
  .lead {{ font-size: clamp(17px, 2vw, 21px); color: var(--muted); max-width: 62ch; margin: 0; }}
  .stats {{ display: flex; gap: 28px; flex-wrap: wrap; margin: 34px 0 0; padding: 0; list-style: none; }}
  .stats li {{ font-size: 15px; color: var(--muted); }}
  .stats b {{ display: block; font-size: 26px; color: var(--paper); letter-spacing: -.01em; }}
  main {{ max-width: 1180px; margin: 0 auto; padding: 8px 32px 40px; }}
  .grid {{ display: grid; gap: 26px; grid-template-columns: repeat(auto-fill, minmax(320px, 1fr)); }}
  .card {{
    background: var(--card); border: 1px solid var(--line); border-radius: 18px;
    overflow: hidden; display: flex; flex-direction: column;
  }}
  .card .shot {{ position: relative; background: #0F0F14; }}
  .card .shot img {{ width: 100%; display: block; }}
  .card .shot span {{
    position: absolute; left: 10px; top: 10px; font-size: 11px; letter-spacing: .06em;
    text-transform: uppercase; color: #C9C9D6; background: rgba(11,11,15,.72);
    border: 1px solid #2A2A33; border-radius: 999px; padding: 3px 9px;
  }}
  .body {{ padding: 18px 20px 22px; }}
  h2 {{ font-size: 21px; margin: 0 0 6px; letter-spacing: -.01em; }}
  .meta {{ margin: 0 0 14px; color: #B9B9C8; font-size: 14.5px; font-weight: 500; }}
  .chips {{ list-style: none; margin: 0; padding: 0; display: flex; flex-wrap: wrap; gap: 8px; }}
  .chips li {{
    font-size: 13px; color: #EDEDF5; background: #23232C;
    border: 1px solid var(--line); border-radius: 999px; padding: 5px 11px;
  }}
  section.how {{ max-width: 1180px; margin: 0 auto; padding: 10px 32px 90px; }}
  section.how h2 {{ font-size: 26px; margin: 0 0 14px; }}
  pre {{
    background: #131318; border: 1px solid var(--line); border-radius: 14px;
    padding: 18px 20px; overflow-x: auto; font-size: 14px; line-height: 1.7;
  }}
  code {{ font-family: ui-monospace, SFMono-Regular, Menlo, monospace; }}
  .note {{ color: var(--muted); font-size: 14px; }}
  footer {{ border-top: 1px solid var(--line); padding: 26px 32px; color: var(--muted); font-size: 13px; text-align: center; }}
  a {{ color: var(--cyan); }}
</style>
<div class="hero"><header>
  <p class="eyebrow">VibeCut · pack gastronómico</p>
  <h1>{count} plantillas para el local</h1>
  <p class="lead">Verticales, 1080×1920, listas para Reels y TikTok. El local pone sus
  fotos y su logo: los tiempos, los textos y el ritmo ya están armados.</p>
  <ul class="stats">
    <li><b>{count}</b>plantillas</li>
    <li><b>{slots}</b>slots en total</li>
    <li><b>{shortest}–{longest}</b>de duración</li>
    <li><b>1080 × 1920 px</b>a 30 fps</li>
  </ul>
</header></div>
<main>
  <div class="grid">{cards}
  </div>
</main>
<section class="how">
  <h2>Cómo se instalan</h2>
  <pre><code>python scripts/install-templates.py            # las {count}
python scripts/install-templates.py --list     # ver el pack
python scripts/install-templates.py plato-del-dia
python scripts/install-templates.py --portable D:/vibecut-portable</code></pre>
  <p class="note">Copian a la carpeta de plantillas del editor
  (<code>%APPDATA%\\pro.mercadodigital.vibecut\\templates</code> en Windows, el
  equivalente en macOS y Linux). Después: <b>Inicio › Plantillas</b>.</p>
  <p class="note">Las previsualizaciones de esta página se dibujan leyendo el
  propio <code>template.json</code>: misma fuente, tamaño, color y placa. Las
  fotos son marcadores con el nombre del slot, no un render del motor.</p>
</section>
<footer>VibeCut · AGPL-3.0-or-later · las plantillas son datos, no llevan
música ni imágenes de terceros.</footer>
</html>
"""


def main():
    os.makedirs(PREVIEWS, exist_ok=True)
    rows = []
    slots_total, durations = 0, []
    for path in sorted(glob.glob(os.path.join(PACK, "*", "template.json"))):
        slug = os.path.basename(os.path.dirname(path))
        doc = json.load(open(path, encoding="utf-8"))
        pics = strip(doc, os.path.join(PREVIEWS, f"{slug}.png"))
        slots = [m for m in doc["media"] if m.get("placeholder")]
        total = max(c["start"] + c["duration"] for c in doc["clips"])
        slots_total += len(slots)
        durations.append(total)
        print(f"{slug:<24} {len(slots)} slots  {human(total)}")
        rows.append((total, CARDS.format(
            slug=slug,
            name=html.escape(doc["name"]),
            n=len(slots),
            dur=human(total),
            chips="".join(f"<li>{html.escape(m['name'])}</li>" for m in slots[:4])
                  + (f"<li>y {len(slots) - 4} más</li>" if len(slots) > 4 else ""),
        )))

    rows.sort(key=lambda r: r[0])
    page = PAGE.format(
        ink=INK, amber=AMBER,
        count=len(rows), slots=slots_total,
        shortest=human(min(durations)).replace(" s", ""),
        longest=human(max(durations)),
        cards="".join(card for _, card in rows),
    )
    out = os.path.join(PACK, "index.html")
    open(out, "w", encoding="utf-8").write(page)
    print(f"\n{out}  ({len(page) // 1024} KB)")


if __name__ == "__main__":
    main()
