# 第一周：魔法神箭：从 Hello world 到实用的 CLI 工具

## 内容

- 支持通用的 JSON 和 YAML 数据类型

## 安装依赖

```bash
cargo add serde_yaml
```

## 支持通用的 JSON 和 YAML 数据类型

以下说明的主要的代码变化。

定义 OutputFormat 枚举类型，支持 json 和 yaml 两种格式。

```rust
#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}
```

创建 `parse_format` 函数用于解析 `--format` 参数。

```rust
#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(long, value_parser = parse_format, default_value = "json")]
    pub format: OutputFormat,
    ......
}

// 处理输入参数：方法一
// 直接使用一个函数和匹配语句来处理转换，不依赖于 Rust 的 trait 系统。这对于简单的用途来说可能更直接易懂。
// fn parse_format(format: &str) -> Result<OutputFormat, String> {
//     match format {
//         "json" => Ok(OutputFormat::Json),
//         "yaml" => Ok(OutputFormat::Yaml),
//         _ => Err(format!("Invalid format: {}", format)),
//     }
// }


// 处理输入参数：方法二
// 通过实现 FromStr trait，使得任何字符串都可以使用标准的 .parse() 方法尝试转换为 OutputFormat。
// 这是 Rust 惯用的方法来处理从字符串到某个类型的转换。

// parse_format 函数接受一个字符串参数，尝试将其解析为 OutputFormat 枚举类型。
// 如果字符串是 "json" 或 "yaml"，则解析成功并返回对应的枚举值；否则，返回一个错误。
fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse()
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}
```

以下这两段代码紧密联系，共同支持将 OutputFormat 枚举类型转换成字符串表示。

```rust
// 将 OutputFormat 枚举类型转换为字符串

// 在 main.rs 中的 format!("output.{}", opts.format) 代码中的 "{}" 会调用 Display 的这个实现。
// fmt::Display for OutputFormat 是一个 Display trait 的实现，它定义了如何将 OutputFormat 枚举类型格式化为字符串。
// 这个实现利用了 From<OutputFormat> for &'static str trait 的实现，将枚举值转换为字符串，然后写入到提供的格式化器中。
impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 通过 Into trait 将 self（OutputFormat 枚举的一个实例）转换为其对应的字符串表示（例如 "json" 或 "yaml"），然后写入到格式化输出流 f 中。
        // 在 Rust 中，当你实现了 From<T> for U trait，Into<U> for T trait 也会自动被实现。
        // 这是因为 Rust 标准库中 Into trait 的定义包括一个默认的实现，它基于已存在的 From 实现。
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

// From<OutputFormat> for &'static str 是一个 From trait 的实现，它定义了如何将 OutputFormat 枚举类型转换为静态生命周期的字符串引用。
// 具体来说，Json 转换为 "json"，Yaml 转换为 "yaml"
impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}
```

process_csv 函数根据 format 参数的值，将 ret 转换为 JSON 或 YAML 格式的字符串。

```rust
pub fn process_csv(input: &str, output: String, format: OutputFormat) -> anyhow::Result<()> {
    ......
    // 根据 format 参数的值，将 ret 转换为 JSON 或 YAML 格式的字符串
    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };
    ......
}
```

执行下面命令，读取 CSV 并生成 json 文件。

```bash
cargo run -- csv -i assets/juventus.csv -o test.json
```

执行下面命令，读取 CSV 并生成 yaml 文件。


```bash
cargo run -- csv -i assets/juventus.csv -o test.yaml --format yaml
```
