use std::{collections::HashMap, fs::{File, OpenOptions}, io::{Read, Write}, path::Path, rc::Rc};

pub fn createConfigFile() {
    
    // 创建config.json文件
    let mut file = File::create("config.json").expect("无法创建文件");
    // file.write_all(b"{\"cookie\":\"\"}").expect("无法写入文件");
    // println!("config.json文件已创建，请手动填写cookie");
    // println!("按任意键退出");
    #[cfg(target_os = "windows")]
    let home = std::env::var("USERPROFILE").ok();
    
    #[cfg(not(target_os = "windows"))]
    let home = std::env::var("HOME").ok();
    
    let home=home.map(|h| Path::new(&h).join("Downloads"));
    // let default_path = Path::new(std::env::var("HOME").unwrap().as_str()).join("Downloads");
    println!("{:?}",home);
    let config_json = json::object! {
        "path" : home.clone().unwrap().to_str().unwrap().to_string(),
    };
    file.write_all(config_json.to_string().as_bytes()).expect("无法写入文件");
    println!("设置写入默认值：\n转换后输出路径：{}",home.unwrap().to_str().unwrap().to_string());
    println!("若更改，请在主菜单的设置内进行修改");
}

pub fn writeConfigFile(safePath: String) -> Result<(),std::string::String> {
    // TODO: 写入config.json文件
    let mut file = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .truncate(true)  // 清空文件内容
    .open("config.json")
    .map_err(|e| format!("无法打开/创建config文件: {}", e))?;    // let mut buffer = String::new();
    // file.read_to_string(&mut buffer).expect("无法读取config文件");
    // let mut config_json = json::parse(&buffer).expect("无法解析config文件");
    let safePath = Rc::new(safePath);
    let trim_path = safePath.clone().trim().to_string();
    let path = Path::new(&trim_path);
    println!("{:?}",path);
    if !path.exists() {
        println!("路径不存在，请重新输入");
        return Err("路径不存在，请重新输入".to_string());
    }
    let config_json = json::object! {
        "path" : *safePath.trim()
    };
    file.write_all(config_json.to_string().as_bytes()).expect("无法写入文件");
    return Ok(());
}
pub fn checkConfigFile() ->Result<(),()> {
    let path = Path::new("config.json");
    if path.exists() {
        return Ok(()); // 文件存在，无需创建，退出检查函数
    }
    println!("未找到config.json文件，进入创建程序");
    drop(path);
    return Err(());
}
pub fn getConfigMap() -> HashMap<String, String> {
    let mut config_map = HashMap::new();
    let mut file = File::open("config.json").expect("无法打开config文件");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).expect("无法读取config文件");
    let config_json = json::parse(&buffer).expect("无法解析config文件");
    config_map.insert("path".to_string(), config_json["path"].to_string());
    // config_map.insert("filetype".to_string(), config_json["filetype"].to_string());
    return config_map;
}