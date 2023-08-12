use bevy::prelude::*;

mod simple_rooms;
mod common;
mod room_corridors;
mod room_start_pos;
mod bsp;
mod bsp_interior;
mod cellular_automata;

use crate::{point::Point, random::{self, PRngBuilder}, config, board::{Board, components::Tile}, rect::Rect, state::MainState};

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum MapGenSet {
    Generation,
    Spawning,
}

pub struct MapGenPlugin;

impl Plugin for MapGenPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BuildData>()
            .configure_set(
                OnEnter(MainState::Game), 
                MapGenSet::Generation.before(MapGenSet::Spawning)
            );
    }
}

pub fn random_builder(seed: u64, depth: u32) -> BuildData {
    MapBuilder::new(
        depth, 
        seed, 
        config::map::MAP_TILE_WIDTH, 
        config::map::MAP_TILE_HEIGHT
    )
        .with_starter(simple_rooms::SimpleRoomBuilder::new())
        .with(room_corridors::RoomCorridors::new())
        .with(room_start_pos::RoomBasedStartingPosition::new())
        .build()
}

#[derive(Debug, Resource, Clone)]
pub struct BuildData {
    pub board: Board,
    pub starting_position: Option<Point>,
    pub rects: Option<Vec<Rect>>,
    pub corridors: Option<Vec<Vec<usize>>>,
    pub history: Vec<Vec<Tile>>,
}

impl Default for BuildData {
    fn default() -> Self {
        Self {
            board: Board::new(0, 2, 2),
            starting_position: None,
            rects: None,
            corridors: None,
            history: Vec::new(),
        }
    }
}

impl BuildData {
    fn take_snapshot(&mut self) {
        if config::SHOW_MAP_GEN {
            let ss = self.board.get_tiles_cloned();
            self.history.push(ss);
        }
    }
}

trait InitBuilder {
    fn build(&mut self, rng: &mut random::PRng, build_data: &mut BuildData);
}

trait MetaBuilder {
    fn build(&mut self, rng: &mut random::PRng, build_data: &mut BuildData);
}

struct MapBuilder {
    seed: u64,
    depth: u32,
    starter: Option<Box<dyn InitBuilder>>,
    chainers: Vec<Box<dyn MetaBuilder>>,
    pub build_data: BuildData,
}

impl MapBuilder {
    fn new(depth: u32, seed: u64, width: u32, height: u32) -> Self {
        Self {
            depth,
            seed,
            starter: None,
            chainers: Vec::new(),
            build_data: BuildData {
                board: Board::new(depth, width, height),
                starting_position: None,
                rects: None,
                corridors: None,
                history: Vec::new(),
            }
        }
    }

    fn with_starter(mut self, starter: Box<dyn InitBuilder>) -> Self {
        match self.starter {
            None => self.starter = Some(starter),
            Some(_) => panic!("You can only have one starting builder."),
        };
        self
    }

    fn with(mut self, metabuilder: Box<dyn MetaBuilder>) -> Self {
        self.chainers.push(metabuilder);
        self
    }

    fn build(&mut self) -> BuildData {
        let mut prng = PRngBuilder::new_seeded(config::MAP_GENERATION_SEED)
            .write_u32(self.depth)
            .write_u64(self.seed)
            .build();

        match &mut self.starter {
            None => panic!("Cannot run map builder chain without an initial builder"),
            Some(starter) => {
                starter.build(&mut prng, &mut self.build_data);
            }
        }

        for builder in self.chainers.iter_mut() {
            builder.build(&mut prng, &mut self.build_data);
        }

        self.build_data.clone()
    }
}
