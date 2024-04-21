use rand::prelude::SliceRandom;
use zxcvbn::zxcvbn;

// 定义了四个常量 UPPER、LOWER、NUMBER 和 SYMBOL，分别代表了大写字母、小写字母、数字和符号。
const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_";

pub fn process_genpass(length: u8, no_upper: bool, no_lower: bool, no_number: bool, no_symbol: bool) -> anyhow::Result<()> {
    // 创建了一个随机数生成器 rng。
    let mut rng = rand::thread_rng();
    // 创建了两个空的 Vec，分别用来存储密码和字符集。
    let mut password = Vec::new();
    // 根据用户的输入来决定是否包含大写字母、小写字母、数字和符号。
    let mut chars = Vec::new();

    // 默认情况下，包含大写字母、小写字母、数字和符号。用户可以通过 --no-uppercase、--no-lowercase、--no-number 和 --no-symbol 参数来控制是否不包含对应的字符。
    // 如果用户选择了包含大写字母，我们就将 UPPER 集合中的字符添加到字符集中，并从中随机选择一个字符添加到密码中。
    // 如果用户选择了包含小写字母、数字和符号，我们也分别将对应的字符集添加到字符集中，并从中随机选择一个字符添加到密码中。(这样可以保证密码中至少包含一个大写字母、一个小写字母、一个数字和一个符号)
    if !no_upper {
        chars.extend_from_slice(UPPER);
        // 使用随机数生成器 rng 从 UPPER 集合中随机选择一个字符。
        password.push(*UPPER.choose(&mut rng).expect("UPPER won't be empty"));
    }
    if !no_lower {
        chars.extend_from_slice(LOWER);
        password.push(*LOWER.choose(&mut rng).expect("LOWER won't be empty"));
    }
    if !no_number {
        chars.extend_from_slice(NUMBER);
        password.push(*NUMBER.choose(&mut rng).expect("NUMBER won't be empty"));
    }
    if !no_symbol {
        chars.extend_from_slice(SYMBOL);
        password.push(*SYMBOL.choose(&mut rng).expect("SYMBOL won't be empty"));
    }

    // 根据用户输入的密码长度，从字符集中随机选择字符，添加到密码中，直到密码的长度达到用户输入的长度。
    for _ in 0..(length - password.len() as u8) {
        let c = chars.choose(&mut rng).expect("chars won't be empty");
        password.push(*c);
    }
    // 将密码打乱顺序
    password.shuffle(&mut rng);

    // 将密码转换为字符串，并打印出来。
    let password = String::from_utf8(password)?;
    println!("{}", password);

    let estimate = zxcvbn(&password, &[])?;
    // eprintln! 的输出会被打印到标准错误流中，而不是标准输出流中。
    // 比如 cargo run -- genpass -l 16 > password.txt 只会将密码保存到 password.txt 文件中，而不会包含密码强度的信息。
    eprintln!("Password strength: {}", estimate.score());

    Ok(())
}