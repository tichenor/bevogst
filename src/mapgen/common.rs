use crate::{random::PRng, rect::Rect, board::{Board, components::Tile}, point::Point};

/// Set all [Tile]s in the given [Rect] to the specified tile type.
pub fn set_rect(board: &mut Board, rect: &Rect, tile: Tile) {
    assert!(rect.x1 >= 0 && rect.x2 < board.width);
    assert!(rect.y1 >= 0 && rect.y2 < board.height);

    for y in rect.y1..=rect.y2 {
        for x in rect.x1..=rect.x2 {
            board.set_tile_xy(x, y, tile);
        }
    }
}

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

/// Draw an L-shaped corridor (i.e. set [Tile::Floor]s) between the specified coordinates.
/// Goes horizontally first and then vertically if `h_then_v` is `true`. Returns a list
/// of the corridor's tile indices. Only the indices that were modified are returned.
pub fn draw_corridor(
    board: &mut Board,
    source: Point,
    destination: Point,
    h_then_v: bool,
) -> Vec<usize> {
    let (from_x, from_y, to_x, to_y) = (source.x, source.y, destination.x, destination.y);

    if h_then_v {
        draw_corridor_h_then_v(board, from_x, from_y, to_x, to_y)
    } else {
        draw_corridor_v_then_h(board, from_x, from_y, to_x, to_y)
    }
}

fn draw_corridor_v_then_h(
    board: &mut Board,
    from_x: i32,
    from_y: i32,
    to_x: i32,
    to_y: i32,
) -> Vec<usize> {
    let mut x = from_x;
    let mut y = from_y;
    let mut corridor = Vec::new();

    while x != to_x || y != to_y {
        if x < to_x {
            x += 1;
        } else if x > to_x {
            x -= 1;
        } else if y < to_y {
            y += 1
        } else if y > to_y {
            y -= 1;
        }
        assert!(x >= 0);
        assert!(y >= 0);
        if board.get_tile_xy(x as u32, y as u32) != Tile::Floor {
            board.set_tile_xy(x as u32, y as u32, Tile::Floor);
            corridor.push(board.xy_to_index(x as u32, y as u32));
        }
    }
    corridor
}

fn draw_corridor_h_then_v(
    board: &mut Board,
    from_x: i32,
    from_y: i32,
    to_x: i32,
    to_y: i32,
) -> Vec<usize> {
    let mut x = from_x;
    let mut y = from_y;
    let mut corridor = Vec::new();

    while x != to_x || y != to_y {
        if y < to_y {
            y += 1;
        } else if y > to_y {
            y -= 1;
        } else if x < to_x {
            x += 1;
        } else if x > to_x {
            x -= 1;
        }
        assert!(x >= 0);
        assert!(y >= 0);
        if board.get_tile_xy(x as u32, y as u32) != Tile::Floor {
            board.set_tile_xy(x as u32, y as u32, Tile::Floor);
            corridor.push(board.xy_to_index(x as u32, y as u32));
        }
    }
    corridor
}
