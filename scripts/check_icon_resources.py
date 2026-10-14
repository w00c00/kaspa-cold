"""Verify public icon assets and native package resource declarations."""
from pathlib import Path
import plistlib
import struct

root = Path(__file__).resolve().parents[1]
exe = root / "target/x86_64-pc-windows-gnu/release/kaspa-cold.exe"
if exe.exists():
    data = exe.read_bytes()
    pe = struct.unpack_from("<I", data, 0x3c)[0]
    assert data[pe:pe+4] == b"PE\0\0"
    count = struct.unpack_from("<H", data, pe+6)[0]
    optional_size = struct.unpack_from("<H", data, pe+20)[0]
    sections = pe+24+optional_size
    resources = None
    for i in range(count):
        section = sections+i*40
        if data[section:section+8].rstrip(b"\0") == b".rsrc":
            resources = struct.unpack_from("<I", data, section+20)[0]
    assert resources is not None, "EXE has no resource section"
    named, ids = struct.unpack_from("<HH", data, resources+12)
    types = {struct.unpack_from("<I",data,resources+16+i*8)[0] for i in range(named+ids)}
    assert {3,14} <= types, "EXE must contain RT_ICON and RT_GROUP_ICON"
    print("Windows: RT_ICON and RT_GROUP_ICON verified")
for app in (root / "dist").glob("*apple-darwin/Kaspa Cold.app"):
    info = plistlib.loads((app / "Contents/Info.plist").read_bytes())
    icon = app / "Contents/Resources" / info["CFBundleIconFile"]
    assert icon.read_bytes().startswith(b"icns")
    print(f"macOS: ICNS and CFBundleIconFile verified ({app.parent.name})")
