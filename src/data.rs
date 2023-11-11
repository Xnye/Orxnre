#![allow(non_snake_case)]

use colored::{ColoredString, Colorize};

pub const VERSION: &str = "v1.0-beta.3";
pub const SPACES: &str = "                                                ";

fn c(text: &str, rgb: (u8, u8, u8, bool)) -> ColoredString {
    let (r, g, b, bold) = rgb;
    let mut out = format!("{}", text).truecolor(r, g, b);
    if bold { out = out.bold(); }
    out
}

// 标题文本颜色
pub fn TITLE() -> ColoredString {
    let tc1: (u8, u8, u8, bool) = (218, 187, 244, true);
    let tc2: (u8, u8, u8, bool) = (213, 187, 246, true);
    let tc3: (u8, u8, u8, bool) = (198, 184, 248, true);
    let tc4: (u8, u8, u8, bool) = (187, 180, 250, true);
    let tc5: (u8, u8, u8, bool) = (168, 174, 252, true);
    let tc6: (u8, u8, u8, bool) = (150, 160, 254, true);
    format!("{}{}{}{}{}{}", c("O", tc1), c("r", tc2), c("x", tc3), c("n", tc4), c("r", tc5), c("e", tc6)).white()
}

pub fn block_name(id: i8) -> String {
    match id {
        0 => format!("{}", c("土", (170, 127, 118, false))),
        1 => format!("{}", c("草", (101, 174, 89, false))),
        2 => format!("{}", c("石", (176, 169, 161, false))),
        _ => "　".to_string(),
    }
}