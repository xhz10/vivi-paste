
use serde::Serialize;
use crate::database::get_instance;


#[derive(Serialize)]
pub struct VuePasteData {
    title_list: Vec<String>, // 返回的title详情
    detail_list: Vec<String>, // 返回的内容详情
    size: usize, // 剪切板的内容大小
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
pub async fn get_now_paste() -> VuePasteData{
    let db = get_instance();
    // 返回值已经拿到了
    let paste = db.get_safe_paste_list_copy().await;
    // 构建数据结构
    let size = paste.len();
    let mut title_list = Vec::new();
    let mut detail_list = Vec::new();
    paste.iter().for_each(|p| {
        if p.len() > 10 {
            title_list.push(p[0..10].to_string());
        } else {
            title_list.push(p.to_string());
        }
        detail_list.push(p.to_string());
    });
    println!("输出这里了rust");
    VuePasteData::form(size,title_list,detail_list)
}