use crate::config_io::writeConfigFile;
pub fn setting() {
    println!("1. 更改保存路径");
    println!("2. 返回主菜单");

    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).expect("Failed to read line");
    if choice.trim() != "1" && choice.trim() != "2" {
        println!("请输入正确的选项");
        setting();
    }
    match choice.trim() {
        "1" => {
            // TODO: 更改保存路径
            println!("输入默认保存路径");
            println!("请务必绝对路径");
            let mut path = String::new();
            std::io::stdin().read_line(&mut path).expect("Failed to read line");
            if let Err(e) = writeConfigFile(path) {
                println!("{e}");
                println!("保存路径未更改");
            }else {
                println!("保存路径已更改");
            }
        }
        "2" => {
            return;
        }
        _ => {}
    }
}