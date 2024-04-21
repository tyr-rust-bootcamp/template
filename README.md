# 第一周：魔法神箭：从 Hello world 到实用的 CLI 工具

## 内容

- 使用 Serde 从 CSV 文件中反序列化数据，并将数据序列化为 JSON 格式。
- 使用 Anyhow 简化错误处理。

## 安装依赖

- CSV crate 提供快速灵活的 CSV 读取器和写入器，并支持 Serde。
- Serde 是 Rust 编程语言中的一个序列化和反序列化框架。它被广泛用于将复杂的数据类型转换成易于存储和传输的格式，如 JSON、YAML、Bincode 等，并且可以从这些格式中重构数据。
- Serde-json 是 Serde 框架的一个扩展库，专门处理 JSON 数据的序列化和反序列化。
- Anyhow 可以用于简化错误处理和提供更好的错误报告。关于使用 Anyhow 的优点，可以参考这篇文章：[Rust中的错误处理](https://rustcc.cn/article?id=1e20f814-c7d5-4aca-bb67-45dcfb65d9f9)。

```bash
cargo add csv
cargo add serde --features derive
cargo add serde-json
cargo add anyhow
```

## 使用 DuckDB 读取 CSV 文件

DuckDB是一个轻量级、内存优化的分析型数据库管理系统。我们可以使用 DuckDB 来读取 CSV 文件。

首先安装 DuckDB。

```bash
brew install duckdb
```

启动 DuckDB。

```sql
duckdb
```

读取 CSV 文件。

```sql
D SELECT * FROM read_csv('assets/juventus.csv', auto_detect=true);
┌───────────────────────┬────────────────────┬───────────────────┬────────────────────┬────────────┐
│         Name          │      Position      │        DOB        │    Nationality     │ Kit Number │
│        varchar        │      varchar       │      varchar      │      varchar       │   int64    │
├───────────────────────┼────────────────────┼───────────────────┼────────────────────┼────────────┤
│ Wojciech Szczesny     │ Goalkeeper         │ Apr 18, 1990 (29) │ Poland             │          1 │
│ Mattia Perin          │ Goalkeeper         │ Nov 10, 1992 (26) │ Italy              │         37 │
│ Gianluigi Buffon      │ Goalkeeper         │ Jan 28, 1978 (41) │ Italy              │         77 │
│ Carlo Pinsoglio       │ Goalkeeper         │ Mar 16, 1990 (29) │ Italy              │         31 │
│ Matthijs de Ligt      │ Centre-Back        │ Aug 12, 1999 (20) │ Netherlands        │          4 │
│ Leonardo Bonucci      │ Centre-Back        │ May 1, 1987 (32)  │ Italy              │         19 │
│ Daniele Rugani        │ Centre-Back        │ Jul 29, 1994 (25) │ Italy              │         24 │
│ Merih Demiral         │ Centre-Back        │ Mar 5, 1998 (21)  │ Turkey             │         28 │
│ Giorgio Chiellini     │ Centre-Back        │ Aug 14, 1984 (35) │ Italy              │          3 │
│ Alex Sandro           │ Left-Back          │ Jan 26, 1991 (28) │ Brazil             │         12 │
│ Danilo                │ Right-Back         │ Jul 15, 1991 (28) │ Brazil             │         13 │
│ Mattia De Sciglio     │ Right-Back         │ Oct 20, 1992 (27) │ Italy              │          2 │
│ Emre Can              │ Defensive Midfield │ Jan 12, 1994 (25) │ Germany            │         23 │
│ Miralem Pjanic        │ Central Midfield   │ Apr 2, 1990 (29)  │ Bosnia-Herzegovina │          5 │
│ Aaron Ramsey          │ Central Midfield   │ Dec 26, 1990 (28) │ Wales              │          8 │
│ Adrien Rabiot         │ Central Midfield   │ Apr 3, 1995 (24)  │ France             │         25 │
│ Rodrigo Bentancur     │ Central Midfield   │ Jun 25, 1997 (22) │ Uruguay            │         30 │
│ Blaise Matuidi        │ Central Midfield   │ Apr 9, 1987 (32)  │ France             │         14 │
│ Sami Khedira          │ Central Midfield   │ Apr 4, 1987 (32)  │ Germany            │          6 │
│ Cristiano Ronaldo     │ Left Winger        │ Feb 5, 1985 (34)  │ Portugal           │          7 │
│ Marko Pjaca           │ Left Winger        │ May 6, 1995 (24)  │ Croatia            │         15 │
│ Federico Bernardeschi │ Right Winger       │ Feb 16, 1994 (25) │ Italy              │         33 │
│ Douglas Costa         │ Right Winger       │ Sep 14, 1990 (29) │ Brazil             │         11 │
│ Juan Cuadrado         │ Right Winger       │ May 26, 1988 (31) │ Colombia           │         16 │
│ Paulo Dybala          │ Second Striker     │ Nov 15, 1993 (25) │ Argentina          │         10 │
│ Gonzalo Higuaín       │ Centre-Forward     │ Dec 10, 1987 (31) │ Argentina          │         21 │
│ Mario Mandzukic       │ Centre-Forward     │ May 21, 1986 (33) │ Croatia            │         17 │
├───────────────────────┴────────────────────┴───────────────────┴────────────────────┴────────────┤
│ 27 rows                                                                                5 columns │
└──────────────────────────────────────────────────────────────────────────────────────────────────┘
```

## Anyhow 的好处

如果你选择不使用 Anyhow 或者 `Box<dyn Error>`（这是一个通用的动态错误类型，可以包含几乎所有类型的错误。这类似于 anyhow::Result 提供的灵活性，但不依赖于 anyhow 库。） 来处理错误，你将需要使用更具体的错误类型来明确地处理每种可能的错误。这通常意味着你需要定义或使用一个自定义的错误枚举，这个枚举包含所有可能的错误类型。

```rust
use csv::Error as CsvError;
#[derive(Debug)]
enum AppError {
    Io(io::Error),
    Csv(CsvError),
    // 添加更多错误类型，根据你的应用需求
}

impl From<io::Error> for AppError {
    fn from(err: io::Error) -> AppError {
        AppError::Io(err)
    }
}

impl From<CsvError> for AppError {
    fn from(err: CsvError) -> AppError {
        AppError::Csv(err)
    }
}

fn main() -> Result<(), AppError> {
    let opts: Opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let mut reader = Reader::from_path(opts.input)?;
            for result in reader.deserialize() {
                let player: Player = result?;
                println!("{:?}", player);
            }
        }
    }
    Ok(())
}
```

## 读取 CSV 文件并将输出写入到 JSON 文件

定义了一个 Player 结构体，用于表示 CSV 文件中的球员，通过 Serde 的 Deserialize 和 Serialize 特性，可以将 CSV 文件中的数据读取到 Player 结构体中，并将 Player 结构体的数据写入到 JSON 文件中。

```rust
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
```

```rust
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
```


执行命令后，会将 CSV 文件中的数据读取出来，并将数据写入到 JSON 文件（默认是 output.json，可以通过 -o 参数指定）中。

```bash
cargo run -- csv -i assets/juventus.csv
```