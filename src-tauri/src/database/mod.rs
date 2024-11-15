use std::sync::Arc;
use tauri::{App, Wry};
use tauri_plugin_store::{Store, StoreExt};

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
    paste_list: Vec<String>,
    /// 我自己的db
    db_store: Arc<Store<Wry>>,
}

impl PasteDB {
    /// 构建DB的方法
    pub fn form(app: &App) -> PasteDB {
        let store = app.store("paste.json").expect("初始化失败");

        PasteDB {
            max_cache_size: 2,
            max_list_size: 20,
            paste_list: vec![],
            db_store: store,
        }
    }

    /// 放入一个剪切板的内容
    pub fn put_paste_info(&mut self, key: String) -> Result<(), &str> {
        if key.is_empty() {
            return Err("空数据");
        }
        // 存在列表里面
        self.paste_list.insert(0, key);
        Ok(())
    }

    /// 返回一个列表信息
    pub fn show_now_paste_list(&self) -> impl Iterator<Item = &String> {
        self.paste_list.iter()
    }

    /// 落库
    pub fn put(&self, key: &str, value: serde_json::Value) -> Result<(), &str> {
        self.db_store.set(key, value);
        Ok(())
    }

    /// 取出来
    pub fn get(&self, key: &str) -> Result<serde_json::Value, &str> {
        Ok(self.db_store.get(key).expect("数据不存在"))
    }

    /// 关闭资源
    pub fn close(&self) {
        self.db_store.close_resource();
    }
}
