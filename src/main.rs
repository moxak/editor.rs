// Include the Slint UI definitions and bindings
slint::include_modules!();
// Import SharedString for text properties and rfd for file dialogs
use slint::SharedString;
use rfd::FileDialog;

fn count_lines(text: &str) -> usize {
    // Count the number of lines in the text
    text.lines().count()
}

fn get_current_path() -> String {
    // Get the current working directory
    std::env::current_dir()
        .map(|path| path.to_string_lossy().to_string())
        .unwrap_or_else(|_| "Unknown".to_string())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize Slint backend and create the TextEditor window
    let app = TextEditor::new()?;

    // Set the initial title of the window
    app.set_file_path(SharedString::from(get_current_path() + "/" + "Untitled"));

    // Count line numbers and set them in the UI
    {
        let editor_handle = app.as_weak();
        // Bind to the Slint callback for counting lines
        app.on_count_lines(move || {
            // Debug log: Count lines callback received
            eprintln!("[TextEditor] Count lines callback triggered");
            if let Some(editor) = editor_handle.upgrade() {
                // Get the current document content and count lines
                let content = editor.get_document_content().to_string();
                let line_count = count_lines(&content);
                // Update the line count in the UI
                editor.set_line_count(line_count as i32);
            }
        });
    }
    
    // Connect the Open button to show a file-open dialog and load content
    {
        let editor_handle = app.as_weak();
        // Bind to the Slint callback for opening a file
        app.on_open_file(move || {
            // Debug log: Open callback received
            eprintln!("[TextEditor] Open callback triggered");
            if let Some(editor) = editor_handle.upgrade() {
                // If a path is already set, open that file directly
                let current_path = editor.get_file_path().to_string();
                if !current_path.is_empty() {
                    if let Ok(content) = std::fs::read_to_string(&current_path) {
                        editor.set_document_content(SharedString::from(content));
                        // Update title from the path
                        if let Some(name) = std::path::Path::new(&current_path)
                            .file_name()
                            .and_then(|n| n.to_str().map(|s| s.to_string()))
                        {
                            editor.set_document_title(SharedString::from(name));
                        }
                    }
                } else if let Some(path_buf) = FileDialog::new().pick_file() {
                    // Show file dialog result and load file
                    if let Ok(content) = std::fs::read_to_string(&path_buf) {
                        editor.set_document_content(SharedString::from(content));
                        // Store the file path
                        if let Some(path_str) = path_buf.to_str() {
                            editor.set_file_path(SharedString::from(path_str));
                        }
                        // Update the title to the file name
                        if let Some(name) = path_buf.file_name()
                            .and_then(|n| n.to_str().map(|s| s.to_string()))
                        {
                            editor.set_document_title(SharedString::from(name));
                        }
                    }
                } else {
                    // FileDialog backend not available or user canceled; prompt manual entry
                    editor.set_document_content(SharedString::from(
                        "⚠️ No file dialog available. Please install zenity or kdialog, or enter a file path above and click Open again."
                    ));
                }
            }
        });
    }
    // Connect the Save button to show a file-save dialog and write content
    {
        let editor_handle = app.as_weak();
        // Bind to the Slint callback for saving a file
        app.on_save_file(move || {
            // Debug log: Save callback received
            eprintln!("[TextEditor] Save callback triggered");
            if let Some(editor) = editor_handle.upgrade() {
                // Determine if we already have a file path
                let current_path = editor.get_file_path().to_string();
                if !current_path.is_empty() {
                    // Save to existing path
                    let content = editor.get_document_content().to_string();
                    if let Err(e) = std::fs::write(&current_path, content) {
                        eprintln!("Failed to save file: {}", e);
                    }
                } else if let Some(path_buf) = FileDialog::new().save_file() {
                    let content = editor.get_document_content().to_string();
                    if let Err(e) = std::fs::write(&path_buf, content) {
                        eprintln!("Failed to save file: {}", e);
                    } else {
                        // Store the new path
                        if let Some(path_str) = path_buf.to_str() {
                            editor.set_file_path(SharedString::from(path_str));
                        }
                        // Update the title to file name
                        if let Some(name) = path_buf.file_name()
                            .and_then(|n| n.to_str().map(|s| s.to_string()))
                        {
                            editor.set_document_title(SharedString::from(name));
                        }
                    }
                } else {
                    // FileDialog backend missing or canceled; prompt manual entry
                    editor.set_document_content(SharedString::from(
                        "⚠️ No file dialog available. Please install zenity or kdialog, or enter a file path above and click Save again."
                    ));
                }
            }
        });
    }

    // ウィンドウを表示＆イベントループ開始
    app.run()?;
    Ok(())
}
