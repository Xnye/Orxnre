use std::process::exit;
use colored::*;
use console::Key;
use noise::{NoiseFn, Perlin};
use rand::{Rng, thread_rng};

use Key::Char;

use crate::{data::{self, S}, battle, shop, Buffer, cls, cls_pro, read, random};

use battle::{Enemy, EnemyType};
use data::{block_name, item_name};

// 玩家数据
pub struct Player {
    pub position: (u8, u8),
    pub money: i32,
    pub max_hp: i32,
    pub hp: i32,
    pub atk: i32,
    pub bag: [i32; 5],
}

impl Player {
    fn new() -> Self { Player { position: (0, 0), money: 0, max_hp: 500, hp: 500, atk: 20, bag: [0; 5]} }

    pub fn convert(&self) -> String {
        if self.money < 1024 {
            format!("{}.00KB", self.money)
        } else if self.money < 1048576 {
            format!("{:.2}MB", self.money as f64 / 1024.0)
        } else {
            format!("{:.2}GB", self.money as f64 / 1048576.0)
        }
    }
}

// 地图数据
struct Map {
    id: Vec<Vec<i8>>,
    gift: Vec<Vec<i32>>,
    entity: Vec<Vec<Enemy>>,
}

#[allow(dead_code)]
impl Map {
    fn new(x: usize, y: usize) -> Self {
        Map { id: vec![vec![0; x]; y], gift: vec![vec![0; x]; y], entity: vec![vec![Enemy::new_empty(); x]; y] }
    }

    fn measure(&self) -> (usize, usize) {
        let x_len = self.id[0].len();
        let y_len = self.id.len();
        (x_len, y_len)
    }

    fn set(&mut self, id: i8, y: u8, x: u8) -> &mut Map {
        let map = self;
        map.id[y as usize][x as usize] = id;
        map
    }

    fn spawn(&mut self, y: u8, x: u8, e: EnemyType) -> &mut Map {
        let map = self;

        map.entity[y as usize][x as usize] = match e {
            EnemyType::Normal(level ) => {
                let l = level as i32;
                let hp = 90 + l * 10; // 血量 f(x) = 90 + 10x
                let atk = 12 + l * 3; // 攻击 f(x) = 12 + 3x
                let award = 500 + l * 100; // 奖励 f(x) = (600 + 100x) ± 200
                Enemy::new(hp, hp, atk, random(award .. award + 200))
            },
            EnemyType::Full(e) => e,
        };
        
        map
    }

    fn map_terrain(&mut self, id: i8, seed: u32, soft_limit: f64, form: u8) -> &mut Map {
        let (x_len, y_len) = self.measure();
        let map = self;
        
        for y in 0..y_len {
            let mut row: Vec<i8> = Vec::new();
            for x in 0..x_len {
                let perlin = match form {
                    0 => Perlin::new(seed), // 默认
                    1 => Perlin::new(seed + (y * x_len + x) as u32), // 散点
                    _ => Perlin::new(seed),
                };

                let nx = x as f64 / x_len as f64;
                let ny = y as f64 / y_len as f64;
                let noise_value = perlin.get([nx, ny]);

                let next = if (noise_value + 1.0) / 2.0 * 255.0 >= soft_limit {
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

    fn gift_random(&mut self, max: i32, min: i32, tries: u32) -> &mut Map {
        let (x_len, y_len) = self.measure();
        let map = self;

        for _ in 0..tries {
            let (y, x) = (random(0..y_len as i32), random(0..x_len as i32));
            map.gift[y as usize][x as usize] = random(min..max);
        }
        map
    }

    fn set_random(&mut self, id: i8) -> &mut Map {
        let (x_len, y_len) = self.measure();
        let map = self;

        let (y, x) = (random(0..y_len as i32), random(0..x_len as i32));
        map.id[y as usize][x as usize] = id;
        map
    }

    fn print(&self, position: (u8, u8)) -> String {
        let (x_len, y_len) = self.measure();
        let (x_pos, y_pos) = (position.1 as i32, position.0 as i32);
        let (x_cam, y_cam) = (7, 7); // 玩家视图位置

        let mut out = String::new();

        for y in (y_pos - y_cam)..(y_pos + y_cam) {
            for x in (x_pos - x_cam)..(x_pos + x_cam) {
                if y < 0 || x < 0 || y >= y_len as i32 || x >= x_len as i32 {
                    out = format!("{}{} ", out, "　".on_truecolor(0, 0, 0));
                } else {
                    out = if self.entity[y as usize][x as usize].exist {
                        format!("{}{} ", out, "敌".on_truecolor(200, 120, 120).black())
                    } else if (y as u8, x as u8) != position {
                        format!("{}{} ", out, block_name(self.id[y as usize][x as usize]))
                    } else {
                        format!("{}{} ", out, "您".on_white().black())
                    }
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
    let mut b = Buffer::new(); // 主要缓冲区 打印所有内容 (buffer)
    let mut h = Buffer::new(); // 打印提示信息用 (hint)

    let mut map = Map::new(64, 64); // 初始化地图信息
    let (x_len, y_len) = (map.measure().0 as i32, map.measure().0 as i32) ; // 并获取其长度

    let mut player = Player::new(); // 初始化玩家信息

    let seed: u32 = thread_rng().gen(); // 种子

    // 生成地形、宝藏、商店
    for _ in 0..3 {
        map.map_terrain(2, seed + 2, 190.0, 1);
        map.map_terrain(1, seed + 1, 190.0, 1);
        map.map_terrain(3, seed, 170.0, 0);
        
    }
    for _ in 0..48 {
        map.spawn(random(0..y_len) as u8, random(0..x_len) as u8, EnemyType::Normal(1));
    }
    map.set_random(-1);
    map.gift_random(300, 200, 400);

    cls_pro();

    loop {
        cls();

        b.wl(format!("{} | {}{}", data::TITLE(), data::VERSION, S)); // 标题
        // Orxnre | v1.0-beta.3

        b.wl(format!("${} | {}/{}{}", player.convert(), player.hp, player.max_hp, S)); // 玩家信息
        // 5.14MB | 495/500

        b.wl(map.print(player.position)); // 写入地图
        b.wl(if h.modified {h.read()} else {format!("{S}\n{S}").white()}); // 写入提示消息
        b.wl(S); // 空行
        b.print(); // 打印到屏幕

        h = Buffer::new(); // 清空消息

        let (mut next_y, mut next_x) = player.position;

        if let Ok(key) = read() {
            match key {
                // Esc 退出
                Key::Escape => exit(0),

                // WASD 移动
                Char('w') | Char('W') | Key::ArrowUp => next_y = next_y.wrapping_sub(1),
                Char('s') | Char('S') | Key::ArrowDown => next_y = next_y.wrapping_add(1),
                Char('a') | Char('A') | Key::ArrowLeft => next_x = next_x.wrapping_sub(1),
                Char('d') | Char('D') | Key::ArrowRight => next_x = next_x.wrapping_add(1),

                // H 帮助
                Char('h') | Char('H') | Key::Enter => h.w("H > WASD 移动 E 探索 Q 背包 H 提示 Esc 退出游戏"),

                // E 探索
                Char('e') | Char('E') => {
                    let gift = map.explore(player.position);

                    // 如果宝藏不为0, 清空宝藏, 获得金钱, 提示
                    if gift != 0 {
                        map.gift[player.position.0 as usize][player.position.1 as usize] = 0;
                        player.money += gift;
                        h.w(format!("E > 找到了宝藏 +{}KB{}", gift, S));
                    } else {
                        h.w(format!("E > 空空如也{}", S));
                    }
                }

                // Q 背包
                Char('q') | Char('Q') => {
                    h.wl(format!("Q > 背包{}", S));
                    for (index, item) in data::ITEM.iter().enumerate() {
                        h.w(format!("{} {} ", item, player.bag[index]));
                    }
                    h.w(S);
                }

                // Others 处理默认情况
                _ => {}
            };
        }

        // 超出边界不允许移动
        if next_y < y_len as u8 && next_x < x_len as u8 {
            // 进入商店
            if map.id[next_y as usize][next_x as usize] == -1 {
                cls_pro();
                player = shop::main(player);
            }
            // 触发战斗
            else if map.entity[next_y as usize][next_x as usize].exist {
                let (a, b) = battle::main(player, map.entity[next_y as usize][next_x as usize].clone(), true);
                player = a;
                map.entity[next_y as usize][next_x as usize] = if player.hp <= 0 { // 玩家失败
                    break
                } else if b.hp > 0 { // 敌人存活
                    b
                } else { // 敌人被击败
                    player.money += b.reward;
                    Enemy::new_empty()
                };
            }
            else {
                player.position = (next_y, next_x);
            }
        }
    }
}