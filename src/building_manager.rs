use crate::button_manager;
use crate::constants;
use crate::event_manager;
use crate::player_manager;
use crate::game_manager;
use crate::level_manager;
use crate::level_manager::LevelTile;
use crate::level_manager::TileData;
use crate::texture_manager;
use crate::projectile_manager;
use crate::gui_manager;
use crate::tower_manager;
use crate::enemy_manager;
use crate::upgrade_manager;

#[derive(PartialEq)]
pub enum BuildingType {
    Base,
    None,
}

pub struct Building {
    pub bottom_left_rect: sdl2::rect::Rect,
    pub bottom_right_rect: sdl2::rect::Rect,
    pub top_left_rect: sdl2::rect::Rect,
    pub top_right_rect: sdl2::rect::Rect,
    pub texture_path_bottom_left: String,
    pub texture_path_bottom_right: String,
    pub texture_path_top_left: String,
    pub texture_path_top_right: String,    
    pub building_type: BuildingType,
    pub grid_index: (usize, usize),
    pub pixel_index: (i32, i32),
    pub last_damaged: u16,
    pub max_health: u16,
    pub health: u16,
}

pub struct BuildingManager {
    pub building_vec: Vec<Building>,
    pub base_created: bool,
}

impl BuildingManager {
    pub fn new() -> BuildingManager {
        let buildings = BuildingManager {
            building_vec: Vec::new(),
            base_created: false,
        };
        buildings
    }
    pub fn create_building (
        &mut self, 
        game: &mut game_manager::GameManager,
        gui_manager: &mut gui_manager::GUIManager,
        building_type: BuildingType,
        temp_tile: &mut LevelTile,
        col_index: usize, 
        row_index: usize, 
    ) {
        match building_type {
            BuildingType::Base => {
                temp_tile.tile_type = constants::TILE_TYPE_BASE;
                temp_tile.tile_data = TileData::Base;

                let building = self::Building {
                    bottom_left_rect: sdl2::rect::Rect::new(temp_tile.rect.x(), temp_tile.rect.y(), constants::TILE_SIZE, constants::TILE_SIZE),
                    bottom_right_rect: sdl2::rect::Rect::new(temp_tile.rect.x() + constants::TILE_SIZE as i32, temp_tile.rect.y(), constants::TILE_SIZE, constants::TILE_SIZE),
                    top_left_rect: sdl2::rect::Rect::new(temp_tile.rect.x(), temp_tile.rect.y() + constants::TILE_SIZE as i32, constants::TILE_SIZE, constants::TILE_SIZE),
                    top_right_rect: sdl2::rect::Rect::new(temp_tile.rect.x() + constants::TILE_SIZE as i32, temp_tile.rect.y() + constants::TILE_SIZE as i32, constants::TILE_SIZE, constants::TILE_SIZE),
                    texture_path_bottom_left: constants::TEXTURE_BUILDING_HOUSE_BOTTOM_LEFT.to_string(),
                    texture_path_bottom_right: constants::TEXTURE_BUILDING_HOUSE_BOTTOM_RIGHT.to_string(),
                    texture_path_top_left: constants::TEXTURE_BUILDING_HOUSE_TOP_LEFT.to_string(),
                    texture_path_top_right: constants::TEXTURE_BUILDING_HOUSE_TOP_RIGHT.to_string(),
                    building_type: BuildingType::Base,
                    grid_index: (col_index, row_index),
                    pixel_index: ((col_index * constants::TILE_SIZE as usize) as i32, (row_index * constants::TILE_SIZE as usize) as i32),
                    last_damaged: 0,
                    max_health: constants::BUILDING_BASE_HEALTH,
                    health: constants::BUILDING_BASE_HEALTH,
                };

                if !self.base_created {
                    self.base_created = true;
                    gui_manager.create_message("base created, make sure you keep it safe".to_string(), 256);
                    self.building_vec.push(building);
                    game.base_location = Some((col_index, row_index));
                    if let Some(base_location) = game.base_location {
                        game.target_vec.push(base_location);
                    }
                }
                else {
                    gui_manager.create_message("base already created".to_string(), 128);
                }
            },
            BuildingType::None => {
                let building = self::Building {
                    bottom_left_rect: sdl2::rect::Rect::new(temp_tile.rect.x(), temp_tile.rect.y(), constants::TILE_SIZE, constants::TILE_SIZE),
                    bottom_right_rect: sdl2::rect::Rect::new(temp_tile.rect.x() + constants::TILE_SIZE as i32, temp_tile.rect.y(), constants::TILE_SIZE, constants::TILE_SIZE),
                    top_left_rect: sdl2::rect::Rect::new(temp_tile.rect.x(), temp_tile.rect.y() + constants::TILE_SIZE as i32, constants::TILE_SIZE, constants::TILE_SIZE),
                    top_right_rect: sdl2::rect::Rect::new(temp_tile.rect.x() + constants::TILE_SIZE as i32, temp_tile.rect.y() + constants::TILE_SIZE as i32, constants::TILE_SIZE, constants::TILE_SIZE),
                    texture_path_bottom_left: constants::TEXTURE_DEFAULT.to_string(),
                    texture_path_bottom_right: constants::TEXTURE_DEFAULT.to_string(),
                    texture_path_top_left: constants::TEXTURE_DEFAULT.to_string(),
                    texture_path_top_right: constants::TEXTURE_DEFAULT.to_string(),
                    building_type: BuildingType::None,
                    grid_index: (col_index, row_index),
                    pixel_index: ((col_index * constants::TILE_SIZE as usize) as i32, (row_index * constants::TILE_SIZE as usize) as i32),
                    last_damaged: 0,
                    max_health: 0,
                    health: 0,
                };
                self.building_vec.push(building);
            },
        }
    }
    pub fn render_buildings (
        &mut self,
        game: &mut game_manager::GameManager,
        tex_man: &mut texture_manager::TextureManager<sdl2::video::WindowContext>,
        gui_manager: &mut gui_manager::GUIManager,
    ) -> Result<(), String> {
        for building in &mut self.building_vec {
            building.bottom_left_rect.set_x(building.pixel_index.0 - game.cam_x);
            building.bottom_left_rect.set_y(building.pixel_index.1 - game.cam_y);
            let texture_bottom_left = tex_man.load(&building.texture_path_bottom_left)?;
            building.bottom_right_rect.set_x(building.pixel_index.0 - game.cam_x + constants::TILE_SIZE as i32);
            building.bottom_right_rect.set_y(building.pixel_index.1 - game.cam_y);
            let texture_bottom_right = tex_man.load(&building.texture_path_bottom_right)?;
            building.top_left_rect.set_x(building.pixel_index.0 - game.cam_x);
            building.top_left_rect.set_y(building.pixel_index.1 - game.cam_y - constants::TILE_SIZE as i32);
            let texture_top_left = tex_man.load(&building.texture_path_top_left)?;
            building.top_right_rect.set_x(building.pixel_index.0 - game.cam_x + constants::TILE_SIZE as i32);
            building.top_right_rect.set_y(building.pixel_index.1 - game.cam_y - constants::TILE_SIZE as i32);
            let texture_top_right = tex_man.load(&building.texture_path_top_right)?;


            game.canvas.copy_ex(
                &texture_bottom_left,
                None,
                building.bottom_left_rect,
                0.0,
                None,
                false,
                false,
            )?;
            game.canvas.copy_ex(
                &texture_bottom_right,
                None,
                building.bottom_right_rect,
                0.0,
                None,
                false,
                false,
            )?;
            game.canvas.copy_ex(
                &texture_top_left,
                None,
                building.top_left_rect,
                0.0,
                None,
                false,
                false,
            )?;
            game.canvas.copy_ex(
                &texture_top_right,
                None,
                building.top_right_rect,
                0.0,
                None,
                false,
                false,
            )?;
            if building.health < building.max_health {
                gui_manager.render_health_bar_buildings(game, building);
                building.last_damaged += 1;
                if building.last_damaged > 256 {
                    building.health += 1;
                }
            }
        }
        Ok(())
    }

    pub fn update_buildings(
        &mut self,
        game: &mut game_manager::GameManager,
        events: &mut event_manager::EventManager,
        level: &mut level_manager::LevelManager, 
        player: &mut player_manager::PlayerManager,
        towers: &mut tower_manager::TowerManager, 
        enemies: &mut enemy_manager::EnemyManager, 
        upgrade_manager: &mut upgrade_manager::UpgradeManager,
        gui_manager: &mut gui_manager::GUIManager,
        seed_buttons: &mut button_manager::ButtonManager, 
        build_buttons: &mut button_manager::ButtonManager,
        projectiles: &mut projectile_manager::ProjectileManager,
        // temp_tile: &mut LevelTile,
        // col_index: usize,
        // row_index: usize,
    ) {
        for col_index in 0..level.level_vec.len() {
            for row_index in 0..level.level_vec[col_index].len() {
                let temp_tile = &mut level.level_vec[col_index][row_index];

                if !game.hovering_button && sdl2::rect::Rect::contains_point(&temp_tile.rect, game.mouse_point){
                    if game.build_mode {
                        self.build_mode(game, events, towers, enemies, gui_manager, build_buttons, temp_tile, col_index, row_index);
                    }
                    else if game.seed_mode {
                        self.seed_mode(game, events, player, gui_manager, seed_buttons, projectiles, temp_tile, col_index, row_index);
                    }
                    else if game.mouse_button == sdl2::mouse::MouseButton::Left {
                        upgrade_manager.check_upgrade(game, towers, self, gui_manager, temp_tile, (col_index, row_index));
                        //upgrade mode
                    }
                    else if game.mouse_button == sdl2::mouse::MouseButton::Right {
                        println!("INSPECT TILE TYPE: {:?}", temp_tile.tile_type);
                    }
                }
                //check for farm updates
                Self::update_farms(temp_tile);
            }
        }
    }

    fn build_mode(
        &mut self,
        game: &mut game_manager::GameManager,
        events: &mut event_manager::EventManager,
        towers: &mut tower_manager::TowerManager,
        enemies: &mut enemy_manager::EnemyManager,
        gui_manager: &mut gui_manager::GUIManager,
        build_buttons: &mut button_manager::ButtonManager,
        temp_tile: &mut LevelTile,
        col_index: usize,
        row_index: usize,
    ) {
        let is_not_a_tower: bool = temp_tile.tile_type != constants::TILE_TYPE_ARCHER_BOTTOM && temp_tile.tile_type != constants::TILE_TYPE_FIREBALL_BOTTOM;
        match game.current_build {
            constants::CURRENT_BUILD_ARCHER_TOWER => {
                if !game.placed && temp_tile.tile_type == constants::TILE_TYPE_GRASS && 
                is_not_a_tower {
                    if game.preview_mode && game.mouse_button == sdl2::mouse::MouseButton::Left {
                        game.placed = true;
                        temp_tile.tile_type = constants::TILE_TYPE_ARCHER_BOTTOM;
                        temp_tile.tile_data = TileData::ArcherTowerBottom;
                        towers.place_tower(game, &temp_tile, (col_index, row_index));
                    } else if game.build_mode && build_buttons.button_vec[constants::CURRENT_BUILD_ARCHER_TOWER].outline_visible {
                        game.preview_mode = true;
                        gui_manager.preview.texture_path_bottom_left = constants::TEXTURE_PREVIEW_TOWER_ARCHER_BOTTOM.to_string();
                        gui_manager.preview.texture_path_top_left = constants::TEXTURE_PREVIEW_TOWER_ARCHER_TOP.to_string();
                        gui_manager.preview.texture_path_bottom_right = "".to_string();
                        gui_manager.preview.texture_path_top_right = "".to_string();
                        gui_manager.preview.index = (col_index, row_index);
                    } else {
                        game.preview_mode = false;
                    }
                } else if game.preview_mode {
                    game.preview_mode = false;
                }
            }
            constants::CURRENT_BUILD_FIREBALL_TOWER => {
                if !game.placed && temp_tile.tile_type == constants::TILE_TYPE_GRASS && is_not_a_tower {
                    if game.preview_mode && game.mouse_button == sdl2::mouse::MouseButton::Left {
                        game.placed = true;
                        temp_tile.tile_type = constants::TILE_TYPE_FIREBALL_BOTTOM;
                        temp_tile.tile_data = TileData::FireballTowerBottom;
                        towers.place_tower(game, &temp_tile, (col_index, row_index));
                    } else if game.build_mode && build_buttons.button_vec[constants::CURRENT_BUILD_FIREBALL_TOWER].outline_visible {
                        game.preview_mode = true;
                        gui_manager.preview.texture_path_bottom_left = constants::TEXTURE_PREVIEW_TOWER_FIREBALL_BOTTOM.to_string();
                        gui_manager.preview.texture_path_top_left = constants::TEXTURE_PREVIEW_TOWER_FIREBALL_TOP.to_string();
                        gui_manager.preview.texture_path_bottom_right = "".to_string();
                        gui_manager.preview.texture_path_top_right = "".to_string();
                        gui_manager.preview.index = (col_index, row_index);
                    } else {
                        game.preview_mode = false;
                    }
                } 
                else if game.preview_mode {
                    game.preview_mode = false;
                }

            }
            constants::CURRENT_BUILD_GOBLIN => {
                if temp_tile.tile_type == constants::TILE_TYPE_GRASS {
                    if /* !game.placed &&  */game.preview_mode && game.mouse_button == sdl2::mouse::MouseButton::Left {
                        game.placed = true;
                        enemies.place_enemy(game, temp_tile, TileData::Goblin, (col_index, row_index));
                    } else if game.build_mode && build_buttons.button_vec[constants::CURRENT_BUILD_GOBLIN].outline_visible {
                        game.preview_mode = true;
                        gui_manager.preview.texture_path_bottom_left = constants::TEXTURE_PREVIEW_GOBLIN_ENEMY.to_string();
                        gui_manager.preview.texture_path_bottom_right = "".to_string();
                        gui_manager.preview.texture_path_top_left = "".to_string();
                        gui_manager.preview.texture_path_top_right = "".to_string();
                        gui_manager.preview.index = (col_index, row_index);
                    } else {
                        game.preview_mode = false;
                    }
                } else if game.preview_mode {
                    game.preview_mode = false;
                }
            }
            constants::CURRENT_BUILD_WALL => {
                if !game.placed && !temp_tile.is_occupied && temp_tile.tile_type == constants::TILE_TYPE_GRASS && temp_tile.tile_type != constants::TILE_TYPE_GOBLIN {
                    if game.preview_mode && game.mouse_button == sdl2::mouse::MouseButton::Left {
                        game.placed = true;
                        temp_tile.tile_type = constants::TILE_TYPE_WALL;
                        temp_tile.texture_path = constants::TEXTURE_TILE_WALL.to_string();
                        temp_tile.tile_data = TileData::None;
                    } else if game.build_mode && build_buttons.button_vec[constants::CURRENT_BUILD_WALL].outline_visible {
                        game.preview_mode = true;
                        gui_manager.preview.texture_path_bottom_left = constants::TEXTURE_PREVIEW_COBBLESTONE.to_string();
                        gui_manager.preview.texture_path_bottom_right = "".to_string();
                        gui_manager.preview.texture_path_top_left = "".to_string();
                        gui_manager.preview.texture_path_top_right = "".to_string();
                        gui_manager.preview.index = (col_index, row_index);
                    } else {
                        game.preview_mode = false;
                    }
                } else if game.preview_mode {
                    game.preview_mode = false;
                }
            }
            constants::CURRENT_BUILD_BASE => {
                //add check if tiles in 2x2 area are occupied
                if !game.placed && !temp_tile.is_occupied && temp_tile.tile_type == constants::TILE_TYPE_GRASS && temp_tile.tile_type != constants::TILE_TYPE_BASE {
                    if game.preview_mode && game.mouse_button == sdl2::mouse::MouseButton::Left {
                        game.placed = true;
                        self.create_building(game, gui_manager, BuildingType::Base, temp_tile, col_index, row_index);
                    } else if game.build_mode && build_buttons.button_vec[constants::CURRENT_BUILD_BASE].outline_visible {
                        game.preview_mode = true;
                        gui_manager.preview.texture_path_bottom_left = constants::TEXTURE_PREVIEW_HOUSE_BOTTOM_LEFT.to_string();
                        gui_manager.preview.texture_path_bottom_right = constants::TEXTURE_PREVIEW_HOUSE_BOTTOM_RIGHT.to_string();
                        gui_manager.preview.texture_path_top_left = constants::TEXTURE_PREVIEW_HOUSE_TOP_LEFT.to_string();
                        gui_manager.preview.texture_path_top_right = constants::TEXTURE_PREVIEW_HOUSE_TOP_RIGHT.to_string();
                        gui_manager.preview.index = (col_index, row_index);
                    } else {
                        game.preview_mode = false;
                    }
                } else if game.preview_mode {
                    game.preview_mode = false;
                }
            }
            _ => {}
        }
        if game.placed {
            enemy_manager::EnemyManager::repath_all_enemies(enemies);
        }
    }

    fn seed_mode (
        &mut self,
        game: &mut game_manager::GameManager, 
        events: &mut event_manager::EventManager,
        player: &mut player_manager::PlayerManager,
        gui_manager: &mut gui_manager::GUIManager,
        seed_buttons: &mut button_manager::ButtonManager,
        projectiles: &mut projectile_manager::ProjectileManager,
        temp_tile: &mut LevelTile,
        col_index: usize,
        row_index: usize, 
    ) {
        match game.current_seed {
            seed if seed == constants::CURRENT_SEED_GUN => {
                if game.preview_mode && game.mouse_button == sdl2::mouse::MouseButton::Left {
                    let start = (player.rect.x() + player.x, player.rect.y() + player.y);
                    println!("spawned gun projectile");
                    projectiles.spawn_player_projectile(player, start, start, (events.mouse_point.x + game.cam_x, events.mouse_point.y + game.cam_y));
                    let texture = constants::TEXTURE_PREVIEW_GUN.to_string();
                    gui_manager.preview.texture_path_bottom_left = texture;
                    gui_manager.preview.texture_path_bottom_right = "".to_string();
                    gui_manager.preview.texture_path_top_left = "".to_string();
                    gui_manager.preview.texture_path_top_right = "".to_string();
                    gui_manager.preview.index = (col_index, row_index);
                }
                else if game.seed_mode && seed_buttons.button_vec[constants::CURRENT_SEED_GUN].outline_visible {
                    game.preview_mode = true;
                    let texture = constants::TEXTURE_PREVIEW_GUN.to_string();
                    gui_manager.preview.texture_path_bottom_left = texture;
                    gui_manager.preview.texture_path_bottom_right = "".to_string();
                    gui_manager.preview.texture_path_top_left = "".to_string();
                    gui_manager.preview.texture_path_top_right = "".to_string();
                    gui_manager.preview.index = (col_index, row_index);

                }
            }
            seed if seed == constants::CURRENT_SEED_SHOVEL => {
                if game.preview_mode && game.mouse_button == sdl2::mouse::MouseButton::Left {
                    temp_tile.tile_type = temp_tile.original_type;
                    match temp_tile.tile_type {
                        constants::TILE_TYPE_GRASS => temp_tile.texture_path = constants::TEXTURE_TILE_GRASS.to_string(),
                        _ => {},
                    }
                    temp_tile.tile_data = TileData::None;
                    let texture = constants::TEXTURE_BUTTON_SHOVEL.to_string();
                    gui_manager.preview.texture_path_bottom_left = texture;
                    gui_manager.preview.texture_path_bottom_right = "".to_string();
                    gui_manager.preview.texture_path_top_left = "".to_string();
                    gui_manager.preview.texture_path_top_right = "".to_string();
                    gui_manager.preview.index = (col_index, row_index);

                }
                else if game.seed_mode && seed_buttons.button_vec[constants::CURRENT_SEED_SHOVEL].outline_visible {
                    game.preview_mode = true;
                    let texture = constants::TEXTURE_BUTTON_SHOVEL.to_string();
                    gui_manager.preview.texture_path_bottom_left = texture;
                    gui_manager.preview.texture_path_bottom_right = "".to_string();
                    gui_manager.preview.texture_path_top_left = "".to_string();
                    gui_manager.preview.texture_path_top_right = "".to_string();
                    gui_manager.preview.index = (col_index, row_index);
                }
                else {
                    game.preview_mode = false;
                }
            }
            seed if seed == constants::CURRENT_SEED_HO => {
                if temp_tile.tile_type == constants::TILE_TYPE_GRASS || temp_tile.tile_type == constants::TILE_TYPE_FIELD_HARVESTABLE || temp_tile.tile_type == constants::TILE_TYPE_FIELD_GROWING || temp_tile.tile_type == constants::TILE_TYPE_FIELD_EMPTY {
                    if game.preview_mode && game.mouse_button == sdl2::mouse::MouseButton::Left {
                        game.placed = true;
                        if temp_tile.tile_type == constants::TILE_TYPE_FIELD_HARVESTABLE {
                            match temp_tile.tile_data {
                                TileData::Carrots => game.carrot_amount += 1,
                                TileData::Tomatoes => game.tomato_amount += 1,
                                _ => {},
                            }
                        }                    
                        temp_tile.tile_type = constants::TILE_TYPE_FIELD_EMPTY;
                        temp_tile.texture_path = constants::TEXTURE_FIELD_EMPTY.to_string();
                        temp_tile.tile_data = TileData::None;
                        gui_manager.preview.texture_path_bottom_left = constants::TEXTURE_BUTTON_HO.to_string();
                        gui_manager.preview.texture_path_bottom_right = "".to_string();
                        gui_manager.preview.texture_path_top_left = "".to_string();
                        gui_manager.preview.texture_path_top_right = "".to_string();                        
                        gui_manager.preview.index = (col_index, row_index);
                        gui_manager.preview.index = (col_index, row_index);

                    }
                    else if game.seed_mode && seed_buttons.button_vec[constants::CURRENT_SEED_HO].outline_visible {
                        game.preview_mode = true;
                        gui_manager.preview.texture_path_bottom_left = constants::TEXTURE_BUTTON_HO.to_string();
                        gui_manager.preview.texture_path_bottom_right = "".to_string();
                        gui_manager.preview.texture_path_top_left = "".to_string();
                        gui_manager.preview.texture_path_top_right = "".to_string();                        
                        gui_manager.preview.index = (col_index, row_index);
                    }
                    else {
                        game.preview_mode = false;
                    }
                }
                else if game.preview_mode {
                    game.preview_mode = false;
                }
            }
            seed if seed == constants::CURRENT_SEED_CARROT => {
                if game.mouse_button == sdl2::mouse::MouseButton::Left && temp_tile.tile_type != constants::TILE_TYPE_FIELD_EMPTY {
                    gui_manager.create_unique_message("you need to plant those on a field...".to_string(), 128);
                }
                if temp_tile.tile_type == constants::TILE_TYPE_FIELD_EMPTY {
                    if game.preview_mode && game.mouse_button == sdl2::mouse::MouseButton::Left {
                        game.placed = true;
                        temp_tile.tile_type = constants::TILE_TYPE_FIELD_EMPTY;
                        temp_tile.texture_path = constants::TEXTURE_FIELD_SEEDS.to_string();
                        temp_tile.tile_data = TileData::Carrots;
                        gui_manager.preview.texture_path_bottom_left = constants::TEXTURE_FIELD_CARROT.to_string();
                        gui_manager.preview.texture_path_bottom_right = "".to_string();
                        gui_manager.preview.texture_path_top_left = "".to_string();
                        gui_manager.preview.texture_path_top_right = "".to_string();                        
                        gui_manager.preview.index = (col_index, row_index);

                    }
                    else if game.seed_mode && seed_buttons.button_vec[constants::CURRENT_SEED_CARROT].outline_visible {
                        game.preview_mode = true;
                        gui_manager.preview.texture_path_bottom_left = constants::TEXTURE_FIELD_CARROT.to_string();
                        gui_manager.preview.texture_path_bottom_right = "".to_string();
                        gui_manager.preview.texture_path_top_left = "".to_string();
                        gui_manager.preview.texture_path_top_right = "".to_string();                        
                        gui_manager.preview.index = (col_index, row_index);
                    }
                    else {
                        game.preview_mode = false;
                    }
                }
                else if game.preview_mode {
                    game.preview_mode = false;
                }
            }
            seed if seed == constants::CURRENT_SEED_TOMATO => {
                if game.mouse_button == sdl2::mouse::MouseButton::Left && temp_tile.tile_type != constants::TILE_TYPE_FIELD_EMPTY {
                    gui_manager.create_unique_message("you need to plant those on a field...".to_string(), 128);
                }
                if temp_tile.tile_type == constants::TILE_TYPE_FIELD_EMPTY {
                    if game.preview_mode && game.mouse_button == sdl2::mouse::MouseButton::Left {
                        game.placed = true;
                        temp_tile.tile_type = constants::TILE_TYPE_FIELD_EMPTY;
                        temp_tile.texture_path = constants::TEXTURE_FIELD_SEEDS.to_string();
                        temp_tile.tile_data = TileData::Tomatoes;
                        gui_manager.preview.texture_path_bottom_left = constants::TEXTURE_FIELD_TOMATO.to_string();
                        gui_manager.preview.texture_path_bottom_right = "".to_string();
                        gui_manager.preview.texture_path_top_left = "".to_string();
                        gui_manager.preview.texture_path_top_right = "".to_string();                        
                        gui_manager.preview.index = (col_index, row_index);

                    }
                    else if game.seed_mode && seed_buttons.button_vec[constants::CURRENT_SEED_TOMATO].outline_visible {
                        game.preview_mode = true;
                        gui_manager.preview.texture_path_bottom_left = constants::TEXTURE_FIELD_TOMATO.to_string();
                        gui_manager.preview.texture_path_bottom_right = "".to_string();
                        gui_manager.preview.texture_path_top_left = "".to_string();
                        gui_manager.preview.texture_path_top_right = "".to_string();                        
                        gui_manager.preview.index = (col_index, row_index);
                    }
                    else {
                        game.preview_mode = false;
                    }
                }
                else if game.preview_mode {
                    game.preview_mode = false;
                }
            }
            _ => {}
        }
    }
    fn update_farms (temp_tile: &mut LevelTile) {
        //INCREASE ALL FARM STATE
        match temp_tile.tile_data {
            TileData::Carrots | TileData::Tomatoes => {
                match temp_tile.tile_type {
                    constants::TILE_TYPE_FIELD_EMPTY | constants::TILE_TYPE_FIELD_GROWING | constants::TILE_TYPE_FIELD_HARVESTABLE => temp_tile.state += 1,
                    _ => {},
                }
            }
            _ => {}
        }

        //CHANGE TO GROWING STATE
        if temp_tile.tile_type == constants::TILE_TYPE_FIELD_EMPTY && temp_tile.state == constants::CROP_TIME {
            match temp_tile.tile_data {
                TileData::Carrots | TileData::Tomatoes => {
                    temp_tile.tile_type = constants::TILE_TYPE_FIELD_GROWING;
                    temp_tile.texture_path = constants::TEXTURE_FIELD_GROWING.to_string();
                    temp_tile.state = 0;
                }
                _ => {
                    temp_tile.tile_type = constants::TILE_TYPE_FIELD_EMPTY;
                    temp_tile.texture_path = constants::TEXTURE_DEFAULT.to_string();
                    temp_tile.state = 0;
                }
            }
        }

        //CHANGE TO HARVEST FARM STATE
        if temp_tile.tile_type == constants::TILE_TYPE_FIELD_GROWING && temp_tile.state == constants::CROP_TIME {
            match temp_tile.tile_data {
                TileData::Carrots => {
                    temp_tile.tile_type = constants::TILE_TYPE_FIELD_HARVESTABLE;
                    temp_tile.texture_path = constants::TEXTURE_FIELD_CARROT.to_string();
                    temp_tile.state = 0;
                }
                TileData::Tomatoes => {
                    temp_tile.tile_type = constants::TILE_TYPE_FIELD_HARVESTABLE;
                    temp_tile.texture_path = constants::TEXTURE_FIELD_TOMATO.to_string();
                    temp_tile.state = 0;
                }
                _ => {
                    temp_tile.tile_type = constants::TILE_TYPE_GRASS;
                    temp_tile.texture_path = constants::TEXTURE_DEFAULT.to_string();
                    temp_tile.state = 0;
                }
            }
        }
    }


}
