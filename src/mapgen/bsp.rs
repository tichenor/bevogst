use crate::{rect::Rect, random::PRng, board::{components::Tile, Board}};

use super::{common, InitBuilder, BuildData};

const NUM_TRIES: u32 = 600;
const MAX_SUBRECT_WIDTH: u32 = 10;
const MAX_SUBRECT_HEIGHT: u32 = 10;

pub struct BspMapBuilder {
    rects: Vec<Rect>,
}

impl InitBuilder for BspMapBuilder {

    fn build(&mut self, rng: &mut PRng, build_data: &mut BuildData) {
        let mut rects: Vec<Rect> = Vec::new();

        self.rects.clear();
        self.rects.push(Rect::new(
            2,
            2,
            build_data.board.width - 4,
            build_data.board.height - 4,
        ));

        let first_rect = self.rects[0];
        self.add_subrects(first_rect);

        let mut n_rects = 0;

        while n_rects < NUM_TRIES {
            let rect = self.get_random_rect(rng);
            let candidate = self.get_random_sub_rect(rect, rng);

            if self.is_possible(candidate, &build_data.board) {
                common::set_rect(&mut build_data.board, &candidate, Tile::Floor);
                rects.push(candidate);
                self.add_subrects(rect);
                build_data.take_snapshot();
            }

            n_rects += 1;
        }

        for i in 0..rects.len() - 1 {
            let r = rects[i];
            let next_r = rects[i + 1];
            let start_x = r.x1 + rng.gen_range(0..u32::abs_diff(r.x1, r.x2));
            let start_y = r.y1 + rng.gen_range(0..u32::abs_diff(r.y1, r.y2));
            let end_x = next_r.x1 + rng.gen_range(0..u32::abs_diff(next_r.x1, next_r.x2));
            let end_y = next_r.y1 + rng.gen_range(0..u32::abs_diff(next_r.y1, next_r.y2));

            common::draw_corridor(
                &mut build_data.board, 
                (start_x, start_y).into(), 
                (end_x, end_y).into(), 
                true
            );

            build_data.take_snapshot();
        }
        build_data.rects = Some(rects);
    }
}

impl BspMapBuilder {
    #[allow(dead_code)]
    pub fn new() -> Box<BspMapBuilder> {
        Box::new(BspMapBuilder { rects: Vec::new() })
    }

    /// Panics if self.rects.len() == 0
    fn get_random_rect(&self, rng: &mut PRng) -> Rect {
        if self.rects.len() == 1 {
            self.rects[0]
        } else {
            let i = rng.gen_range(0..self.rects.len()) as usize;
            self.rects[i]
        }
    }

    fn add_subrects(&mut self, rect: Rect) {
        let width = rect.width();
        let height = rect.height();
        let half_width = u32::max(width / 2, 1);
        let half_height = u32::max(height / 2, 1);

        self.rects
            .push(Rect::new(rect.x1, rect.y1, half_width, half_height));

        self.rects.push(Rect::new(
            rect.x1,
            rect.y1 + half_height,
            half_width,
            half_height,
        ));

        self.rects.push(Rect::new(
            rect.x1 + half_width,
            rect.y1,
            half_width,
            half_height,
        ));

        self.rects.push(Rect::new(
            rect.x1 + half_width,
            rect.y1 + half_height,
            half_width,
            half_height,
        ));
    }

    fn get_random_sub_rect(&self, rect: Rect, rng: &mut PRng) -> Rect {
        let mut result = rect;
        let r_width = rect.width();
        let r_height = rect.height();

        let w = u32::max(3, rng.gen_range(0..u32::min(r_width, MAX_SUBRECT_WIDTH))) + 1;
        let h = u32::max(3, rng.gen_range(0..u32::min(r_height, MAX_SUBRECT_HEIGHT))) + 1;

        result.x1 += rng.gen_range(0..6);
        result.y1 += rng.gen_range(0..6);
        result.x2 = result.x1 + w - 1;
        result.y2 = result.y1 + h + 1;

        result
    }

    fn is_possible(&self, rect: Rect, board: &Board) -> bool {
        let mut expanded = rect;
        let x1 = expanded.x1 as i32 - 2;
        let y1 = expanded.y1 as i32 - 2;
        let x2 = expanded.x2 as i32 + 2;
        let y2 = expanded.y2 as i32 + 2;

        let mut can_build = true;

        for y in y1..=y2 {
            for x in x1..=x2 {
                if x > board.width as i32 - 2 || y > board.height as i32 - 2 || x < 1 || y < 1 {
                    can_build = false;
                } else if !matches!(board.get_tile_xy(x as u32, y as u32), Tile::Wall) {
                    can_build = false;
                }
            }
        }
        can_build
    }
}
