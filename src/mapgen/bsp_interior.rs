use crate::{random::PRng, rect::Rect, board::components::Tile};

use super::{common, InitBuilder, BuildData};

const MIN_ROOM_SIZE: u32 = 8; // Must be at least 4 (probably?)

pub struct BspInteriorBuilder {
    rects: Vec<Rect>,
}

impl InitBuilder for BspInteriorBuilder {
    fn build(&mut self, rng: &mut PRng, build_data: &mut BuildData) {

        let mut rooms: Vec<Rect> = Vec::new();

        self.rects.clear();
        self.rects.push(Rect::new(
            1,
            1,
            build_data.board.width - 2,
            build_data.board.height - 2,
        ));

        let first_room = self.rects[0];
        self.add_subrects(first_room, rng);

        let rects_copy = self.rects.clone();

        for r in rects_copy.iter() {
            let rect = *r;
            rooms.push(rect);
            common::set_rect(&mut build_data.board, &rect, Tile::Floor);
        }

        build_data.take_snapshot();

        // Corridors.
        for i in 0..rooms.len() - 1 {
            let r = rooms[i];
            let next = rooms[i + 1];
            let from_x = r.x1 + rng.gen_range(0..r.width());
            let from_y = r.y1 + rng.gen_range(0..r.height());
            let to_x = next.x1 + rng.gen_range(0..next.width());
            let to_y = next.y1 + rng.gen_range(0..next.height());

            common::draw_corridor(
                &mut build_data.board, 
                (from_x, from_y).into(), 
                (to_x, to_y).into(),
                true
            );

            build_data.take_snapshot();
        }

        build_data.rects = Some(rooms);
    }
}

impl BspInteriorBuilder {
    #[allow(dead_code)]
    pub fn new() -> Box<Self> {
        Box::new(Self { rects: Vec::new() })
    }

    fn add_subrects(&mut self, rect: Rect, rng: &mut PRng) {
        // Remove last rect from the list
        self.rects.pop();

        let width = rect.width();
        let height = rect.height();
        let half_width = width / 2;
        let half_height = height / 2;

        let split = rng.gen_bool(1.0 / 2.0);

        if split {
            // Horizontal split
            let h1 = Rect::new(rect.x1, rect.y1, half_width - 1, height);
            self.rects.push(h1);
            if half_width > MIN_ROOM_SIZE {
                self.add_subrects(h1, rng);
            }

            let h2 = Rect::new(rect.x1 + half_width, rect.y1, half_width, height);
            self.rects.push(h2);
            if half_width > MIN_ROOM_SIZE {
                self.add_subrects(h2, rng);
            }
        } else {
            // Vertical split
            let v1 = Rect::new(rect.x1, rect.y1, width, half_height - 1);
            self.rects.push(v1);
            if half_height > MIN_ROOM_SIZE {
                self.add_subrects(v1, rng);
            }

            let v2 = Rect::new(rect.x1, rect.y1 + half_height, width, half_height);
            self.rects.push(v2);
            if half_height > MIN_ROOM_SIZE {
                self.add_subrects(v2, rng);
            }
        }
    }
}
