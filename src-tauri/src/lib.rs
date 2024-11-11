mod tray;

use tauri::tray::TrayIconBuilder;
use tauri_plugin_clipboard_manager::ClipboardExt;
use tray::init_system_tray;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            // 初始化系统托盘信息
            init_system_tray(app)?;

            // 看看剪切板的功能
            // app.clipboard()
            //     .write_text("哈哈哈哈，剪切板".to_string())
            //     .unwrap();
            let content = app.clipboard().read_text();
            println!("{:?}", content);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
