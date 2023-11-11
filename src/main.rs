mod data;
mod game;

use std::io;
use std::process::exit;
use colored::*;
use console::{Key, Term, Style};
use Key::Char;

// 缓冲区和打印相关定义
struct Buffer {
    buffer: ColoredString,
}

impl Buffer {
    fn new() -> Self { Buffer { buffer: ColoredString::default() } }
    fn w<T: ToString>(&mut self, text: T) { self.buffer = format!("{}{}", self.buffer, text.to_string()).white(); }
    fn wl<T: ToString>(&mut self, text: T) { self.buffer = format!("{}{}\n", self.buffer, text.to_string()).white(); }

    fn cls(&mut self) { self.buffer = ColoredString::default() }
    fn read(&mut self) -> ColoredString { self.buffer.clone() }

    fn print(&mut self) {
        println!("{}", self.buffer);
        self.cls();
    }
}

fn cls() {
    Term::stdout().move_cursor_to(0, 0).expect("Print Error: move cur");
}

fn cls_pro() {
    Term::stdout().clear_screen().expect("Print Error: cls");
}

fn read() -> io::Result<Key> {
    Term::stdout().read_key()
}

// 主程序
fn main() {
    // 初始化缓冲区
    let mut b = Buffer { buffer: ColoredString::default() };

    // 菜单相关
    let menu = vec![
        "开始游戏".to_string(),
        "退出程序".to_string(),
    ];

    let mut menu_sel = 0;
    let mut menu_next = 0;
    let mut menu_selected = -1;
    let menu_len = menu.len() as i8;

    // 应用虚拟终端 狗皮膏药 (暂时解决Win10默认终端不适配ANSI转义的问题)
    print!("{}", Style::new().apply_to(""));

    cls_pro();

    loop {
        b.wl(format!("{} | {}", data::TITLE(), data::VERSION));

        // 打印选项并高亮选中项
        for (index, selected) in menu.iter().enumerate() {
            if menu_sel == index as i8 {
                b.wl(format!(" < {} > ", selected).on_white().black());
            } else {
                b.wl(format!("   {}   ", selected));
            }
        }

        b.print();

        // 处理按键
        if let Ok(key) = read() {
            match key {
                Key::ArrowUp | Char('w') | Char('W') => menu_next = menu_next - 1,
                Key::ArrowDown | Char('s') | Char('S') => menu_next = menu_next + 1,
                Key::Enter => menu_selected = menu_sel,
                _ => {
                    cls();
                    continue;
                }
            }
            if 0 <= menu_next && menu_next < menu_len {
                menu_sel = menu_next;
            } else { menu_next = menu_sel; }
        }

        // 处理 Enter 事件
        if menu_selected >= 0 && menu_selected < menu_len {
            match menu[menu_selected as usize].as_str() {
                "开始游戏" => {
                    game::main();
                    cls_pro();
                    break;
                }
                "退出程序" => {
                    cls_pro();
                    exit(0);
                },
                _ => {}
            }
        }

        cls();
    }
}