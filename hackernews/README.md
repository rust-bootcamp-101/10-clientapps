dioxus开发前端说明

1.转换前端文件成dioxus组件

将前端html文件放在htmls文件夹里，查看效果，然后拆分成组件
然后使用 dx 命令将html组件转成 dioxus 组件

```bash
dx translate --file htmls/sotry_item.html
```

如何将前端应用打包成tauri桌面应用

首先将src-tauri生成到当前程序目录(或拷贝), 然后修改tauri.conf.json

将build中的命令修改为你的前端运行/构建的命令即可
```
  "build": {
    "beforeDevCommand": "cd ../hackernews && dx serve --port 8080 --hot-reload",
    "devUrl": "http://localhost:8080",
    "beforeBuildCommand": "cd ../hackernews && dx build",
    "frontendDist": "../hackernews/dist"
  },
```
