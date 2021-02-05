# rust-learn-japanese
[日本語](./README.md) [中文](./README_ch.md)  
一个用 Rust 语言写的帮助日语学习的命令行工具。  

## 下载
```bash
git clone https://github.com/SKTT1Ryze/rust-learn-japanese
```

## 使用方法
平时使用 `Linux` 操作系统的时候，会有大量的终端命令输入操作。  
该命令行工具实现的功能是在输入命令的时候，终端里面先会打印一个日语单词。  
再次按回车后，终端会打印日语单词的假名注释和中文注释，甚至还有例句。  
当然会根据你敲的命令进行相应的操作。  
这样子就可以一边在 `Linux` 上工作一边记单词，一举两得。    
比如：  
```bash
cargo run ls ./
```
输出结果：  
```
アニメ

あにめ 动漫
このアニメはすごく面白い
```
如果将这个东西加入到环境变量，会有意想不到的效果！  

## 词库
日语单词词库收集在 `dict.json` 这个 `Json` 格式的文件中。  
目前还实现了自动将`MOJi辞書`的`每日热词`网页的 `HTML` 文件里面的单词添加到词库中的功能。  
```bash
cargo run rlj /path/to/MOJi辞書-日语实用词典.html
```

## 一起来用 Rust 学习日语吧！

## TODO
+ 将单词分类
+ 更多的帮助日语学习的方式

