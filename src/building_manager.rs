use crate::button_manager;
use crate::constants;
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

#[derive(PartialEq)]
pub enum BuildingType {
    Base,
    None,
}

pub struct Building {
    rect: sdl2::rect::Rect,
    texture_path: String,
    building_type: BuildingType,
    pub grid_index: (i32, i32),
    pub pixel_index: (i32, i32),
    pub max_health: u16,
    pub health: u16,
}

pub struct BuildingManager {
    pub building_vec: Vec<Building>,
}

impl BuildingManager {
    pub fn new() -> BuildingManager {
        let buildings = BuildingManager {
            building_vec: Vec::new(),
        };
        buildings
    }
    pub fn create_building (
        &mut self, 
        game: &mut game_manager::GameManager,
        building_type: BuildingType,
        temp_tile: &mut LevelTile,
        col_index: usize, 
        row_index: usize, 
    ) {
        match building_type {
            BuildingType::Base => {
                temp_tile.tile_type = constants::TILE_TYPE_BASE;
                let building = self::Building {
                    rect: sdl2::rect::Rect::new(temp_tile.rect.x(), temp_tile.rect.y(), constants::TILE_SIZE, constants::TILE_SIZE),
                    texture_path: constants::TEXTURE_BUILDING_HOUSE.to_string(),
                    building_type: BuildingType::Base,
                    grid_index: (col_index as i32, row_index as i32),
                    pixel_index: (col_index as i32 * constants::TILE_SIZE as i32, row_index as i32 * constants::TILE_SIZE as i32),
                    max_health: constants::BUILDING_BASE_HEALTH,
                    health: constants::BUILDING_BASE_HEALTH,
                };

                if !self.building_vec.iter().any(|building| building.building_type == BuildingType::Base) {
                    println!("Building created, vec len: {}, index: {:?}", self.building_vec.len(), (col_index, row_index));
                    self.building_vec.push(building);
                    game.base_location = Some((col_index, row_index));
                    if let Some(base_location) = game.base_location {
                        game.target_vec.push(base_location);
                    }
                }
                else {
                    println!("Base already created");
                }
            },
            BuildingType::None => {
                let building = self::Building {
                    rect: sdl2::rect::Rect::new(temp_tile.rect.x(), temp_tile.rect.y(), constants::TILE_SIZE, constants::TILE_SIZE),
                    texture_path: constants::TEXTURE_DEFAULT.to_string(),
                    building_type: BuildingType::None,
                    grid_index: (col_index as i32, row_index as i32), 
                    pixel_index: (col_index as i32 * constants::TILE_SIZE as i32, row_index as i32 * constants::TILE_SIZE as i32),
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
    ) -> Result<(), String> {
        for building in &mut self.building_vec {
            building.rect.set_x(building.pixel_index.0 - game.cam_x);
            building.rect.set_y(building.pixel_index.1 - game.cam_y);
            let texture = tex_man.load(&building.texture_path)?;

            game.canvas.copy_ex(
                &texture, // Texture object
                None,      // source rect
                building.rect,     // destination rect
                0.0,      // angle (degrees)
                None,   // center
                false,    // flip horizontal
                false,     // flip vertical
            )?;
        }
        Ok(())
    }

    pub fn update_buildings(
        &mut self,
        game: &mut game_manager::GameManager, 
        towers: &mut tower_manager::TowerManager, 
        enemies: &mut enemy_manager::EnemyManager, 
        gui_manager: &mut gui_manager::GUIManager,
        seed_buttons: &mut button_manager::ButtonManager, 
        build_buttons: &mut button_manager::ButtonManager,
        temp_tile: &mut LevelTile,
        col_index: usize,
        row_index: usize,
    ) {
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

        if !game.hovering_button && sdl2::rect::Rect::contains_point(&temp_tile.rect, game.mouse_point){
            if game.build_mode /* && mouse_left */ {
                self.build_mode(game, towers, enemies, gui_manager, build_buttons, temp_tile, col_index, row_index);
            }

            if game.seed_mode {
                Self::seed_mode(game, gui_manager, seed_buttons, temp_tile, col_index, row_index);
            }

        }
        //CHECK FOR FARM UPDATES
        Self::update_farms(temp_tile);
    }

    fn build_mode (
        &mut self,
        game: &mut game_manager::GameManager, 
        towers: &mut tower_manager::TowerManager, 
        enemies: &mut enemy_manager::EnemyManager,
        gui_manager: &mut gui_manager::GUIManager,
        build_buttons: &mut button_manager::ButtonManager,
        temp_tile: &mut LevelTile,
        col_index: usize,
        row_index: usize, 
    ) {
        match game.current_build {
            build if build == constants::CURRENT_BUILD_ARCHER_TOWER => {
                if !game.placed && temp_tile.tile_type == constants::TILE_TYPE_GRASS && temp_tile.tile_type != constants::TILE_TYPE_ARCHER_BOTTOM {
                    if game.preview_mode && game.mouse_button == sdl2::mouse::MouseButton::Left {
                        game.placed = true;
                        temp_tile.tile_type = constants::TILE_TYPE_ARCHER_BOTTOM;
                        temp_tile.tile_data = TileData::ArcherTowerBottom;
                        towers.place_tower(game, &temp_tile, (col_index, row_index));
                    }
                    else if game.build_mode && build_buttons.button_vec[constants::CURRENT_BUILD_ARCHER_TOWER].outline_visible {
                        game.preview_mode = true;
                        let texture_bottom_left = constants::TEXTURE_PREVIEW_TOWER_ARCHER_BOTTOM.to_string();
                        gui_manager.preview.texture_path_bottom_left = texture_bottom_left;
                        let texture_top_left = constants::TEXTURE_PREVIEW_TOWER_ARCHER_TOP.to_string();
                        gui_manager.preview.texture_path_top_left = texture_top_left;
                        let texture = "".to_string();
                        gui_manager.preview.texture_path_bottom_right = texture;
                        let texture = "".to_string();
                        gui_manager.preview.texture_path_top_right = texture;

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
            build if build == constants::CURRENT_BUILD_GOBLIN => {
                if /* !game.placed &&  */temp_tile.tile_type == constants::TILE_TYPE_GRASS && temp_tile.tile_type != constants::TILE_TYPE_GOBLIN {
                    if game.preview_mode && game.mouse_button == sdl2::mouse::MouseButton::Left {
                        game.placed = true;
                        temp_tile.tile_type = constants::TILE_TYPE_GOBLIN;
                        temp_tile.tile_data = TileData::Goblin;
                        enemies.place_enemy(temp_tile, (col_index, row_index));
                    }
                    else if game.build_mode && build_buttons.button_vec[constants::CURRENT_BUILD_GOBLIN].outline_visible {
                        game.preview_mode = true;
                        let texture = constants::TEXTURE_PREVIEW_GOBLIN_ENEMY.to_string();
                        gui_manager.preview.texture_path_bottom_left = texture;
                        let texture = "".to_string();
                        gui_manager.preview.texture_path_bottom_right = texture;
                        let texture = "".to_string();
                        gui_manager.preview.texture_path_top_left = texture;
                        let texture = "".to_string();
                        gui_manager.preview.texture_path_top_right = texture;
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
            build if build == constants::CURRENT_BUILD_WALL => {
                if !game.placed && !temp_tile.is_occupied && temp_tile.tile_type == constants::TILE_TYPE_GRASS && temp_tile.tile_type != constants::TILE_TYPE_GOBLIN {
                    if game.preview_mode && game.mouse_button == sdl2::mouse::MouseButton::Left {
                        game.placed = true;
                        temp_tile.tile_type = constants::TILE_TYPE_WALL;
                        temp_tile.texture_path = constants::TEXTURE_TILE_WALL.to_string();
                        temp_tile.tile_data = TileData::None;
                    }
                    else if game.build_mode && build_buttons.button_vec[constants::CURRENT_BUILD_WALL].outline_visible {
                        game.preview_mode = true;
                        let texture = constants::TEXTURE_PREVIEW_COBBLESTONE.to_string();
                        gui_manager.preview.texture_path_bottom_left = texture;
                        let texture = "".to_string();
                        gui_manager.preview.texture_path_bottom_right = texture;
                        let texture = "".to_string();
                        gui_manager.preview.texture_path_top_left = texture;
                        let texture = "".to_string();
                        gui_manager.preview.texture_path_top_right = texture;
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
            build if build == constants::CURRENT_BUILD_BASE => {
                if !game.placed && !temp_tile.is_occupied && temp_tile.tile_type == constants::TILE_TYPE_GRASS && temp_tile.tile_type != constants::TILE_TYPE_BASE {
                    if game.preview_mode && game.mouse_button == sdl2::mouse::MouseButton::Left {
                        game.placed = true;
                        self.create_building(game, BuildingType::Base, temp_tile, col_index, row_index)
                    }
                    else if game.build_mode && build_buttons.button_vec[constants::CURRENT_BUILD_BASE].outline_visible {
                        game.preview_mode = true;
                        let texture = constants::TEXTURE_PREVIEW_HOUSE.to_string();
                        gui_manager.preview.texture_path_bottom_left = texture;
                        let texture = constants::TEXTURE_PREVIEW_HOUSE.to_string();
                        gui_manager.preview.texture_path_bottom_right = texture;
                        let texture = constants::TEXTURE_PREVIEW_HOUSE.to_string();
                        gui_manager.preview.texture_path_top_left = texture;
                        let texture = constants::TEXTURE_PREVIEW_HOUSE.to_string();
                        gui_manager.preview.texture_path_top_right = texture;
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

    fn seed_mode (
        game: &mut game_manager::GameManager, 
        gui_manager: &mut gui_manager::GUIManager,
        seed_buttons: &mut button_manager::ButtonManager,
        temp_tile: &mut LevelTile,
        col_index: usize,
        row_index: usize, 
    ) {
        match game.current_seed {
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
                    gui_manager.preview.index = (col_index, row_index);

                }
                else if game.seed_mode && seed_buttons.button_vec[constants::CURRENT_SEED_SHOVEL].outline_visible {
                    game.preview_mode = true;
                    let texture = constants::TEXTURE_BUTTON_SHOVEL.to_string();
                    gui_manager.preview.texture_path_bottom_left = texture;
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
                        let texture = constants::TEXTURE_BUTTON_HO.to_string();
                        gui_manager.preview.texture_path_bottom_left = texture;
                        gui_manager.preview.index = (col_index, row_index);

                    }
                    else if game.seed_mode && seed_buttons.button_vec[constants::CURRENT_SEED_HO].outline_visible {
                        game.preview_mode = true;
                        let texture = constants::TEXTURE_BUTTON_HO.to_string();
                        gui_manager.preview.texture_path_bottom_left = texture;
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
                if temp_tile.tile_type == constants::TILE_TYPE_FIELD_EMPTY {
                    if game.preview_mode && game.mouse_button == sdl2::mouse::MouseButton::Left {
                        game.placed = true;
                        temp_tile.tile_type = constants::TILE_TYPE_FIELD_EMPTY;
                        temp_tile.texture_path = constants::TEXTURE_FIELD_SEEDS.to_string();
                        temp_tile.tile_data = TileData::Carrots;
                        let texture = constants::TEXTURE_FIELD_CARROT.to_string();
                        gui_manager.preview.texture_path_bottom_left = texture;
                        gui_manager.preview.index = (col_index, row_index);

                    }
                    else if game.seed_mode && seed_buttons.button_vec[constants::CURRENT_SEED_CARROT].outline_visible {
                        game.preview_mode = true;
                        let texture = constants::TEXTURE_FIELD_CARROT.to_string();
                        gui_manager.preview.texture_path_bottom_left = texture;
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
                if temp_tile.tile_type == constants::TILE_TYPE_FIELD_EMPTY {
                    if game.preview_mode && game.mouse_button == sdl2::mouse::MouseButton::Left {
                        game.placed = true;
                        temp_tile.tile_type = constants::TILE_TYPE_FIELD_EMPTY;
                        temp_tile.texture_path = constants::TEXTURE_FIELD_SEEDS.to_string();
                        temp_tile.tile_data = TileData::Tomatoes;
                        let texture = constants::TEXTURE_FIELD_TOMATO.to_string();
                        gui_manager.preview.texture_path_bottom_left = texture;
                        gui_manager.preview.index = (col_index, row_index);

                    }
                    else if game.seed_mode && seed_buttons.button_vec[constants::CURRENT_SEED_TOMATO].outline_visible {
                        game.preview_mode = true;
                        let texture = constants::TEXTURE_FIELD_TOMATO.to_string();
                        gui_manager.preview.texture_path_bottom_left = texture;
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
