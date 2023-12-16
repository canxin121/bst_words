//! # 单词和遍历顺序定义
//!
//! 该模块定义了表示单词和遍历顺序的结构体和枚举类型，并实现了一些 trait 用于比较和显示。
//!
//! ## 单词结构体 Word
//!
//! 用于表示单词的结构体，包括单词的值和出现次数。
//!
//! ### 使用示例
//!
//! ```rust
//! use crate::word_order::{Word, Order};
//! let word1 = Word::new("apple".to_string());
//! let word2 = Word::new("apples".to_string());
//! assert_eq!(word1.cmp(&word2), std::cmp::Ordering::Less);
//! assert_eq!(word2.cmp(&word1), std::cmp::Ordering::Greater);
//! assert_eq!(word1.cmp(&word1), std::cmp::Ordering::Equal);
//! ```
//!
//! ## 遍历顺序枚举 Order
//!
//! 用于表示二叉树遍历的顺序，包括前序、中序和后序遍历。
//!
//! ### 使用示例
//!
//! ```rust
//! use crate::word_order::{Word, Order};
//! let order = Order::Pre;
//! ```
//!
//! ## Word 结构体实现的 trait
//!
//! - `PartialOrd`: 部分比较，用于定义单词的大小关系。
//! - `PartialEq`: 判断相等性。
//! - `Ord`: 完全比较，用于排序。
//! - `Display`: 格式化输出，用于打印到屏幕或写入到文件。
//! - `Clone`: 克隆自身。
//!
//! ## Word 结构体的方法
//!
//! - `new(value: String) -> Word`: 创建一个新的 Word 实例。
//! - `add()`: 将单词的出现次数加一。
//! - `count() -> u32`: 获取单词的出现次数。
//!
//! ## 使用注意事项
//!
//! - 在比较单词时，按照字典顺序进行比较。
//! - Word 结构体实现了 `Clone` trait，表示可以克隆自身。
//!
//! ## 示例
//!
//! ```rust
//! use crate::word_order::{Word, Order};
//!
//! // 创建两个单词
//! let word1 = Word::new("apple".to_string());
//! let word2 = Word::new("apples".to_string());
//! // 比较两个单词
//! assert_eq!(word1.cmp(&word2), std::cmp::Ordering::Less);
//! assert_eq!(word2.cmp(&word1), std::cmp::Ordering::Greater);
//! assert_eq!(word1.cmp(&word1), std::cmp::Ordering::Equal);
//!
//! let word3 = Word::new("pear".to_string());
//! let mut word4 = word3.clone();
//! word4.add();
//! assert!(word3 == word4)
//! ```
//!
//! 注意：以上示例中的单词值仅供参考，实际使用时应根据需求替换。
// 引入serde库中的Deserialize和Serialize两个trait，它们用于实现JSON的反序列化和序列化功能
use serde::{Deserialize, Serialize};
// 引入std库中的cmp模块，它用于实现比较大小的功能
use std::{cmp::Ordering, fmt::Display};

/// 表示二叉树的遍历顺序，包括前序、中序和后序遍历。
#[derive(Clone)]
pub enum Order {
    Pre,
    In,
    Post,
}

/// 表示一个单词，包括单词的值和出现次数。
#[derive(Serialize, Deserialize, Eq, Debug, Clone)]
pub struct Word {
    /// 单词的值
    pub value: String,
    /// 单词的出现次数
    count: u32,
}

impl PartialOrd for Word {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_bytes = self.value.as_bytes();
        let other_bytes = other.value.as_bytes();

        for (i, j) in self_bytes.iter().zip(other_bytes.iter()) {
            if i < j {
                return Some(Ordering::Less);
            } else if i > j {
                return Some(Ordering::Greater);
            }
        }

        if self_bytes.len() < other_bytes.len() {
            Some(Ordering::Less)
        } else if self_bytes.len() > other_bytes.len() {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}

impl PartialEq for Word {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Ord for Word {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_bytes = self.value.as_bytes();
        let other_bytes = other.value.as_bytes();

        for (i, j) in self_bytes.iter().zip(other_bytes.iter()) {
            if i < j {
                return Ordering::Less;
            } else if i > j {
                return Ordering::Greater;
            }
        }

        if self_bytes.len() < other_bytes.len() {
            Ordering::Less
        } else if self_bytes.len() > other_bytes.len() {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.value, self.count)
    }
}

impl Word {
    /// 创建一个新的 Word 实例。
    pub fn new(value: String) -> Word {
        Word { value, count: 1 }
    }

    /// 将单词的出现次数加一。
    pub fn add(&mut self) {
        self.count += 1;
    }

    /// 获取单词的出现次数。
    pub fn count(&self) -> u32 {
        self.count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cmp() {
        let word1 = Word::new("apple".to_string());
        let word2 = Word::new("apples".to_string());
        assert_eq!(word1.cmp(&word2), Ordering::Less);
        assert_eq!(word2.cmp(&word1), Ordering::Greater);
        assert_eq!(word1.cmp(&word1), Ordering::Equal);

        let word3 = Word::new("pear".to_string());
        let mut word4 = word3.clone();
        word4.add();
        assert!(word3 == word4)
    }
}
