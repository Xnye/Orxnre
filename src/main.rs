mod color;

use std::io;
use colored::*;
use console::{Key, Term};
use Key::Char;

struct Buffer {
    buffer: ColoredString,
}

impl Buffer {
    fn w<T: ToString>(&mut self, text: T) { self.buffer = format!("{}{}", self.buffer, text.to_string()).white(); }
    fn wl<T: ToString>(&mut self, text: T) { self.buffer = format!("{}{}\n", self.buffer, text.to_string()).white(); }

    fn print(&mut self) {
        println!("{}", self.buffer);
        self.buffer = ColoredString::default();
    }
}

fn cls() {
    let t = Term::stdout();
    t.clear_screen().expect("Print Error: cls");
}

fn read() -> io::Result<Key> {
    Term::stdout().read_key()
}

fn main() {
    const VERSION: &str = "v1.0-beta.3";

    let mut b = Buffer { buffer: ColoredString::default() };

    let menu = vec![
        "开始游戏".to_string(),
        "退出游戏".to_string(),
    ];

    let mut menu_sel = 0;
    let mut menu_next = 0;

    loop {
        b.wl(format!("{Title} | {VERSION}", Title = color::title()));
        for (index, selected) in menu.iter().enumerate() {
            if menu_sel == index as i8 {
                b.wl(format!(" < {} > ", selected).on_white().black());
            } else {
                b.wl(format!("   {}   ", selected));
            }
        }

        b.print();

        if let Ok(key) = read() {
            match key {
                Key::ArrowUp | Char('w') | Char('W') => menu_next = menu_next - 1,
                Key::ArrowDown | Char('s') | Char('S') => menu_next = menu_next + 1,
                _ => {
                    cls();
                    continue;
                }
            }
            if 0 <= menu_next && menu_next < menu.len() as i8 {
                menu_sel = menu_next;
            } else { menu_next = menu_sel; }
        }

        cls();
    }
}