# Kaspa Cold v0.1.0 — Offline Wallet Generator / 离线钱包生成器

## Downloads / 下载

Only three common platform packages are published, each with a matching `.sha256`. Windows defaults to the lightweight CPU/GDI build. Superseded Windows OpenGL/diagnostic packages and the Intel Mac archive were removed; the tag and retained binaries/checksums are unchanged.

只发布三个常用平台包，各附 `.sha256` 校验文件。Windows 默认轻量 CPU／GDI 版。旧 Windows OpenGL 版、诊断版和 Intel Mac 包已移除；标签及保留的二进制、校验值不变。

| Platform / 平台 | Package / 文件 | Size / 体积 |
|---|---|---|
| Windows x64 — minimal dependencies / 最少依赖 | `Kaspa-Cold-0.1.0-windows-portable-cpu1-x86_64-pc-windows-gnu.zip` | 4.61 MiB |
| macOS Apple Silicon — M series / M 系列 | `Kaspa-Cold-0.1.0-aarch64-apple-darwin.zip` | 6.20 MiB |
| Linux x64 | `Kaspa-Cold-0.1.0-x86_64-unknown-linux-gnu.tar.gz` | 5.02 MiB |

Windows: extract and run `kaspa-cold.exe`. CPU rendering + built-in GDI; no OpenGL driver, Electron, WebView, Node.js or extra MinGW DLLs. EXE **4.74 MiB**, no UPX/packing. / Windows 解压双击即可运行，CPU 绘制、系统 GDI 显示，不需要额外图形运行库，不加壳。

macOS is ad-hoc signed, not Developer signed/notarized; Windows is unsigned. Linux requires glibc 2.28+, X11/Wayland, OpenGL and a desktop portal for file export. Verify SHA256; do not globally disable system security. / Mac 无开发者签名或公证，Windows 无发行者签名。Linux 需要上述桌面依赖。运行前核对 SHA256，不要全局关闭安全保护。

## Features / 功能

- Fully local generation/recovery verification; no node, website, telemetry or auto-update. / 完全离线生成助记词和验证恢复，无节点、网站、遥测或自动更新。
- English BIP39, 12/24 words, default 24; OS cryptographic randomness, fail closed. / 英文 BIP39，12／24 词，默认 24 词，系统安全随机源失败即停止。
- Mainnet default, TN10 address-format switch; account 0, path `m/44'/111111'/0'/0/i`. / 默认主网，可切换 TN10 地址格式，标准账户 0。
- Handwritten backup and three-word confirmation; retain only public derivation state afterwards. / 手写备份、核对三个指定单词，之后只保留公开派生状态。
- Copy public address; export QR PNG and public recovery TXT. No secret export or wallet persistence. / 复制公开地址、导出二维码及公开恢复说明，不导出秘密、不保存钱包。
- Chinese/English, system-language detection, manual switching and Kaspa K-key icon. / 中英双语、系统语言识别、手动切换及 Kaspa 风格 K 钥匙图标。

## Verification and safety / 验证与安全

18 automated tests and host/Windows-target Clippy passed. The user confirmed running on physical Windows 11 on 2026-10-14, resolving the reported OpenGL startup failure on that machine. Mac Apple Silicon native UI smoke checks passed. Linux, Windows 10 and detailed operation-by-operation Windows validation remain pending. Bundled validation files record build-time status; subsequent confirmation is recorded here and in the repository.

18 项自动测试及主机／Windows 目标 Clippy 通过。用户于 2026-10-14 确认 Windows 11 实体机可运行，该机器的 OpenGL 启动故障已解决。Mac Apple Silicon 已做原生界面检查。Linux、Windows 10 及 Windows 逐项验证仍待完成。包内文件记录构建时状态，后续确认以仓库和本说明为准。

No BIP39 passphrase, legacy derivation, balance lookup, transfer or signing. The app does not disconnect the device's network. Use a clean, permanently offline device and handwritten backups. Malware, screenshots, swap and crash dumps remain risks; complete memory erasure is not guaranteed.

不支持附加口令、旧版派生、查询余额、转账或签名。程序不会替你关闭网络，请使用干净、长期离线设备并手写备份。不能保证防恶意软件、截屏、交换内存或崩溃转储。

**Not independently audited. Verify recovery and use small tests first; never fund public test mnemonics.** / **未经独立安全审计，请先验证恢复、再小额测试，公开测试助记词绝不能用于实际资金。** Dependency audit has 10 informational warnings; see `VALIDATION.md`. / 依赖审计仍有 10 项信息性警告，见验证记录。
