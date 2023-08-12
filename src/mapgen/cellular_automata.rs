use crate::{point, board::components::Tile, random::PRng};

use super::{BuildData, InitBuilder, MetaBuilder};

const PERCENT_FLOOR: u32 = 55;
const NUM_ITERATIONS: u32 = 15;

pub struct CellularAutomataBuilder {}

impl CellularAutomataBuilder {

    #[allow(dead_code)]
    pub fn new() -> Box<Self> {
        Box::new(CellularAutomataBuilder {})
    }

    fn iterate(build_data: &mut BuildData) {
        let mut new_tiles = build_data.board.get_tiles_cloned();

        for y in 1..build_data.board.height - 1 {
            for x in 1..build_data.board.width - 1 {
                let mut neighbors = 0;

                // cardinal + diagonal directions
                for p in point::Point::OCTANT {
                    let nbr_x = x as i32 + p.x;
                    let nbr_y = y as i32 + p.y;
                    if !build_data.board.in_bounds_xy(nbr_x, nbr_y) {
                        continue;
                    }
                    if build_data.board.get_tile_xy(nbr_x as u32, nbr_y as u32) == Tile::Wall {
                        neighbors += 1;
                    }
                }

                if neighbors > 4 || neighbors == 0 {
                    new_tiles[build_data.board.xy_to_index(x, y)] = Tile::Wall;
                } else {
                    new_tiles[build_data.board.xy_to_index(x, y)] = Tile::Floor;
                }
            }
        }
        build_data.board.set_all_tiles(new_tiles);
        build_data.take_snapshot();
    }
}

impl InitBuilder for CellularAutomataBuilder {
    fn build(&mut self, rng: &mut PRng, build_data: &mut BuildData) {
        // Randomize the map (keeping a border)
        for y in 1..build_data.board.height - 1 {
            for x in 1..build_data.board.width - 1 {
                if rng.gen_ratio(PERCENT_FLOOR, 100) {
                    build_data.board.set_tile_xy(x, y, Tile::Floor);
                }
            }
        }
        build_data.take_snapshot();

        // Iteratively apply cellular automata rules.
        for _ in 0..NUM_ITERATIONS {
            Self::iterate(build_data);
        }
    }
}

impl MetaBuilder for CellularAutomataBuilder {
    fn build(&mut self, _rng: &mut PRng, build_data: &mut BuildData) {
        Self::iterate(build_data);
    }
}
