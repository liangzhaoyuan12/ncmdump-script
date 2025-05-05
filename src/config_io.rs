use std::{collections::HashMap, fs::File, io::{Read, Write}, path::Path};

pub fn createConfigFile() {
    
    // 创建config.json文件
    let mut file = File::create("config.json").expect("无法创建文件");
    // file.write_all(b"{\"cookie\":\"\"}").expect("无法写入文件");
    // println!("config.json文件已创建，请手动填写cookie");
    // println!("按任意键退出");
    let mut config_json = json::object! {
        "path" : "~/Downloads"
    };
    file.write_all(config_json.to_string().as_bytes()).expect("无法写入文件");
    println!("设置写入默认值：\n转换后输出路径：~/Downloads\n转换后文件格式：mp3");
    println!("若更改，请在主菜单的设置内进行修改");
}

pub fn writeConfigFile() {
    // TODO: 写入config.json文件
    let mut file = File::open("config.json").expect("无法打开config文件");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).expect("无法读取config文件");
    let mut config_json = json::parse(&buffer).expect("无法解析config文件");
}
pub fn checkConfigFile() ->Result<(),()> {
    let path = Path::new("config.json");
    if path.exists() {
        return Ok(()); // 文件存在，无需创建，退出检查函数
    }
    println!("未找到config.json文件，进入创建程序");
    return Err(());
}
pub fn getConfigMap() -> HashMap<String, String> {
    let mut config_map = HashMap::new();
    let mut file = File::open("config.json").expect("无法打开config文件");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).expect("无法读取config文件");
    let config_json = json::parse(&buffer).expect("无法解析config文件");
    config_map.insert("path".to_string(), config_json["path"].to_string());
    config_map.insert("filetype".to_string(), config_json["filetype"].to_string());
    return config_map;
}