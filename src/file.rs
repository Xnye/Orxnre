use std::{env, path, fs, io::{Write, Read, BufReader, BufWriter}};
use console::Key::Char;
use serde::{Serialize, Deserialize};
use serde_json;
use zip::*;
use zip::write::FileOptions;
use crate::{cls_pro, data, file, read};

pub fn path() -> (String, bool) {
    match env::current_exe() {
        Ok(path) => {
            let exe_dir = path.parent().unwrap();
            let absolute_path = exe_dir.to_string_lossy();
            (absolute_path.to_string(), true)
        }
        Err(e) => (format!("ERROR ({})", e), false),
    }
}

// 保存
pub fn s<T: Serialize>(file_path: String, data: T) -> result::ZipResult<()> {
    let json = serde_json::to_string(&data).map_err(|_| result::ZipError::FileNotFound)?;
    let file = fs::File::create(file_path)?;
    let mut zip = ZipWriter::new(BufWriter::new(file));
    let options: FileOptions<()> = FileOptions::default().compression_method(CompressionMethod::Deflated);
    
    zip.start_file("data.json", options)?;
    zip.write_all(json.as_bytes())?;
    zip.finish()?;
    
    Ok(())
}

// 读取
pub fn l<T: for<'de> Deserialize<'de>>(file_path: String) -> result::ZipResult<T> {
    let file = fs::File::open(&file_path)?;
    let mut archive = ZipArchive::new(BufReader::new(file))?;

    let mut file = archive.by_name("data.json")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let data: T = serde_json::from_str(&content).map_err(|_| result::ZipError::FileNotFound)?;
    Ok(data)
}

// 检测存档是否存在
pub fn load_data() -> String {
    let p = format!("{}\\save.zip", &file::path().0);
    let d = "Disabled".to_string();
    if file::path().1 && path::Path::new(&p).exists() { p } else { d }
}

// 创建存档文件
pub fn create_save_wizard() -> String {
    let mut save_path = load_data();
    
    while save_path == "Disabled" {
        cls_pro();
        println!("{} | {}\n", data::TITLE(), data::VERSION);
        println!("未检测到存档文件 你可能需要创建一个");
        println!("位置: {}\\save.zip", file::path().0);
        println!("[ Y 创建 ] [ N 取消 ]\n");
        if let Ok(key) = read() {
            match key {
                Char('y') | Char('Y') => {
                    save_path = format!("{}\\save.zip", file::path().0);
                    match s(save_path.clone(), 0) {
                        Ok(_) => {}
                        Err(e) => { println!("ERROR ({})", e); _ = read(); }
                    };
                    break;
                }
                Char('n') | Char('N') => { break; }
                _ => {}
            }
        }
    }
    save_path
}