use colored::{ColoredString, Colorize};

fn c(text: &str, rgb:(u8, u8, u8, bool)) -> ColoredString {
    let (r, g, b, bold) = rgb;
    let mut out = format!("{}", text).truecolor(r, g, b);
    if bold { out = out.bold(); }
    out
}

// 标题文本颜色
pub fn title() -> ColoredString{
    const TC1: (u8, u8, u8, bool) = (218, 187, 244, true);
    const TC2: (u8, u8, u8, bool) = (213, 187, 246, true);
    const TC3: (u8, u8, u8, bool) = (198, 184, 248, true);
    const TC4: (u8, u8, u8, bool) = (187, 180, 250, true);
    const TC5: (u8, u8, u8, bool) = (168, 174, 252, true);
    const TC6: (u8, u8, u8, bool) = (150, 160, 254, true);
    format!("{}{}{}{}{}{}", c("O", TC1), c("r", TC2), c("x", TC3), c("n", TC4), c("r", TC5), c("e", TC6)).white()
}