use crate::prelude::*;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

// Map uses row-first encoding
pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES], // create NUM_TILES number of entries each set to TileType::Floor
        }
    }

    // reciprocal -> x = index % WIDTH | y = index / WIDTH
    pub fn map_idx(x: i32, y: i32) -> usize {
        ((y * SCREEN_WIDTH) + x) as usize
    }

    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..SCREEN_HEIGHT {
            // Iterating y first is faster in row-first striding due to memcache usage
            for x in 0..SCREEN_WIDTH {
                let idx = Self::map_idx(x, y);
                match self.tiles[idx] {
                    TileType::Floor => ctx.set(x, y, YELLOW, BLACK, to_cp437('.')),
                    TileType::Wall => ctx.set(x, y, GREEN, BLACK, to_cp437('#')),
                }
            }
        }
    }
}
