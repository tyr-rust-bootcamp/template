# 第一周：魔法神箭：从 Hello world 到实用的 CLI 工具

## 内容

- 初始化 CLI 项目，能够读取并打印命令行参数
- 增加字段校验，例如校验输入文件是否存在

## 安装依赖

Clap 是 Rust 中用于构建 CLI 的 crate。
我们可以使用 --feature 指定只安装某些 feature 而不是整个 crate。

```bash
cargo add clap --features derive
```

## 初始代码

```bash
cargo run -- csv -i test.csv

# 输出
Opts { cmd: Csv(CsvOpts { input: "test.csv", output: "output.json", delimiter: ',', header: true }) }
```

## 增加字段校验

通过 verify_input_file 函数校验输入文件是否存在。

```rust
#[derive(Debug, Parser)]
struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    input: String, 
    ......
}


fn verify_input_file(filename: &str) -> Result<String, String> {
    if std::path::Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err(format!("File not found: {}", filename))
    }
}
```

由于输入文件不存在，运行时会报错。

```bash
cargo run -- csv -i test.csv

# 输出
error: invalid value 'test.csv' for '--input <INPUT>': File not found: test.csv
```
