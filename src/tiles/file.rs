use std::collections::HashMap;

use crate::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TilemapFile {
    pub name: String,
    ///recommended tileset for this map
    ///some systems may use this to autoload the atlas
    pub tileset: String,
    pub flags: HashMap<u32, String>,
    pub tiles: Vec<TileDescriptor>,
    pub map: Vec<Vec<u16>>,
    pub data: TilemapDataDescriptor,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct TileDescriptor {
    pub image: String,
    pub flags: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct TilemapDataDescriptor {
    pub start: (u32, u32),
    pub exits: Vec<(u32, u32, String, u32, u32)>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct MapExit {
    pub position: MapPosition,
    pub target_map: String,
    pub target_pos: MapPosition,
}

impl MapExit {
    pub fn from_file(data: (u32, u32, String, u32, u32)) -> MapExit {
        MapExit {
            position: MapPosition::new(data.0, data.1),
            target_map: data.2,
            target_pos: MapPosition::new(data.3, data.4),
        }
    }
}

impl From<(u32, u32, String, u32, u32)> for MapExit {
    fn from(value: (u32, u32, String, u32, u32)) -> Self {
        MapExit::from_file(value)
    }
}
