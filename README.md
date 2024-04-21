# 第一周：魔法神箭：从 Hello world 到实用的 CLI 工具

## 内容

- 重构代码模块

## 重构代码模块

为了提升代码的可读性，我们将原先 main.rs 中的代码安装逻辑拆分为多个模块。

- opts.rs：定义命令行参数。
- process.rs：读取 CSV 文件中的数据，并将数据写入到 JSON 文件中。
- lib.rs：声明模块以及公开导出函数供 main.rs 使用。
- main.rs：主程序入口。

对于需要公开导出的结构体或者函数，我们需要在 opts.rs 和 process.rs 中使用 pub 关键字进行声明。

执行命令后，会将 CSV 文件中的数据读取出来，并将数据写入到 JSON 文件（默认是 output.json，可以通过 -o 参数指定）中。

```bash
cargo run -- csv -i assets/juventus.csv
```