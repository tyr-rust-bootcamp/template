use std::fs;
use csv::Reader;
use serde_json::Value;
use crate::opts::OutputFormat;

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    // headers() 方法会返回 CSV 文件的第一行，即表头
    let headers = reader.headers()?.clone();
    // 通过 records() 方法迭代 CSV 文件中的每一行
    for result in reader.records() {
        let record = result?;
        // headers.iter() 与 record.iter() 一一对应，将两个迭代器 zip 起来，再通过 collect 方法将其转换为 Value 类型
        // zip() 方法会返回一个元组，元组的第一个元素是 headers 的元素，第二个元素是 record 的元素
        // collect::<Value>() 会将元组转换为 Value 类型
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        ret.push(json_value);
    }

    // 根据 format 参数的值，将 ret 转换为 JSON 或 YAML 格式的字符串
    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };
    fs::write(output, content)?;
    Ok(())
}