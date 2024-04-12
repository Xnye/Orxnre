use crate::{data, game, Buffer, read, cls};
use game::Player;
use console::Key::*;
use colored::*;
use data::ItemAttr;

pub fn main(mut a: Player) -> Player {
    let mut b = Buffer::new(); // 主要缓冲区

    let mut bag_highlighted: i32 = 0; // 高亮位置
    let mut bag_next: i32 = 0; // 防溢出
    let mut bag_selected_attr: &Vec<ItemAttr> = &Vec::new(); // 高亮位置的物品
    let mut item_selectable = false;
    
    let bag = a.bag.iter();
    let bag_len: i32 = bag.len() as i32;
    
    loop {
        b.wl(format!("{} | {}{}", data::TITLE(), data::VERSION, data::S));
        b.wl(format!("${} | {}/{}{}", a.convert(), a.hp, a.max_hp, data::S));
        b.wl("< 背包 >");

        for (item_index, amount) in bag.clone() {
            let (item_name, item_attr_list) = &data::ITEM[*item_index as usize];
            
            let item_name = item_name.clone();
            let mut item_rgb = (0, 0, 0);

            // 获取颜色
            for item_attr in item_attr_list {
                if let ItemAttr::Color(rgb) = item_attr { item_rgb = (rgb.0, rgb.1, rgb.2) };
            }
            
            // 选中高亮
            if bag_highlighted == *item_index as i32 {
                bag_selected_attr = item_attr_list;
                b.w(format!(" {} x{} ", item_name, amount).on_truecolor(item_rgb.0, item_rgb.1, item_rgb.2).black());
                
            } else {
                b.w(format!(" {} x{} ", item_name, amount).on_black().truecolor(item_rgb.0, item_rgb.1, item_rgb.2));
            }

            // 手持指示
            b.wl(if a.hand == *item_index { " [ON]" } else { "     " });
        }

        for item_attr in bag_selected_attr {
            if let ItemAttr::Rarity(_rarity) = item_attr { item_selectable = true };
        }

        item_selectable = true;
        
        b.w("[ ← 返回 ] ");
        b.w(if item_selectable { "[ Enter 使用 ] ".white() } else {"  Enter 使用   ".truecolor(150, 150, 150)});
        
        b.print();

        if let Ok(key) = read() {
            cls();
            match key {
                ArrowUp | Char('w') | Char('W') => bag_next -= 1,
                ArrowDown | Char('s') | Char('S') => bag_next += 1,
                Enter => if item_selectable { a.hand = bag_highlighted as u8; },
                ArrowLeft => break,
                _ => {
                    continue;
                }
            }
            if 0 <= bag_next && bag_next < bag_len {
                bag_highlighted = bag_next;
            } else { bag_next = bag_highlighted; }
        }
    }
    a
}