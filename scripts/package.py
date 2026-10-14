"""Package only public release artifacts. Never invokes a wallet or publishes."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import plistlib
import shutil
import subprocess
import tarfile
import zipfile

ROOT = Path(__file__).resolve().parents[1]
parser = argparse.ArgumentParser()
parser.add_argument("--target", default="aarch64-apple-darwin")
parser.add_argument("--variant", choices=("windows-startup-diag1",))
args = parser.parse_args()
target = args.target
if target not in ("aarch64-apple-darwin", "x86_64-apple-darwin", "x86_64-pc-windows-gnu", "x86_64-pc-windows-msvc", "x86_64-unknown-linux-gnu"):
    raise SystemExit("Unsupported target")
exe = "kaspa-cold.exe" if "windows" in target else "kaspa-cold"
binary = ROOT / "target" / target / "release" / exe
if not binary.exists() and target == "aarch64-apple-darwin":
    binary = ROOT / "target/release" / exe
if not binary.exists():
    raise SystemExit(f"Build first: {binary}")
name = f"Kaspa-Cold-0.1.0-{target}"
if args.variant:
    if "windows" not in target: raise SystemExit("The diagnostic variant is Windows-only")
    name = f"Kaspa-Cold-0.1.0-{args.variant}-{target}"
dist = ROOT / "dist"
folder = dist / name
folder.mkdir(parents=True, exist_ok=True)
for document in ("README.md", "LICENSE", "VALIDATION.md", "Cargo.lock", "rust-toolchain.toml", "WINDOWS_DIAGNOSTICS.md"):
    if (ROOT / document).exists(): shutil.copy2(ROOT / document, folder / document)
licenses = folder / "licenses"
licenses.mkdir(exist_ok=True)
shutil.copy2(ROOT / "assets/FONT-LICENSE.txt", licenses / "Noto-CJK-OFL.txt")
metadata = json.loads(subprocess.check_output(["cargo", "metadata", "--locked", "--offline", "--format-version=1"], cwd=ROOT))
inventory = []
for package in metadata["packages"]:
    inventory.append({"name": package["name"], "version": package["version"], "license": package["license"], "source": package["source"]})
    source = Path(package["manifest_path"]).parent
    candidates = [p for p in source.iterdir() if p.is_file() and p.name.lower().startswith(("license", "copying", "notice"))]
    for p in candidates:
        shutil.copy2(p, licenses / f'{package["name"]}-{package["version"]}-{p.name}')
(licenses / "inventory.json").write_text(json.dumps(inventory, indent=2), encoding="utf-8")
if args.variant:
    commit=subprocess.check_output(["git","rev-parse","HEAD"],cwd=ROOT,text=True).strip()
    (folder / "BUILD_COMMIT.txt").write_text(f"Source commit: {commit}\nVariant: {args.variant}\nWindows runtime UI is not yet verified on a real Windows device.\n",encoding="utf-8")
if "apple" in target:
    contents = folder / "Kaspa Cold.app/Contents"
    (contents / "MacOS").mkdir(parents=True, exist_ok=True)
    (contents / "Resources").mkdir(parents=True, exist_ok=True)
    shutil.copy2(ROOT / "assets/icon.icns", contents / "Resources/KaspaCold.icns")
    app_binary = contents / "MacOS/kaspa-cold"
    shutil.copy2(binary, app_binary)
    app_binary.chmod(0o755)
    info = {"CFBundleName": "Kaspa Cold", "CFBundleDisplayName": "Kaspa Cold", "CFBundleIdentifier": "org.kaspa.cold", "CFBundleExecutable": "kaspa-cold", "CFBundlePackageType": "APPL", "CFBundleShortVersionString": "0.1.0", "CFBundleVersion": "2", "NSHighResolutionCapable": True, "LSMinimumSystemVersion": "11.0"}
    info["CFBundleIconFile"] = "KaspaCold.icns"
    (contents / "Info.plist").write_bytes(plistlib.dumps(info))
    subprocess.run(["codesign", "--force", "--sign", "-", str(folder / "Kaspa Cold.app")], check=True)
else:
    shutil.copy2(binary, folder / exe)
    if "linux" in target: (folder / exe).chmod(0o755)
    if "linux" in target:
        shutil.copy2(ROOT / "assets/icon-512.png", folder / "kaspa-cold.png")
        (folder / "org.kaspa.cold.desktop").write_text("[Desktop Entry]\nType=Application\nName=Kaspa Cold\nComment=Offline Kaspa mnemonic generator\nExec=kaspa-cold\nIcon=kaspa-cold\nTerminal=false\nCategories=Utility;Finance;\n", encoding="utf-8")
if "linux" in target:
    archive = dist / (name + ".tar.gz")
    with tarfile.open(archive, "w:gz") as out: out.add(folder, arcname=name)
else:
    archive = dist / (name + ".zip")
    with zipfile.ZipFile(archive, "w", zipfile.ZIP_DEFLATED, compresslevel=9, strict_timestamps=False) as out:
        for p in sorted(folder.rglob("*")):
            if p.is_file(): out.write(p, p.relative_to(dist))
digest = hashlib.sha256(archive.read_bytes()).hexdigest()
(dist / (archive.name + ".sha256")).write_text(f"{digest}  {archive.name}\n", encoding="ascii")
print(f"{archive}\n{archive.stat().st_size:,} bytes archive; {binary.stat().st_size:,} bytes executable\nSHA256 {digest}")
