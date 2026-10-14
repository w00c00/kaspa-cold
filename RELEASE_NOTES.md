# Kaspa Cold v0.1.0 — Offline generator / 离线钱包生成器

## Additional Windows diagnostic / 新增 Windows 诊断包

If the original Windows EXE silently exits, download the extra asset whose name contains `windows-startup-diag1`. It adds native startup error dialogs and `--diagnostic-self-test` (no wallet generated). It writes no logs and transmits no diagnostics. App error/panic payloads and post-initialization details are withheld. **This is a diagnostic build, not a confirmed fix**, and still needs Windows real-device testing. Its source commit is recorded inside `BUILD_COMMIT.txt`; the original v0.1.0 tag and downloads remain unchanged.

如果原版 Windows EXE 双击无反应，请下载名称含 `windows-startup-diag1` 的额外诊断包。启动失败会尝试显示原生错误弹窗；`--diagnostic-self-test` 可主动测试弹窗，不生成钱包。不记录日志、不上传诊断数据，不显示应用异常／panic 载荷及初始化后的详情。**这是诊断版，不代表已修复故障**；仍待 Windows 实机验证。源码提交记录在包内 `BUILD_COMMIT.txt`，原版标签和下载保留不变。

## English

A small native Kaspa mnemonic and receiving-address generator built with Rust and egui. No Electron, WebView or Node.js runtime. This is a **pre-release test build**.

- Fully local generation and recovery verification: no node, website, balance API, telemetry or auto-update calls.
- Standard English BIP39: 12 or 24 words, 24 by default. OS cryptographic randomness; failures stop generation.
- Mainnet by default, with a TN10 address-format switch. Standard account 0 receiving path `m/44'/111111'/0'/0/i`.
- Handwritten backup confirmation before showing a newly generated wallet's address. After confirmation, only public derivation state is retained.
- Public address copying, receiving QR PNG export and public recovery-info TXT export. No mnemonic/private-key export or wallet persistence.
- Chinese and English UI; system-language detection and manual switching.
- Kaspa-inspired K-key icon in the window/header, macOS bundle and Windows executable; Linux icon and optional desktop launcher included.

### Downloads

| Package | Platform | Archive size |
|---|---|---|
| `aarch64-apple-darwin.zip` | macOS Apple Silicon | 6.20 MiB |
| `x86_64-apple-darwin.zip` | macOS Intel | 6.31 MiB |
| `x86_64-pc-windows-gnu.zip` | Windows 10/11 x64 | 5.31 MiB |
| `x86_64-unknown-linux-gnu.tar.gz` | Linux x64, glibc 2.28+ | 5.02 MiB |

Verify the matching `.sha256` before running. macOS builds are ad-hoc signed, not Developer signed/notarized; Windows builds are unsigned. Windows needs a functioning OpenGL driver; Linux needs an X11/Wayland desktop, OpenGL and a desktop portal for file export. No external MinGW runtime DLLs are imported by the Windows package.

### Verification and limitations

13 automated tests passed, including an official Kaspa known-address vector, BIP39, failure paths, backup checks, network-format changes, public export checks and icon decoding. Mac Apple Silicon received native UI smoke checks before the icon integration; the final icon resources were verified in the bundle and executable. Intel core tests passed under translation, not physical Intel hardware. **Windows/Linux are compiled, not real-device verified.**

No BIP39 passphrase, legacy-wallet derivation, balance lookup, transaction construction or signing. The app does not turn off the computer's network. Use a clean, permanently offline device; protect handwritten backups. It cannot prevent malware, screenshots, swap or OS crash dumps, and complete memory erasure is not guaranteed. See `VALIDATION.md` for dependency warnings and remaining tests.

**Not independently audited. Verify recovery and use small tests first; do not use this test build for significant funds.** Never fund the publicly known mnemonics used by source tests.

## 中文

Kaspa Cold 是一个 Rust＋egui 原生离线工具，不捆绑 Electron、WebView 或 Node.js。本次为 **v0.1.0 预发布测试版**。

- 完全本地生成助记词和验证恢复，不连接节点、网站或余额 API，没有遥测、自动更新。
- 12／24 个英文 BIP39 助记词，默认 24 个；使用系统安全随机源，失败即停止。
- 默认主网，可切换 TN10 地址格式；固定标准账户 0，收款路径为 `m/44'/111111'/0'/0/i`。
- 创建后先手写备份、核对三个指定单词，再显示地址；确认后只保留公开派生状态。
- 复制公开地址，导出地址二维码 PNG、公开恢复说明 TXT；不导出助记词和私钥，不保存钱包。
- 中英双语，自动识别系统语言，支持手动切换。
- 集成 Kaspa 风格 K 钥匙图标；Mac 应用、Windows EXE 和 Linux 图标资源均已更新。

四个平台包及 SHA256 校验文件见下方 Release Assets，体积约 5–6.3 MiB。Mac 仅 ad-hoc 签名，没有 Apple 开发者签名／公证；Windows 没有发行者签名。不要全局关闭系统安全保护。

13 项自动化测试通过。Mac 已做原生界面检查，最终图标资源已核对；Intel 核心测试通过转译运行。**Windows／Linux 尚待实机验证，编译成功不等于运行验证通过。**

当前不支持附加口令、旧版钱包派生、查询余额、转账或离线签名。软件不会替你关闭电脑网络，应在干净、长期离线的设备上使用。手写保管助记词，不要截图、上传云盘、发送聊天或联网打印。依赖警告和未完成检查见 `VALIDATION.md`。

**软件未经独立安全审计。请先验证恢复并做小额测试，不要直接存入大额资金。公开测试助记词绝不能用于实际资金。**
