//! 维吉尼亚密码交互式命令行程序

use std::io::{self, Write};
use vigenere_demo::StringCipher;

fn main() {
    println!("=== 维吉尼亚密码加解密程序 ===\n");
    println!("架构设计:");
    println!("  • core.rs    - 泛型 VigenereCipher<T: CipherElement>");
    println!("  • lib.rs     - CharElement, DigitElement, StringCipher");
    println!("  • main.rs    - 交互式用户界面");
    println!("  • 元素自带索引，值实现 PartialEq\n");

    loop {
        println!("\n选择字符集:");
        println!("1. 大写英文字母 (A-Z)");
        println!("2. 小写英文字母 (a-z)");
        println!("3. 大小写英文字母");
        println!("4. 字母+数字");
        println!("5. 可打印ASCII字符");
        println!("6. 自定义字符集");
        println!("0. 退出");

        print!("\n请选择: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        let cipher = match choice {
            "0" => break,
            "1" => StringCipher::uppercase_alpha(),
            "2" => StringCipher::lowercase_alpha(),
            "3" => StringCipher::mixed_alpha(),
            "4" => StringCipher::alphanumeric(),
            "5" => StringCipher::printable_ascii(),
            "6" => {
                print!("请输入自定义字符集: ");
                io::stdout().flush().unwrap();
                let mut custom = String::new();
                io::stdin().read_line(&mut custom).unwrap();
                match StringCipher::new(custom.trim()) {
                    Ok(c) => c,
                    Err(e) => {
                        println!("❌ 错误: {}", e);
                        continue;
                    }
                }
            }
            _ => {
                println!("❌ 无效选择");
                continue;
            }
        };

        println!("\n✓ {}", cipher.charset_info());

        // 选择操作
        println!("\n选择操作:");
        println!("1. 加密");
        println!("2. 解密");
        print!("请选择: ");
        io::stdout().flush().unwrap();

        let mut operation = String::new();
        io::stdin().read_line(&mut operation).unwrap();
        let operation = operation.trim();

        let is_encrypt = match operation {
            "1" => true,
            "2" => false,
            _ => {
                println!("❌ 无效选择");
                continue;
            }
        };

        // 获取文本
        print!("\n请输入{}: ", if is_encrypt { "明文" } else { "密文" });
        io::stdout().flush().unwrap();
        let mut text = String::new();
        io::stdin().read_line(&mut text).unwrap();
        let text = text.trim();

        // 获取密钥
        print!("请输入密钥: ");
        io::stdout().flush().unwrap();
        let mut key = String::new();
        io::stdin().read_line(&mut key).unwrap();
        let key = key.trim();

        // 执行加密/解密
        let result = if is_encrypt {
            cipher.encrypt(text, key)
        } else {
            cipher.decrypt(text, key)
        };

        match result {
            Ok(output) => {
                println!("\n✓ {}: {}", if is_encrypt { "密文" } else { "明文" }, output);
            }
            Err(e) => {
                println!("\n❌ 错误: {}", e);
            }
        }
    }

    println!("\n再见！");
}
