use std::cell::OnceCell;
use std::sync::Arc;
/// 内置的事件队列
use tauri::{App, Emitter, Manager, Window};

///vivi的事件有哪些
pub enum VIVIEvent {
    Test,
    PasteUpdate,
}

// payload类型必须实现 `Serialize` 和 `Clone`。
#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

/// 发送一个事件到前端
pub fn send_message(event: VIVIEvent, window: Arc<Window>) {
    let key = match event {
        VIVIEvent::Test => "test_event",
        _ => "other",
    };
    // 获取窗口
    // 发送消息
    window
        .emit(
            key,
            Payload {
                message: "Tauri is awesome!".into(),
            },
        )
        .unwrap();
}
