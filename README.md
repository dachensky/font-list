# 介绍

本服务基于 rust 开发

- 运行 `cargo run`
- 构建 `cargo build --release`

服务

> 端口号 `3030`

| 地址                                                          | 说明                    | 返回值 |
| ------------------------------------------------------------- | ----------------------- | ------ |
| http://127.0.0.1:3030/fonts                                   | 获取系统字体列表        | json   |
| http://127.0.0.1:3030/font?path=C:\\WINDOWS\\FONTS\\MSYHL.TTC | 字体文件转化成 web 地址 | File   |

json返回值示例
```json
[
  {
    "family_name": [
      {
        "font_family": "Microsoft YaHei",
        "language": "English",
        "style": "Regular"
      },
      {
        "font_family": "微软雅黑",
        "language": "Chinese",
        "style": "Regular"
      }
    ],
    "file_name": "MSYHL.TTC",
    "font_name": "微软雅黑 Light",
    "path": "C:\\WINDOWS\\FONTS\\MSYHL.TTC",
    "post_script": "微软雅黑 Light",
    "weight": 290
  }, {
    // ...
  }
]
```

