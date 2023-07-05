//some notes on types..
//
// u8s can save on stack space
// but for just going math, i32 is the default integer promotion target for llvm
// (you should store them as i8s if going to i32s. signed math is optimized differently)
// so storing it as an i8 will just sign extend to an i32 for basically free
// say you have a vec of some struct that has a few i32s in it
// say that vec has 100k elements
// you'd save 3*1e5 bytes of stack/heap space by storing them as i8s over i32s
// (will your values overflow/underflow an i8?)
pub const TIMEOUT_DURATION: u128 = 10;

pub const SCREEN_WIDTH: u16 = 1920;
pub const SCREEN_HEIGHT: u16 = 1080;

pub const IMAGE_WIDTH:u8 = 32;
pub const IMAGE_HEIGHT:u8 = 32;
pub const IMAGE_SCALING:u8 = 1;

pub const OUTPUT_WIDTH: u8 = IMAGE_WIDTH * IMAGE_SCALING;
pub const OUTPUT_HEIGHT: u8 = IMAGE_HEIGHT * IMAGE_SCALING;

pub const COLOR_BACKGROUND: sdl2::pixels::Color = sdl2::pixels::Color::RGBA(69, 69, 69, 255);
pub const COLOR_OUTLINE: sdl2::pixels::Color = sdl2::pixels::Color::RGBA(252, 186, 3, 255);
pub const COLOR_RED: sdl2::pixels::Color = sdl2::pixels::Color::RGBA(255, 0, 0, 255);
pub const COLOR_GREEN: sdl2::pixels::Color = sdl2::pixels::Color::RGBA(0, 255, 0, 255);

pub const TILE_SIZE: u32 = 32;
pub const MAX_HEIGHT: u16 = 150;
pub const MAX_WIDTH: u16 = 300;


pub const PLAYER_SPEED: u8 = 16;
pub const PLAYER_SPEED_DIAGONAL: u8 = 4; //SQUARE ROOT OF PLAYER_SPEED
pub const CROP_TIME: u16 = 100;

pub const SEED_BUTTON_AMT: usize = 8;
pub const BUILD_BUTTON_AMT: usize = 6;

pub static TEXTURE_DEFAULT: &str = "assets/default-texture.png";

pub static TEXTURE_PLAYER_FRONT: &str = "assets/player0-front.png";
pub static TEXTURE_PLAYER_BACK: &str = "assets/player0-back.png";
pub static TEXTURE_PLAYER_LEFT: &str = "assets/player0-left.png";
pub static TEXTURE_PLAYER_RIGHT: &str = "assets/player0-right.png";
pub static TEXTURE_PLAYER_FRONT_LEFT: &str = "assets/player0-front-left.png";
pub static TEXTURE_PLAYER_FRONT_RIGHT: &str = "assets/player0-front-right.png";
pub static TEXTURE_PLAYER_BACK_LEFT: &str = "assets/player0-back-left.png";
pub static TEXTURE_PLAYER_BACK_RIGHT: &str = "assets/player0-back-right.png";
pub static TEXTURE_PLAYER_MOVING_FRONT: &str = "assets/player1-front.png";
pub static TEXTURE_PLAYER_MOVING_BACK: &str = "assets/player1-back.png";
pub static TEXTURE_PLAYER_MOVING_LEFT: &str = "assets/player1-left.png";
pub static TEXTURE_PLAYER_MOVING_RIGHT: &str = "assets/player1-right.png";
pub static TEXTURE_PLAYER_MOVING_FRONT_LEFT: &str = "assets/player1-front-left.png";
pub static TEXTURE_PLAYER_MOVING_FRONT_RIGHT: &str = "assets/player1-front-right.png";
pub static TEXTURE_PLAYER_MOVING_BACK_LEFT: &str = "assets/player1-back-left.png";
pub static TEXTURE_PLAYER_MOVING_BACK_RIGHT: &str = "assets/player1-back-right.png";


pub static TEXTURE_FIELD_EMPTY: &str = "assets/field-empty.png";
pub static TEXTURE_FIELD_SEEDS: &str = "assets/field-seeds.png";
pub static TEXTURE_FIELD_GROWING: &str = "assets/field1.png";
pub static TEXTURE_FIELD_CARROT: &str = "assets/carrots0.png";
pub static TEXTURE_FIELD_TOMATO: &str = "assets/tomatoes0.png";

pub static TEXTURE_BUTTON_SHOVEL: &str = "assets/shovel-button.png";
pub static TEXTURE_BUTTON_HO: &str = "assets/ho-button.png";
pub static TEXTURE_BUTTON_CARROT: &str = "assets/carrot-button.png";
pub static TEXTURE_BUTTON_TOMATO: &str = "assets/tomato-button.png";
pub static TEXTURE_BUTTON_ARCHER: &str = "assets/archer-button.png";

pub static TEXTURE_TILE_EMPTY: &str = "assets/grass-0.png";
pub static TEXTURE_TILE_WALL: &str = "assets/tile2.png";
pub static TEXTURE_TILE_FLOOR: &str = "assets/tile3.png";

pub static TEXTURE_TOWER_ARCHER_FRONT: &str = "assets/archer-tower-front-top.png";
pub static TEXTURE_TOWER_ARCHER_BACK: &str = "assets/archer-tower-back-top.png";
pub static TEXTURE_TOWER_ARCHER_LEFT: &str = "assets/archer-tower-left-top.png";
pub static TEXTURE_TOWER_ARCHER_RIGHT: &str = "assets/archer-tower-right-top.png";
pub static TEXTURE_TOWER_ARCHER_BOTTOM: &str = "assets/archer-tower-bottom.png";

pub static TEXTURE_GOBLIN_ENEMY_FRONT: &str = "assets/goblin-enemy-front.png";

pub static TEXTURE_PROJECTILE_ARROW: &str = "assets/archer-arrow-large.png";

pub const CURRENT_BUILD_ARCHER_TOWER: usize = 0;
pub const CURRENT_BUILD_GOBLIN: usize = 1;

pub const CURRENT_SEED_SHOVEL: usize = 0;
pub const CURRENT_SEED_HO: usize = 1;
pub const CURRENT_SEED_CARROT: usize = 2;
pub const CURRENT_SEED_TOMATO: usize = 3;

pub const TILE_TYPE_GRASS: char = '0';
pub const TILE_TYPE_WALL: char = '2';
pub const TILE_TYPE_FLOOR: char = '3';

pub const TILE_TYPE_FIELD_EMPTY: char = 'F';
pub const TILE_TYPE_FIELD_GROWING: char = 'G';
pub const TILE_TYPE_FIELD_HARVESTABLE: char = 'H';

pub const TILE_TYPE_ARCHER_TOP: char = 'A';
pub const TILE_TYPE_ARCHER_BOTTOM: char = 'a';

pub const TILE_TYPE_GOBLIN: char = 'G';

pub const ENEMY_GOBLIN_HEALTH: u16 = 100;
pub const ENEMY_GOBLIN_RADIUS: u8 = 1;
pub const ENEMY_GOBLIN_SPEED: u8 = 3;
pub const ENEMY_GOBLIN_DAMAGE: u8 = 5;
pub const ENEMY_GOBLIN_ATTACK_SPEED: u8 = 16;

pub const ENEMY_GOBLIN_HEALTH_BAR_WIDTH: u32 = 24;
pub const ENEMY_GOBLIN_HEALTH_BAR_HEIGHT: u32 = 4;

pub const TOWER_ARCHER_DAMAGE: u8 = 5;
pub const TOWER_ARCHER_ATTACK_SPEED: u8 = 32;
pub const TOWER_ARCHER_RADIUS: i32 = 10;
pub const TOWER_ARCHER_HEALTH: u16 = 1000;
pub const TOWER_ARCHER_HEALTH_BAR_WIDTH: u32 = 32;
pub const TOWER_ARCHER_HEALTH_BAR_HEIGHT: u32 = 6;

pub const PROJECTILE_DESPAWN_DURATION: u8 = 32;
pub const PROJECTILE_HIT_DESPAWN_DURATION: u8 = 2;
pub const PROJECTILE_ARROW_SPEED: u8 = 16;

