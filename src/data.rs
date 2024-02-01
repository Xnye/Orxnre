#![allow(non_snake_case)]

use colored::{ColoredString, Colorize};
use lazy_static::lazy_static;

pub const VERSION: &str = "v1.0-beta.14";
pub const S: &str = "                                                ";

fn c(text: &str, rgb: (u8, u8, u8)) -> ColoredString {
    let (r, g, b) = rgb;
    let mut out = format!("{}", text).truecolor(r, g, b);
    out
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
        0 => format!("{}", c("土", (190, 147, 138))),
        1 => format!("{}", c("草", (121, 204, 109))),
        2 => format!("{}", c("石", (196, 189, 181))),
        3 => format!("{}", c("水", (166, 185, 211))),
        _ => "　".to_string(),
    }
}

lazy_static!{
    pub static ref ITEM: [String; 3] = [
        format!("{}", c("黎明核心", (227, 203, 171))),
        format!("{}", c("雾霭核心", (168, 219, 187))),
        format!("{}", c("蛰伏核心", (159, 155, 243))),
    ];
}

pub fn item_name(id: i32) -> String {
    let len = || ITEM.len();

    if id < 0 || id >= len() as i32 { 
        "-?-".to_string()
    } else {
        ITEM[id as usize].clone()
    }
}