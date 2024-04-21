use std::fmt;
use std::str::FromStr;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
}

// OutputFormat 是一个枚举类型，有两个可能的值：Json 和 Yaml，分别表示两种不同的输出格式
#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,

    #[arg(short, long)]
    pub output: Option<String>,

    #[arg(long, value_parser = parse_format, default_value = "json")]
    pub format: OutputFormat,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}

fn verify_input_file(filename: &str) -> Result<String, String> {
    if std::path::Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err(format!("File not found: {}", filename))
    }
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
// FromStr for OutputFormat 是一个 FromStr trait 的实现，它定义了如何从字符串解析为 OutputFormat 枚举类型。
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


// 将 OutputFormat 枚举类型转换为字符串
// fmt::Display for OutputFormat 是一个 Display trait 的实现，它定义了如何将 OutputFormat 枚举类型格式化为字符串。
// 这个实现利用了 From<OutputFormat> for &'static str trait 的实现，将枚举值转换为字符串，然后写入到提供的格式化器中。
// 在 main.rs 中的 format!("output.{}", opts.format) 代码中的 "{}" 会调用这个实现。
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
