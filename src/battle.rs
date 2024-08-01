use crate::{cls, cls_pro, data, game, random, read, time_sleep, Buffer};
use colored::{ColoredString, Colorize};
use console::Key::*;
use serde::{Deserialize, Serialize};
use game::Player;

#[derive(Clone, Serialize, Deserialize)]
pub struct Enemy {
    pub exist: bool,
    pub max_hp: i32,
    pub hp: i32,
    pub atk: i32,
    pub reward: i32,
}
impl Enemy {
    pub fn new_empty() -> Self {
        Enemy {
            exist: false,
            max_hp: 0,
            hp: 0,
            atk: 0,
            reward: 0,
        }
    }
    pub fn new(max_hp: i32, hp: i32, atk: i32, reward: i32) -> Self {
        Enemy {
            exist: true,
            max_hp,
            hp,
            atk,
            reward,
        }
    }
}

#[allow(dead_code)]
pub enum EnemyType {
    Normal(u8), // level
    Full(Enemy),
}

// 战斗动作
enum Action {
    Skip,
    Attack,
    End,
}

fn fill(text: &str) -> String {
    format!("{:<50}", text)
}

pub fn main(mut a: Player, mut b: Enemy, mut priority: bool) -> (Player, Enemy) {
    let mut s: Buffer = Buffer::new(); // 主要缓冲区 打印所有内容 (screen)
    let mut c: Buffer = Buffer::new(); // 辅助缓冲区
                                       // 战斗相关
    let mut log: Vec<ColoredString> = Vec::new(); // 战斗日志
    let log_line: usize = 8; // 战斗日志显示行数
    let mut march: bool = true; // 要继续对战吗?
    let mut march2: bool = true; // 便于打印结果
    let mut auto_fight: bool = false; // 自动战斗
    let mut next_action: Action = Action::Skip; // 下一步动作
                                                // 菜单相关
    let sel = vec!["AUTO".to_string(), "ATTACK".to_string(), "QUIT".to_string()];
    let mut sel_highlighted = 0; // 高亮位置
    let mut sel_next = 0; // 防溢出
    let mut sel_selected; // 回车后选中的高亮位置
    let sel_len = sel.len() as i8;

    cls_pro();

    while march2 {
        if !march {
            march2 = !march2
        }
        // 打印标题
        s.wl(format!("{} | {}{}", data::TITLE(), data::VERSION, data::S)); // 标题
                                                                           // Orxnre | v1.0-beta.3
        s.wl(format!(
            "${} | {}/{}{}",
            a.money_convert(),
            a.hp,
            a.max_hp,
            data::S
        )); // 玩家信息
            // 5.14MB | 495/500
        s.wl(format!(
            "[ ENEMY {} ]{}",
            if b.hp < 0 {
                format!("0/{}", b.max_hp).red()
            } else {
                format!("{}/{}", b.hp, b.max_hp).white()
            },
            data::S
        ));
        // [ ENEMY 67/100 ] (归零显示红色)

        // 打印战斗日志并填补空行
        // log.len()-log_line..log.len()
        let start_index = if log.len() <= log_line {
            0
        } else {
            log.len().saturating_sub(log_line)
        };
        for log_entry in log[start_index..].iter() {
            s.wl(log_entry.clone());
        }
        for _ in 0..(log_line as i32).checked_sub(log.len() as i32).unwrap_or(0) {
            s.wl("");
        }

        sel_selected = -1;

        if auto_fight {
            s.wl("\n AUTO PLAY ".black().on_bright_green());
        }

        cls();
        s.print();

        if a.hp <= 0 || b.hp <= 0 {
            // 对战结束
            next_action = Action::End;
        } else if !auto_fight {
            // 轮到你了
            for (index, selected) in sel.iter().enumerate() {
                let selected = if !priority && selected == "ATTACK" {
                    "CONTINUE"
                } else {
                    selected
                };
                c.w(format!(
                    "{} ",
                    if sel_highlighted == index as i8 {
                        format!(" {} ", selected).on_white().black()
                    } else {
                        format!(" {} ", selected).white()
                    }
                ));
            }

            c.print();

            if let Ok(key) = read() {
                match key {
                    ArrowLeft | Char('a') | Char('A') => sel_next -= 1,
                    ArrowRight | Char('d') | Char('D') => sel_next += 1,
                    Enter => sel_selected = sel_highlighted,
                    _ => {}
                }
                if 0 <= sel_next && sel_next < sel_len {
                    sel_highlighted = sel_next;
                } else {
                    sel_next = sel_highlighted;
                }
            }
            if sel_selected >= 0 && sel_selected < sel_len {
                match sel[sel_selected as usize].as_str() {
                    "AUTO" => {
                        auto_fight = !auto_fight;
                        cls_pro();
                    }
                    "ATTACK" => {
                        next_action = Action::Attack;
                    }
                    "QUIT" => {
                        log.push(fill("你不能走").white());
                    }
                    _ => {}
                }
            }
        } else if auto_fight {
            // 自动游玩
            next_action = Action::Attack;
            time_sleep(150);
        }

        // 开始战斗
        match next_action {
            Action::Attack => {
                march = !march;
                // 判断双方状态
                let log_entry: ColoredString = format!(
                    "{}",
                    if a.hp <= 0 {
                        "你寄了".red()
                    } else if b.hp <= 0 {
                        fill(&format!("你征服了敌人 +{}KB", b.reward)).green()
                    } else {
                        // 如果对战未结束则为 true, 反之亦然
                        march = !march;
                        // 先交换优先权, 再反向判断, 如果 priority 为 true 则 a 先攻击
                        priority = !priority;
                        let noise = random(-1..2);
                        if !priority {
                            b.hp -= a.atk - noise;
                            fill(&format!("你造成了 {} 伤害", a.atk + noise)).white()
                        } else {
                            time_sleep(60);
                            a.hp -= b.atk - noise;
                            fill(&format!("敌方造成了 {} 伤害", b.atk + noise)).yellow()
                        }
                    }
                )
                .white();

                log.push(log_entry);
            }
            Action::End => {
                break;
            }
            _ => {}
        }
        next_action = Action::Skip;
    }
    println!("按下任意键继续...{}", data::S);
    let _ = read();

    (a, b)
}
