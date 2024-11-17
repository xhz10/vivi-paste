use crate::{clipboard::write_to_clipboard, database::get_instance};
use serde::Serialize;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Serialize)]
pub struct VuePasteData {
    title_list: Vec<String>,  // 返回的title详情
    detail_list: Vec<String>, // 返回的内容详情
    size: usize,              // 剪切板的内容大小
}
impl VuePasteData {
    pub fn form(size: usize, title_list: Vec<String>, detail_list: Vec<String>) -> VuePasteData {
        VuePasteData {
            title_list,
            detail_list,
            size,
        }
    }
}

/// 取出当前的剪切板的全部内容
#[tauri::command]
pub async fn get_now_paste() -> VuePasteData {
    let db = get_instance();
    // 返回值已经拿到了
    let paste = db.get_safe_paste_list_copy().await;
    // 构建数据结构
    let size = paste.len();
    let mut title_list = Vec::new();
    let mut detail_list = Vec::new();
    paste.iter().for_each(|p| {
        if p.len() > 10 {
            title_list.push(safe_truncate(&p, 10));
        } else {
            title_list.push(p.to_string());
        }
        detail_list.push(p.to_string());
    });
    VuePasteData::form(size, title_list, detail_list)
}

#[tauri::command]
pub async fn write_to_paste(index: usize, window: tauri::Window) {
    // 写第index个数据到剪切板
    // write_to_clipboard
    println!("收到的消息是第{}位的数据", index);
    let db = get_instance();
    // 返回值已经拿到了
    let paste = db.get_safe_paste_list_copy().await;
    let input_clipboard = paste.get(index);
    match input_clipboard {
        Some(info) => {
            write_to_clipboard(info.to_string(), window);
        }
        None => {}
    }
}

// 安全切割字符串
fn safe_truncate(s: &str, max_graphemes: usize) -> String {
    UnicodeSegmentation::graphemes(s, true)
        .filter(|g| !g.trim().is_empty())
        .take(max_graphemes)
        .collect()
}
