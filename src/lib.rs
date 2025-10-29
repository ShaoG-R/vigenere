//! 维吉尼亚密码库
//! 
//! 提供优雅的泛型维吉尼亚密码实现，支持自定义字符集

pub mod core;

pub use core::{CipherElement, VigenereCipher};

pub use nonempty_tools::{NonEmptySliceRef, NonEmptyVec};

// ==================== 预定义元素类型 ====================

/// 字符元素 - 基于字符集的实现
#[derive(Debug, Clone, PartialEq)]
pub struct CharElement {
    value: char,
    index: usize,
}

impl CharElement {
    /// 创建新的字符元素
    pub fn new(value: char, index: usize) -> Self {
        Self { value, index }
    }
}

impl CipherElement for CharElement {
    type Value = char;
    
    fn index(&self) -> usize {
        self.index
    }
    
    fn value(&self) -> Self::Value {
        self.value
    }
}

/// 数字元素 - 基于模运算的实现（0-9）
#[derive(Debug, Clone, PartialEq)]
pub struct DigitElement {
    value: u8, // 0-9
}

impl DigitElement {
    /// 创建新的数字元素
    /// 
    /// # 参数
    /// - `value`: 数字值，必须在 0-9 范围内
    /// 
    /// # 返回
    /// 如果值有效则返回 Some(DigitElement)，否则返回 None
    pub fn new(value: u8) -> Option<Self> {
        if value < 10 {
            Some(Self { value })
        } else {
            None
        }
    }
    
    /// 转换为字符输出
    pub fn to_char(&self) -> char {
        char::from_digit(self.value as u32, 10).unwrap()
    }
}

impl CipherElement for DigitElement {
    type Value = u8;
    
    fn index(&self) -> usize {
        self.value as usize
    }
    
    fn value(&self) -> Self::Value {
        self.value
    }
}

// ==================== 便捷的字符串接口 ====================

/// 字符串密码器 - 对字符集密码器的便捷封装
/// 
/// 提供友好的字符串加密/解密接口
pub struct StringCipher {
    charset: Vec<CharElement>,
    modulus: usize,
}

impl StringCipher {
    /// 从字符串创建密码器
    /// 
    /// # 参数
    /// - `charset`: 字符集字符串，不能为空且不能包含重复字符
    /// 
    /// # 示例
    /// ```
    /// use vigenere_demo::StringCipher;
    /// 
    /// let cipher = StringCipher::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ").unwrap();
    /// ```
    pub fn new(charset: &str) -> Result<Self, String> {
        if charset.is_empty() {
            return Err("字符集不能为空".to_string());
        }
        
        let chars: Vec<char> = charset.chars().collect();
        
        // 检查重复
        let unique: std::collections::HashSet<_> = chars.iter().collect();
        if unique.len() != chars.len() {
            return Err("字符集包含重复字符".to_string());
        }
        
        let charset: Vec<CharElement> = chars
            .into_iter()
            .enumerate()
            .map(|(i, c)| CharElement::new(c, i))
            .collect();
        
        let modulus = charset.len();
        Ok(Self { charset, modulus })
    }
    
    /// 预定义：大写英文字母 (A-Z)
    pub fn uppercase_alpha() -> Self {
        Self::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ").unwrap()
    }
    
    /// 预定义：小写英文字母 (a-z)
    pub fn lowercase_alpha() -> Self {
        Self::new("abcdefghijklmnopqrstuvwxyz").unwrap()
    }
    
    /// 预定义：大小写英文字母
    pub fn mixed_alpha() -> Self {
        Self::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz").unwrap()
    }
    
    /// 预定义：字母和数字
    pub fn alphanumeric() -> Self {
        Self::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789").unwrap()
    }
    
    /// 预定义：可打印ASCII字符
    pub fn printable_ascii() -> Self {
        let chars: String = (32..=126).map(|c| c as u8 as char).collect();
        Self::new(&chars).unwrap()
    }
    
    /// 将字符串解析为元素序列（严格模式）
    /// 
    /// 所有字符必须在字符集中，否则返回错误
    fn parse_string(&self, s: &str) -> Result<Vec<CharElement>, String> {
        s.chars()
            .map(|c| {
                self.charset
                    .iter()
                    .find(|elem| elem.value() == c)
                    .cloned()
                    .ok_or_else(|| format!("字符 '{}' 不在字符集中", c))
            })
            .collect()
    }
    
    /// 加密字符串
    /// 
    /// 只处理字符集中的字符，其他字符保持不变
    /// 
    /// # 参数
    /// - `plaintext`: 明文字符串
    /// - `key`: 密钥字符串（必须只包含字符集中的字符）
    /// 
    /// # 示例
    /// ```
    /// use vigenere_demo::StringCipher;
    /// 
    /// let cipher = StringCipher::uppercase_alpha();
    /// let encrypted = cipher.encrypt("HELLO", "KEY").unwrap();
    /// assert_eq!(encrypted, "RIJVS");
    /// ```
    pub fn encrypt(&self, plaintext: &str, key: &str) -> Result<String, String> {
        if key.is_empty() {
            return Err("密钥不能为空".to_string());
        }
        
        let key_elements = self.parse_string(key)?;
        
        let mut result = String::new();
        let mut key_index = 0;
        
        for ch in plaintext.chars() {
            if let Some(elem) = self.charset.iter().find(|e| e.value() == ch) {
                let key_elem = &key_elements[key_index % key_elements.len()];
                let new_index = (elem.index() + key_elem.index()) % self.modulus;
                result.push(self.charset[new_index].value());
                key_index += 1;
            } else {
                result.push(ch); // 保留不在字符集中的字符
            }
        }
        
        Ok(result)
    }
    
    /// 解密字符串
    /// 
    /// 只处理字符集中的字符，其他字符保持不变
    /// 
    /// # 参数
    /// - `ciphertext`: 密文字符串
    /// - `key`: 密钥字符串（必须只包含字符集中的字符）
    /// 
    /// # 示例
    /// ```
    /// use vigenere_demo::StringCipher;
    /// 
    /// let cipher = StringCipher::uppercase_alpha();
    /// let decrypted = cipher.decrypt("RIJVS", "KEY").unwrap();
    /// assert_eq!(decrypted, "HELLO");
    /// ```
    pub fn decrypt(&self, ciphertext: &str, key: &str) -> Result<String, String> {
        if key.is_empty() {
            return Err("密钥不能为空".to_string());
        }
        
        let key_elements = self.parse_string(key)?;
        
        let mut result = String::new();
        let mut key_index = 0;
        
        for ch in ciphertext.chars() {
            if let Some(elem) = self.charset.iter().find(|e| e.value() == ch) {
                let key_elem = &key_elements[key_index % key_elements.len()];
                let new_index = (elem.index() + self.modulus - key_elem.index()) % self.modulus;
                result.push(self.charset[new_index].value());
                key_index += 1;
            } else {
                result.push(ch); // 保留不在字符集中的字符
            }
        }
        
        Ok(result)
    }
    
    /// 获取字符集信息
    pub fn charset_info(&self) -> String {
        let chars: String = self.charset.iter().map(|e| e.value()).collect();
        format!("字符集大小: {}, 字符: \"{}\"", self.modulus, chars)
    }
}

// ==================== 单元测试 ====================

#[cfg(test)]
mod tests {
    use super::*;

    // === StringCipher 测试 ===
    
    #[test]
    fn test_string_basic_encryption() {
        let cipher = StringCipher::uppercase_alpha();
        let encrypted = cipher.encrypt("HELLO", "KEY").unwrap();
        assert_eq!(encrypted, "RIJVS");
    }

    #[test]
    fn test_string_basic_decryption() {
        let cipher = StringCipher::uppercase_alpha();
        let decrypted = cipher.decrypt("RIJVS", "KEY").unwrap();
        assert_eq!(decrypted, "HELLO");
    }

    #[test]
    fn test_string_round_trip() {
        let cipher = StringCipher::alphanumeric();
        let original = "Hello123World";
        let encrypted = cipher.encrypt(original, "SecretKey").unwrap();
        let decrypted = cipher.decrypt(&encrypted, "SecretKey").unwrap();
        assert_eq!(original, decrypted);
    }

    #[test]
    fn test_string_custom_charset() {
        let cipher = StringCipher::new("0123456789").unwrap();
        let encrypted = cipher.encrypt("123", "456").unwrap();
        assert_eq!(encrypted, "579");
    }

    #[test]
    fn test_string_preserve_unknown_chars() {
        let cipher = StringCipher::uppercase_alpha();
        let encrypted = cipher.encrypt("HELLO, WORLD!", "KEY").unwrap();
        // 逗号、空格、感叹号保持不变
        assert_eq!(encrypted, "RIJVS, UYVJN!");
    }

    #[test]
    fn test_string_empty_key_error() {
        let cipher = StringCipher::uppercase_alpha();
        assert!(cipher.encrypt("HELLO", "").is_err());
    }

    #[test]
    fn test_string_invalid_key_char() {
        let cipher = StringCipher::uppercase_alpha();
        assert!(cipher.encrypt("HELLO", "key").is_err()); // 小写字母不在大写字符集中
    }

    #[test]
    fn test_string_duplicate_charset_error() {
        let result = StringCipher::new("ABBA");
        assert!(result.is_err());
    }
    
    // === 泛型 VigenereCipher 测试 ===
    
    #[test]
    fn test_generic_cipher_with_char_elements() {
        // 创建字符元素字符集
        let charset: Vec<CharElement> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
            .chars()
            .enumerate()
            .map(|(i, c)| CharElement::new(c, i))
            .collect();
        
        let cipher = VigenereCipher::new(NonEmptyVec::try_from_vec(charset.clone()).unwrap());
        
        // H=7, E=4, L=11, L=11, O=14
        // K=10, E=4, Y=24
        let plaintext = vec![
            charset[7].clone(),  // H
            charset[4].clone(),  // E
            charset[11].clone(), // L
            charset[11].clone(), // L
            charset[14].clone(), // O
        ];
        
        let key = vec![
            charset[10].clone(), // K
            charset[4].clone(),  // E
            charset[24].clone(), // Y
        ];
        
        let encrypted = cipher.encrypt(&plaintext, NonEmptySliceRef::new(key.as_slice()).unwrap());
        
        // R=17, I=8, J=9, V=21, S=18
        assert_eq!(encrypted[0].value(), 'R');
        assert_eq!(encrypted[1].value(), 'I');
        assert_eq!(encrypted[2].value(), 'J');
        assert_eq!(encrypted[3].value(), 'V');
        assert_eq!(encrypted[4].value(), 'S');
        
        // 测试解密
        let decrypted = cipher.decrypt(&encrypted, NonEmptySliceRef::new(key.as_slice()).unwrap());
        assert_eq!(decrypted[0].value(), 'H');
        assert_eq!(decrypted[1].value(), 'E');
        assert_eq!(decrypted[2].value(), 'L');
        assert_eq!(decrypted[3].value(), 'L');
        assert_eq!(decrypted[4].value(), 'O');
    }
    
    #[test]
    fn test_generic_cipher_with_digit_elements() {
        // 创建数字密码器（0-9）
        let charset: Vec<DigitElement> = (0..10)
            .map(|i| DigitElement::new(i).unwrap())
            .collect();
        
        let cipher = VigenereCipher::new(NonEmptyVec::try_from_vec(charset.clone()).unwrap());
        
        let plaintext = vec![
            charset[1].clone(), // 1
            charset[2].clone(), // 2
            charset[3].clone(), // 3
        ];
        
        let key = vec![
            charset[4].clone(), // 4
            charset[5].clone(), // 5
            charset[6].clone(), // 6
        ];
        
        let encrypted = cipher.encrypt(&plaintext, NonEmptySliceRef::new(key.as_slice()).unwrap());
        
        // (1+4)%10=5, (2+5)%10=7, (3+6)%10=9
        assert_eq!(encrypted[0].to_char(), '5');
        assert_eq!(encrypted[1].to_char(), '7');
        assert_eq!(encrypted[2].to_char(), '9');
        
        let decrypted = cipher.decrypt(&encrypted, NonEmptySliceRef::new(key.as_slice()).unwrap());
        assert_eq!(decrypted[0].to_char(), '1');
        assert_eq!(decrypted[1].to_char(), '2');
        assert_eq!(decrypted[2].to_char(), '3');
    }
    
    #[test]
    fn test_cipher_element_index() {
        let elem = CharElement::new('A', 0);
        assert_eq!(elem.index(), 0);
        assert_eq!(elem.value(), 'A');
        
        let elem2 = CharElement::new('Z', 25);
        assert_eq!(elem2.index(), 25);
        assert_eq!(elem2.value(), 'Z');
    }
    
    #[test]
    fn test_digit_element() {
        let digit = DigitElement::new(5).unwrap();
        assert_eq!(digit.index(), 5);
        assert_eq!(digit.to_char(), '5');
        assert_eq!(digit.value(), 5);
        
        // 测试边界
        assert!(DigitElement::new(10).is_none());
        assert!(DigitElement::new(0).is_some());
        assert!(DigitElement::new(9).is_some());
    }
}

