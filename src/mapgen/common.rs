use crate::{random::PRng, rect::Rect};


/// Generate a [Rect] of dimensions `min_width <= w < max_width`, `min_height <= h < max_height` at a
/// random position `(x, y)` where `x_min <= x < x_max - w` and `y_min <= y < y_max - h`.
pub fn random_rect(
    min_width: u32,
    max_width: u32,
    min_height: u32,
    max_height: u32,
    x_min: u32,
    x_max: u32,
    y_min: u32,
    y_max: u32,
    rng: &mut PRng,
) -> Rect {
    let w: u32 = rng.gen_range(min_width..max_width);
    let h: u32 = rng.gen_range(min_height..max_height);
    let x: u32 = rng.gen_range(x_min..(x_max - w));
    let y: u32 = rng.gen_range(y_min..(y_max - h));
    Rect::new(x, y, w, h)
}
