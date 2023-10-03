use crate::data;
use std::process::exit;
use colored::*;
use noise::{NoiseFn, Perlin};
use rand::{Rng, thread_rng};

use crate::{Buffer, cls, read};

// 玩家数据
struct Player {
    position: (u8, u8),
    money: i32,
    max_hp: i32,
    hp: i32,
}
impl Player {
    fn new() -> Self { Player {position: (0, 0), money: 0, max_hp: 500, hp: 500 } }
}

// 地图数据
#[derive(Clone, Copy)]
struct Block {
    id: i8,
    gift: i32,
}

impl Block {
    fn new() -> Self { Block {id: 0, gift: 0,} }

    fn name(&self) -> String {
        data::block_name(self.id)
    }
}

struct Map {
    data: Vec<Vec<Block>>,
}

impl Map {
    fn create(x: usize, y: usize) -> Self {
        Map { data: vec![vec![Block::new(); x]; y] }
    }

    fn measure(&self) -> (usize, usize) {
        let x_len = self.data[0].len();
        let y_len = self.data.len();
        (x_len, y_len)
    }

    fn terrain(&mut self, block: Block, seed: u32) -> &mut Map {
        let (x_len, y_len) = self.measure();
        let map = self;
        let perlin = Perlin::new(seed);

        for y in 0..y_len {
            let mut row: Vec<Block> = Vec::new();
            for x in 0..x_len {
                let nx = x as f64 / x_len as f64;
                let ny = y as f64 / y_len as f64;
                let noise_value = perlin.get([nx, ny]);

                let next = if (noise_value + 1.0) / 2.0 * 255.0 <= 127f64 {
                    block
                } else {
                    map.data[y][x]
                };

                row.push(next);
            }
            map.data[y] = row;
        }
        map
    }

    fn gen_gift(&mut self, size: i32, tries: u8) -> &mut Map {
        let (x_len, y_len) = self.measure();
        let map = self;

        for _ in 0..tries {
            let (y, x) = (thread_rng().gen_range(0..y_len), thread_rng().gen_range(0..x_len));
            map.data[y][x].gift = size;
        }
        map
    }

    fn print(&self, position: (u8, u8)) -> String {
        let (x_len, y_len) = self.measure();

        let mut out = String::new();

        for y in 0..y_len {
            for x in 0..x_len {
                if (x as u8, y as u8) != position {
                    out = format!("{}{} ", out, self.data[y][x].name());
                } else {
                    out = format!("{}{} ", out, "您".white());
                }
            }
            out = format!("{}\n", out);
        }
        out
    }
}

pub(crate) fn main() {
    let mut b = Buffer { buffer: ColoredString::default() };

    let mut map = Map::create(16, 16);

    let mut player = Player::new();

    let seed: u32 = thread_rng().gen();

    map.terrain(Block { id: 2, gift: 0 }, seed - 1);
    map.terrain(Block { id: 1, gift: 0 }, seed - 2);
    map.gen_gift(thread_rng().gen_range(200..300), 10);

    loop {
        cls();
        b.wl(format!("{} | {}", data::title(), data::VERSION));

        b.wl(map.print(player.position));

        b.print();

        if let Ok(_key) = read() { exit(0); }
    }
}