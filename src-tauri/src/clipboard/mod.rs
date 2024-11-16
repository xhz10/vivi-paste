use copypasta::{ClipboardContext, ClipboardProvider};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::sleep;

/// 放入一个多线程安全的列表指针
pub async fn start_clipboard_monitor(paste_list: Arc<Mutex<Vec<String>>>) {
    // 线程安全的可以在多个任务之间共享的字符串
    // Arc 是原子引用计数器：允许多个线程安全地共享该值
    // Mutex 确保同时只有一个任务可以修改字符串
    let last_content = Arc::new(Mutex::new(String::new()));
    // 获取db数据

    // tokio::spawn 启动一个新的异步任务，不会阻塞当前线程，任务会在后台异步运行
    tokio::spawn({
        // 克隆一个Arc指针使新的任务共享last_content
        let last_content = Arc::clone(&last_content);
        // 克隆一个智能指针
        let paste_list = Arc::clone(&paste_list);
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
                    let mut last = last_content.lock().await;
                    let mut paste = paste_list.lock().await;
                    // 检查当前内容与上一次内容是否不同
                    if *last != current_content {
                        *last = current_content.clone();
                        paste.insert(0, current_content);

                        // 输出一下
                        // paste.iter().for_each(|s| {
                        //     println!("输出一下结果: {}", s);
                        // });
                    }
                }
                sleep(Duration::from_secs(1)).await;
            }
        }
    });
}
