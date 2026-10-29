#!/usr/bin/env python3
"""Instala el pack de plantillas en la carpeta de plantillas de VibeCut.

    python scripts/install-templates.py              # todas las del pack
    python scripts/install-templates.py --list       # solo lista
    python scripts/install-templates.py plato-del-dia promo-mediodia
    python scripts/install-templates.py --portable D:/vibecut-portable
    python scripts/install-templates.py --force      # sobrescribe

La carpeta la decide el identificador de la app, igual que el editor:
  Windows  %APPDATA%\\pro.mercadodigital.vibecut\\templates
  macOS    ~/Library/Application Support/pro.mercadodigital.vibecut/templates
  Linux    ${XDG_CONFIG_HOME:-~/.config}/pro.mercadodigital.vibecut/templates
Un build portable guarda lo suyo en la carpeta `portable` que esta al lado del
binario: con --portable se copia ahi, sin tocar nada mas.
"""
import argparse
import os
import shutil
import sys

IDENTIFIER = "pro.mercadodigital.vibecut"
PACK = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))),
                    "templates", "gastronomia")


def templates_dir(portable=None):
    if portable:
        return os.path.join(portable, "templates")
    if sys.platform.startswith("win"):
        base = os.environ.get("APPDATA")
        if not base:
            raise SystemExit("APPDATA no esta definido")
        return os.path.join(base, IDENTIFIER, "templates")
    if sys.platform == "darwin":
        return os.path.join(os.path.expanduser("~"), "Library", "Application Support",
                            IDENTIFIER, "templates")
    base = os.environ.get("XDG_CONFIG_HOME") or os.path.join(os.path.expanduser("~"), ".config")
    return os.path.join(base, IDENTIFIER, "templates")


def pack_names():
    if not os.path.isdir(PACK):
        raise SystemExit(f"no encuentro el pack en {PACK}")
    return sorted(name for name in os.listdir(PACK)
                  if os.path.isfile(os.path.join(PACK, name, "template.json")))


def main():
    ap = argparse.ArgumentParser(add_help=True)
    ap.add_argument("names", nargs="*", help="plantillas a instalar (por defecto, todas)")
    ap.add_argument("--list", action="store_true", help="listar el pack y salir")
    ap.add_argument("--portable", metavar="DIR", help="carpeta de un build portable")
    ap.add_argument("--force", action="store_true", help="sobrescribir si ya existe")
    args = ap.parse_args()

    available = pack_names()
    if args.list or not available:
        for name in available:
            print(name)
        return

    names = args.names or available
    unknown = [n for n in names if n not in available]
    if unknown:
        raise SystemExit(f"no estan en el pack: {', '.join(unknown)}")

    target = templates_dir(args.portable)
    os.makedirs(target, exist_ok=True)
    print(f"plantillas -> {target}")
    copied = skipped = 0
    for name in names:
        src = os.path.join(PACK, name)
        dst = os.path.join(target, name)
        if os.path.exists(dst) and not args.force:
            print(f"  ya esta   {name}")
            skipped += 1
            continue
        if os.path.exists(dst):
            shutil.rmtree(dst)
        shutil.copytree(src, dst)
        print(f"  instalada {name}")
        copied += 1
    print(f"\n{copied} instaladas, {skipped} ya estaban. Abri VibeCut: Inicio > Plantillas.")


if __name__ == "__main__":
    main()
