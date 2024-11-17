use copypasta::{ClipboardContext, ClipboardProvider};
use enigo::{
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Settings,
};
use std::sync::Arc;
use std::time::Duration;
use tauri::Window;
use tokio::sync::Mutex;
use tokio::time::sleep;

use crate::{
    database::get_instance,
    event::{send_message, VIVIEvent},
};

// 写内容到剪切板
pub fn write_to_clipboard(info: String, window: Window) {
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(info).unwrap();
    println!("写了内容到剪切板了");
    // 关闭焦点
    // 隐藏一下
    window.hide().unwrap();
    // 隐藏完毕之后操作键盘按钮
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    // 模拟Ctrl + V粘贴操作
    enigo.key(Key::Control, Press).unwrap();
    enigo.key(Key::Unicode('v'), Click).unwrap();
    enigo.key(Key::Control, Release).unwrap();
}

/// 放入一个多线程安全的列表指针
pub async fn start_clipboard_monitor(paste_list: Arc<Mutex<Vec<String>>>, window: Arc<Window>) {
    // 线程安全的可以在多个任务之间共享的字符串
    // Arc 是原子引用计数器：允许多个线程安全地共享该值
    // Mutex 确保同时只有一个任务可以修改字符串
    // let last_content = Arc::new(Mutex::new(String::new()));
    // tokio::spawn 启动一个新的异步任务，不会阻塞当前线程，任务会在后台异步运行
    let mut refresh_list = Vec::new();
    let clone_list = Arc::clone(&paste_list);
    let pre_list = clone_list.lock().await;
    pre_list.iter().for_each(|i| {
        refresh_list.push(i.to_string());
    });
    tokio::spawn({
        // 克隆一个Arc指针使新的任务共享last_content
        // let last_content = Arc::clone(&last_content);
        // 克隆一个智能指针
        let paste_list = Arc::clone(&paste_list);
        // 克隆一个安全的windows指针
        let safe_window = window.clone();
        // async move 定义一个闭包 move 表示该闭包会获取它捕获的值的所有权
        async move {
            // 创建一个剪切板的上下文ctx 用于访问系统剪切板
            let mut ctx = ClipboardContext::new().unwrap();
            loop {
                // 无线循环
                // match 读取剪切板的内容的过程
                // OK 代表读取成功获取到当前剪切板的值
                if let Ok(current_content) = ctx.get_contents() {
                    // 处理剪切板的内容
                    // 获取到Mutex的锁来保证修改是安全的
                    // let mut last = last_content.lock().await;
                    let mut paste = paste_list.lock().await;
                    // 当前剪切板的内容如果不存在就刷新
                    if !paste.contains(&current_content) {
                        paste.insert(0, current_content.clone());
                        refresh_list.insert(0, current_content.clone());
                        // 需要发送一个内置的事件出去
                        let db_instance = get_instance();
                        db_instance.refresh_db(&refresh_list);
                        send_message(VIVIEvent::Test, safe_window.clone());
                    }
                }
                sleep(Duration::from_secs(1)).await;
            }
        }
    });
}
