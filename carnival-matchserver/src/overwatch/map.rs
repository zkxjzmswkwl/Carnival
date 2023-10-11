use core::time;
use std::thread;

use winput::Vk;

use crate::input;

#[derive(Debug)]
pub struct MapData {
    idx: u32,
    varied_skybox: bool
}

pub enum Map {
    Antarctic,
    Busan,
    Ilios,
    Lijiang,
    Nepal,
    Oasis,
    Samoa, 
    Circuit,
    Dorado,
    Havana,
    Junkertown,
    Rialto,
    Route66,
    Shambali,
    Gibraltar,
    JunkCity,
    Suravasa,
    BlizzardWorld,
    Eichenwalde,
    Hollywood,
    KingsRow,
    Midtown,
    Numbani,
    Paraiso,
    Colosseo,
    Esperanca,
    NewQueenStreet
}

impl Map {
    pub fn data(&self) -> MapData {
        match self {
            Map::Antarctic => MapData {idx: 1, varied_skybox: false },
            Map::Busan => MapData {idx: 2, varied_skybox: true },
            Map::Ilios => MapData {idx: 3, varied_skybox: true },
            Map::Lijiang => MapData {idx: 4, varied_skybox: true },
            Map::Nepal => MapData {idx: 6, varied_skybox: true },
            Map::Oasis => MapData {idx: 7, varied_skybox: true },
            Map::Samoa => MapData {idx: 8, varied_skybox: false },
            Map::Circuit => MapData {idx: 9, varied_skybox: true },
            Map::Dorado => MapData {idx: 10, varied_skybox: true },
            Map::Havana => MapData {idx: 11, varied_skybox: true },
            Map::Junkertown => MapData {idx: 12, varied_skybox: false },
            Map::Rialto => MapData {idx: 13, varied_skybox: true },
            Map::Route66 => MapData {idx: 14, varied_skybox: true },
            Map::Shambali => MapData {idx: 15, varied_skybox: false },
            Map::Gibraltar => MapData {idx: 16, varied_skybox: true },
            Map::JunkCity => MapData {idx: 17, varied_skybox: false },
            Map::Suravasa  => MapData {idx: 18, varied_skybox: false },
            Map::BlizzardWorld => MapData {idx: 19, varied_skybox: true },
            Map::Eichenwalde => MapData {idx: 21, varied_skybox: true },
            Map::Hollywood => MapData {idx: 23, varied_skybox: true },
            Map::KingsRow => MapData {idx: 25, varied_skybox: true },
            Map::Midtown => MapData {idx: 27, varied_skybox: true },
            Map::Numbani => MapData {idx: 28, varied_skybox: false },
            Map::Paraiso => MapData {idx: 29, varied_skybox: true },
            Map::Colosseo => MapData {idx: 30, varied_skybox: true },
            Map::Esperanca => MapData {idx: 31, varied_skybox: false},
            Map::NewQueenStreet => MapData {idx: 32, varied_skybox: true },
        }
    }
    pub fn select(self) {
        for _ in 1..self.data().idx as usize {
            input::keypress(Vk::DownArrow, 30);
        }
        thread::sleep(time::Duration::from_millis(200));
        if self.data().varied_skybox {
            input::keypress_for_duration(Vk::Space, 300);
            input::keypress(Vk::DownArrow, 100);
        }
        input::keypress(Vk::RightArrow, 30);
    }
}