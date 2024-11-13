use tauri::{App, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};


#[cfg(desktop)]
pub fn init_window_status(app: & App) {
    // let ctrl_n_shortcut = Shortcut::new(Some(Modifiers::CONTROL), Code::KeyY);
    let window = app.get_window("main").unwrap();
    window.hide().unwrap(); // 初始化就是隐藏起来


    app.global_shortcut().on_shortcut("Ctrl+Shift+V", move |_,_,event| {
        println!("Ctrl + Y was pressed!");
        if event.state == ShortcutState::Pressed {
            if window.is_visible().unwrap() {
                println!("到这了");
                window.hide().unwrap();
            } else {
                println!("没到这");
                window.show().unwrap();
            }
        }
    }).unwrap();
}