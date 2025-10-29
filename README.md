# 维吉尼亚密码（Vigenere Cipher）

一个优雅的维吉尼亚密码实现，采用泛型设计，支持自定义字符集。

**核心特性**：使用类型系统在**编译期**保证字符集和密钥非空，而非运行时检查。

## 📁 项目结构

```
src/
├── core.rs    # 核心密码算法（泛型实现）
├── lib.rs     # 库接口（元素类型 + StringCipher）
└── main.rs    # 交互式命令行程序
```

### 模块职责

- **`core.rs`**: 定义 `CipherElement` trait 和泛型 `VigenereCipher<T>`
  - 纯数学运算，与具体类型无关
  - 使用高阶函数抽象加密/解密逻辑
  - 使用 `NonEmptyVec` 和 `NonEmptySliceRef` 提供编译期非空保证
  
- **`lib.rs`**: 提供具体实现和便捷接口
  - `CharElement` 和 `DigitElement` 预定义元素类型
  - `StringCipher` 便捷的字符串加密接口
  - 完整的单元测试
  
- **`main.rs`**: 用户交互界面
  - 交互式命令行程序
  - 支持多种预定义字符集和自定义字符集

## 📦 依赖

```toml
[dependencies]
nonempty_tools = "0.1.0"
```

使用 [`nonempty_tools`](https://crates.io/crates/nonempty_tools) 在类型层面保证集合非空性。

## 🎯 设计特点

### 1. 核心架构：泛型 + Trait + 类型安全

```rust
use nonempty_tools::{NonEmptyVec, NonEmptySliceRef};

/// 维吉尼亚密码元素 trait
/// 元素自带索引，无需外部映射表
pub trait CipherElement: Clone + Debug {
    /// 值类型 - 需要支持相等性比较
    type Value: PartialEq;
    
    /// 获取元素索引（算法核心）
    fn index(&self) -> usize;
    
    /// 获取元素值（用于比较、显示）
    fn value(&self) -> Self::Value;
}

/// 泛型密码器 - 使用 NonEmptyVec 保证字符集非空
pub struct VigenereCipher<T: CipherElement> {
    charset: Vec<T>,
    modulus: usize,
}

impl<T: CipherElement> VigenereCipher<T> {
    /// 编译期保证字符集非空
    pub fn new(charset: NonEmptyVec<T>) -> Self;
    
    /// 编译期保证密钥非空
    pub fn encrypt(&self, plaintext: &[T], key: NonEmptySliceRef<T>) -> Vec<T>;
    pub fn decrypt(&self, ciphertext: &[T], key: NonEmptySliceRef<T>) -> Vec<T>;
}
```

### 2. 优雅的算法设计

核心加密/解密逻辑使用**纯粹的数学运算**和**高阶函数**：

```rust
// 加密：E(M, K) = (M + K) mod n
cipher.process(plaintext, key, |m, k, n| (m + k) % n)

// 解密：D(C, K) = (C - K + n) mod n
cipher.process(ciphertext, key, |c, k, n| (c + n - k) % n)
```

### 3. 核心优势

- ✅ **编译期非空保证**：使用 `NonEmptyVec` 和 `NonEmptySliceRef`，将运行时检查提升到编译期
- ✅ **元素自带索引**：无需 HashMap 进行字符到索引的映射
- ✅ **值类型分离**：`Value: PartialEq` 而非 `Element: PartialEq`，更清晰的语义
- ✅ **泛型设计**：支持任意类型（字符、数字、自定义类型）
- ✅ **零成本抽象**：编译期优化，运行时无性能损失
- ✅ **函数式风格**：使用高阶函数抽象加密/解密差异
- ✅ **类型安全**：Rust 类型系统 + NonEmpty 保证正确性
- ✅ **模块化清晰**：core.rs (算法) + lib.rs (实现) + main.rs (界面)

## 📦 使用示例

### 示例 1：使用泛型密码器（CharElement）

```rust
use vigenere::{CharElement, VigenereCipher, NonEmptyVec, NonEmptySliceRef};

// 创建字符元素字符集
let charset: Vec<CharElement> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
    .chars()
    .enumerate()
    .map(|(i, c)| CharElement::new(c, i))
    .collect();

// 使用 NonEmptyVec 保证字符集非空（编译期检查）
let ne_charset = NonEmptyVec::try_from_vec(charset.clone()).unwrap();
let cipher = VigenereCipher::new(ne_charset);

// 构建明文和密钥（使用元素）
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

// 使用 NonEmptySliceRef 保证密钥非空（编译期检查）
let ne_key = NonEmptySliceRef::new(&key).unwrap();

// 加密 - 不会再有运行时空检查！
let encrypted = cipher.encrypt(&plaintext, ne_key).unwrap();
// 结果: RIJVS

// 解密
let decrypted = cipher.decrypt(&encrypted, ne_key).unwrap();
// 结果: HELLO
```

### 示例 2：使用泛型密码器（DigitElement）

```rust
use vigenere::{DigitElement, VigenereCipher, NonEmptyVec, NonEmptySliceRef};

// 数字密码器（0-9）
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

let encrypted = cipher.encrypt(
    &plaintext, 
    NonEmptySliceRef::new(&key).unwrap()
).unwrap();
// 结果: [5, 7, 9] (对应 '5', '7', '9')
```

### 示例 3：使用便捷的字符串接口

```rust
// 使用预定义字符集
let cipher = StringCipher::uppercase_alpha();
let encrypted = cipher.encrypt("HELLO", "KEY").unwrap();
// 结果: "RIJVS"

// 自定义字符集
let cipher = StringCipher::new("0123456789").unwrap();
let encrypted = cipher.encrypt("123", "456").unwrap();
// 结果: "579"

// 处理混合文本（保留不在字符集中的字符）
let cipher = StringCipher::uppercase_alpha();
let encrypted = cipher.encrypt("HELLO, WORLD!", "KEY").unwrap();
// 结果: "RIJVS, UYVJN!" (逗号、空格、感叹号保持不变)
```

## 🏗️ 架构层次

```
┌─────────────────────────────────────┐
│         main.rs                     │
│    交互式命令行界面                  │
│    • 用户输入/输出                   │
│    • 菜单选择                        │
└──────────────┬──────────────────────┘
               │ uses
┌──────────────▼──────────────────────┐
│         lib.rs                      │
│  ┌─────────────────────────────┐    │
│  │   StringCipher              │   │
│  │   字符串便捷接口            │   │
│  └──────────┬──────────────────┘   │
│             │                      │
│  ┌──────────▼──────────────────┐   │
│  │   CharElement, DigitElement │   │
│  │   具体元素类型实现          │   │
│  └──────────┬──────────────────┘   │
│             │ implements            │
└─────────────┼───────────────────────┘
              │
┌─────────────▼───────────────────────┐
│         core.rs                     │
│  ┌─────────────────────────────┐   │
│  │   trait CipherElement       │   │
│  │   • type Value: PartialEq   │   │
│  │   • fn index() -> usize     │   │
│  │   • fn value() -> Value     │   │
│  └──────────┬──────────────────┘   │
│             │                       │
│  ┌──────────▼──────────────────┐   │
│  │   VigenereCipher<T>         │   │
│  │   泛型密码器 - 纯数学运算    │   │
│  │   E(M,K) = (M+K) mod n      │   │
│  │   D(C,K) = (C-K+n) mod n    │   │
│  └─────────────────────────────┘   │
└─────────────────────────────────────┘
```

## 🚀 运行程序

### 编译并运行

```bash
cargo run
```

### 运行测试

```bash
cargo test
```

### 交互式使用

程序提供了友好的交互式界面：

```
=== 维吉尼亚密码加解密程序 ===

架构设计:
  • core.rs    - 泛型 VigenereCipher<T: CipherElement>
  • lib.rs     - CharElement, DigitElement, StringCipher
  • main.rs    - 交互式用户界面
  • 元素自带索引，值实现 PartialEq

选择字符集:
1. 大写英文字母 (A-Z)
2. 小写英文字母 (a-z)
3. 大小写英文字母
4. 字母+数字
5. 可打印ASCII字符
6. 自定义字符集
0. 退出
```

**注意**：`StringCipher` 为了用户友好性，仍然接受 `&str` 并在内部进行检查。而底层的 `VigenereCipher<T>` 则使用 `NonEmptyVec` 和 `NonEmptySliceRef` 提供类型安全保证。这是**便利性**和**类型安全**的良好平衡。

## 📊 预定义字符集

| 方法 | 字符集 | 说明 |
|------|--------|------|
| `uppercase_alpha()` | A-Z | 大写英文字母（26个） |
| `lowercase_alpha()` | a-z | 小写英文字母（26个） |
| `mixed_alpha()` | A-Za-z | 大小写英文字母（52个） |
| `alphanumeric()` | A-Za-z0-9 | 字母和数字（62个） |
| `printable_ascii()` | ASCII 32-126 | 可打印ASCII字符（95个） |
| `new(charset)` | 自定义 | 任意自定义字符集 |

## 🎨 设计亮点

### 0. 编译期非空保证：类型安全的终极形式

传统实现需要在运行时检查空值：

```rust
// ❌ 传统方式 - 运行时检查
pub fn new(charset: Vec<T>) -> Result<Self, String> {
    if charset.is_empty() {
        return Err("字符集不能为空".to_string());
    }
    // ...
}

pub fn encrypt(&self, text: &[T], key: &[T]) -> Result<Vec<T>, String> {
    if key.is_empty() {
        return Err("密钥不能为空".to_string());
    }
    // ...
}
```

我们的实现将检查提升到**类型层面**：

```rust
// ✅ 现代方式 - 编译期保证
pub fn new(charset: NonEmptyVec<T>) -> Self {
    // 无需检查 - 类型系统已保证非空！
}

pub fn encrypt(&self, text: &[T], key: NonEmptySliceRef<T>) -> Result<Vec<T>, String> {
    // 无需检查 - 类型系统已保证非空！
}
```

**优势**：
- 🚀 **性能**：零运行时开销，无需重复检查
- 🛡️ **安全**：编译期捕获错误，不可能传入空集合
- 📝 **文档**：API 签名即文档，一目了然
- 🧹 **代码简洁**：消除大量 `if` 检查和错误处理

### 1. 元素自带索引

传统实现需要 `HashMap<char, usize>` 来映射字符到索引，而我们的设计中元素本身就包含索引：

```rust
pub struct CharElement {
    value: char,
    index: usize,  // 索引是元素的固有属性
}
```

**优势**：
- 无需额外的映射表
- 更快的索引访问（O(1) 直接访问 vs HashMap 查找）
- 更清晰的概念模型

### 1.5. 值类型分离：PartialEq 的智慧

我们要求 `Value: PartialEq` 而非 `Element: PartialEq`：

```rust
pub trait CipherElement: Clone + Debug {
    type Value: PartialEq;  // 值需要可比较
    
    fn index(&self) -> usize;
    fn value(&self) -> Self::Value;
}
```

**为什么这样设计？**
- **语义清晰**：我们比较的是元素的"值"（如字符 'A'），而不是元素本身
- **灵活性**：元素可以包含额外的元数据（如频率统计），不影响值的比较
- **职责分离**：算法关心索引，用户关心值，两者独立

### 2. 高阶函数抽象

加密和解密的唯一区别是运算方式，我们使用高阶函数优雅地抽象这一差异：

```rust
fn process<F>(&self, input: &[T], key: &[T], operation: F) -> Result<Vec<T>, String>
where
    F: Fn(usize, usize, usize) -> usize,
{
    input.iter().enumerate().map(|(i, element)| {
        let key_element = &key[i % key.len()];
        let new_index = operation(element.index(), key_element.index(), self.modulus);
        self.charset[new_index].clone()
    }).collect()
}
```

### 3. 泛型的可扩展性

只需实现 `CipherElement` trait，就能支持任意类型的密码系统：

```rust
use vigenere::{CipherElement, VigenereCipher, NonEmptyVec, NonEmptySliceRef};

// 示例：二进制元素
#[derive(Debug, Clone, PartialEq)]
pub struct BinaryElement {
    value: bool,
}

impl CipherElement for BinaryElement {
    type Value = bool;
    
    fn index(&self) -> usize {
        self.value as usize  // 0 或 1
    }
    
    fn value(&self) -> Self::Value {
        self.value
    }
}

// 创建二进制密码器
let charset = NonEmptyVec::try_from_vec(vec![
    BinaryElement { value: false },
    BinaryElement { value: true },
]).unwrap();

let cipher = VigenereCipher::new(charset);

// 使用类型安全的密钥
let key = NonEmptyVec::new(BinaryElement { value: true }, vec![]);
cipher.encrypt(&plaintext, NonEmptySliceRef::from_nonempty_vec(&key));
```

## 🧪 测试覆盖

项目包含全面的单元测试：

- ✅ StringCipher 基础加密/解密
- ✅ 自定义字符集
- ✅ 保留未知字符
- ✅ 错误处理（重复字符集、无效字符等）
- ✅ 泛型 VigenereCipher 与 CharElement（使用 NonEmpty 类型）
- ✅ 泛型 VigenereCipher 与 DigitElement（使用 NonEmpty 类型）
- ✅ 元素索引验证
- ✅ **编译期保证**：空字符集和空密钥在编译期就无法创建

运行 `cargo test` 查看所有测试结果。

### 编译期错误示例

以下代码**无法编译**，体现类型安全：

```rust
// ❌ 编译错误：类型不匹配
let empty_vec: Vec<CharElement> = vec![];
let cipher = VigenereCipher::new(empty_vec); 
// 期望 NonEmptyVec<CharElement>，得到 Vec<CharElement>

// ❌ 编译错误：无法创建空的 NonEmptySliceRef
let empty_key: &[CharElement] = &[];
cipher.encrypt(&plaintext, NonEmptySliceRef::new(empty_key).unwrap());
// unwrap() 会 panic，但类型系统已强制检查
```

## 📚 算法原理

维吉尼亚密码是一种多表替换密码：

1. **加密公式**：`C[i] = (P[i] + K[i mod len(K)]) mod n`
2. **解密公式**：`P[i] = (C[i] - K[i mod len(K)] + n) mod n`

其中：
- `P[i]` 是明文的第 i 个元素的索引
- `K[i]` 是密钥的第 i 个元素的索引
- `C[i]` 是密文的第 i 个元素的索引
- `n` 是字符集大小（modulus）

## 🔐 安全性说明

⚠️ **注意**：维吉尼亚密码是一种**历史密码**，已被现代密码分析技术攻破（如 Kasiski 测试、重合指数分析等）。

**本项目仅用于教学和演示目的，请勿用于任何实际的安全加密需求。**

对于实际应用，请使用现代加密算法，如：
- AES（对称加密）
- RSA（非对称加密）
- ChaCha20-Poly1305（流密码）

## 📖 License

MIT License

## 💡 关于 NonEmpty 类型

### 为什么使用 NonEmpty？

维吉尼亚密码的数学定义要求：
1. **字符集不能为空**：`modulus = |charset|`，若 `|charset| = 0` 则除以零
2. **密钥不能为空**：密钥循环使用，若长度为 0 则无法取模

传统方法是运行时检查：

```rust
if charset.is_empty() {
    return Err("字符集不能为空");
}
```

但这有几个问题：
- 每次调用都要检查
- 可能遗漏检查点
- 用户不知道这个前提条件（除非看文档）

### NonEmpty 的优势

使用类型系统表达约束：

```rust
pub fn new(charset: NonEmptyVec<T>) -> Self
```

这样：
1. **API 即文档**：一眼看出需要非空集合
2. **编译期保证**：不可能传入空集合
3. **零开销**：`NonEmptyVec` 在运行时就是普通 `Vec`
4. **更好的错误**：在调用点就发现问题，而非深入函数内部

### 创建 NonEmpty 类型

```rust
use nonempty_tools::{NonEmptyVec, NonEmptySliceRef};

// 方式 1: 从已知非空的 Vec 创建
let charset = vec![/* ... */];
let ne_charset = NonEmptyVec::try_from_vec(charset)?; // 运行时检查一次

// 方式 2: 直接构造（编译期保证）
let ne_charset = NonEmptyVec::new(first_elem, rest_elems);

// 方式 3: 从切片引用
let slice = &[1, 2, 3];
let ne_slice = NonEmptySliceRef::new(slice)?; // 运行时检查一次
```

一旦拥有 `NonEmptyVec` 或 `NonEmptySliceRef`，就可以在整个程序中传递，**不再需要任何检查**。

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

