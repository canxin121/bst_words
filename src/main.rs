// 引入bst库中的BSTree结构体，Order枚举类型，和Word结构体
use bst::bst::BSTree;
use bst::r#type::{Order, Word};
// 引入bst库中的一些工具函数，比如过滤文本中的单词，从文件中读取文本，和将字符串写入文件
use bst::utils::{filter_word, read_from_file, write_str_to_file};
// 引入dialoguer库中的ColorfulTheme结构体，Input结构体，和Select结构体，它们用于创建彩色的主题，输入框，和选择框
use dialoguer::{theme::ColorfulTheme, Input, Select};
// 引入std库中的process模块，它用于处理程序的退出
use std::process;

// 定义程序的主函数
fn main() {
    // 打印欢迎信息，使用ASCII艺术字表示程序的名称
    println!(
        r#"
▄▄▄▄· .▄▄ · ▄▄▄▄▄    ▄▄▌ ▐ ▄▌      ▄▄▄  ·▄▄▄▄  .▄▄ · 
▐█ ▀█▪▐█ ▀. •██      ██· █▌▐█▪     ▀▄ █·██▪ ██ ▐█ ▀. 
▐█▀▀█▄▄▀▀▀█▄ ▐█.▪    ██▪▐█▐▐▌ ▄█▀▄ ▐▀▀▄ ▐█· ▐█▌▄▀▀▀█▄
██▄▪▐█▐█▄▪▐█ ▐█▌·    ▐█▌██▐█▌▐█▌.▐▌▐█•█▌██. ██ ▐█▄▪▐█
·▀▀▀▀  ▀▀▀▀  ▀▀▀      ▀▀▀▀ ▀▪ ▀█▄▀▪.▀  ▀▀▀▀▀▀•  ▀▀▀▀ "#
    );

    // 创建一个彩色的主题，用于美化界面
    let theme = ColorfulTheme::default();

    // 定义一个字符串变量，用于存储文本内容
    let mut context: String;

    // 创建一个布尔变量，用于标记文本是否是JSON格式
    let mut is_json = false;
    // 使用一个无限循环，用于让用户选择读入文本的方式
    loop {
        // 定义一个字符串数组，用于存储三种读入文本的方式
        let choices = ["输入文本", "输入路径", "加载缓存"];
        // 创建一个选择框，用于让用户选择其中一种方式
        let selection = Select::with_theme(&theme)
            .with_prompt("选择读入英文语句方式")
            .default(0)
            .items(&choices[..])
            .interact()
            .expect("无法读取输入");
        // 根据用户的选择，执行相应的操作
        match selection {
            0 => {
                // 如果用户选择输入文本，那么创建一个输入框，用于让用户输入文本内容，并将其赋值给context变量
                context = Input::with_theme(&theme)
                    .with_prompt("输入文本内容")
                    .interact_text()
                    .expect("无法读取输入");
                // 跳出循环
                break;
            }
            1 => {
                // 如果用户选择输入路径，那么创建一个输入框，用于让用户输入纯文本文件的路径，并调用read_from_file函数，从文件中读取文本内容，并将其赋值给context变量
                let path: String = Input::with_theme(&theme)
                    .with_prompt("输入纯文本文件路径")
                    .interact_text()
                    .expect("无法读取输入");
                context = read_from_file(path).ok().unwrap();
                // 跳出循环
                break;
            }
            2 => {
                // 如果用户选择加载缓存，那么创建一个输入框，用于让用户输入JSON文件的路径，并调用read_from_file函数，从文件中读取文本内容，并将其赋值给context变量
                let path: String = Input::with_theme(&theme)
                    .with_prompt("输入json文件路径")
                    .interact_text()
                    .expect("无法读取输入");
                context = read_from_file(path).ok().unwrap();
                // 将is_json变量设为true，表示文本是JSON格式
                is_json = true;
                // 跳出循环
                break;
            }
            _ => {
                // 如果用户选择了其他的选项，那么打印错误信息，并继续循环
                println!("无效的选择，请重新输入(0-1)");
            }
        }
    }
    // 定义一个BSTree<Word>类型的变量，用于存储单词和它们的出现次数
    let mut bstree: BSTree<Word>;
    // 如果文本是JSON格式，那么调用serde_json库中的from_str函数，将文本反序列化为BSTree<Word>类型，并赋值给bstree变量，如果反序列化失败，那么panic结束程序，并打印错误信息
    if is_json {
        bstree = serde_json::from_str(&context).unwrap_or_else(|e| {
            panic!("加载缓存失败,请确保文件为正确json格式.\n{e}");
        });
    } else {
        // 如果文本不是JSON格式，那么创建一个空的BSTree<Word>类型的变量，并赋值给bstree变量
        bstree = BSTree::new();
        // 调用filter_word函数，将文本中的单词过滤出来，并返回一个迭代器
        for word in filter_word(&mut context) {
            // 对于每个单词，调用bstree的insert_fn方法，将单词作为Word类型的结点插入到BST中，如果BST中已经存在该单词，那么调用匿名函数，将该单词的出现次数加一
            bstree.insert_fn(Word::new(word.to_string()), |node| {
                node.data.add();
            })
        }
    }

    // 使用一个无限循环，用于让用户选择菜单中的一项操作
    loop {
        // 定义一个字符串数组，用于存储五种操作的名称
        let options = [
            "输入停用词，删除二叉查找树中的相应结点",
            "遍历二叉查找树，输出每个单词及其出现次数",
            "输入查询词，搜索二叉查找树中的相应结点",
            "保存为缓存",
            "退出程序",
        ];
        // 创建一个选择框，用于让用户选择其中一种操作
        let option = Select::with_theme(&theme)
            .with_prompt("请选择以下四项操作之一：")
            .default(0)
            .items(&options[..])
            .interact()
            .expect("无法读取输入");

        // 根据用户的选择执行相应的操作
        match option {
            0 => {
                // 停用词功能
                // 创建一个输入框，用于让用户输入停用词，停用词是指那些在文本中没有实际意义的词，比如“的”、“了”等
                let mut input: String = Input::with_theme(&theme)
                    .with_prompt("输入停用词，以空格分割.")
                    .interact_text()
                    .expect("无法读取输入");
                // 调用filter_word函数，将输入的停用词过滤出来，并返回一个迭代器
                for word in filter_word(&mut input) {
                    // 对于每个停用词，调用bstree的delete方法，将其从BST中删除，如果删除成功，那么打印成功信息，如果删除失败，那么打印失败信息
                    match bstree.delete(Word::new(word.to_string())) {
                        Ok(_) => {
                            println!("停用词:{word} 删除成功");
                        }
                        Err(_) => {
                            println!("停用词:{word} 不在bstree中.");
                        }
                    }
                }
            }
            1 => {
                // 遍历二叉树
                // 使用一个无限循环，用于让用户选择遍历二叉树的方式
                loop {
                    // 定义一个字符串数组，用于存储三种遍历方式的名称，分别是前序遍历，中序遍历，和后序遍历
                    let choices = ["PreOrder", "InOrder", "PostOrder"];
                    // 创建一个选择框，用于让用户选择其中一种方式
                    let selection = Select::with_theme(&theme)
                        .with_prompt("请选择遍历方式")
                        .default(0)
                        .items(&choices[..])
                        .interact()
                        .expect("无法读取输入");
                    // 根据用户的选择，执行相应的操作
                    match selection {
                        0 => {
                            // 如果用户选择前序遍历，那么调用bstree的traverse方法，传入Order::Pre作为参数，表示按照前序遍历的顺序输出每个结点的数据
                            bstree.traverse(Order::Pre);
                            // 跳出循环
                            break;
                        }
                        1 => {
                            // 如果用户选择中序遍历，那么调用bstree的traverse方法，传入Order::In作为参数，表示按照中序遍历的顺序输出每个结点的数据
                            bstree.traverse(Order::In);
                            // 跳出循环
                            break;
                        }
                        2 => {
                            // 如果用户选择后序遍历，那么调用bstree的traverse方法，传入Order::Post作为参数，表示按照后序遍历的顺序输出每个结点的数据
                            bstree.traverse(Order::Post);
                            // 跳出循环
                            break;
                        }
                        _ => {
                            // 如果用户选择了其他的选项，那么打印错误信息，并继续循环
                            println!("无效的选择，请输入(0-2)");
                        }
                    }
                }
            }
            2 => {
                // 查询单词出现次数
                // 创建一个输入框，用于让用户输入要查询的单词，可以输入多个单词，以空格分割
                let mut input: String = Input::with_theme(&theme)
                    .with_prompt("请输入要查询的单词，以空格分割.")
                    .interact_text()
                    .expect("无法读取输入");

                // 调用filter_word函数，将输入的单词过滤出来，并返回一个迭代器
                for word in filter_word(&mut input) {
                    // 对于每个单词，调用bstree的find方法，将其作为Word类型的结点在BST中查找，如果找到了，那么打印该单词及其出现次数，如果没找到，那么打印未找到的信息
                    if let Some(node) = bstree.find(Word::new(word.to_string())) {
                        println!("{}出现次数: {}", word, node.data.count());
                    } else {
                        println!("{}: 不在bstree中.", word)
                    }
                }
            }
            3 => {
                // 保存为缓存
                // 调用serde_json库中的to_string函数，将bstree序列化为JSON格式的字符串，并赋值给cache变量，如果序列化失败，那么panic结束程序，并打印错误信息
                let cache = serde_json::to_string(&bstree).unwrap();
                // 创建一个输入框，用于让用户输入要保存的文件名，不包含扩展名
                let mut input: String = Input::with_theme(&theme)
                    .with_prompt("请输入要保存为的文件名(不含拓展名).")
                    .interact_text()
                    .expect("无法读取输入");
                // 如果输入的文件名不以.json结尾，那么在文件名后面加上.json
                if !input.ends_with(".json") {
                    input += ".json";
                }
                // 调用write_str_to_file函数，将cache变量中的字符串写入到指定的文件中，如果写入失败，那么panic结束程序，并打印错误信息
                write_str_to_file(input, &cache).unwrap();
            }
            4 => {
                // 退出程序
                // 调用process模块中的exit函数，传入0作为参数，表示正常退出程序
                process::exit(0);
            }
            _ => {
                // 不应该发生，但如果发生了，提示用户重新输入
                println!("无效的选择,请重新输入(0-3)");
            }
        }
    }
}
