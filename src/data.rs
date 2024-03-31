#![allow(non_snake_case)]

use std::ops::Range;
use colored::{ColoredString, Colorize};
use lazy_static::lazy_static;
use crate::data::ItemAttr::*;

pub const VERSION: &str = "v1.0-beta.20";
pub const S: &str = "                                                ";

fn c(text: &str, rgb: (u8, u8, u8)) -> ColoredString {
    let (r, g, b) = rgb;
    text.to_string().truecolor(r, g, b)
}

// 标题文本颜色
pub fn TITLE() -> ColoredString {
    let tc1: (u8, u8, u8) = (218, 187, 244);
    let tc2: (u8, u8, u8) = (213, 187, 246);
    let tc3: (u8, u8, u8) = (198, 184, 248);
    let tc4: (u8, u8, u8) = (187, 180, 250);
    let tc5: (u8, u8, u8) = (168, 174, 252);
    let tc6: (u8, u8, u8) = (150, 160, 254);
    format!("{}{}{}{}{}{}", c("O", tc1), c("r", tc2), c("x", tc3), c("n", tc4), c("r", tc5), c("e", tc6)).white()
}

pub fn block_name(id: i8) -> String {
    match id {
        -2 => format!("{}", "门".black().on_truecolor(229, 189, 187)),
        -1 => format!("{}", "店".black().on_truecolor(180, 240, 190)),
        0 => format!("{}", c("土", (190, 147, 138))),
        1 => format!("{}", c("草", (121, 204, 109))),
        2 => format!("{}", c("石", (196, 189, 181))),
        3 => format!("{}", c("水", (166, 185, 211))),
        4 => format!("{}", c("沙", (239, 231, 142))),
        _ => "　".to_string(),
    }
}

#[derive(Clone)]
pub enum ItemAttr {
    None,
    Rarity(i32),
    Color((u8, u8, u8)),
    Attack(Range<i32>),
}

lazy_static! {
    static ref EMPTY: (String, Vec<ItemAttr>) = ("".to_string(), vec![]);

    pub static ref ITEM: [(String, Vec<ItemAttr>); 9] = [
        ("光芒核心".to_string(), vec![Color((227, 203, 171))],),
        ("消色核心".to_string(), vec![Color((168, 189, 187))],),
        ("纷争核心".to_string(), vec![Color((159, 155, 243))],),
        ("Eltaw".to_string(), vec![Rarity(1), Attack(5..5)]),
        ("Sigma".to_string(), vec![Rarity(1), Attack(4..7)]),
        EMPTY.clone(),
        EMPTY.clone(),
        EMPTY.clone(),
        EMPTY.clone(),
    ];
    
    pub static ref ITEM_LEN: usize = ITEM.len();
}
