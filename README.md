# vivi-paste

#### 介绍
灵活的粘贴工具
一个剪切板

#### 软件架构
前端是vite配合element-ui
后端是rust


#### 安装教程
嘻嘻

#### 使用说明

安装完成后按一下ctrl + shift + v 显示出剪切板页面 再按一下就隐藏了
选中要粘贴的历史按一下回车就自动放在剪切板里了


0. tauri的开发安装去官网看吧 https://tauri.app/
1. git clone 之后进入vivi-paste文件夹
2. 不放心就把package-lock.json和src-tauri/Cargo.lock删了
3. 更新前端资源包
```bash
npm install
```
4. 基本上这一关过了就无所谓了
5. 下面是本地调试
```bash
cargo tauri dev
```
6. 或者可以直接生成对应平台下的可执行文件
```bash
cargo tauri build
```

#### 参与贡献

1. 有懂前端的可以帮弟弟我搞一搞页面，我的V1.0太丑了
