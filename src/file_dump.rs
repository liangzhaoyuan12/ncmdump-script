use crate::config_io::getConfigMap;
use std::{path::{Path, PathBuf}, process};
use include_dir::{include_dir, Dir, File};
static LIB_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/lib");
static mut IS_FOLDER:bool=false;
// folder只支持单个，不支持多个进行转换
pub fn file_dump(is_folder:bool, ncm_path:Vec<PathBuf>) {
    unsafe {
        IS_FOLDER= is_folder;
    }
    // let mut ncm_path_total:String = String::new();
    // for (index, i) in ncm_path.iter().enumerate() {
    //     let buf = format!("{}",i.to_str().unwrap());
    //     // ncm_path_total.push_str(i.to_str().unwrap());
    //     ncm_path_total.push_str(&buf);
    //     if index != (ncm_path.len()-1) { // 判定在不是最后一个元素的时候才添加空格
    //         ncm_path_total.push_str(" ");
    //     }
    // }
    // let os = std::env::consts::OS;
    // let arch = std::env::consts::ARCH;
    let map= getConfigMap();
    println!("保存路径：\n1.配置文件内保存的地址：{}\n2.转换文件/文件夹所在的源地址",map.get("path").unwrap());
    let mut safe_choice = String::new();
    std::io::stdin().read_line(&mut safe_choice).expect("输入数字有误");
    let choice:u8 = safe_choice.trim().parse().unwrap();
    if choice!=1 && choice!=2 {
        eprintln!("输入数字有误");
        return;
    }
    // match (os, arch) {
    //     ("windows", "x86_64") => file_dump_win(choice,&ncm_path_total),
    //     ("macos", "aarch64") => file_dump_mac_arm64(choice,&ncm_path_total),
    //     ("macos", "x86_64") => file_dump_mac_x64(choice,&ncm_path_total),
    //     ("linux", "x86_64") => file_dump_linux_x64(choice,&ncm_path_total),
    //     _ => println!("Unsupported OS/Arch: {} {}", os, arch),
    // }
    // if os!="windows" {
        
    // }
    // 使用了条件编译，根据不同的操作系统和架构选择不同的函数
    file_dump_sys(choice,ncm_path);
}
#[cfg(all(target_os = "windows" , target_arch = "x86_64"))]
fn file_dump_sys(safe_choice: u8,ncm_path:Vec<PathBuf> ) {
    println!("进入windows_x86系统模块");
    let current_dir = std::env::current_dir().unwrap();
    let mut src_path = PathBuf::from(current_dir);
    src_path.push("lib");
    src_path.push("ncmdump.exe");
    println!("{:?}", src_path);
    let (file_folder_arg,safe_args) = get_args(safe_choice,ncm_path_total);
    println!("{:?}\n{:?}", file_folder_arg,safe_args);
// 先file_folder_arg后safe_args
    println!("进入转换程序");
    println!("{:?} {:?} {:?}", src_path, file_folder_arg, safe_args);

    let output = process::Command::new(src_path);
    if IS_FOLDER{
        let output= output.arg("-d")
        .arg(ncm_path[0])
       .arg("-r");
    }else{
        for i in ncm_path{
            let output=output.arg(i);
        }
    }
    let output=output.output()
    .expect("err");
    //     .arg(file_folder_arg)
    //     .arg(safe_args)
    //     .output()
    //     .expect("执行转换指令错误");
    println!("{:?}", output.stdout)

}
#[cfg(all(target_os = "macos" , target_arch = "aarch64"))]
fn file_dump_sys(choice: u8,ncm_path_total:&str) {

    let current_dir = std::env::current_dir().unwrap();
    let mut src_path = PathBuf::from(current_dir);
    src_path.push("lib");
    src_path.push("ncmdump_mac_arm64");
    unix_add_execute_permission(&src_path).expect("添加unix执行权限错误");
    
}
#[cfg(all(target_os = "macos" , target_arch = "x86_64"))]
fn file_dump_sys(choice: u8,ncm_path_total:&str) {
    
    let current_dir = std::env::current_dir().unwrap();
    let mut src_path = PathBuf::from(current_dir);
    src_path.push("lib");
    src_path.push("ncmdump_mac_x64");
    unix_add_execute_permission(&src_path).expect("添加unix执行权限错误");

}
#[cfg(all(target_os = "linux" , target_arch = "x86_64"))]
fn file_dump_sys(choice: u8,ncm_path_total:&str) {
    
    let current_dir = std::env::current_dir().unwrap();
    let mut src_path = PathBuf::from(current_dir);
    src_path.push("lib");
    src_path.push("ncmdump_linux_x64");
    unix_add_execute_permission(&src_path).expect("添加unix执行权限错误");

}
#[cfg(all(target_os = "linux", target_os = "macos"))]
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
// return (file_folder_arg,safe_args);
fn get_args(safe_choice: u8,ncm_path_total:&str) ->(String,String) {
    let mut safe_args = String::new();
    // 设定保存路径
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
    return (file_folder_arg,safe_args);
}