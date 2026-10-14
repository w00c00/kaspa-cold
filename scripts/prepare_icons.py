"""Resize the approved icon into native platform formats; no design changes."""
from pathlib import Path
import subprocess
import sys
from PIL import Image

root = Path(__file__).resolve().parents[1]
assets = root / "assets"
source = Image.open(assets / "icon-concept-v3.png").convert("RGBA")
for size in (32, 256, 512):
    source.resize((size, size), Image.Resampling.LANCZOS).save(assets / f"icon-{size}.png", optimize=True)
source.resize((256, 256), Image.Resampling.LANCZOS).save(
    assets / "icon.ico", sizes=[(16,16),(24,24),(32,32),(48,48),(64,64),(128,128),(256,256)]
)
if sys.platform == "darwin":
    iconset = root / "target" / "KaspaCold-compact.iconset"
    iconset.mkdir(parents=True, exist_ok=True)
    for size in (16,32,128,256,512):
        for scale in (1,2):
            if size * scale > 512:
                continue
            filename = f"icon_{size}x{size}{'@2x' if scale == 2 else ''}.png"
            source.resize((size*scale,size*scale), Image.Resampling.LANCZOS).save(iconset / filename)
    subprocess.run(["iconutil", "-c", "icns", str(iconset), "-o", str(assets / "icon.icns")], check=True)
print("Prepared PNG, ICO and (on macOS) ICNS from approved v3.")
