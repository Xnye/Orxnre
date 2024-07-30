use crate::{cls, data, game, read, Buffer};
use colored::*;
use console::Key::*;
use data::ItemAttr;
use game::Player;

pub fn main(mut a: Player, filter: ItemAttr) -> (Player, u8) {
    let mut b = Buffer::new();

    let mut bag_highlighted: Option<u8> = None; // 高亮物品ID
    let mut bag_selected_attr: &Vec<ItemAttr> = &Vec::new(); // 高亮位置的物品属性
    let mut item_selectable = false;

    let bag: Vec<(u8, i32)> = a.bag.iter().map(|(&id, &amount)| (id, amount)).collect();
    let bag_len = bag.len();

    loop {
        b.wl(format!("{} | {}{}", data::TITLE(), data::VERSION, data::S));
        b.wl(format!(
            "${} | {}/{}{}", a.money_convert(), a.hp, a.max_hp, data::S));
        b.wl("< 背包 >");

        for (item_index, amount) in &bag {
            let (item_name, item_attr_list) = &data::ITEM[*item_index as usize];

            match filter {
                ItemAttr::None => {}
                _ => {
                    if !item_attr_list.contains(&filter) {
                        continue;
                    };
                }
            }

            let item_name = item_name.clone();
            let mut item_rgb = (167, 167, 167);

            // 获取颜色
            for item_attr in item_attr_list {
                if let ItemAttr::Color(rgb) = item_attr {
                    item_rgb = *rgb
                };
            }

            // 选中高亮
            if bag_highlighted == Some(*item_index) {
                bag_selected_attr = item_attr_list;
                b.w(format!(" {} x{} ", item_name, amount)
                    .on_truecolor(item_rgb.0, item_rgb.1, item_rgb.2)
                    .black());
            } else {
                b.w(format!(" {} x{} ", item_name, amount)
                    .on_black()
                    .truecolor(item_rgb.0, item_rgb.1, item_rgb.2));
            }

            // 手持指示
            b.wl(if a.hand == *item_index {
                " [ON]"
            } else {
                "     "
            });
        }

        for item_attr in bag_selected_attr {
            if let ItemAttr::Rarity(_) = item_attr {
                item_selectable = true
            };
        }

        b.w("[ ← 返回 ] ");
        b.w(if item_selectable {
            "[ Enter 使用 ] ".white()
        } else {
            "  Enter 使用   ".truecolor(150, 150, 150)
        });

        b.print();

        if let Ok(key) = read() {
            cls();
            match key {
                ArrowUp | Char('w') | Char('W') => {
                    if let Some(highlighted) = bag_highlighted {
                        let current_index =
                            bag.iter().position(|(id, _)| *id == highlighted).unwrap();
                        if current_index > 0 {
                            bag_highlighted = Some(bag[current_index - 1].0);
                        }
                    } else if !bag.is_empty() {
                        bag_highlighted = Some(bag[0].0);
                    }
                }
                ArrowDown | Char('s') | Char('S') => {
                    if let Some(highlighted) = bag_highlighted {
                        let current_index =
                            bag.iter().position(|(id, _)| *id == highlighted).unwrap();
                        if current_index < bag_len - 1 {
                            bag_highlighted = Some(bag[current_index + 1].0);
                        }
                    } else if !bag.is_empty() {
                        bag_highlighted = Some(bag[0].0);
                    }
                }
                Enter => {
                    if item_selectable {
                        if let Some(highlighted) = bag_highlighted {
                            a.hand = highlighted;
                        }
                    }
                }
                ArrowLeft => {
                    bag_highlighted = None;
                    break;
                }
                _ => {
                    continue;
                }
            }
        }
    }

    (a, bag_highlighted.unwrap_or(0))
}
