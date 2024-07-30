use colored::*;
use console::Key;
use noise::{NoiseFn, Perlin};
use rand::{thread_rng, Rng};
use std::collections::BTreeMap;
use std::process::exit;
use Key::Char;

use crate::data::{self, ItemAttr, S, SS};
use crate::{bag, battle, cls, cls_pro, random, read, shop, time_sleep, Buffer};

use battle::{Enemy, EnemyType};
use data::block_name;

// 玩家数据
pub struct Player {
    pub position: (u8, u8),
    pub money: i32,
    pub max_hp: i32,
    pub hp: i32,
    pub atk: i32,
    pub bag: BTreeMap<u8, i32>,
    pub hand: u8,
}

impl Player {
    fn new() -> Self {
        Player {
            position: (0, 0),
            money: 0,
            max_hp: 500,
            hp: 500,
            atk: 20,
            bag: BTreeMap::new(),
            hand: 0,
        }
    }

    pub fn money_convert(&self) -> String {
        if self.money < 1024 {
            format!("{}.00KB", self.money)
        } else if self.money < 1048576 {
            format!("{:.2}MB", self.money as f64 / 1024.0)
        } else {
            format!("{:.2}GB", self.money as f64 / 1048576.0)
        }
    }
}

#[derive(Clone, PartialEq)]
enum Act {
    None,
    Tp(i8),
    Gift(i32),
}

// 地图数据
#[derive(Clone)]
struct Map {
    id: Vec<Vec<i8>>,
    act: Vec<Vec<Act>>,
    entity: Vec<Vec<Enemy>>,
}

#[allow(dead_code)]
impl Map {
    fn new(x: usize, y: usize) -> Self {
        Map {
            id: vec![vec![0; x]; y],
            act: vec![vec![Act::None; x]; y],
            entity: vec![vec![Enemy::new_empty(); x]; y],
        }
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
            EnemyType::Normal(level) => {
                let l = level as i32;
                let hp = 90 + l * 10; // 血量 f(x) = 90 + 10x
                let atk = 12 + l * 3; // 攻击 f(x) = 12 + 3x
                let award = 750 + l * 200; // 奖励 f(x) = (750 + 200x) ± 200
                Enemy::new(hp, hp, atk, random(award..award + 200))
            }
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
                    0 => Perlin::new(seed),                          // 默认
                    1 => Perlin::new(seed + (y * x_len + x) as u32), // 散点
                    _ => Perlin::new(seed),
                };

                let nx = (x as f64 / x_len as f64 + 0.5) / 2f64;
                let ny = (y as f64 / y_len as f64 + 0.5) / 2f64;
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
            map.act[y as usize][x as usize] = Act::Gift(random(min..max));
        }
        map
    }

    fn set_random(&mut self, id: i8, act: Act) -> &mut Map {
        let map = self;
        let (x_len, y_len) = map.measure();
        let (y, x) = (random(0..y_len as i32), random(0..x_len as i32));

        match act {
            Act::None => {}
            _ => map.act[y as usize][x as usize] = act,
        }

        map.id[y as usize][x as usize] = id;
        map
    }

    fn print(&self, position: (u8, u8)) -> String {
        let (x_len, y_len) = self.measure();
        let (x_pos, y_pos) = (position.1 as i32, position.0 as i32);
        let (x_cam, y_cam) = (7, 7); // 玩家视图位置

        let mut out = String::new();

        for y in (y_pos - y_cam)..(y_pos + y_cam + 1) {
            for x in (x_pos - x_cam)..(x_pos + x_cam + 1) {
                if y < 0 || x < 0 || y >= y_len as i32 || x >= x_len as i32 {
                    out = format!("{}{} ", out, "　");
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
            // 如果有宝藏, 返回宝藏金额
            if let Act::Gift(y) = self.act[y as usize][x as usize] {
                gift = y
            }
        }

        gift
    }
}
#[warn(unused_assignments)]
pub fn main(enable_debug: bool) {
    let mut b = Buffer::new(); // 主要缓冲区 打印所有内容 (buffer)
    let mut h = Buffer::new(); // 打印提示信息用 (hint)

    let mut player = Player::new(); // 初始化玩家信息
    let seed: u32 = thread_rng().gen(); // 种子
    let mut local: i8 = 0; // 当前地图编号
    let mut goto: i8 = 0; // 跳转地图编号

    // 初始化地图
    let mut map;
    let mut map_list: Vec<Map> = vec![Map::new(64, 64); 2];

    let (x_len, y_len) = (
        map_list[0].measure().0 as i32,
        map_list[0].measure().0 as i32,
    );
    for _ in 0..3 {
        map_list[0].map_terrain(2, seed + 2, 190.0, 1);
        map_list[0].map_terrain(1, seed + 1, 190.0, 1);
        map_list[0].map_terrain(3, seed, 170.0, 0);
    }
    for _ in 0..48 {
        map_list[0].spawn(
            random(0..y_len) as u8,
            random(0..x_len) as u8,
            EnemyType::Normal(1),
        );
    }
    for l in 0..5 {
        map_list[1].spawn(
            random(0..y_len) as u8,
            random(0..x_len) as u8,
            EnemyType::Normal((l + 2) * 3),
        );
    }
    map_list[0].set_random(-1, Act::None); // 商店
    map_list[0].set_random(-2, Act::Tp(1)); // 传送门 to 1
    map_list[0].gift_random(300, 200, 400);

    map_list[1].map_terrain(4, seed + 4, 190.0, 1);
    map_list[1].set_random(-2, Act::Tp(0)); // 传送门 to 0
    map_list[1].gift_random(6000, 200, 40);

    map = map_list[0].clone();

    cls_pro();

    let basic_info = || -> String {
        format!(
            "{} | {}{}{}",
            data::TITLE(),
            data::VERSION,
            if enable_debug { " | DEBUG" } else { "" },
            S
        )
    };

    if enable_debug {
        player.money = 9999999;
    }

    // 主循环
    'main: loop {
        // 切换地图
        if goto != local {
            map = map_list[goto as usize].clone();
            local = goto;
        }
        let (x_len, y_len) = (map.measure().0 as i32, map.measure().0 as i32);

        'minor: loop {
            cls();

            b.wl(basic_info());
            b.wl(format!("${} | {}/{}{}", player.money_convert(), player.hp, player.max_hp, S));
            // Orxnre | v1.0-beta.3
            // 5.14MB | 495/500

            b.wl(map.print(player.position)); // 写入地图
            b.wl(if h.modified {
                h.read()
            } else {
                format!("{SS}\n{SS}").white()
            }); // 写入提示消息
            b.wl(SS); // 空行
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
                    Char('h') | Char('H') | Key::Enter => {
                        h.w("H > WASD 移动 E 探索 Q 背包 H 提示 P 耕地 Esc 退出游戏")
                    }

                    // E 探索
                    Char('e') | Char('E') => {
                        let gift = map.explore(player.position);

                        // 如果宝藏不为0, 清空宝藏, 获得金钱, 提示
                        if gift != 0 {
                            map.act[player.position.0 as usize][player.position.1 as usize] =
                                Act::Gift(0);
                            player.money += gift;
                            h.w(format!("E > 找到了宝藏 +{}KB{}", gift, S));
                        } else {
                            h.w(format!("E > 空空如也{}", S));
                        }
                    }

                    // Q 背包
                    Char('q') | Char('Q') => {
                        cls_pro();
                        (player, _) = bag::main(player, ItemAttr::None);
                    }

                    // P 耕地
                    Char('p') | Char('P') => {
                        // 检查是否有锄头
                        let (y, x) = (player.position.0 as usize, player.position.1 as usize);
                        if player.bag.contains_key(&3) {
                            if map.id[y][x] == 0 || map.id[y][x] == 1 {
                                map.id[y][x] = 5;
                                h.w(format!("P > 耕地成功{}", S));
                            } else {
                                h.w(format!("P > 此处无法耕地{}", S));
                            }
                        } else {
                            h.w(format!("P > 没有锄{}", S));
                        }
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
                // 传送门
                else if map.id[next_y as usize][next_x as usize] == -2 {
                    if let Act::Tp(destination) = map.act[next_y as usize][next_x as usize] {
                        goto = destination;
                        break 'minor;
                    }
                }
                // 触发战斗
                else if map.entity[next_y as usize][next_x as usize].exist {
                    let (a, b) = battle::main(
                        player,
                        map.entity[next_y as usize][next_x as usize].clone(),
                        true,
                    );
                    player = a;
                    map.entity[next_y as usize][next_x as usize] = if player.hp <= 0 {
                        // 玩家失败
                        break 'main;
                    } else if b.hp > 0 {
                        // 敌人存活
                        b
                    } else {
                        // 敌人被击败
                        player.money += b.reward;
                        Enemy::new_empty()
                    };
                } else {
                    player.position = (next_y, next_x);
                }
            }
        }
    }
    cls_pro();
    b.wl(basic_info());
    b.wl("✖ 你失败了");
    b.print();
    time_sleep(1500);
    println!("按下任意键继续...");
    let _ = read();
}
