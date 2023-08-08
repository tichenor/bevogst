
mod simple_rooms;
mod common;

use crate::{point::Point, random::{self, PRngBuilder}, config, board::{Board, components::Tile}, rect::Rect};

pub struct BuildData {
    pub board: Board,
    pub starting_position: Option<Point>,
    pub rects: Option<Vec<Rect>>,
    pub history: Vec<Vec<Tile>>,
}

impl BuildData {
    fn take_snapshot(&mut self) {
        if config::SHOW_MAP_GEN {
            let ss = self.board.get_tiles_cloned();
            self.history.push(ss);
        }
    }
}

pub trait InitBuilder {
    fn build(&mut self, rng: &mut random::PRng, build_data: &mut BuildData);
}

pub trait MetaBuilder {
    fn build(&mut self, rng: &mut random::PRng, build_data: &mut BuildData);
}

pub struct MapBuilder {
    seed: u64,
    depth: u32,
    starter: Option<Box<dyn InitBuilder>>,
    chainers: Vec<Box<dyn MetaBuilder>>,
    pub build_data: BuildData,
}

impl MapBuilder {
    pub fn new(depth: u32, seed: u64, width: u32, height: u32) -> Self {
        Self {
            depth,
            seed,
            starter: None,
            chainers: Vec::new(),
            build_data: BuildData {
                board: Board::new(depth, width, height),
                starting_position: None,
                rects: None,
                history: Vec::new(),
            }
        }
    }

    pub fn with_starter(mut self, starter: Box<dyn InitBuilder>) -> Self {
        match self.starter {
            None => self.starter = Some(starter),
            Some(_) => panic!("You can only have one starting builder."),
        };
        self
    }

    pub fn with(mut self, metabuilder: Box<dyn MetaBuilder>) -> Self {
        self.chainers.push(metabuilder);
        self
    }

    pub fn build(&mut self) {
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
    }
}
