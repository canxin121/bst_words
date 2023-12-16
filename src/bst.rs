//! # 二叉搜索树
//!
//! 该模块定义了二叉搜索树（Binary Search Tree）的结构体 `BSTree` 和结点的结构体 `BSTNode`，
//! 以及与二叉搜索树相关的操作，包括插入、查找、删除和遍历等。
//!
//! ## 结点结构体 BSTNode
//!
//! 表示二叉搜索树的结点，包括结点的数据、左子结点和右子结点。
//!
//! ### 使用示例
//!
//! ```rust
//! use crate::bstree::{BSTNode, BSTree, Order};
//! let mut bstree = BSTree::new();
//! bstree.insert_fn(1, |n| {});
//! bstree.insert_fn(2, |n| {});
//! let node = bstree.find(1);
//! ```
//!
//! ## 二叉搜索树结构体 BSTree
//!
//! 表示二叉搜索树，包括根结点。
//!
//! ### 使用示例
//!
//! ```rust
//! use crate::bstree::{BSTNode, BSTree, Order};
//! let mut bstree = BSTree::new();
//! bstree.insert_fn(1, |n| {});
//! bstree.insert_fn(2, |n| {});
//! bstree.insert_fn(3, |n| {});
//! bstree.traverse(Order::In);
//! ```
//!
//! ## BSTree 结构体实现的方法
//!
//! - `new() -> BSTree<T>`: 创建一个新的二叉搜索树实例。
//! - `insert_fn(data: T, f: impl FnMut(&mut Box<BSTNode<T>>))`: 插入结点并执行指定操作。
//! - `find(data: T) -> Option<&BSTNode<T>>`: 查找指定数据的结点。
//! - `delete(data: T) -> Result<(), String>`: 删除指定数据的结点。
//! - `traverse(order: Order)`: 遍历二叉搜索树，打印结点数据。
//!
//! ## 使用注意事项
//!
//! - `T` 类型必须实现 `Ord`, `Clone`, `Display` trait。
//!
//! ## 示例
//!
//! ```rust
//! use crate::bstree::{BSTNode, BSTree, Order};
//!
//! let mut bstree = BSTree::new();
//! bstree.insert_fn(1, |n| {});
//! bstree.insert_fn(2, |n| {});
//! bstree.insert_fn(3, |n| {});
//! let node = bstree.find(2);
//! match node {
//!     Some(n) => println!("Found: {}", n.data),
//!     None => println!("Not found."),
//! }
//! bstree.delete(2).unwrap();
//! println!("Deleted.");
//! bstree.traverse(Order::In);
//! ```
//!
// 引入r#type模块中的Order枚举类型，表示二叉树的遍历顺序
use crate::r#type::Order;
// 引入serde库中的Deserialize和Serialize两个trait，它们用于实现JSON的反序列化和序列化功能
use serde::{Deserialize, Serialize};
// 引入std库中的fmt模块，它用于实现格式化输出的功能
use std::fmt::Display;

/// 表示二叉搜索树的结点，包括结点的数据、左子结点和右子结点。
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BSTNode<T> {
    /// 结点的数据
    pub data: T,
    /// 结点的左子结点
    pub left: Option<Box<BSTNode<T>>>,
    /// 结点的右子结点
    pub right: Option<Box<BSTNode<T>>>,
}

impl<T> BSTNode<T> {
    /// 创建一个新的结点实例。
    pub fn new(data: T) -> BSTNode<T> {
        BSTNode {
            data,
            left: None,
            right: None,
        }
    }
}

/// 表示二叉搜索树，包括根结点。
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BSTree<T> {
    /// 树的根结点
    pub root: Option<Box<BSTNode<T>>>,
}

impl<T> BSTree<T>
where
    T: Ord + Clone + Display,
{
    /// 创建一个新的二叉搜索树实例。
    pub fn new() -> BSTree<T> {
        BSTree { root: None }
    }

    /// 插入结点并执行指定操作。
    pub fn insert_fn(&mut self, data: T, mut f: impl FnMut(&mut Box<BSTNode<T>>)) {
        fn insert_node<T: Ord>(
            node: &mut Option<Box<BSTNode<T>>>,
            data: T,
            f: &mut impl FnMut(&mut Box<BSTNode<T>>),
        ) {
            match node {
                None => *node = Some(Box::new(BSTNode::new(data))),
                Some(node) => {
                    if data < node.data {
                        insert_node(&mut node.left, data, f);
                    } else if data > node.data {
                        insert_node(&mut node.right, data, f);
                    } else {
                        f(node);
                    }
                }
            }
        }
        insert_node(&mut self.root, data, &mut f);
    }

    /// 查找指定数据的结点。
    pub fn find(&self, data: T) -> Option<&BSTNode<T>> {
        fn find_node<T: Ord + Display>(
            node: &Option<Box<BSTNode<T>>>,
            data: T,
        ) -> Option<&BSTNode<T>> {
            match node {
                None => None,
                Some(node) => {
                    if data < node.data {
                        find_node(&node.left, data)
                    } else if data > node.data {
                        find_node(&node.right, data)
                    } else {
                        Some(node.as_ref())
                    }
                }
            }
        }
        find_node(&self.root, data)
    }

    /// 删除指定数据的结点。
    pub fn delete(&mut self, data: T) -> Result<(), String> {
        fn delete_node<T: Ord + Clone + Display>(
            node: &mut Option<Box<BSTNode<T>>>,
            data: T,
        ) -> Result<(), String> {
            match node {
                None => Err(format!("Node of data: {data} not found")),
                Some(n) => {
                    if data < n.data {
                        delete_node(&mut n.left, data)
                    } else if data > n.data {
                        delete_node(&mut n.right, data)
                    } else {
                        if n.left.is_none() && n.right.is_none() {
                            *node = None;
                        } else if n.left.is_some() && n.right.is_none() {
                            *node = n.left.take();
                        } else if n.left.is_none() && n.right.is_some() {
                            *node = n.right.take();
                        } else if n.left.is_some() && n.right.is_some() {
                            n.data = min_node(&n.right).unwrap().clone();
                            delete_node(&mut n.right, n.data.clone())?;
                        }
                        Ok(())
                    }
                }
            }
        }

        fn min_node<T>(node: &Option<Box<BSTNode<T>>>) -> Result<&T, String> {
            match node {
                None => Err("Node is empty".to_string()),
                Some(n) => {
                    if n.left.is_none() {
                        Ok(&n.data)
                    } else {
                        min_node(&n.left)
                    }
                }
            }
        }

        delete_node(&mut self.root, data)
    }

    /// 遍历二叉搜索树，打印结点数据。
    pub fn traverse(&self, order: Order) {
        fn traverse_node<T: Ord + Display>(node: &Option<Box<BSTNode<T>>>, order: Order) {
            match node {
                None => return,
                Some(n) => match order {
                    Order::Pre => {
                        print!("{} ", n.data);
                        traverse_node(&n.left, order.clone());
                        traverse_node(&n.right, order);
                    }
                    Order::In => {
                        traverse_node(&n.left, order.clone());
                        print!("{} ", n.data);
                        traverse_node(&n.right, order);
                    }
                    Order::Post => {
                        traverse_node(&n.left, order.clone());
                        traverse_node(&n.right, order);
                        print!("{} ", n.data);
                    }
                },
            }
        }
        traverse_node(&self.root, order);
        println!();
    }
}

/// 测试二叉搜索树
#[test]
fn test_bst() {
    let mut bstree = BSTree::new();
    bstree.insert_fn(1, |n| {});
    bstree.insert_fn(2, |n| {});
    bstree.insert_fn(3, |n| {});
    bstree.insert_fn(4, |n| {});
    let rst = bstree.find(3);
    bstree.delete(3).unwrap();
    println!("Deleted.");
    bstree.traverse(Order::In);
}
