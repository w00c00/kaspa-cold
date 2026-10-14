# Windows startup diagnostic 1 / Windows 启动诊断版 1

This is an additional diagnostic build, not a claim that the Windows startup problem is fixed. The original v0.1.0 downloads remain unchanged. / 这是额外提供的诊断版，不代表已经修复 Windows 启动故障；原版下载保留不变。

## 使用 / Use

1. Download `Kaspa-Cold-0.1.0-windows-startup-diag1-x86_64-pc-windows-gnu.zip` and its matching SHA256. Extract to a local folder. / 下载诊断版及对应校验文件，解压到本地目录。
2. Double-click `kaspa-cold.exe`. Startup errors should produce a native Windows dialog, independent of the OpenGL renderer. / 双击 EXE，启动失败时会尝试显示不依赖 OpenGL 的 Windows 原生错误弹窗。
3. Share the error code and dialog screenshot. Do not include any mnemonic or private-key material. / 发回错误代码和弹窗截图，不要包含助记词或私钥。
4. If it still silently exits, use the dialog self-test in PowerShell from the EXE folder:

```powershell
.\kaspa-cold.exe --diagnostic-self-test
```

Expected code: `COLD-DIALOG-TEST`. It deliberately displays a test message and generates **no wallet**. The self-test returns 0 after closing the dialog. / 预期显示 `COLD-DIALOG-TEST`，是主动测试弹窗，不会生成钱包；关闭弹窗后退出码为 0。

## Codes / 代码

| Code | Meaning / 含义 |
|---|---|
| `COLD-GL-CONFIG` | No usable OpenGL display configuration / 找不到可用 OpenGL 显示配置 |
| `COLD-GL-CONTEXT` | Graphics context initialization failure / 图形上下文初始化失败 |
| `COLD-GL-PAINTER` | OpenGL drawing backend failure / OpenGL 绘制后端失败 |
| `COLD-WINDOW` | Native window creation failure / 原生窗口创建失败 |
| `COLD-EVENT-LOOP` | Native event-loop failure / 原生事件循环失败 |
| `COLD-APP-CREATE` | App initialization failure; payload withheld / 应用初始化失败，不显示内部载荷 |
| `COLD-RUNTIME` | Failure after initialization; details withheld / 初始化后发生错误，不显示详情 |
| `COLD-PANIC` | Unexpected exception; panic payload withheld / 异常，不显示异常载荷 |

## Privacy and limits / 隐私与限制

- The dialog displays bounded OS/driver details only for recognized platform failures before the app home screen is initialized. Arbitrary app errors and post-initialization errors are suppressed. / 只有首页初始化前已知的平台错误显示限长系统或驱动信息；应用内部错误和初始化后的错误不显示载荷。
- Panic messages never include the panic payload, source path, mnemonic, seed or key. / Panic 弹窗不包含异常载荷、源码路径、助记词、种子或密钥。
- No diagnostic log file, automatic clipboard writes, telemetry, network connection or uploads are added. / 不新增诊断日志文件、自动复制、遥测、联网或上传。
- Loader failures occurring before Rust main, forced termination, access violations or a broken native dialog backend may still not show a dialog. / Rust 主函数执行前的加载失败、强制终止、访问冲突或原生弹窗后端损坏仍可能无法显示弹窗。
- Windows 10/11 x64 with functioning OpenGL remains required. Do not install random DLL bundles or disable antivirus/SmartScreen globally. / 仍需要 Windows 10／11 x64 和可用 OpenGL，不要安装不明 DLL 包或全局关闭杀毒／SmartScreen。
- Cross-compilation and automated tests do not substitute for Windows real-device testing. / 交叉编译和自动测试不能替代 Windows 实机验证。
