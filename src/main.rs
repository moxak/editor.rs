use eframe::egui;
use std::default::Default;
use epaint;

struct DocumentApp {
    document_text: String,
    file_path: Option<String>,
}

impl DocumentApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // フォント設定
        let mut fonts = egui::FontDefinitions::default();
        
        // 日本語フォントを追加
        // システムにインストールされているフォントのパスを指定するか、
        // アプリにバンドルするフォントファイルのデータを使用します
        
        // フォントファイルをバンドルする場合
        let font_data = egui::FontData::from_static(include_bytes!("../assets/IBMPlexSansJP-Regular.ttf"));
        
        // フォントファミリーに追加
        fonts.font_data.insert("japanese_font".to_owned(), font_data.into());
        
        // ファミリーに追加 (proportional = 可変幅フォント)
        fonts.families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "japanese_font".to_owned()); // 優先度を高くするために先頭に挿入
        
        // 固定幅フォントファミリーにも追加
        fonts.families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .push("japanese_font".to_owned());
        
        // フォント設定を適用
        cc.egui_ctx.set_fonts(fonts);
        
        Self {
            document_text: String::new(),
            file_path: None,
        }
    }
}

impl eframe::App for DocumentApp {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Configuration
        ctx.set_visuals(egui::Visuals::light());

        // Configuration for central panel
        let frame = egui::containers::Frame {
            inner_margin: epaint::Margin::same(0),
            outer_margin: epaint::Margin::same(0),
            fill: ctx.style().visuals.extreme_bg_color,
            stroke: egui::Stroke { width: 0.0, color: egui::Color32::TRANSPARENT },
            corner_radius: egui::CornerRadius { nw: 0, ne: 0, sw: 0, se: 0 },
            ..Default::default()
        };

        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            // メニューバー
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New").clicked() {
                        self.document_text.clear();
                        self.file_path = None;
                    }
                    if ui.button("Open").clicked() {
                        // ファイルを開く処理
                    }
                    if ui.button("Save").clicked() {
                        // ファイルを保存する処理
                    }
                });
                ui.menu_button("Edit", |ui| {
                    if ui.button("Cut").clicked() {
                        // カット処理
                    }
                    if ui.button("Copy").clicked() {
                        // コピー処理
                    }
                    if ui.button("Paste").clicked() {
                        // ペースト処理
                    }
                });
            });

            let editor_area = egui::TextEdit::multiline(&mut self.document_text)
                .font(egui::TextStyle::Monospace) // for cursor height
                .code_editor()
                .desired_width(f32::INFINITY)
                .desired_rows(10)
                .lock_focus(true)
                .desired_width(f32::INFINITY);
            ui.add(editor_area);

            
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let mut options = eframe::NativeOptions::default();
    options.viewport.inner_size = Some(egui::vec2(800.0, 600.0));
    
    eframe::run_native(
        "Document Editor",
        options,
        Box::new(|_cc| Ok(Box::new(DocumentApp::new(_cc)))),
    )
}