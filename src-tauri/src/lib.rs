mod tray;
mod clipboard;

use std::sync::Arc;
use tauri::tray::TrayIconBuilder;
use tauri_plugin_clipboard_manager::ClipboardExt;
use tokio::spawn;
use tokio::sync::Mutex;
use tray::init_system_tray;
use crate::clipboard::start_clipboard_monitor;

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
            // 异步的启动一个监控程序
            // 然而，当你在 Tauri 的设置或启动函数中调用
            // start_clipboard_monitor 时，为了确保该异步任务正确地在
            // Tauri 的异步运行时环境中调度，
            // 需要再次使用 tauri::async_runtime::spawn
            tauri::async_runtime::spawn(async {
                start_clipboard_monitor().await;
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
