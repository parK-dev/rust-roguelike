use crate::prelude::*;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

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

    // Map uses row-first encoding
    // Iterating y first is faster in row-first striding due to memcache usage
    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let idx = Self::map_idx(x, y);
                match self.tiles[idx] {
                    TileType::Floor => ctx.set(x, y, YELLOW, BLACK, to_cp437('.')),
                    TileType::Wall => ctx.set(x, y, GREEN, BLACK, to_cp437('#')),
                }
            }
        }
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point) && self.tiles[Self::map_idx(point.x, point.y)] == TileType::Floor
    }

    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(Self::map_idx(point.x, point.y))
        }
    }
}
