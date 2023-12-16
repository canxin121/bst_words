//! # 文件操作模块
//!
//! 该模块提供了基本的文件操作功能，包括将字符串写入文件、从文件中读取字符串以及从字符串中过滤单词等功能。
//!
//! ## 示例
//!
//! ```rust
//! use crate::file_operations::{write_str_to_file, read_from_file, filter_word};
//!
//! #[test]
//! fn test_write() {
//!     write_str_to_file("data.txt", "Hello, canxin!").unwrap();
//! }
//!
//! #[test]
//! fn test_read() {
//!     let data = read_from_file("data.txt").unwrap();
//!     println!("{}", data);
//! }
//!
//! #[test]
//! fn test_filter() {
//!     let mut context = "This is a sample text with some repeated words. Some words may appear more than once. Let's test the BST program.".to_string();
//!     for (index, word) in filter_word(&mut context).enumerate() {
//!         println!("{}:{}", index, word);
//!     }
//! }
//! ```
//!
//! ## 函数
//!
//! - `write_str_to_file`: 将字符串写入指定文件。
//! - `read_from_file`: 从指定文件中读取字符串。
//! - `filter_word`: 从字符串中过滤出单词。
//!
//! ## 使用注意事项
//!
//! 请确保文件操作时的路径和文件名正确，以避免出现意外的错误。
//!
//! ## 示例
//!
//! ```rust
//! // 写入文件示例
//! write_str_to_file("data.txt", "Hello, canxin!").unwrap();
//!
//! // 读取文件示例
//! let data = read_from_file("data.txt").unwrap();
//! println!("{}", data);
//!
//! // 过滤单词示例
//! let mut context = "This is a sample text with some repeated words. Some words may appear more than once. Let's test the BST program.".to_string();
//! for (index, word) in filter_word(&mut context).enumerate() {
//!     println!("{}:{}", index, word);
//! }
//! ```
//!
//! 注意：以上示例中的文件路径仅供参考，实际使用时应根据项目目录结构进行调整。
//!
//! ## 附加信息
//!
//! 本模块使用了 `anyhow` 库来处理可能的错误类型，请确保在项目的 `Cargo.toml` 文件中添加了相应的依赖。
//!
//! ```toml
//! [dependencies]
//! anyhow = "1.0"
//! ```
//!
//! 请根据实际需要修改版本号。
//!
//! ## 注意
//!
//! 在进行文件操作时，请确保对文件路径和权限有充分的了解，并小心处理可能的错误情况。
//! 否则可能导致数据丢失或其他不可逆的问题。
//! 使用本模块时，请根据实际场景谨慎处理文件路径和内容。
//!
//! 若有疑问或需要帮助，请参考 Rust 官方文档或向相关社区寻求支持。
// 引入anyhow库中的Error类型，它用于表示任何可能的错误
use anyhow::Error;
// 引入std库中的fs模块，它用于处理文件系统的操作，比如创建目录，打开文件等
use std::fs::{self, OpenOptions};
// 引入std库中的io模块，它用于处理输入输出的操作，比如读写文件，缓冲区等
use std::io::{BufReader, BufWriter, Read, Write};
// 引入std库中的path模块，它用于处理路径的操作，比如获取父目录，判断是否是文件等
use std::path::Path;

/// 将字符串写入指定文件。
///
/// # 参数
///
/// * `path`: 泛型类型 `P`，必须实现 `AsRef<Path>` trait，表示文件路径。
/// * `s`: 要写入的字符串的引用。
///
/// # 返回
///
/// 返回 `Result<(), Error>` 类型。如果成功，返回空元组；否则返回 `Error`，表示错误原因。
pub fn write_str_to_file<P: AsRef<Path>>(path: P, s: &str) -> Result<(), Error> {
    let path = path.as_ref();
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent)?;
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;
    let mut writer = BufWriter::new(file);
    writer.write_all(s.as_bytes())?;
    Ok(())
}

/// 从指定文件中读取字符串。
///
/// # 参数
///
/// * `path`: 泛型类型 `P`，必须实现 `AsRef<Path>` trait，表示文件路径。
///
/// # 返回
///
/// 返回 `Result<String, Error>` 类型。如果成功，返回读取的字符串；否则返回 `Error`，表示错误原因。
pub fn read_from_file<P: AsRef<Path>>(path: P) -> Result<String, Error> {
    let file = OpenOptions::new().read(true).open(path)?;
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    reader
        .get_mut()
        .read_to_string(&mut buf)
        .unwrap_or_else(|err| {
            buf = "{}".to_string();
            2
        });
    Ok(buf)
}

/// 从字符串中过滤出单词。
///
/// # 参数
///
/// * `context`: 可变引用的字符串，将对其进行单词过滤。
///
/// # 返回
///
/// 返回实现了 `Iterator<Item = &str>` trait 的类型，表示一个迭代器，可以遍历字符串中的单词，每个单词是一个 `&str` 类型的引用。
pub fn filter_word(context: &mut String) -> impl Iterator<Item = &str> {
    *context = context.to_lowercase();
    context
        .split(|c: char| !c.is_ascii_alphabetic())
        .filter(|s| !s.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write() {
        write_str_to_file("data.txt", "Hello, canxin!").unwrap();
    }

    #[test]
    fn test_read() {
        let data = read_from_file("data.txt").unwrap();
        println!("{}", data);
    }

    #[test]
    fn test_filter() {
        let mut context =
            "This is a sample text with some repeated words. Some words may appear more than once. Let's test the BST program."
                .to_string();
        for (index, word) in filter_word(&mut context).enumerate() {
            println!("{}:{}", index, word);
        }
    }
}
