use eframe::egui;
use std::default::Default;

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
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Document Editor");
            
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
            
            // テキストエディタ領域
            let text_edit = egui::TextEdit::multiline(&mut self.document_text)
                .desired_width(f32::INFINITY)
                .desired_rows(30);
            ui.add(text_edit);
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