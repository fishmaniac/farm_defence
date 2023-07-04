use sdl2::mouse::MouseButton;
use sdl2::rect::Rect;
use sdl2::video::WindowContext;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;

use crate::{constants, projectile_manager, gui_manager, game_manager};
use crate::game_manager::GameManager;
use crate::texture_manager::TextureManager;
use crate::player_manager::PlayerManager;
use crate::tower_manager;
use crate::enemy_manager::EnemyManager;
use crate::enemy_manager;
use crate::button_manager;

pub enum TileData {
    Carrots,
    Tomatoes,
    ArcherTowerBottom,
    ArcherTowerTop,
    Goblin,
    None,
}

pub struct LevelManager {
    pub level_vec: Vec<Vec<LevelTile>>,
}

pub struct LevelTile {
    pub tile_type: char,
    pub prev_type: char,
    pub texture_path: String,
    pub rect: Rect,
    pub state: u16,
    pub tile_data: TileData,
}

impl LevelManager {
    pub fn new() -> LevelManager {
        let level = LevelManager {
            level_vec: Vec::new(),
        };
        level
    }

    pub fn create_level(&mut self) {
        for _ in 0..constants::MAX_HEIGHT {
            let mut row = Vec::new();
            for _ in 0..constants::MAX_WIDTH {
                let rect = Rect::new(0, 0, constants::TILE_SIZE, constants::TILE_SIZE);

                row.push(LevelTile { 
                    tile_type: constants::TILE_TYPE_GRASS,
                    prev_type: constants::TILE_TYPE_GRASS,
                    texture_path: constants::TEXTURE_TILE_EMPTY.to_string(),
                    rect,
                    state: 0,
                    tile_data: TileData::None,
                });
            }
            self.level_vec.push(row);
        }
    }

    pub fn read_file(
        &mut self, 
        filename: &str) 
    -> Result<(), std::io::Error> {
        println!("Reading from dir: {:?}", env::current_dir()?);
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        let mut temp_vec: Vec<Vec<LevelTile>> = Vec::new();
        let rect = Rect::new(0, 0, constants::TILE_SIZE, constants::TILE_SIZE);

        for line in reader.lines() {
            let line = line?;
            let mut row_vec: Vec<LevelTile> = Vec::new();
            for ch in line.chars() {
                match ch {
                    constants::TILE_TYPE_GRASS => {
                        let tile = LevelTile {
                            tile_type: ch,
                            prev_type: ch,
                            texture_path: constants::TEXTURE_TILE_EMPTY.to_string(),
                            rect,
                            state: 0,
                            tile_data: TileData::None,
                        };
                        row_vec.push(tile);
                    }
                    constants::TILE_TYPE_WALL => {
                        let tile = LevelTile {
                            tile_type: ch,
                            prev_type: ch,
                            texture_path: constants::TEXTURE_TILE_WALL.to_string(),
                            rect,
                            state: 0,
                            tile_data: TileData::None,
                        };
                        row_vec.push(tile);
                    }
                    constants::TILE_TYPE_FLOOR => {
                        let tile = LevelTile {
                            tile_type: ch,
                            prev_type: ch,
                            texture_path: constants::TEXTURE_TILE_FLOOR.to_string(),
                            rect,
                            state: 0,
                            tile_data: TileData::None,
                        };
                        row_vec.push(tile);
                    }
                    constants::TILE_TYPE_FIELD_EMPTY => {
                        let tile = LevelTile {
                            tile_type: ch,
                            prev_type: ch,
                            texture_path: constants::TEXTURE_FIELD_EMPTY.to_string(),
                            rect,
                            state: 0,
                            tile_data: TileData::None,
                        };
                        row_vec.push(tile);
                    }
                    _ => {
                        let tile = LevelTile {
                            tile_type: ch,
                            prev_type: ch,
                            texture_path: constants::TEXTURE_DEFAULT.to_string(),
                            rect,
                            state: 0,
                            tile_data: TileData::None,
                        };
                        row_vec.push(tile);
                    }
                }
            }
            temp_vec.push(row_vec);
        }
        self.level_vec = temp_vec;
        Ok(())
    }

    pub fn render_level(
        game: &mut GameManager, 
        player: &mut PlayerManager, 
        tex_man: &mut TextureManager<WindowContext>,
        temp_tile: &mut self::LevelTile, 
        col_index: usize,
        row_index: usize,
    ) -> Result<(), String> {
        temp_tile.rect.set_x((constants::TILE_SIZE as i32 * col_index as i32) - game.cam_x);
        temp_tile.rect.set_y((constants::TILE_SIZE as i32 * row_index as i32) - game.cam_y);
        let texture = tex_man.load(&temp_tile.texture_path)?;
        game.canvas.copy_ex(
            &texture, // Texture object
            None,      // source rect
            temp_tile.rect,     // destination rect
            0.0,      // angle (degrees)
            None,   // center
            false,    // flip horizontal
            false,     // flip vertical
        )?;
        check_collisions(player, temp_tile);

        fn check_collisions(player: &mut PlayerManager, temp_tile: &mut LevelTile) {
            if Rect::has_intersection(&player.rect, temp_tile.rect){
                if temp_tile.tile_type == constants::TILE_TYPE_WALL {
                    player.colliding = true;
                }
                else {
                    player.colliding = false;
                }
            }
        }
        Ok(())
    }

    pub fn update_buildings( 
        game: &mut GameManager, 
        towers: &mut tower_manager::TowerManager, 
        enemies: &mut EnemyManager, 
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

        //PRETTY SURE HOVERING ALL BUTTONS = BUG....i think i fixed it?
        //CHECK FOR CLICK ON BUTTON
        if Rect::contains_point(&temp_tile.rect, game.mouse_point) && game.mouse_button == MouseButton::Left {
            if game.build_mode && !button_manager::ButtonManager::check_clicked(build_buttons) {
                Self::build_mode(game, towers, enemies, temp_tile, row_index, col_index);
            }
            if game.seed_mode && !button_manager::ButtonManager::check_clicked(seed_buttons) && temp_tile.tile_type == constants::TILE_TYPE_FIELD_EMPTY {
                Self::seed_mode(game,temp_tile);
            }
        }
        //CHECK FOR FARM UPDATES
        Self::update_farms(temp_tile);
    }

    fn build_mode (
        game: &mut GameManager, 
        towers: &mut tower_manager::TowerManager, 
        enemies: &mut EnemyManager,
        temp_tile: &mut LevelTile,
        row_index: usize, 
        col_index: usize, 
    ) {
        match game.current_build {
            //BUILD MODE HO
            build if build == constants::CURRENT_BUILD_HO as usize => {
                if temp_tile.prev_type == constants::TILE_TYPE_GRASS {
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
                }
            }
            //BUILD MODE ARCHER TOWER
            build if build == constants::CURRENT_BUILD_ARCHER_TOWER as usize => {
                if temp_tile.prev_type == constants::TILE_TYPE_GRASS && temp_tile.tile_type != constants::TILE_TYPE_ARCHER_BOTTOM {
                    temp_tile.tile_type = constants::TILE_TYPE_ARCHER_BOTTOM;
                    temp_tile.tile_data = TileData::ArcherTowerBottom;
                    println!("\nPLACING TOWER: {:?},{:?}\n", col_index, row_index);
                    towers.place_tower(game, &temp_tile, (col_index, row_index));
                }
            }
            build if build == constants::CURRENT_BUILD_GOBLIN_TEST as usize => {
                if temp_tile.prev_type == constants::TILE_TYPE_GRASS && temp_tile.tile_type != constants::TILE_TYPE_GOBLIN {
                    temp_tile.tile_type = constants::TILE_TYPE_GOBLIN;
                    temp_tile.tile_data = TileData::Goblin;
                    enemies.place_enemy(temp_tile, (col_index, row_index));
                }
            }
            _ => {}
        }

    }
    fn seed_mode (game: &mut GameManager, temp_tile: &mut LevelTile) {
        match game.current_seed {
            seed if seed == constants::CURRENT_SEED_CARROT as usize => {
                temp_tile.tile_type = constants::TILE_TYPE_FIELD_EMPTY;
                temp_tile.texture_path = constants::TEXTURE_FIELD_SEEDS.to_string();
                temp_tile.tile_data = TileData::Carrots;
            }
            seed if seed == constants::CURRENT_SEED_TOMATO as usize => {
                temp_tile.tile_type = constants::TILE_TYPE_FIELD_EMPTY;
                temp_tile.texture_path = constants::TEXTURE_FIELD_SEEDS.to_string();
                temp_tile.tile_data = TileData::Tomatoes;
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
    pub fn check_attacks (
        game: &mut game_manager::GameManager,
        enemies: &mut enemy_manager::EnemyManager, 
        towers: &mut tower_manager::TowerManager,
        projectiles: &mut projectile_manager::ProjectileManager,
        health_bars: &mut gui_manager::GUIManager,
    ) {
        for tower in &mut towers.tower_vec {
            let tower_pos_pixel = (constants::TILE_SIZE as i32 * tower.top_index.0 as i32, constants::TILE_SIZE as i32 * tower.top_index.1 as i32);
            for enemy in &mut enemies.enemy_vec {
                let enemy_pos_pixel = (constants::TILE_SIZE as i32 * enemy.index.0 as i32, constants::TILE_SIZE as i32 * enemy.index.1 as i32);
                let tower_can_attack: bool = tower_manager::TowerManager::is_within_area((tower.bottom_index.0 as i32, tower.bottom_index.1 as i32), (enemy.index.0 as i32, enemy.index.1 as i32), tower.attack_radius) && game.frame_time % tower.attack_speed as u32 == 0;
                let enemy_can_attack: bool = tower_manager::TowerManager::is_within_area((tower.bottom_index.0 as i32, tower.bottom_index.1 as i32), (enemy.index.0 as i32, enemy.index.1 as i32), enemy.attack_radius as i32) && game.frame_time % enemy.attack_speed as u32 == 0;

                //TOWER ATTACK
                if enemy.health != 0 {
                    if tower_can_attack {
                        if tower_pos_pixel != enemy_pos_pixel && !tower.is_attacking {
                            projectiles.spawn_projectile(tower, tower_pos_pixel, tower_pos_pixel, enemy_pos_pixel);
                            tower.is_attacking = true;
                        }
                        for projectile in &mut projectiles.projectile_vec {
                            let projectile_hit: bool = tower_manager::TowerManager::is_within_area(projectile.position, enemy_pos_pixel, projectile.radius as i32);

                            if projectile_hit && enemy.health != 0 {
                                enemy.health -= projectile.damage as u16;
                                projectile.hit_target = true;
                            }

                        }
                    }
                    else {
                        tower.is_attacking = false;
                    }
                }
                //ENEMY ATTACK
                if tower.health != 0 {
                    if enemy_can_attack {
                        tower.health -= enemy.attack_damage as u16;
                        enemy.found_target = true;
                    }
                }

                if enemy.health < enemy.max_health {
                    health_bars.render_health_bar_enemy(game, enemy);
                }
            }
            if tower.health < tower.max_health {
                health_bars.render_health_bar_tower(game, tower);
            }
        }
    }
    pub fn delete_all_dead (
        &mut self,
        game: &mut game_manager::GameManager,
        enemies: &mut enemy_manager::EnemyManager, 
        towers: &mut tower_manager::TowerManager,
        projectiles: &mut projectile_manager::ProjectileManager,
    ) {
        /* enemies.enemy_vec.retain(|enemy| enemy.health != 0); */
        for enemy_index in (0..enemies.enemy_vec.len()).rev() {
            let enemy = &mut enemies.enemy_vec[enemy_index];

            if enemy.health == 0 {
                self.level_vec[enemy.index.0][enemy.index.1].tile_type = self.level_vec[enemy.index.0][enemy.index.1].prev_type;
                enemies.enemy_vec.remove(enemy_index);

            }
        }
        for tower_index in (0..towers.tower_vec.len()).rev() {
            let tower = &mut towers.tower_vec[tower_index];

            if tower.health == 0 {
                for target_index in (0..game.target_vec.len()).rev() {
                    let target = game.target_vec[target_index];

                    if target == tower.bottom_index {
                        game.target_vec.remove(target_index);

                    }
                }
                for enemy_index in (0..enemies.enemy_vec.len()).rev() {
                    let enemy = &mut enemies.enemy_vec[enemy_index];
                    let within_range = tower_manager::TowerManager::is_within_area((tower.bottom_index.0 as i32, tower.bottom_index.1 as i32), (enemy.index.0 as i32, enemy.index.1 as i32), enemy.attack_radius as i32);

                    if within_range {
                        enemies.enemy_vec[enemy_index].found_target = false;
                    }
                }
                towers.tower_vec.remove(tower_index);
            }
        }

        for projectile_index in (0..projectiles.projectile_vec.len()).rev() {
            let projectile = &mut projectiles.projectile_vec[projectile_index];
            let do_despawn_projectile: bool = (projectile.hit_target && projectile.time > constants::PROJECTILE_HIT_DESPAWN_DURATION) || projectile.time > constants::PROJECTILE_DESPAWN_DURATION;

            if do_despawn_projectile {
                projectiles.projectile_vec.remove(projectile_index);
            }


        }
    }
}
