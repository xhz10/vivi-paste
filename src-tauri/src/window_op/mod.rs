use tauri::{App, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcut, GlobalShortcutExt, Shortcut};

pub fn init_window_status(app: &mut App)  {

    let window = app.get_window("main").unwrap();
    window.hide().unwrap(); // 初始化就是隐藏起来


    // Shortcut::new()
    // let short_cut = tauri_plugin_global_shortcut::Builder::new()
    //     .with_handler(move |_, _, _| {
    //         let window = app
    //             .get_window("main")
    //             .unwrap();
    //         if window.is_visible().unwrap() {
    //             window.hide().unwrap();
    //         } else {
    //             window.show().unwrap();
    //         }
    //     }).with_shortcut("Ctrl+Shift+V").unwrap().build();

    // app.global_shortcut()
    //     .register(short_cut).unwrap();
    app.global_shortcut().on_shortcut("Ctrl+Shift+V",move |_, _, _| {
        let window = app
            .get_window("main")
            .unwrap();
        if window.is_visible().unwrap() {
            window.hide().unwrap();
        } else {
            window.show().unwrap();
        }
    }).unwrap();
}