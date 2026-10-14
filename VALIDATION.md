# Validation / 验证记录

Date / 日期: 2026-10-14 (Asia/Shanghai). Release / 版本: 0.1.0 test build; artwork revision 2.

## Completed / 已完成

- `cargo test --locked --offline`: **13 tests passed** (8 core/QR, 4 UI-state/render tests, 1 embedded-icon RGBA decoder test).
- Official rusty-kaspa published gen1 mnemonic/address vector matched at TN10 receiving index 1.
- BIP39 published zero-entropy vector matched; 12/24-word round trips, invalid words/checksum, nonstandard word count and bounded index rejection tested.
- Injected OS entropy failures abort both generation and backup challenge selection; no weak fallback.
- Three distinct backup positions, correct/incorrect answers tested. Successful completion clears app-owned secret buffers and leaves only public derivation state.
- Mainnet/TN10 changes preserve the public key payload, use different network checksums and reproduce the same original mainnet address on switching back.
- Headless rendering tested in both languages across all six pages; no clipboard-copy output emitted by secret screens during those tests.
- Error handling stays on the input page and does not echo supplied input. Public recovery exports exclude test mnemonic/private-key representations.
- `cargo clippy --locked --offline --all-targets -- -D warnings`: passed.
- QR PNG independently decoded using OpenCV 4.13.0; payload exactly matched the public test address. Quiet zone validated.
- Intel-target core tests: **8 passed**, executed on Apple Silicon macOS via translation, not on physical Intel hardware.
- Release builds produced Mach-O arm64/x86_64, PE32+ Windows x64 GUI and ELF Linux x64 executables.
- Windows import inspection: only Windows/system-UCRT DLLs, no additional MinGW runtime DLLs. Linux imports only libc/libdl/libm/libpthread directly; GUI libraries are loaded at runtime. Highest required glibc symbol version: **2.28**.
- macOS Apple Silicon native smoke: app launch, Chinese glyphs, English toggle, masked public-vector recovery, address and QR rendering, TN10 toggle, native export dialog opening, clear confirmation and return to mainnet Chinese home.
- Process-scoped `lsof -b -nP -a -c kaspa-cold -i` snapshots after launch and recovery showed no network sockets. This is sampling, not a full packet trace.
- Public test session cleared after testing. No actual user seeds/private keys were used.

## Still required / 尚待验证

- Windows CPU build: launch/running confirmed by the user on physical Windows 11. Detailed operation-by-operation checks (clipboard, QR/TXT exports, focus, firewall-denied operation) and Windows 10 testing remain pending. Linux real-device smoke remains pending. Compilation is **not** runtime verification.
- Physical Intel Mac GUI smoke. Translated core tests do not substitute for hardware testing.
- Full end-to-end native generation/handwritten backup flow, clipboard checks and file-saving checks on each platform. Backup logic is unit-tested, but the native smoke intentionally did not capture a newly generated secret.
- Full packet-trace/firewall-denied testing across all native actions. No network settings were changed on the user's machine.
- Independent security review before recommending meaningful funds.

## Dependency scan / 依赖扫描

`cargo audit --file Cargo.lock` completed with exit code 0 and **10 informational warnings**, not a clean bill of health. The full lockfile inventory includes optional and platform-only dependencies.

Unmaintained: async-std, atty, derivative, instant, paste, proc-macro-error, ttf-parser, unsafe-libyaml.

Unsoundness warnings:

- `atty 0.2.14`: RUSTSEC-2021-0145, potential unaligned read.
- `faster-hex 0.9.0`: RUSTSEC-2026-0306, unchecked AVX2 decoder can read past source.

These are inherited dependencies; this application's source does not call the unchecked hex decoder or terminal-detection APIs. Their existence is recorded rather than dismissed. Lockfile advisories need to be rechecked before every release; no independent reachability audit was performed.

这些警告主要来自上游间接依赖。本程序没有直接调用 unchecked hex 解码或终端检测 API，但这不等于完成了可达性审计。依赖扫描不证明冷钱包安全。公开发布前仍需复核、更新依赖并做独立安全审查。

## Build provenance / 构建来源

- Rust/rustc: 1.95.0, `59807616e`.
- Native/macOS build: macOS Apple Silicon host, Apple clang/Xcode SDK.
- Windows GNU/Linux cross-linker: Zig 0.14.1 via cargo-zigbuild 0.23.4.
- Cryptographic primitives: official kaspa-wallet-keys/kaspa-bip32/kaspa-addresses 2.1.0, pinned in Cargo.lock.
- UI: egui 0.33.3; macOS/Linux eframe/Glow; new Windows variant egui_software_backend 0.0.2 + softbuffer 0.4.8 (CPU + GDI). No Electron/WebView/Node runtime.
- Chinese font: bundled Noto CJK subset, approximately 127 KiB, SIL OFL 1.1.
- macOS packages: ad-hoc signed only, not Apple Developer signed/notarized. Windows: unsigned.

This is a testing delivery, not a claim of audited or mainnet-ready custody security.

## Windows startup diagnostic 1 / Windows 启动诊断版 1

- Additional Windows-only archive; original published packages and v0.1.0 tag are preserved.
- Native Windows MessageDialog reports recognized startup OS/driver errors without requiring OpenGL to draw the dialog.
- Unknown application errors, all post-initialization errors and panic payloads are withheld; no log files, telemetry or automatic clipboard output added.
- Current diagnostic source: **16 automated tests passed**, including three diagnostic payload/privacy/length tests; clippy passed with warnings denied.
- `--diagnostic-self-test` exercised on the Mac host's text fallback and returns success without creating a wallet. This does **not** verify the Windows MessageBox at runtime.
- Windows x64 cross-build passed. Native Windows testing and the user's actual startup error remain outstanding; this diagnostic build is **not a confirmed startup fix**.

## Windows portable CPU 1 / Windows CPU 兼容版 1

- User screenshot confirmed `COLD-GL-PAINTER`: OpenGL 2.0+ unavailable on a physical Windows 11 machine.
- Windows renderer replaced with CPU rasterization + softbuffer GDI. Official key derivation/address encoding unchanged; macOS/Linux retain their original renderer.
- **18 automated tests passed**: original 16 plus actual CPU rasterization in both languages/all six pages, and focus-loss/secret-screen clipboard suppression. Clippy with warnings denied passed on the host.
- Rasterized tile caching disabled to avoid retaining secret glyph pixels there; app buffers cleaned best-effort on drop. System framebuffer, swap and screenshots are outside the guarantee.
- Windows x64 cross-build passed. On 2026-10-14, the user confirmed the CPU build runs on the same physical Windows 11 machine that previously failed with unsupported OpenGL. This is user-reported runtime confirmation, not an independently observed full feature/security validation.
- PE imports verified: only system Windows/UCRT DLLs; GDI `CreateDIBSection`/`BitBlt` present, no OpenGL/Direct3D/MinGW DLL import. Icon resources verified.
- CPU-rendered public receive-screen QR independently decoded with OpenCV and matched the published BIP39 zero-entropy vector's address. No generated secret was captured.
- EXE **4,967,424 bytes (4.74 MiB)**; portable ZIP including public documentation/licenses approximately **4.61 MiB**. Target-specific license inventory avoids shipping unrelated platform dependencies; no executable packer/UPX used.
- `cargo audit` re-run: exit 0 with the same **10 informational warnings** listed above, not an independent security audit.
- Windows-target `cargo clippy --all-targets -- -D warnings` also passed using Zig CC and LLVM AR. Initial plain cross-Clippy attempts failed on missing MinGW CC and a misconfigured archiver, not application diagnostics; corrected tooling passed.

## Publication / 发布

The user approved the v0.1.0 GitHub release after Windows 11 runtime confirmation. Existing binary assets, checksums and tag are preserved; their bundled validation files describe the earlier build-time status. This repository record and release notes contain the subsequent runtime confirmation. Formal publication is not an independent security audit. / 用户确认 Windows 11 可运行后授权正式发布。现有二进制、校验值和标签保留，包内验证说明仍记录构建时状态；后续实机确认以本记录和 Release 说明为准。正式发布不等于完成独立安全审计。
