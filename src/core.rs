//! 维吉尼亚密码核心算法模块
//! 
//! 提供基于泛型和 trait 的纯数学实现

use std::fmt::Debug;

use nonempty_tools::{NonEmptySliceRef, NonEmptyVec};

/// 维吉尼亚密码元素 trait
/// 
/// 任何实现此 trait 的类型都可以作为密码系统的基本元素
/// 
/// # 核心设计理念
/// - 元素自带索引，算法只需要索引值即可完成加密/解密运算
/// - 值类型需要支持相等性比较（PartialEq），但元素本身不需要
/// - 这样可以灵活地为不同的元素类型定义不同的相等性语义
pub trait CipherElement: Clone + Debug {
    /// 值类型 - 元素所代表的实际值（如 char, u8 等）
    /// 值需要支持相等性比较
    type Value: PartialEq;
    
    /// 获取元素在字符集中的索引
    /// 
    /// 这是维吉尼亚密码算法的核心 - 所有运算都基于索引
    fn index(&self) -> usize;
    
    /// 获取元素的值
    /// 
    /// 用于比较、显示等非算法操作
    fn value(&self) -> Self::Value;
}

/// 维吉尼亚密码核心结构（泛型版本）
/// 
/// 使用泛型 T 支持任意实现 CipherElement 的类型
/// 
/// # 示例
/// ```
/// use vigenere::{CharElement, VigenereCipher};
/// 
/// // 创建字符元素字符集
/// let charset: Vec<CharElement> = "ABC"
///     .chars()
///     .enumerate()
///     .map(|(i, c)| CharElement::new(c, i))
///     .collect();
/// 
/// let cipher = VigenereCipher::new(charset).unwrap();
/// assert_eq!(cipher.modulus(), 3);
/// ```
#[derive(Debug, Clone)]
pub struct VigenereCipher<T: CipherElement> {
    charset: Vec<T>,
    modulus: usize,
}

impl<T: CipherElement> VigenereCipher<T> {
    /// 使用字符集创建密码器
    /// 
    /// # 参数
    /// - `charset`: 元素集合
    /// 
    pub fn new(charset: NonEmptyVec<T>) -> Self {
        let modulus = charset.len();
        Self { charset: charset.into_inner(), modulus }
    }
    
    /// 获取字符集大小（模数）
    pub fn modulus(&self) -> usize {
        self.modulus
    }
    
    /// 获取字符集引用
    pub fn charset(&self) -> &[T] {
        &self.charset
    }
    
    /// 加密：使用纯粹的数学运算
    /// 
    /// # 算法
    /// ```text
    /// E(M, K) = (M + K) mod n
    /// ```
    /// 
    /// # 参数
    /// - `plaintext`: 明文元素序列
    /// - `key`: 密钥元素序列
    /// 
    /// # 返回
    /// 加密后的元素序列
    pub fn encrypt(&self, plaintext: &[T], key: NonEmptySliceRef<T>) -> Vec<T> {
        self.process(plaintext, key.as_slice(), |m, k, n| (m + k) % n)
    }
    
    /// 解密：使用纯粹的数学运算
    /// 
    /// # 算法
    /// ```text
    /// D(C, K) = (C - K + n) mod n
    /// ```
    /// 
    /// # 参数
    /// - `ciphertext`: 密文元素序列
    /// - `key`: 密钥元素序列
    /// 
    /// # 返回
    /// 解密后的元素序列
    pub fn decrypt(&self, ciphertext: &[T], key: NonEmptySliceRef<T>) -> Vec<T> {
        self.process(ciphertext, key.as_slice(), |c, k, n| (c + n - k) % n)
    }
    
    /// 核心处理函数：优雅的函数式设计
    /// 
    /// 使用高阶函数将加密/解密的差异抽象为不同的运算函数
    /// 
    /// # 类型参数
    /// - `F`: 运算函数，接受 (元素索引, 密钥索引, 模数) 返回新索引
    fn process<F>(&self, input: &[T], key: &[T], operation: F) -> Vec<T>
    where
        F: Fn(usize, usize, usize) -> usize,
    {
        let result = input
            .iter()
            .enumerate()
            .map(|(i, element)| {
                let key_element = &key[i % key.len()];
                let new_index = operation(element.index(), key_element.index(), self.modulus);
                self.charset[new_index].clone()
            })
            .collect();
        
        result
    }
}

