use crate::{game, Buffer, cls, cls_pro, read, data};
use std::{thread::sleep, time};
use colored::{Colorize, ColoredString};
use game::Player;

#[derive(Clone)]
pub struct Enemy {
    pub exist: bool,
    pub max_hp: i32,
    pub hp: i32,
    pub atk: i32,
}
impl Enemy {
    pub fn new() -> Self { Enemy { exist: false, max_hp: 0, hp: 0, atk: 0, } }
    pub fn new_exist() -> Self { Enemy { exist: true, max_hp: 100, hp: 100, atk: 10, } }
}

pub fn main(mut a: Player, mut b: Enemy, mut priority: bool) -> (Player, Enemy) {
    let mut log: Vec<ColoredString> = Vec::new(); // 战斗日志
    let mut s: Buffer = Buffer::new(); // 主要缓冲区 打印所有内容 (screen)
    let mut march: bool = true; // 要继续对战吗?

    cls_pro();

    while march {
        march = !march;
        // 判断双方状态
        let log_entry: ColoredString = format!("{}{}", if a.hp <= 0 { "寄".red() } else if b.hp <= 0 { "赢".green() } else {
            // 如果对战未结束则为 true, 反之亦然
            march = !march;
            // 先交换优先权, 再反向判断, 如果 priority 为 true 则 a 先攻击
            priority = !priority;
            if !priority {
                b.hp -= a.atk;
                format!("你造成了 {} 伤害", a.atk).white()
            } else {
                a.hp -= b.atk;
                format!("敌方造成了 {} 伤害", b.atk).yellow()
            }
        }, data::SPACES).white();
        log.push(log_entry);

        // 打印标题
        s.wl(format!("{} | {}", data::TITLE(), data::VERSION));
        // Orxnre | v1.0-beta.3
        s.wl(format!("${}", a.money));
        // $514
        s.wl(format!("[YOU] {}/{}  -*-  {}/{} [ENEMY]{}", a.hp, a.max_hp, b.hp, b.max_hp, data::SPACES));
        // [YOU] 495/500  -*-  67/100 [ENEMY]

        // 打印战斗日志并填补空行
        for i in (if log.len() <= 4 {0} else {log.len().checked_sub(4).unwrap_or(0)})..log.len() {
            s.wl(log[i].clone());
        }
        for _ in 0..4_i32.checked_sub(log.len() as i32).unwrap_or(0) {
            s.wl("");
        }

        cls();
        s.print();

        sleep(time::Duration::from_millis(300));
    }
    println!("按下任意键继续...");
    let _ = read();

    (a, b)
}