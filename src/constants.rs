pub const SCREEN_WIDTH: i32 = 1920;
pub const SCREEN_HEIGHT: i32 = 1080;

pub const IMAGE_WIDTH:u32 = 32;
pub const IMAGE_HEIGHT:u32 = 32;
pub const IMAGE_SCALING:u32 = 1;

pub const OUTPUT_WIDTH: u32 = IMAGE_WIDTH * IMAGE_SCALING;
pub const OUTPUT_HEIGHT: u32 = IMAGE_HEIGHT * IMAGE_SCALING;

pub const PLAYER_SPEED: i32 = 16;

pub const TILE_SIZE: u32 = 32;
pub const MAX_HEIGHT: u32 = 150;
pub const MAX_WIDTH: u32 = 300;
pub const CROP_TIME: u32 = 100;

pub const SEED_BUTTON_AMT: u8 = 8;
pub const BUILD_BUTTON_AMT: u8 = 4;

pub static TEXTURE_DEFAULT: &str = "assets/default-texture.png";

pub static TEXTURE_FIELD_EMPTY: &str = "assets/field-empty.png";
pub static TEXTURE_FIELD_SEEDS: &str = "assets/field-seeds.png";
pub static TEXTURE_FIELD_GROWING: &str = "assets/field1.png";
pub static TEXTURE_FIELD_CARROT: &str = "assets/carrots0.png";
pub static TEXTURE_FIELD_TOMATO: &str = "assets/tomatoes0.png";

pub static TEXTURE_BUTTON_CARROT: &str = "assets/carrot-button.png";
pub static TEXTURE_BUTTON_TOMATO: &str = "assets/tomato-button.png";
pub static TEXTURE_BUTTON_HO: &str = "assets/ho-button.png";
pub static TEXTURE_BUTTON_ARCHER: &str = "assets/archer-button.png";

pub static TEXTURE_TILE_EMPTY: &str = "assets/grass-1.png";
pub static TEXTURE_TILE_WALL: &str = "assets/tile2.png";
pub static TEXTURE_TILE_FLOOR: &str = "assets/tile3.png";

pub static TEXTURE_TOWER_ARCHER_FRONT: &str = "assets/archer-tower-front-top.png";
pub static TEXTURE_TOWER_ARCHER_BACK: &str = "assets/archer-tower-back-top.png";
pub static TEXTURE_TOWER_ARCHER_LEFT: &str = "assets/archer-tower-left-top.png";
pub static TEXTURE_TOWER_ARCHER_RIGHT: &str = "assets/archer-tower-right-top.png";
pub static TEXTURE_TOWER_ARCHER_BOTTOM: &str = "assets/archer-tower-bottom.png";

pub static TEXTURE_GOBLIN_ENEMY_FRONT: &str = "assets/goblin-enemy-front.png";

pub const CURRENT_BUILD_HO: u8 = 0;
pub const CURRENT_BUILD_ARCHER_TOWER: u8 = 1;
pub const CURRENT_BUILD_GOBLIN_TEST: u8 = 2;

pub const CURRENT_SEED_CARROT: u8 = 0;
pub const CURRENT_SEED_TOMATO: u8 = 1;

pub const TILE_TYPE_GRASS: char = '0';
pub const TILE_TYPE_WALL: char = '2';
pub const TILE_TYPE_FLOOR: char = '3';

pub const TILE_TYPE_FIELD_EMPTY: char = 'F';
pub const TILE_TYPE_FIELD_GROWING: char = 'G';
pub const TILE_TYPE_FIELD_HARVESTABLE: char = 'H';

pub const TILE_TYPE_ARCHER_TOP: char = 'A';
pub const TILE_TYPE_ARCHER_BOTTOM: char = 'a';

pub const TILE_TYPE_GOBLIN_TEST: char = 'G';
