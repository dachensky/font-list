# 介绍

本项目是基于`rust`开发的一款获取本地系统字体列表，并可以把本地字体转换为http地址的小服务


## 可以用来干啥😊😊
- 1、搞一个PC程序获取本地字体实现字体切换功能
- 2、把本服务的二进制文件作为一个小扩展，在网页中调用接口实现字体的切换🐋🐋🐋
- 拿来玩啊👨‍⚖️👨‍⚖️👨‍⚖️👨‍⚖️👨‍⚖️


## 服务
> 端口号 `3030`
- 运行 `cargo run`
- 构建 `cargo build --release`

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

