# 第一周：魔法神箭：从 Hello world 到实用的 CLI 工具

## 内容

- 支持通用的 CSV 字段，通过 serde_json 库的 Value 类型支持任意的 JSON 数据。

## 支持通用的 CSV 字段

在之前的实现中，我们定义了一个 Player 结构体，用于将 CSV 文件中的数据映射到 Player 结构体中，然后再将结构体转换为 JSON 数据。
然而，这样做有一个缺点，就是我们只能将 CSV 文件中的数据转换为 Player 结构体，如果 CSV 文件中的字段不是 Player 结构体中的字段，就无法转换了。

因此，我们需要支持通用的 CSV 字段。我们可以使用 serde_json 库中的 Value 类型，它是一个枚举类型，可以表示 JSON 中的任意类型。

```rust
pub fn process_csv(input: &str, output: &str) -> anyhow::Result<()> {
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

    let json = serde_json::to_string_pretty(&ret)?;
    fs::write(output, json)?;
    Ok(())
}
```

这样我们也无需再定义 Player 结构体了，可以将任何字段的 CSV 文件转换为 JSON 文件。

执行命令后，会将 CSV 文件中的数据读取出来，并将数据写入到 JSON 文件（默认是 output.json，可以通过 -o 参数指定）中。

```bash
cargo run -- csv -i assets/juventus.csv
```