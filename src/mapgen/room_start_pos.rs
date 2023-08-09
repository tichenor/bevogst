use crate::random::PRng;

use super::{MetaBuilder, BuildData};


pub struct RoomBasedStartingPosition {}

impl RoomBasedStartingPosition {
    #[allow(dead_code)]
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

impl MetaBuilder for RoomBasedStartingPosition {
    fn build(&mut self, _rng: &mut PRng, build_data: &mut BuildData) {
        if let Some(ref rooms) = build_data.rects {
            build_data.starting_position = Some(rooms[0].center());
        } else {
            panic!("RoomBasedStartingPosition requires that build_data.rooms is not None");
        }
    }
}
