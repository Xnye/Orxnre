use std::{process::exit};
use colored::*;
use console::Key;
use noise::{NoiseFn, Perlin};
use rand::{Rng, thread_rng};

use Key::Char;

use crate::{data, battle, Buffer, cls, cls_pro, read};
use data::block_name;
use crate::battle::Enemy;

// 玩家数据
pub struct Player {
    position: (u8, u8),
    money: i32,
    max_hp: i32,
    hp: i32,
    atk: i32,
}

impl Player {
    fn new() -> Self { Player { position: (0, 0), money: 0, max_hp: 500, hp: 500, atk: 20 } }
}

// 地图数据
struct Map {
    id: Vec<Vec<i8>>,
    gift: Vec<Vec<i32>>,
    sp: Vec<Vec<Enemy>>,
}

impl Map {
    fn new(x: usize, y: usize) -> Self {
        Map { id: vec![vec![-1; x]; y], gift: vec![vec![0; x]; y], sp: vec![vec![Enemy::new(); x]; y] }
    }

    fn measure(&self) -> (usize, usize) {
        let x_len = self.id[0].len();
        let y_len = self.id.len();
        (x_len, y_len)
    }

    fn map_set(&mut self, id: i8, y: u8, x: u8) -> &mut Map {
        let map = self;
        map.id[y as usize][x as usize] = id;
        map
    }

    fn map_terrain(&mut self, id: i8, seed: u32, limit: f64) -> &mut Map {
        let (x_len, y_len) = self.measure();
        let map = self;
        let perlin = Perlin::new(seed);

        for y in 0..y_len {
            let mut row: Vec<i8> = Vec::new();
            for x in 0..x_len {
                let nx = x as f64 / x_len as f64;
                let ny = y as f64 / y_len as f64;
                let noise_value = perlin.get([nx, ny]);

                let next = if (noise_value + 1.0) / 2.0 * 255.0 >= limit {
                    id
                } else {
                    map.id[y][x]
                };

                row.push(next);
            }
            map.id[y] = row;
        }
        map
    }

    fn gift_random(&mut self, max: i32, min: i32, tries: u8) -> &mut Map {
        let (x_len, y_len) = self.measure();
        let map = self;

        for _ in 0..tries {
            let (y, x) = (thread_rng().gen_range(0..y_len), thread_rng().gen_range(0..x_len));
            map.gift[y][x] = thread_rng().gen_range(min..max);
        }
        map
    }

    fn print(&self, position: (u8, u8)) -> String {
        let (x_len, y_len) = self.measure();

        let mut out = String::new();

        for y in 0..y_len {
            for x in 0..x_len {
                if (y as u8, x as u8) != position {
                    out = format!("{}{} ", out, block_name(self.id[y][x]));
                } else {
                    out = format!("{}{} ", out, "您".on_truecolor(200, 200, 200).black());
                }
            }
            out = format!("{}\n", out);
        }
        out
    }

    fn explore(&self, target: (u8, u8)) -> i32 {
        let (x_len, y_len) = self.measure();
        let (y, x) = target;
        let mut gift = 0;

        if y < y_len as u8 && x < x_len as u8 {
            gift = self.gift[y as usize][x as usize];
        }

        gift
    }
}

pub fn main() {
    let mut b = Buffer::new(); // 主要缓冲区 打印所有内容
    let mut h = Buffer::new(); // 打印提示信息用

    let mut map = Map::new(16, 16); // 初始化地图信息
    let (x_len, y_len) = map.measure(); // 并获取其长度

    let mut player = Player::new(); // 初始化玩家信息

    let seed: u32 = thread_rng().gen(); // 种子

    // 生成地形以及宝藏
    map.map_terrain(2, seed - 1, 200.0);
    map.map_terrain(1, seed - 2, 150.0);
    map.map_set(0, 3, 11);
    map.gift_random(300, 200, 25);

    cls_pro();

    loop {
        cls();

        b.wl(format!("{} | {}", data::TITLE(), data::VERSION)); // 标题
        // Orxnre | v1.0-beta.3

        b.wl(format!("${} | {}/{}", player.money, player.hp, player.max_hp)); // 玩家信息
        // $514 | 495/500

        b.wl(map.print(player.position)); // 写入地图
        b.wl(h.read()); // 写入提示消息
        b.print(); // 打印到屏幕
        h = Buffer::new(); // 清空消息

        let (mut next_y, mut next_x) = player.position;

        if let Ok(key) = read() {
            match key {
                // Esc 退出
                Key::Escape => exit(0),

                // WASD 移动
                Char('w') | Char('W') => next_y = next_y.wrapping_sub(1),
                Char('s') | Char('S') => next_y = next_y.wrapping_add(1),
                Char('a') | Char('A') => next_x = next_x.wrapping_sub(1),
                Char('d') | Char('D') => next_x = next_x.wrapping_add(1),

                // E 探索
                Char('e') | Char('E') => {
                    let gift = map.explore(player.position);

                    // 如果宝藏不为0, 清空宝藏, 获得金钱, 提示
                    h.w(if gift != 0 {
                        map.gift[player.position.0 as usize][player.position.1 as usize] = 0;
                        player.money += gift;
                        format!("E → 找到了宝藏 $+{}{}", gift, data::SPACES)
                    } else {
                        format!("E → 空空如也{}", data::SPACES)
                    });
                }

                // Others 处理默认情况
                _ => {}
            };
        }

        // 判断下一步是否合法
        if next_y < y_len as u8 && next_x < x_len as u8 {
            if map.id[next_y as usize][next_x as usize] != -1 {
                player.position = (next_y, next_x);
            }
        }
    }
}
