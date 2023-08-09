use std::collections::HashMap;

use bevy::prelude::*;

use crate::{point::Point, state::MainState, bitgrid::BitGrid, rect::Rect};

use self::components::Tile;


pub mod components;
mod systems;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentBoard>()
            .add_systems(OnEnter(MainState::Game), systems::spawn_board);

    }
}

#[derive(Default, Resource)]
pub struct CurrentBoard {
    pub tiles: HashMap<Point, Entity>
}

#[derive(Resource, Clone, Debug)]
pub struct Board {
    pub width: u32,
    pub height: u32,
    tiles: Vec<Tile>,
    pub seen: BitGrid,
}

impl Board {
    pub fn new(depth: u32, width: u32, height: u32) -> Self {
        assert!(width > 1);
        assert!(height > 1);

        Self {
            width,
            height,
            tiles: vec![Tile::Wall; (width * height) as usize],
            seen: BitGrid::new(width, height),
        }
    }

    pub fn get_tile_xy(&self, x: u32, y: u32) -> Tile {
        self.tiles[self.xy_to_index(x, y)]
    }

    pub fn get_tile(&self, i: usize) -> Tile {
        self.tiles[i]
    }

    pub fn get_tiles(&self) -> &Vec<Tile> {
        &self.tiles
    }

    pub fn get_tiles_cloned(&self) -> Vec<Tile> {
        self.tiles.clone()
    }

    pub fn get_tiles_mut(&mut self) -> &mut Vec<Tile> {
        &mut self.tiles
    }

    pub fn set_tile_xy(&mut self, x: u32, y: u32, tile: Tile) {
        assert!(self.in_bounds_xy(x, y));
        self.tiles[self.xy_to_index(x, y)] = tile;
    }

    pub fn set_tile(&mut self, i: usize, tile: Tile) {
        assert!(self.in_bounds(i));
        self.tiles[i] = tile;
    }

    pub fn set_rect(&mut self, rect: &Rect, tile: Tile) {
        let b = self.bounds();
        assert!(rect.x1 >= b.0 && rect.x2 <= b.2);
        assert!(rect.y1 >= b.1 && rect.y2 <= b.3);

        for y in rect.y1..=rect.y2 {
            for x in rect.x1..=rect.x2 {
                self.set_tile_xy(x, y, tile);
            }
        }
    }

    pub fn xy_to_index(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }

    pub fn index_to_xy(&self, i: usize) -> (u32, u32) {
        (i as u32 % self.width, i as u32 / self.width)
    }

    /// (min_width, min_height, max_width, max_height)
    pub fn bounds(&self) -> (u32, u32, u32, u32) {
        (0, 0, self.width - 1, self.height - 1)
    }

    pub fn in_bounds_xy(&self, x: u32, y: u32) -> bool {
        let b = self.bounds();
        x >= b.0 
        && x <= b.2
        && y >= b.1
        && y <= b.3
    }

    pub fn in_bounds(&self, i: usize) -> bool {
        let (x, y) = self.index_to_xy(i);
        self.in_bounds_xy(x, y)
    }
}

