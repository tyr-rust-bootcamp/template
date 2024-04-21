use std::fs;
use clap::{Parser, value_parser};
use csv::{Position, Reader};
use serde::{Deserialize, Serialize};

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
struct Opts {
    #[command(subcommand)]
    cmd: SubCommand,
}

#[derive(Debug, Parser)]
enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")] // 默认将 CSV 的列名（大驼峰式命名法）与 Rust 的字段名进行映射
struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")] // 对不符合 PascalCase 命名规范的列名进行重命名
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}


#[derive(Debug, Parser)]
struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    input: String,

    #[arg(short, long, default_value = "output.json")]
    output: String,

    #[arg(short, long, default_value_t = ',')]
    delimiter: char,

    #[arg(long, default_value_t = true)]
    header: bool,
}

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let mut reader = Reader::from_path(opts.input)?;
            let mut ret = Vec::with_capacity(128);
            for result in reader.deserialize() {
                let record: Player = result?;
                ret.push(record);
            }

            let json = serde_json::to_string_pretty(&ret)?;
            fs::write(opts.output, json)?;
        }
    }
    Ok(())
}

fn verify_input_file(filename: &str) -> Result<String, String> {
    if std::path::Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err(format!("File not found: {}", filename))
    }
}