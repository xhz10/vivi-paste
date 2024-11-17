use device_query::{DeviceQuery, DeviceState, Keycode};
use tauri::{App, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

#[cfg(desktop)]
pub fn init_window_status(app: &App) {
    let window = app.get_window("main").unwrap();
    window.hide().unwrap(); // 初始化就是隐藏起来

    app.global_shortcut()
        .on_shortcut("Ctrl+Shift+V", move |_, _, event| {
            if event.state == ShortcutState::Pressed {
                if window.is_visible().unwrap() {
                    window.hide().unwrap();
                } else {
                    window.show().unwrap();
                    window.set_focus().unwrap();
                    window.set_always_on_top(true).unwrap();
                }
            }
        })
        .unwrap();
}
