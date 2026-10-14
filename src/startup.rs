//! Startup diagnostics never serialize wallet state or write logs.
use std::sync::atomic::{AtomicBool, Ordering};

pub static APP_CREATED: AtomicBool = AtomicBool::new(false);
static REPORTING: AtomicBool = AtomicBool::new(false);

pub struct Report {
    pub code: &'static str,
    pub details: String,
}

fn bounded_os_details(text: &str) -> String {
    text.chars()
        .filter(|c| !c.is_control() || *c == '\n' || *c == '\t')
        .take(2048)
        .collect()
}

pub fn from_error(error: &eframe::Error, app_created: bool) -> Report {
    // No arbitrary app error or post-initialization error is formatted: these
    // may contain user data in future versions. Only platform startup failures
    // before the home screen are allowed to expose their OS/driver details.
    if app_created {
        return Report { code:"COLD-RUNTIME", details:"The application stopped after initialization. Details withheld to protect wallet data.\n程序初始化后停止；为保护钱包数据，不显示异常详情。".into() };
    }
    let code = match error {
        eframe::Error::Glutin(_) => "COLD-GL-CONTEXT",
        eframe::Error::NoGlutinConfigs(_,_) => "COLD-GL-CONFIG",
        eframe::Error::OpenGL(_) => "COLD-GL-PAINTER",
        eframe::Error::Winit(_) => "COLD-WINDOW",
        eframe::Error::WinitEventLoop(_) => "COLD-EVENT-LOOP",
        eframe::Error::AppCreation(_) => return Report { code:"COLD-APP-CREATE", details:"Application initialization failed. App error details are not displayed.\n程序初始化失败，不显示应用内部异常内容。".into() },
    };
    Report {
        code,
        details: bounded_os_details(&error.to_string()),
    }
}

pub fn message(report: &Report) -> String {
    format!(
        "Kaspa Cold 0.1.0 — Windows startup diagnostic 1\n\nCode / 错误代码: {}\nRenderer / 渲染后端: OpenGL (Glow)\n\n{}\n\n请截图反馈此弹窗，勿包含任何助记词。\nPlease share this error dialog, never a mnemonic.\n\n本程序不记录日志、不上传诊断信息。\nNo logs are written and no diagnostics are transmitted.\n不要安装来历不明的 DLL，也不要全局关闭系统安全保护。\nDo not install unofficial DLLs or disable system security.",
        report.code, report.details
    )
}

pub fn show(report: &Report) {
    // Prevent a dialog backend failure from recursively invoking the panic hook.
    if REPORTING.swap(true, Ordering::SeqCst) {
        return;
    }
    #[cfg(target_os = "windows")]
    rfd::MessageDialog::new()
        .set_title("Kaspa Cold — 启动诊断 / Startup diagnostic")
        .set_level(rfd::MessageLevel::Error)
        .set_buttons(rfd::MessageButtons::Ok)
        .set_description(message(report))
        .show();
    #[cfg(not(target_os = "windows"))]
    eprintln!("{}", message(report));
    REPORTING.store(false, Ordering::SeqCst);
}

pub fn panic_report() -> Report {
    Report { code:"COLD-PANIC", details:"The program encountered an unexpected error. Panic payload, source location and wallet data are deliberately omitted.\n程序发生异常；不显示异常载荷、源码位置或钱包数据。".into() }
}

pub fn self_test_report() -> Report {
    Report { code:"COLD-DIALOG-TEST", details:"This is an intentional diagnostic dialog test, not a wallet failure. No wallet is generated.\n这是主动触发的诊断弹窗测试，不是钱包故障，没有生成钱包。".into() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn app_errors_never_disclose_payload() {
        let error =
            eframe::Error::AppCreation(Box::new(std::io::Error::other("SECRET_PAYLOAD_SENTINEL")));
        let before = from_error(&error, false);
        assert_eq!(before.code, "COLD-APP-CREATE");
        assert!(!message(&before).contains("SECRET_PAYLOAD_SENTINEL"));
        let after = from_error(&error, true);
        assert_eq!(after.code, "COLD-RUNTIME");
        assert!(!message(&after).contains("SECRET_PAYLOAD_SENTINEL"));
    }
    #[test]
    fn details_are_bounded_and_controls_removed() {
        let value = bounded_os_details(&format!("\0\x1b\n{}", "a".repeat(4096)));
        assert_eq!(value.chars().count(), 2048);
        assert!(!value.contains('\0'));
        assert!(!value.contains('\x1b'));
    }
    #[test]
    fn panic_and_self_test_have_no_payload_and_are_distinct() {
        assert_eq!(panic_report().code, "COLD-PANIC");
        assert_eq!(self_test_report().code, "COLD-DIALOG-TEST");
        assert!(message(&self_test_report()).contains("No wallet is generated"));
    }
}
