#!/usr/bin/env python3
"""Comprueba que el pack de plantillas siga siendo instanciable.

    scripts/templates-check.py

Lo que el sistema de plantillas necesita, carpeta por carpeta: un
`template.json` que abra, slots vacíos y distintos, cada uno usado por un clip,
una línea de tiempo sin huecos ni solapes y textos con contenido que no se
pisen en su pista. Sale 1 con el detalle de lo que falla.

Lo corre el CI (`.github/workflows/selfhosted.yml`) y se puede correr a mano
después de editar el pack. Para cambiar una plantilla no se edita el JSON a
mano: se genera con la API del motor y se vuelve a correr esto.
"""
import glob
import json
import os
import sys

PACK = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))),
                    "templates", "gastronomia")
FONT = "Hanken Grotesk"
EPS = 1e-6


def check_pack():
    fails = []
    files = sorted(glob.glob(os.path.join(PACK, "*", "template.json")))
    if not files:
        return ["no hay plantillas en templates/gastronomia/"], []
    rows = []
    for path in files:
        slug = os.path.basename(os.path.dirname(path))
        doc = json.load(open(path, encoding="utf-8"))
        media = doc.get("media", [])
        clips = doc.get("clips", [])
        slots = [m for m in media if m.get("placeholder")]
        pics = sorted([c for c in clips if c.get("kind") in ("image", "video")],
                      key=lambda c: c["start"])
        texts = [c for c in clips if c.get("kind") == "text"]
        duration = max((c["start"] + c["duration"] for c in clips), default=0.0)

        video = doc.get("video", {})
        if (video.get("width"), video.get("height")) != (1080, 1920):
            fails.append(f"{slug}: no es 1080x1920")
        if (video.get("rateNum"), video.get("rateDen")) != (30, 1):
            fails.append(f"{slug}: no es 30 fps")
        if len(slots) < 2:
            fails.append(f"{slug}: menos de dos slots")
        if any(m["path"] for m in slots):
            fails.append(f"{slug}: un slot conserva path")
        if len({m["id"] for m in media}) != len(media):
            fails.append(f"{slug}: ids de medio repetidos")
        used = {c["mediaId"] for c in pics}
        for slot in slots:
            if slot["id"] not in used:
                fails.append(f"{slug}: el slot {slot['id']} no lo usa ningun clip")
        if not pics:
            fails.append(f"{slug}: sin clips de imagen o video")
            continue
        if abs(pics[0]["start"]) > EPS:
            fails.append(f"{slug}: no arranca en 0")
        for a, b in zip(pics, pics[1:]):
            if abs((a["start"] + a["duration"]) - b["start"]) > EPS:
                fails.append(f"{slug}: hueco o solape entre {a['id']} y {b['id']}")
        for clip in texts:
            body = (clip.get("text") or {}).get("content", "")
            if not body.strip():
                fails.append(f"{slug}: texto vacio en {clip['id']}")
            if (clip.get("text") or {}).get("fontFamily") != FONT:
                fails.append(f"{slug}: {clip['id']} no usa la fuente de marca")
            if clip["start"] + clip["duration"] > duration + EPS:
                fails.append(f"{slug}: {clip['id']} se sale de la linea de tiempo")
        by_track = {}
        for clip in texts:
            by_track.setdefault(clip["trackId"], []).append(clip)
        for track, items in by_track.items():
            items.sort(key=lambda c: c["start"])
            for a, b in zip(items, items[1:]):
                if a["start"] + a["duration"] > b["start"] + EPS:
                    fails.append(f"{slug}: textos encimados en {track} ({a['id']}, {b['id']})")
        rows.append((slug, len(slots), len(texts), duration))
    return fails, rows


def main():
    fails, rows = check_pack()
    if fails:
        print(f"{len(fails)} problemas:")
        for line in fails:
            print("  -", line)
        return 1
    slots = sum(r[1] for r in rows)
    shortest = min(r[3] for r in rows)
    longest = max(r[3] for r in rows)
    print(f"{len(rows)} plantillas validas, {slots} slots, "
          f"{shortest:.1f}-{longest:.1f} s")
    return 0


if __name__ == "__main__":
    sys.exit(main())
