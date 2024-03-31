use crate::{data, game, Buffer, read, cls};
use game::Player;
use console::Key::*;
use colored::*;

struct Good {
    name: String,
    price: i32,
}
impl Good {
    fn new(name: &str, price: i32) -> Self {
        Good { name: name.to_string(), price }
    }
}

pub fn main(mut a: Player) -> Player {
    let mut b = Buffer::new(); // 主要缓冲区

    let goods = vec![
        Good::new("QUIT", -1),
        Good::new("HP+15 <- 300KB", 300),
        Good::new("HP+150 <- 2MB", 2 * 1024),
        Good::new("光芒核心 <- 222KB", 222),
        Good::new("消色核心 <- 222KB", 222),
        Good::new("纷争核心 <- 222KB", 222),
    ];
    let mut goods_highlighted = 0; // 高亮位置
    let mut goods_next = 0; // 防溢出
    let mut goods_selected = -1; // 回车后选中的高亮位置
    let goods_len = goods.len() as i8;

    loop {
        b.wl(format!("{} | {}{}", data::TITLE(), data::VERSION, data::S));
        b.wl(format!("${} | {}/{}{}", a.convert(), a.hp, a.max_hp, data::S));
        b.wl("[ 商店 ]");

        for (index, selected) in goods.iter().enumerate() {
            if goods_highlighted == index as i8 {
                if a.money >= selected.price {
                    b.wl(format!(" {} ", selected.name).on_white().black());
                } else {
                    b.wl(format!(" {} ", selected.name).on_red().black());
                }
            } else {
                b.wl(format!(" {} ", selected.name));
            }
        }

        b.print();
        
        if let Ok(key) = read() {
            cls();
            goods_selected = -1;
            match key {
                ArrowUp | Char('w') | Char('W') => goods_next -= 1,
                ArrowDown | Char('s') | Char('S') => goods_next += 1,
                Enter => goods_selected = goods_highlighted,
                _ => {
                    continue;
                }
            }
            if 0 <= goods_next && goods_next < goods_len {
                goods_highlighted = goods_next;
            } else { goods_next = goods_highlighted; }
        }

        if goods_selected >= 0 && goods_selected < goods_len {
            match goods[goods_selected as usize].name.as_str() {
                "QUIT" => {
                    break;
                }
                "HP+15 <- 300KB" => {
                    if a.money >= 300 {
                        a.money -= 300;
                        a.hp += 15;
                    }
                }
                "HP+150 <- 2MB" => {
                    if a.money >= 2 * 1024 {
                        a.money -= 2 * 1024;
                        a.hp += 150;
                    } 
                },
                "光芒核心 <- 222KB"  => {
                    if a.money >= 222 {
                        a.money -= 222;
                        *a.bag.entry(0).or_insert(0) += 1;
                    } 
                },
                "消色核心 <- 222KB"  => {
                    if a.money >= 222 {
                        a.money -= 222;
                        *a.bag.entry(1).or_insert(0) += 1;
                    } 
                },
                "纷争核心 <- 222KB"  => {
                    if a.money >= 222 {
                        a.money -= 222;
                        *a.bag.entry(2).or_insert(0) += 1;
                    } 
                },
                _ => {}
            }
        }
    }
    a
}