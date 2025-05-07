use std::{alloc::System, process::{Command,self,exit}};
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use std::io::stdin;
use json;
use std::collections::HashMap;
use include_dir::{include_dir, Dir};
mod setting;
mod file_dump;
mod config_io;
mod get_file_folder_path;

use crate::config_io::*;
use get_file_folder_path::*;

// static mut config_map: HashMap<String, String> = HashMap::new();
static LIB_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/lib");
fn main() {
    init();
}

fn init() {
    // 系统类型（OS类型）
    println!("OS: {}", std::env::consts::OS);      // "linux", "windows", "macos" 等
    // 系统架构（CPU架构）
    println!("Arch: {}", std::env::consts::ARCH);  // "x86_64", "aarch64", "arm", "x86" 等
    println!("支持列表：\n1.Linux_x64 2.Windows_x64 3.MacOS_x64 4.MacOS_arm64");
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;
    if !(os == "linux" && arch == "x86_64" || os == "windows" && arch == "x86_64" || os == "macos"){
        eprintln!("当前系统不支持，请更换系统或架构");
        println!("按任意键退出");
        stdin().read_line(&mut String::new()).unwrap();
        exit(1);
    }
    println!("对ncmdump程序进行封装，学习用途");
    println!("项目地址：https://github.com/liangzhaoyuan12/ncmdump-script");
    println!("作者：liangzhaoyuan12");
    match checkConfigFile() {
        Ok(_) => {
            println!("config.json文件存在，进入主菜单");
            
            mainmenu();
        },
        Err(_) => {
            println!("config.json文件不存在，进入创建程序");
            createConfigFile();
            mainmenu();
        }
    }
}




fn mainmenu() {
    
    loop {
        println!("1.选择多个ncm文件进行转换\n2.选择单个文件夹进行批量转换\n3.设置\n4.退出");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
    
        match choice.trim() {
            "1" => {
                // 选择多个ncm文件进行转换
                let ncm_files = get_ncm(false);
                // .iter().map(|p| p.as_path()).collect();
                file_dump::file_dump(false, ncm_files);
            },
            "2" => {
                // 选择单个文件夹进行批量转换
                let ncm_folders = get_ncm(true);
                file_dump::file_dump(true, ncm_folders);
            },
            "3" => {
                setting::setting();
            },
            "4" => {
                // 退出
                exit(0);
            }
            _ => {
                println!("输入错误，请重新输入");
            }
        }
    }
    
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//         print!("hello world");
//     }
// }