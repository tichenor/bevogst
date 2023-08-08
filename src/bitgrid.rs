use bitvec::prelude::*;
use serde::{Serialize, Deserialize};

/// A width-by-height-sized [BitVec] for convenient handling of a grid of boolean values.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BitGrid {
    width: u32,
    height: u32,
    #[serde(with = "crate::io::saveload::bit_vec")]
    bv: BitVec,
}

impl BitGrid {

    /// Create a new [BitGrid] with the given width and height.
    pub fn new(width: u32, height: u32) -> Self {
        assert!(width >= 0);
        assert!(height >= 0);

        Self {
            width,
            height,
            bv: bitvec![0; (width * height) as usize],
        }
    }

    pub fn set_all_ones(&mut self) {
        self.bv = bitvec![1; (self.width * self.height) as usize];
    }

    fn index(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }

    pub fn zero_out_bits(&mut self) {
        self.bv.set_elements(0);
    }

    /// Get the [bool] at the given `(x, y)` coordinate.
    pub fn get_bit(&self, x: u32, y: u32) -> bool {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            false
        } else {
            self.bv[self.index(x, y)]
        }
    }

    /// Set the [bool] at the given `(x, y)` coordinate to a value.
    pub fn set_bit(&mut self, x: u32, y: u32, value: bool) {
        let index = self.index(x, y);
        self.bv.set(index, value);
    }

    /// Apply all true elements of this [BitGrid] onto another.
    /// 
    /// # Panics
    /// 
    /// Panics if any true bits of self would fall outside of the other grid, given the offset.
    pub fn apply_bits_onto(&self, other: &mut BitGrid, offset_x: u32, offset_y: u32) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.bv[self.index(x, y)] {
                    let other_index = other.index(x + offset_x, y + offset_y);
                    other.bv.set(other_index, true);
                }
            }
        }
    }

}
