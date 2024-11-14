mod clipboard;
mod database;
mod tray;
mod window_op;

use crate::clipboard::start_clipboard_monitor;
use crate::window_op::init_window_status;
use database::PasteDB;
use serde_json::json;
use tauri::Manager;
use tauri_plugin_store::StoreExt;
use tray::init_system_tray;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = app
                .get_webview_window("main")
                .expect("没有主窗口")
                .set_focus();
        }))
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
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

            // 初始化窗口状态
            init_window_status(app);

            // 初始化系统托盘信息
            init_system_tray(app)?;
            // 异步的启动一个监控程序
            // 然而，当你在 Tauri 的设置或启动函数中调用
            // start_clipboard_monitor 时，为了确保该异步任务正确地在
            // Tauri 的异步运行时环境中调度，
            // 需要再次使用 tauri::async_runtime::spawn
            tauri::async_runtime::spawn(async {
                start_clipboard_monitor().await;
            });

            let paste_db = PasteDB::form(app);
            paste_db
                .put("test:key:2", json!({"value":vec!["hello","world"]}))
                .expect("TODO: panic message");

            let value = paste_db.get("test:key:2").expect("错了错了");
            println!("{}", value);
            paste_db.close();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
