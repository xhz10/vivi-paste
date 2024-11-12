use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::App;

/// 初始化系统的托盘操作
/// 参数是一个可变的App 的 引用类型
pub fn init_system_tray(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    // 定义托盘菜单都有啥
    // 1. 退出功能
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

    // 2. 添加到菜单里面  添加的是引用
    let menu = Menu::with_items(app, &[&quit_i])?;

    // 定义系统托盘
    TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu_on_left_click(false) // 不可以点左键哦
        .menu(&menu)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "quit" => {
                println!("Quitting...");
                app.exit(0);
            }
            _ => {
                println!("未发现的事件emmmm: {:?}", event.id);
            }
        })
        .build(app)?;

    Ok(())
}
