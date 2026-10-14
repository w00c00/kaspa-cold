#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]
mod icon;
mod startup;
use eframe::egui::{self, Color32, RichText};
use kaspa_cold::{
    core::{self, Error, Network, OsEntropy, PublicWallet, SecretPhrase},
    qr,
};
use zeroize::{Zeroize, Zeroizing};

const TEAL: Color32 = Color32::from_rgb(112, 222, 207);
#[derive(Clone, Copy, PartialEq)]
enum Page {
    Home,
    Safety,
    Phrase,
    Confirm,
    Restore,
    Receive,
}
struct App {
    icon_texture: Option<egui::TextureHandle>,
    zh: bool,
    page: Page,
    network: Network,
    words: usize,
    acknowledged: bool,
    secret: Option<SecretPhrase>,
    public: Option<PublicWallet>,
    restore_text: Zeroizing<String>,
    positions: [usize; 3],
    answers: [Zeroizing<String>; 3],
    visible: bool,
    index: u32,
    address: String,
    texture: Option<egui::TextureHandle>,
    error: Option<Error>,
    notice: Option<bool>,
    discard: bool,
}
impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_theme(egui::Theme::Dark);
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "cjk".into(),
            egui::FontData::from_static(include_bytes!("../assets/cjk.ttf")).into(),
        );
        fonts
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .push("cjk".into());
        cc.egui_ctx.set_fonts(fonts);
        let mut visuals = egui::Visuals::dark();
        visuals.panel_fill = Color32::from_rgb(15, 24, 29);
        visuals.window_fill = Color32::from_rgb(21, 33, 39);
        visuals.selection.bg_fill = Color32::from_rgb(38, 105, 100);
        cc.egui_ctx.set_visuals(visuals);
        let mut style = (*cc.egui_ctx.style()).clone();
        style.spacing.item_spacing = egui::vec2(12., 12.);
        style.spacing.button_padding = egui::vec2(16., 10.);
        style
            .text_styles
            .insert(egui::TextStyle::Body, egui::FontId::proportional(17.));
        cc.egui_ctx.set_style(style);
        let mut app = Self::empty(
            sys_locale::get_locale().is_some_and(|s| s.to_lowercase().starts_with("zh")),
        );
        let icon = icon::app_icon();
        app.icon_texture = Some(cc.egui_ctx.load_texture(
            "app-brand",
            egui::ColorImage::from_rgba_unmultiplied(
                [icon.width as usize, icon.height as usize],
                &icon.rgba,
            ),
            egui::TextureOptions::LINEAR,
        ));
        app
    }
    fn empty(zh: bool) -> Self {
        Self {
            icon_texture: None,
            zh,
            page: Page::Home,
            network: Network::Mainnet,
            words: 24,
            acknowledged: false,
            secret: None,
            public: None,
            restore_text: Zeroizing::new(String::new()),
            positions: [0; 3],
            answers: std::array::from_fn(|_| Zeroizing::new(String::new())),
            visible: false,
            index: 0,
            address: String::new(),
            texture: None,
            error: None,
            notice: None,
            discard: false,
        }
    }
    fn t<'a>(&self, zh: &'a str, en: &'a str) -> &'a str {
        if self.zh { zh } else { en }
    }
    fn clear_secrets(&mut self, ctx: &egui::Context) {
        self.secret = None;
        self.restore_text.zeroize();
        for a in &mut self.answers {
            a.zeroize();
        }
        ctx.data_mut(|d| d.remove_by_type::<egui::text_edit::TextEditState>());
        self.visible = false;
    }
    fn reset(&mut self, ctx: &egui::Context) {
        self.clear_secrets(ctx);
        self.public = None;
        self.address.clear();
        self.texture = None;
        self.error = None;
        self.notice = None;
        self.page = Page::Home;
        self.index = 0;
        self.acknowledged = false;
        self.discard = false;
    }
    fn refresh_address(&mut self, ctx: &egui::Context) {
        let result = self
            .public
            .as_ref()
            .ok_or(Error::Derivation)
            .and_then(|w| w.address(self.network, self.index));
        match result {
            Ok(address) => {
                self.texture = qr::address_qr(&address).ok().map(|q| {
                    let image = egui::ColorImage::from_gray([q.size, q.size], &q.pixels);
                    ctx.load_texture("public-address", image, egui::TextureOptions::NEAREST)
                });
                self.address = address;
                self.error = None;
                self.notice = None;
            }
            Err(e) => {
                self.address.clear();
                self.texture = None;
                self.error = Some(e);
            }
        }
    }
    fn accept_public(&mut self, result: core::Result<PublicWallet>, ctx: &egui::Context) {
        match result {
            Ok(w) => {
                self.public = Some(w);
                self.clear_secrets(ctx);
                self.page = Page::Receive;
                self.refresh_address(ctx);
            }
            Err(e) => self.error = Some(e),
        }
    }
    fn warning(&self, ui: &mut egui::Ui) {
        ui.label(
            RichText::new(self.t(
                "请在干净、长期离线的设备上使用。",
                "Use a clean, permanently offline device.",
            ))
            .color(TEAL),
        );
        ui.label(self.t("助记词控制全部资金。手写备份；不要截图、上传云盘、发送聊天或联网打印。", "Your mnemonic controls all funds. Write it down. Never screenshot, cloud-sync, message, or print it on a network printer."));
        ui.label(self.t("本软件不联网，但不能防御恶意系统、截屏、交换内存或剪贴板窃取。", "This app makes no network requests. It cannot protect against malware, screenshots, swap, or clipboard theft."));
        ui.label(self.t("仅支持标准账户 0、英文助记词、无附加口令；不支持旧版钱包派生。", "Standard account 0, English mnemonic, NO BIP39 passphrase. Legacy derivation is not supported."));
    }
    fn error_text(&self, error: Error) -> &str {
        match error {
            Error::Random => self.t(
                "系统安全随机源失败，已停止生成。",
                "Secure system randomness failed. Generation stopped.",
            ),
            Error::WordCount => self.t(
                "仅支持 12 或 24 个英文助记词。",
                "Only 12 or 24 English words are supported.",
            ),
            Error::InvalidMnemonic => self.t(
                "助记词无效：请检查词表、顺序和校验和。",
                "Invalid mnemonic: check words, order, and checksum.",
            ),
            Error::BackupMismatch => self.t(
                "备份确认不匹配，请检查指定序号的单词。",
                "Backup confirmation does not match. Check the requested word positions.",
            ),
            Error::Index => self.t("地址序号超出范围。", "Address index is out of range."),
            Error::Derivation => self.t(
                "地址派生失败，未保存任何秘密。",
                "Address derivation failed. No secrets were saved.",
            ),
        }
    }
    fn export(&mut self, png: bool) {
        let data = if png {
            qr::address_qr(&self.address)
                .and_then(|q| qr::png_bytes(&q))
                .ok()
        } else {
            self.public
                .as_ref()
                .and_then(|w| w.receipt(self.network, self.index).ok())
                .map(String::into_bytes)
        };
        let Some(data) = data else {
            self.notice = Some(false);
            return;
        };
        let filename = format!(
            "kaspa-cold-{}-{}.{}",
            if self.network == Network::Mainnet {
                "mainnet"
            } else {
                "tn10"
            },
            self.index,
            if png { "png" } else { "txt" }
        );
        let dialog = rfd::FileDialog::new()
            .set_title(self.t("仅导出公开信息", "Export PUBLIC information only"))
            .set_file_name(filename);
        if let Some(path) = dialog.save_file() {
            self.notice = Some(std::fs::write(path, data).is_ok());
        }
    }
    fn content(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        match self.page {
            Page::Home => {
                ui.add_space(20.);
                ui.heading(self.t(
                    "离线生成，秘密留在离线端。",
                    "Generate offline. Keep secrets offline.",
                ));
                ui.label(self.t(
                    "不连接节点 · 不保存钱包 · 不查询余额",
                    "No nodes · No wallet storage · No balance queries",
                ));
                ui.add_space(16.);
                self.warning(ui);
                ui.add_space(20.);
                ui.horizontal(|ui| {
                    if ui
                        .button(self.t("＋ 创建新钱包", "+ Create wallet"))
                        .clicked()
                    {
                        self.page = Page::Safety;
                    }
                    if ui
                        .button(self.t("离线恢复验证", "Verify recovery offline"))
                        .clicked()
                    {
                        self.page = Page::Restore;
                    }
                });
                ui.add_space(20.);
                ui.label(self.t("这不是硬件钱包，也不提供转账签名。收到资金前，请先验证恢复，再做小额测试。", "Not a hardware wallet. No transaction signing. Verify recovery and test with a small amount before funding."));
            }
            Page::Safety => {
                ui.heading(self.t("创建前检查", "Before you generate"));
                self.warning(ui);
                ui.horizontal(|ui| {
                    ui.label(self.t("助记词长度", "Mnemonic length"));
                    ui.selectable_value(&mut self.words, 24, "24");
                    ui.selectable_value(&mut self.words, 12, "12");
                });
                let label = self.t(
                    "我已断网并准备手写备份，理解系统仍可能泄露秘密。",
                    "I am offline, ready to write my backup, and understand the device risks.",
                );
                ui.checkbox(&mut self.acknowledged, label);
                if ui
                    .add_enabled(
                        self.acknowledged,
                        egui::Button::new(self.t("生成助记词", "Generate mnemonic")),
                    )
                    .clicked()
                {
                    match core::generate(self.words, &mut OsEntropy) {
                        Ok(secret) => match core::challenge(secret.count(), &mut OsEntropy) {
                            Ok(positions) => {
                                self.positions = positions;
                                self.secret = Some(secret);
                                self.page = Page::Phrase;
                                self.visible = true;
                                self.error = None;
                            }
                            Err(e) => self.error = Some(e),
                        },
                        Err(e) => self.error = Some(e),
                    }
                }
            }
            Page::Phrase => {
                ui.heading(self.t("手写保存助记词", "Write down your mnemonic"));
                ui.label(self.t("按顺序记录。备份确认后，程序会丢弃秘密，只保留公开地址派生信息。", "Write every word in order. After confirmation, the app discards secrets and keeps only public address derivation data."));
                if self.visible && ctx.input(|i| i.focused) {
                    if let Some(secret) = &self.secret {
                        egui::Grid::new("phrase")
                            .num_columns(4)
                            .spacing([24., 12.])
                            .show(ui, |ui| {
                                for (i, word) in secret.phrase().split(' ').enumerate() {
                                    ui.add(
                                        egui::Label::new(
                                            RichText::new(format!("{:02}  {}", i + 1, word))
                                                .monospace()
                                                .size(18.),
                                        )
                                        .selectable(false),
                                    );
                                    if i % 4 == 3 {
                                        ui.end_row();
                                    }
                                }
                            });
                    }
                } else if ui
                    .button(self.t(
                        "秘密已隐藏，点击重新显示",
                        "Secrets hidden — click to reveal",
                    ))
                    .clicked()
                {
                    self.visible = true;
                }
                ui.add_space(16.);
                if ui
                    .button(self.t("我已手写备份，继续核对", "Backup written — verify it"))
                    .clicked()
                {
                    self.page = Page::Confirm;
                    self.visible = false;
                }
            }
            Page::Confirm => {
                ui.heading(self.t("确认备份", "Confirm your backup"));
                ui.label(self.t(
                    "输入下列序号的单词。不要从电子备份复制。",
                    "Enter the requested words from your handwritten backup.",
                ));
                // Password inputs are always masked, even when the window is unfocused.
                {
                    for n in 0..3 {
                        ui.label(format!(
                            "{} {}",
                            self.t("单词", "Word"),
                            self.positions[n] + 1
                        ));
                        let mut output = egui::TextEdit::singleline(&mut *self.answers[n])
                            .password(true)
                            .char_limit(16)
                            .id_salt(("answer", n))
                            .show(ui);
                        output.state.clear_undoer();
                        output.state.store(ctx, output.response.id);
                    }
                    if ui
                        .button(self.t("确认并显示地址", "Confirm and show address"))
                        .clicked()
                    {
                        let result = self.secret.as_ref().ok_or(Error::Derivation).and_then(|s| {
                            s.confirm(&self.positions, &self.answers)?;
                            s.public_wallet()
                        });
                        self.accept_public(result, ctx);
                    }
                }
                if ui
                    .button(self.t("返回助记词", "Back to mnemonic"))
                    .clicked()
                {
                    self.page = Page::Phrase;
                    self.error = None;
                }
            }
            Page::Restore => {
                ui.heading(self.t("离线恢复验证", "Verify recovery offline"));
                self.warning(ui);
                ui.label(self.t(
                    "输入 12 或 24 个英文单词；可粘贴，但不会保存。",
                    "Enter 12 or 24 English words. Paste is allowed; nothing is saved.",
                ));
                {
                    let mut output = egui::TextEdit::singleline(&mut *self.restore_text)
                        .password(true)
                        .char_limit(1024)
                        .desired_width(f32::INFINITY)
                        .hint_text("BIP39")
                        .id_salt("restore")
                        .show(ui);
                    output.state.clear_undoer();
                    output.state.store(ctx, output.response.id);
                }
                let label = self.t(
                    "我确认原钱包没有 BIP39 附加口令，且使用标准账户 0。",
                    "My original wallet has NO BIP39 passphrase and uses standard account 0.",
                );
                ui.checkbox(&mut self.acknowledged, label);
                if ui
                    .add_enabled(
                        self.acknowledged && !self.restore_text.is_empty(),
                        egui::Button::new(self.t("验证地址", "Verify address")),
                    )
                    .clicked()
                {
                    let result = core::restore(&self.restore_text).and_then(|s| s.public_wallet());
                    self.accept_public(result, ctx);
                }
            }
            Page::Receive => {
                ui.heading(self.t("公开收款信息", "Public receiving information"));
                ui.label(
                    RichText::new(self.t(
                        "秘密已从本次会话丢弃；请保管好手写备份。",
                        "Secrets discarded from this session. Keep your handwritten backup safe.",
                    ))
                    .color(TEAL),
                );
                ui.horizontal(|ui| {
                    ui.label(self.t("收款地址序号", "Receiving address index"));
                    let response = ui.add(
                        egui::DragValue::new(&mut self.index)
                            .range(0..=2_147_483_647_u32)
                            .speed(1.),
                    );
                    if response.changed() {
                        self.refresh_address(ctx);
                    }
                });
                ui.label(format!("m/44'/111111'/0'/0/{}", self.index));
                if let Some(texture) = &self.texture {
                    ui.add(egui::Image::new(texture).fit_to_exact_size(egui::vec2(210., 210.)));
                }
                ui.add(egui::Label::new(RichText::new(&self.address).monospace().size(15.)).wrap());
                if !self.address.is_empty() {
                    ui.horizontal(|ui| {
                        if ui.button(self.t("复制地址", "Copy address")).clicked() {
                            ctx.copy_text(self.address.clone());
                        }
                        if ui
                            .button(self.t("导出地址二维码", "Export address QR"))
                            .clicked()
                        {
                            self.export(true);
                        }
                        if ui
                            .button(self.t("导出公开恢复说明", "Export public recovery info"))
                            .clicked()
                        {
                            self.export(false);
                        }
                    });
                }
                ui.label(self.t("没有保存钱包。退出后再次使用，需要手写助记词恢复。", "No wallet is stored. After closing, your handwritten mnemonic is needed to recover."));
                if self.network == Network::Tn10 {
                    ui.label(self.t("TN10：kaspatest 前缀也用于其他测试网，请勿当作主网地址。", "TN10: kaspatest is shared with other testnets. Never use it as a mainnet address."));
                }
            }
        }
        if let Some(e) = self.error {
            ui.label(RichText::new(self.error_text(e)).color(Color32::from_rgb(255, 150, 145)));
        }
        if let Some(ok) = self.notice {
            ui.label(if ok {
                self.t("公开信息已导出。", "Public information exported.")
            } else {
                self.t(
                    "导出失败，请检查目标目录的写入权限。",
                    "Export failed. Check destination permissions.",
                )
            });
        }
    }
}
impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        if !ctx.input(|i| i.focused) {
            self.visible = false;
        }
        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if let Some(icon) = &self.icon_texture {
                    ui.add(egui::Image::new(icon).fit_to_exact_size(egui::vec2(36., 36.)));
                }
                ui.label(RichText::new("KASPA COLD").color(TEAL).size(24.).strong());
                ui.label(if cfg!(target_os = "windows") {
                    "0.1.0 · DIAG 1 · OFFLINE"
                } else {
                    "0.1.0 · OFFLINE"
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui
                        .button(if self.zh { "English" } else { "中文" })
                        .clicked()
                    {
                        self.zh = !self.zh;
                    }
                    let before = self.network;
                    ui.selectable_value(&mut self.network, Network::Tn10, "TN10");
                    ui.selectable_value(
                        &mut self.network,
                        Network::Mainnet,
                        if self.zh { "主网" } else { "Mainnet" },
                    );
                    if before != self.network && self.public.is_some() {
                        self.refresh_address(ctx);
                    }
                });
            });
        });
        egui::TopBottomPanel::bottom("footer").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.small(self.t(
                    "本地离线运行 · 无存储 · 无签名",
                    "Local only · No storage · No signing",
                ));
                if self.page != Page::Home
                    && ui
                        .button(self.t("清除／返回首页", "Clear / Home"))
                        .clicked()
                {
                    self.discard = true;
                }
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add_space(18.);
                self.content(ui, ctx);
            });
        });
        if self.discard {
            egui::Window::new(self.t("清除本次会话？", "Clear this session?"))
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label(self.t(
                        "未备份的助记词将无法找回。程序没有保存任何钱包。",
                        "An unbacked mnemonic cannot be recovered. No wallet is stored.",
                    ));
                    ui.horizontal(|ui| {
                        if ui.button(self.t("确认清除", "Clear session")).clicked() {
                            self.reset(ctx);
                        }
                        if ui.button(self.t("取消", "Cancel")).clicked() {
                            self.discard = false;
                        }
                    });
                });
        }
        // Never emit clipboard writes on a secret-bearing screen.
        if self.page != Page::Receive {
            ctx.output_mut(|o| {
                o.commands.retain(|c| {
                    !matches!(
                        c,
                        egui::OutputCommand::CopyText(_) | egui::OutputCommand::CopyImage(_)
                    )
                })
            });
        }
    }
    fn on_exit(&mut self, _: Option<&eframe::glow::Context>) {
        self.secret = None;
        self.restore_text.zeroize();
        for answer in &mut self.answers {
            answer.zeroize();
        }
    }
}
fn run() -> eframe::Result {
    let options = eframe::NativeOptions {
        renderer: eframe::Renderer::Glow,
        viewport: egui::ViewportBuilder::default()
            .with_app_id("org.kaspa.cold")
            .with_icon(icon::app_icon())
            .with_inner_size([850., 720.])
            .with_min_inner_size([700., 620.]),
        ..Default::default()
    };
    eframe::run_native(
        "Kaspa Cold",
        options,
        Box::new(|cc| {
            let app = App::new(cc);
            startup::APP_CREATED.store(true, std::sync::atomic::Ordering::SeqCst);
            Ok(Box::new(app))
        }),
    )
}

fn main() -> std::process::ExitCode {
    std::panic::set_hook(Box::new(|_| startup::show(&startup::panic_report())));
    if std::env::args_os().any(|arg| arg == "--diagnostic-self-test") {
        startup::show(&startup::self_test_report());
        return std::process::ExitCode::SUCCESS;
    }
    match run() {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(error) => {
            let report = startup::from_error(
                &error,
                startup::APP_CREATED.load(std::sync::atomic::Ordering::SeqCst),
            );
            startup::show(&report);
            std::process::ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const VECTOR: &str = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
    #[test]
    fn defaults_and_secret_discard() {
        let ctx = egui::Context::default();
        let mut app = App::empty(true);
        assert_eq!(app.words, 24);
        assert_eq!(app.network, Network::Mainnet);
        app.secret = Some(core::restore(VECTOR).unwrap());
        app.restore_text.push_str(VECTOR);
        app.answers[0].push_str("abandon");
        let mut result = Some(app.secret.as_ref().unwrap().public_wallet());
        // Texture creation needs an active egui pass.
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            if let Some(result) = result.take() {
                app.accept_public(result, ctx);
            }
        });
        assert!(app.secret.is_none());
        assert!(app.restore_text.is_empty());
        assert!(app.answers.iter().all(|a| a.is_empty()));
        assert!(app.page == Page::Receive);
        assert!(app.address.starts_with("kaspa:"));
        app.reset(&ctx);
        assert!(app.public.is_none());
        assert!(app.address.is_empty());
        assert!(app.page == Page::Home);
        assert!(!app.acknowledged);
    }
    #[test]
    fn both_languages_and_all_pages_render() {
        let ctx = egui::Context::default();
        for zh in [true, false] {
            for page in [
                Page::Home,
                Page::Safety,
                Page::Phrase,
                Page::Confirm,
                Page::Restore,
                Page::Receive,
            ] {
                let mut app = App::empty(zh);
                app.page = page;
                app.secret = Some(core::restore(VECTOR).unwrap());
                app.visible = true;
                let mut input = egui::RawInput {
                    focused: true,
                    ..Default::default()
                };
                input.screen_rect = Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(850., 720.),
                ));
                let output = ctx.run(input, |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| app.content(ui, ctx));
                });
                assert!(!output.shapes.is_empty());
                assert!(
                    !output
                        .platform_output
                        .commands
                        .iter()
                        .any(|c| matches!(c, egui::OutputCommand::CopyText(_)))
                );
            }
        }
    }
    #[test]
    fn network_change_keeps_public_identity() {
        let ctx = egui::Context::default();
        let mut app = App::empty(false);
        let mut result = Some(core::restore(VECTOR).unwrap().public_wallet());
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            if let Some(result) = result.take() {
                app.accept_public(result, ctx);
            }
        });
        let main = app.address.clone();
        app.network = Network::Tn10;
        let _ = ctx.run(egui::RawInput::default(), |ctx| app.refresh_address(ctx));
        assert!(app.address.starts_with("kaspatest:"));
        assert!(app.secret.is_none());
        app.network = Network::Mainnet;
        let _ = ctx.run(egui::RawInput::default(), |ctx| app.refresh_address(ctx));
        assert_eq!(app.address, main);
    }
    #[test]
    fn errors_do_not_advance_or_contain_input() {
        let mut app = App::empty(false);
        let ctx = egui::Context::default();
        app.page = Page::Restore;
        app.restore_text.push_str("invalid-secret-input");
        app.accept_public(Err(Error::InvalidMnemonic), &ctx);
        assert!(app.page == Page::Restore);
        assert!(app.public.is_none());
        assert!(
            !app.error_text(Error::InvalidMnemonic)
                .contains(&*app.restore_text)
        );
    }
}
