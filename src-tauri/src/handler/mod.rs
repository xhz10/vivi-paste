use crate::database::get_instance;

#[tauri::command]
pub async fn my_custom_command() {
    let db = get_instance();
    let safe_list = db.get_safe_paste_list();
    let mut paste = safe_list.lock().await;
    paste.iter().for_each(|r|println!("输出一下结果 {}",r));
}