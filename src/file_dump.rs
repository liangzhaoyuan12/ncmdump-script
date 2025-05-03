use crate::config_io::getConfigMap;
use std::{path::{Path, PathBuf}, process};

static mut IS_FOLDER:bool=false;
pub fn file_dump(is_folder:bool, ncm_path:&[&Path]) {
    unsafe {
        IS_FOLDER= is_folder;
    }
    let mut ncm_path_total:String = String::new();
    for i in ncm_path {
        ncm_path_total.push_str(i.to_str().unwrap());
        ncm_path_total.push_str(" ");
    }
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;
    let map= getConfigMap();
    println!("保存路径：\n1.配置文件内保存的地址：{}\n2.转换文件/文件夹所在的源地址",map.get("path").unwrap());
    let mut safe_choice = String::new();
    std::io::stdin().read_line(&mut safe_choice).expect("输入数字有误");
    let choice:u8 = safe_choice.trim().parse().unwrap();
    if choice>=3 {
        return;
    }
    match (os, arch) {
        ("windows", "x86_64") => file_dump_win(choice,&ncm_path_total),
        ("macos", "aarch64") => file_dump_mac_arm64(choice,&ncm_path_total),
        ("macos", "x86_64") => file_dump_mac_x64(choice,&ncm_path_total),
        ("linux", "x86_64") => file_dump_linux_x64(choice,&ncm_path_total),
        _ => println!("Unsupported OS/Arch: {} {}", os, arch),
    }
}

fn file_dump_win(safe_choice: u8,ncm_path_total:&str ) {
    
    let current_dir = std::env::current_dir().unwrap();
    let mut src_path = PathBuf::from(current_dir);
    src_path.push("lib");
    src_path.push("ncmdump.exe");
    let mut safe_args = String::new();
    if safe_choice == 1 {
        safe_args = format!("-o {}",getConfigMap().get("path").unwrap());
    }
    else {
        safe_args = format!("");
    }
    let mut file_folder_arg = String::new();
    if unsafe {IS_FOLDER}{
        file_folder_arg = format!("-d {} -r",ncm_path_total); // 转换ncm文件夹 开启递归,转换整个文件夹内的所有文件
    }else {
        file_folder_arg = format!("{}",ncm_path_total); // 转换ncm文件
    }
// 先file_folder_arg后safe_args
    println!("进入转换程序");
    let output = process::Command::new(src_path)
        .arg(file_folder_arg)
        .arg(safe_args)
        .output()
        .expect("执行转换指令错误");
    

}
fn file_dump_mac_arm64(choice: u8,ncm_path_total:&str) {

    let current_dir = std::env::current_dir().unwrap();
    let mut src_path = PathBuf::from(current_dir);
    src_path.push("lib");
    src_path.push("ncmdump_mac_arm64");
    unix_add_execute_permission(&src_path).expect("添加unix执行权限错误");
    
}
fn file_dump_mac_x64(choice: u8,ncm_path_total:&str) {
    
    let current_dir = std::env::current_dir().unwrap();
    let mut src_path = PathBuf::from(current_dir);
    src_path.push("lib");
    src_path.push("ncmdump_mac_x64");
    unix_add_execute_permission(&src_path).expect("添加unix执行权限错误");

}
fn file_dump_linux_x64(choice: u8,ncm_path_total:&str) {
    
    let current_dir = std::env::current_dir().unwrap();
    let mut src_path = PathBuf::from(current_dir);
    src_path.push("lib");
    src_path.push("ncmdump_linux_x64");
    unix_add_execute_permission(&src_path).expect("添加unix执行权限错误");

}

fn unix_add_execute_permission(absolute_path: &PathBuf) ->Result<(),Box<dyn std::error::Error>>
// where E:From<std::io::Error> + From<String>
{
    let status= process::Command::new("chmod")
        .arg("+x")
        .arg(absolute_path)
        .status()?;
    if !(status.success()) {
        return Err("chmod failed".to_string().into());
    }
    return Ok(());
}