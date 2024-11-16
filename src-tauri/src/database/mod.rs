use std::sync::Arc;

use once_cell::sync::OnceCell;
use tauri::{App, Wry};
use tauri_plugin_store::{Store, StoreExt};
use tokio::sync::Mutex;

// 全局 OnceCell 用于存储 PasteDB 实例
static PASTE_DB: OnceCell<Arc<PasteDB>> = OnceCell::new();

/// 数据存储都在这里做
///
/// 构思一下数据结构吧，我要做一个剪切板的列表
/// 那么先定义一个结构

/// 粘贴板的库里面都有啥呢
#[derive(Clone)]
pub struct PasteDB {
    /// 文件缓存的最大空间 单位 M
    max_cache_size: i32,
    /// 展示列表的最大长度
    max_list_size: i32,
    /// 实际存储数据的地方
    paste_list: Arc<Mutex<Vec<String>>>,
    /// 我自己的db
    db_store: Arc<Store<Wry>>,
}

impl PasteDB {
    /// 构建DB的方法
    fn form(app: &App) -> PasteDB {
        let store = app.store("paste.json").expect("初始化失败");

        PasteDB {
            max_cache_size: 2,
            max_list_size: 20,
            paste_list: Arc::new(Mutex::new(Vec::new())), // 返回一个现成的可用的列表
            db_store: store,
        }
    }

    /// 返回一个安全的paste_list的可变引用
    pub fn get_safe_paste_list(&self) -> Arc<Mutex<Vec<String>>> {
        // 分出去一个引用
        Arc::clone(&self.paste_list)
    }

    /// 关闭资源
    pub fn close(&self) {
        self.db_store.close_resource();
    }
}

pub fn initialize_paste_db(app: &App) -> Option<Arc<PasteDB>> {
    // 初始化 `PasteDB`，只会成功一次
    let db = Arc::new(PasteDB::form(app));
    PASTE_DB.set(db).ok()?;
    Some(PASTE_DB.get()?.clone())
}

