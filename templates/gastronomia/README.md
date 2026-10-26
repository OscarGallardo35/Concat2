# Pack gastronómico

Doce plantillas verticales para locales: 1080×1920, 30 fps, entre 7 y 13
segundos, pensadas para Reels, Shorts y TikTok. Cada una es un proyecto con
huecos: el local pone sus fotos y su logo, todo lo demás ya está armado.

| Plantilla | Slots | Duración | Para qué |
|---|---|---|---|
| `plato-del-dia` | 4 | 10,2 s | Post diario: foto, detalle, clip de cocina, logo |
| `promo-mediodia` | 3 | 7,4 s | Menú ejecutivo con precio y horario |
| `reel-15-nuevo` | 5 | 10,5 s | Novedad en carta, cuatro fotos |
| `antes-despues` | 4 | 11,1 s | Ingredientes → preparación → plato terminado |
| `resena-cliente` | 3 | 7,8 s | Prueba social |
| `reserva-tu-mesa` | 3 | 8,4 s | Llamada a reservar |
| `especial-finde` | 4 | 9,2 s | Promo de fin de semana |
| `delivery-30` | 3 | 7,6 s | Pedidos con zona y horario |
| `detras-de-escena` | 4 | 11,4 s | Equipo y cocina |
| `menu-del-mes` | 7 | 12,7 s | Seis fotos + cierre |
| `eventos-privados` | 3 | 8,2 s | Salón, cumpleaños, empresas |
| `cierre-de-temporada` | 3 | 7,8 s | Agradecimiento / fin de año |

![Las tres primeras, frame a frame](../templates-gastronomia.png)

*Maqueta generada desde los propios `template.json`: mismas fuentes, tamaños,
colores y placas. Las fotos son marcadores con el nombre del slot, no un render
del motor.*

## Cómo se instalan

```bash
python scripts/install-templates.py            # las doce
python scripts/install-templates.py --list     # ver el pack
python scripts/install-templates.py plato-del-dia
python scripts/install-templates.py --portable D:/vibecut-portable
```

Copian a la carpeta de plantillas del editor, que decide el identificador de la
app: `%APPDATA%\pro.mercadodigital.vibecut\templates` en Windows,
`~/Library/Application Support/pro.mercadodigital.vibecut/templates` en macOS,
`${XDG_CONFIG_HOME:-~/.config}/pro.mercadodigital.vibecut/templates` en Linux.
En un build portable, a `portable/templates` al lado del binario.

Después: **Inicio › Plantillas**, elegir una y llenar los slots.

## Qué es una plantilla, técnicamente

Una carpeta con `template.json` —un documento de proyecto normal, el mismo
formato que `vibecut.json`— más `assets/` (lo que viaja dentro del diseño) y
`poster.jpg` (la miniatura). Los slots son medios con `"placeholder": true` y
`path` vacío: al instanciar, el motor los rellena en el lugar
(`Command::FillSlot`) y el clip conserva la posición y el largo que fijó la
plantilla.

Estas doce no traen `assets/`: no hay música ni imágenes de terceros adentro.
Tampoco traen `poster.jpg`, porque el póster es un frame renderizado y este repo
se generó sin toolchain de render. La galería muestra las tarjetas sin
miniatura hasta que se genere con un build completo:

```bash
# con un VibeCut compilado, abrir cada plantilla como proyecto y guardarla de
# nuevo desde Plantillas › Guardar; el póster se escribe solo.
```

## Cómo se generan

No están escritas a mano: las produce
`audit/template-gen` (fuera del repo) usando la API del motor —`Editor`,
`Command::AddTextClip`, `to_document`— y las valida releyéndolas con
`Editor::from_document`. La validación de invariantes (slots vacíos y distintos,
timeline sin huecos, textos sin pisarse) es
`Concat2-rebrand-audit/validar_plantillas.py`.

Para cambiar textos, tiempos o agregar una plantilla, se edita la tabla del
generador y se vuelve a correr; el JSON es salida, no fuente.

## Estilo

- Tipografía: **Hanken Grotesk** 800, con trazo fino negro y sombra, sobre placa
  translúcida para que se lea sobre cualquier foto.
- El cierre va en el último slot, sobre placa `#E4572E`.
- Un solo texto por tramo: el cierre reemplaza al subtítulo del último slot, así
  nunca se pisan dos títulos.
