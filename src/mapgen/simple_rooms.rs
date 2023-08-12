use super::{InitBuilder, common};

use crate::{config, rect::Rect, board::components::Tile};

const NUM_TRIES: i32 = config::map::ROOM_NUM_TRIES;
const MIN_WIDTH: u32 = config::map::ROOM_MIN_WIDTH;
const MIN_HEIGHT: u32 = config::map::ROOM_MIN_HEIGHT;
const MAX_WIDTH: u32 = config::map::ROOM_MAX_WIDTH;
const MAX_HEIGHT: u32 = config::map::ROOM_MAX_HEIGHT;

pub(super) struct SimpleRoomBuilder;

impl SimpleRoomBuilder {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

impl InitBuilder for SimpleRoomBuilder {
    fn build(&mut self, rng: &mut crate::random::PRng, build_data: &mut super::BuildData) {
        let mut rooms: Vec<Rect> = Vec::new();

        for _ in 0..NUM_TRIES {
            let new_room = common::random_rect(
                MIN_WIDTH,
                MAX_WIDTH,
                MIN_HEIGHT,
                MAX_HEIGHT,
                1,
                build_data.board.width - 1,
                1,
                build_data.board.height - 1,
                rng,
            );

            if !rooms.iter().any(|r| new_room.intersects(r, 1)) {
                build_data.board.set_rect(&new_room, Tile::Floor);
                rooms.push(new_room);
                build_data.take_snapshot();
            }
        }

        build_data.rects = Some(rooms);
    }
}
