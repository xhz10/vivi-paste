#[tauri::command]
fn my_custom_command() {
    println!("I was invoked from JS!");
}