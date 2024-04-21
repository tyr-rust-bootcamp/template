# 第一周：魔法神箭：从 Hello world 到实用的 CLI 工具

## 内容

- 测试密码强度

## 安装依赖

```bash
cargo add zxcvbn
```

## 测试密码强度

```rust
pub fn process_genpass(length: u8, no_upper: bool, no_lower: bool, no_number: bool, no_symbol: bool) -> anyhow::Result<()> {
    ......
    // 将密码转换为字符串，并打印出来。
    let password = String::from_utf8(password)?;
    println!("{}", password);

    let estimate = zxcvbn(&password, &[])?;
    // eprintln! 的输出会被打印到标准错误流中，而不是标准输出流中。
    // 比如 cargo run -- genpass -l 16 > password.txt 只会将密码保存到 password.txt 文件中，而不会包含密码强度的信息。
    eprintln!("Password strength: {}", estimate.score());

    Ok(())
}
```

## 验证效果

执行以下命令生成 16 位密码，可以看到密码强度是 4（强）。

```bash
cargo run -- genpass -l 16
 
# 输出
9kgqfQR7JGa!DKTK
Password strength: 4
```

生成 6 位不带数字和符号的密码，可以看到密码强度是 1（弱）。

```bash
cargo run -- genpass -l 6 --no-number --no-symbol

# 输出
bbZBje
Password strength: 1
```