mod bag;
mod battle;
mod data;
mod game;
mod shop;

use colored::*;
use console::{Key, Style, Term};
use rand::{thread_rng, Rng};
use std::net::Shutdown::Read;
use std::{io, ops::Range, process::exit, thread, time};
use Key::Char;

// 缓冲区
struct Buffer {
    buffer: ColoredString,
    modified: bool,
}

impl Buffer {
    // 新建
    fn new() -> Self {
        Buffer {
            buffer: ColoredString::default(),
            modified: false,
        }
    }
    // 写入内容
    fn w<T: ToString>(&mut self, text: T) {
        self.modified = true;
        self.buffer = format!("{}{}", self.buffer, text.to_string()).white();
    }
    // 写入内容+换行
    fn wl<T: ToString>(&mut self, text: T) {
        self.modified = true;
        self.buffer = format!("{}{}\n", self.buffer, text.to_string()).white();
    }
    // 清除内容
    fn cls(&mut self) {
        self.buffer = ColoredString::default()
    }
    // 读取 返回 ColoredString
    fn read(&mut self) -> ColoredString {
        self.buffer.clone()
    }
    // 打印到屏幕并清除
    fn print(&mut self) {
        println!("{}", self.buffer);
        self.cls();
    }
}

// 清屏 移动光标到(0, 0) 下次打印覆盖
fn cls() {
    Term::stdout()
        .move_cursor_to(0, 0)
        .expect("Print Error: move cur");
}

// 真·清屏
fn cls_pro() {
    Term::stdout().clear_screen().expect("Print Error: cls");
}

// 获取按键
fn read() -> io::Result<Key> {
    Term::stdout().read_key()
}

fn random(r: Range<i32>) -> i32 {
    thread_rng().gen_range(r)
}

fn time_sleep(ms: u64) {
    thread::sleep(time::Duration::from_millis(ms));
}

// 主程序
fn main() {
    let mut b = Buffer::new(); // 主要缓冲区

    let mut enable_debug = false;

    let menu = [
        "开始游戏".to_string(),
        "  关于  ".to_string(),
        "退出程序".to_string(),
    ];

    let mut menu_highlighted = 0; // 高亮位置
    let mut menu_next = 0; // 防溢出
    let mut menu_selected = -1; // 回车后选中的高亮位置
    let menu_len = menu.len() as i8;

    print!("{}", Style::new().apply_to("")); // 应用虚拟终端 (狗皮膏药: 暂时解决Win10默认终端不适配ANSI转义的问题)
    cls_pro();

    loop {
        b.wl(format!("{} | {}", data::TITLE(), data::VERSION));

        b.wl("\n** ↑↓ 移动 Enter 选中\n** 游戏内用 H 键查看提示\n".truecolor(150, 150, 150));

        // 打印选项并高亮选中项
        for (index, selected) in menu.iter().enumerate() {
            if menu_highlighted == index as i8 {
                b.wl(format!(" < {} > ", selected).on_white().black());
            } else {
                b.wl(format!("   {}   ", selected));
            }
        }
        b.print();

        // 处理按键
        if let Ok(key) = read() {
            match key {
                Key::ArrowUp | Char('w') | Char('W') => menu_next -= 1,
                Key::ArrowDown | Char('s') | Char('S') => menu_next += 1,
                Key::Enter => menu_selected = menu_highlighted,
                _ => {
                    cls();
                    continue;
                }
            }
            // 如果不溢出则移动高亮位置
            if 0 <= menu_next && menu_next < menu_len {
                menu_highlighted = menu_next;
            } else {
                menu_next = menu_highlighted;
            }
        }

        // Enter 选中
        if menu_selected >= 0 && menu_selected < menu_len {
            match menu[menu_selected as usize].as_str() {
                "开始游戏" => {
                    println!("** 地图生成中...");
                    game::main(enable_debug);
                    cls_pro();
                    break;
                }
                "  关于  " => {
                    cls_pro();
                    println!("{} | {}\n", data::TITLE(), data::VERSION);
                    println!("\"由于学业 本人暂时无法投入精力在该游戏上");
                    println!(" 更新可能会迟到但不会缺席 咕咕咕咕咕咕咕\"");
                    println!("                                     Xnye");
                    println!("                                  2024/07");
                    println!("联系: bilibili@一块Yc");
                    println!("网站: orxnre.github.io");
                    println!(
                        "按任意键继续 (输入D{}调试)",
                        if enable_debug { "禁用" } else { "启用" }
                    );
                    if let Ok(key) = read() {
                        match key {
                            Char('d') | Char('D') => enable_debug = !enable_debug,
                            _ => {}
                        }
                    }
                    cls_pro();
                }
                "退出程序" => {
                    cls_pro();
                    exit(0);
                }
                _ => {}
            }
        }
        menu_selected = -1;
        cls();
    }
}
