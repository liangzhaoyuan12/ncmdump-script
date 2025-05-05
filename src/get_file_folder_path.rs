// use native_dialog::{FileDialog, MessageDialog, MessageType};
use native_dialog::DialogBuilder;
use std::path::PathBuf;
// 文件夹只支持单个文件夹
// 文件可支持多个文件
// fn main() {
    
// }

pub fn get_ncm(is_folder:bool) -> Vec<PathBuf>{
    let mut choice = String::new();
    println!("1.使用系统API选择文件(推荐使用)\n2.输入路径地址(不推荐使用,适合没有桌面gui的系统环境使用)");
    std::io::stdin().read_line(&mut choice).unwrap();
    let choice:u8 = choice.trim().parse().expect("输入非数字");
    match  choice{
        1 => {
            return match is_folder {
                true => get_ncm_folder(),
                false => get_ncm_files(),
            }        
        },
        2 => {
            println!("输入路径,只能输入一个路径，路径不能包含空格和双引号，输入的路径要绝对路径");
            let mut path = String::new();
            std::io::stdin().read_line(&mut path).unwrap();
            return vec![PathBuf::from(path.trim())]; // 此处数组只有一个内容
        },
        _ => {panic!("输入有误");}
    }
}

fn get_ncm_files() -> Vec<PathBuf> {
    let path = DialogBuilder::file()
    .set_location("~/Desktop") // 设置默认路径为桌面
   .set_title("选择ncm文件")
   .add_filter("ncm文件", &["ncm"]) // 添加文件类型过滤器
   .open_multiple_file()
   .show()
//    .unwrap(); // 处理错误
    .expect("选择文件失败");
    return path;
}
fn get_ncm_folder() -> Vec<PathBuf>{
    let path = DialogBuilder::file()
    .set_location("~/Desktop") // 设置默认路径为桌面
   .set_title("选择ncm文件")
   .add_filter("ncm文件", &["ncm"]) // 添加文件类型过滤器
   .open_single_dir()
   .show()
//    .unwrap(); // 处理错误
    .expect("选择文件夹失败");
    if let Some(path) = path  {
        return [path].to_vec();
    }else {
        panic!("选择文件夹失败");
    }
}