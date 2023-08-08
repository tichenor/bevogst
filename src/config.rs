pub const MELEE_ATTACK_SEED: u64 = 0x6faf2f42b2ee28f0;
pub const MAP_GENERATION_SEED: u64 = 0x5e7d30cd44e8330d;
//pub const ENTITY_GENERATION_SEED: u64 = 0x97c8e4be8964d095;
pub const AI_SEED: u64 = 0x3c72906cc95045bb;

pub const SHOW_MAP_GEN: bool = true;

pub const WINDOW_WIDTH: f32 = 960.;
pub const WINDOW_HEIGHT: f32 = 600.;

pub mod map {
    pub const MAP_TILE_WIDTH: u32 = 80;
    pub const MAP_TILE_HEIGHT: u32 = 50;

    pub const ROOM_NUM_TRIES: i32 = 30;

    pub const ROOM_MIN_WIDTH: u32 = 5;
    pub const ROOM_MAX_WIDTH: u32 = 11;
    pub const ROOM_MIN_HEIGHT: u32 = 5;
    pub const ROOM_MAX_HEIGHT: u32 = 11;
}
