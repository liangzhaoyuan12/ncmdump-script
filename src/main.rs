use std::{alloc::System, process::{Command,self,exit}};
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use std::io::stdin;
use json;
use std::collections::HashMap;
mod setting;
mod folder_dump;
mod file_dump;
mod config_io;

use config_io::*;
use setting::*;
use folder_dump::*;
use file_dump::*;

// static mut config_map: HashMap<String, String> = HashMap::new();

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
    if !(os == "linux" && arch == "x86_64" || os == "windows" && arch == "x86_64"){
        eprintln!("当前系统不支持，请更换系统或架构");
        println!("按任意键退出");
        stdin().read_line(&mut String::new()).unwrap();
        exit(1);
    }
    println!("对ncmdump程序进行封装，学习用途");
    match checkConfigFile() {
        Ok(_) => {
            println!("config.json文件存在，进入主菜单");
            
            mainmenu();
        },
        Err(_) => {
            println!("config.json文件不存在，进入创建程序");
            createConfigFile();
        }
    }
}




fn mainmenu() {
    println!("1.选择单个ncm文件进行转换\n2.选择单个文件夹进行批量转换\n3.设置\n4.退出");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    
    loop {
        match choice.trim() {
            "1" => {
                // 选择单个ncm文件进行转换
            },
            "2" => {
                // 选择单个文件夹进行批量转换
            },
            "3" => {
                // 设置
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