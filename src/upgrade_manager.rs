use crate::building_manager;
use crate::constants;
use crate::enemy_manager;
use crate::gui_manager;
use crate::level_manager;
use crate::tower_manager;
use crate::game_manager;

pub enum BuildingType {
    Archer,
    Fireball,
    Base,
    None,
}

pub enum UpgradeType {
    Damage,
    Health,
    None,
}

pub struct Upgrade <'a> {
    texture_surface: sdl2::surface::Surface<'a>,
    upgrade_rect: sdl2::rect::Rect,
}

pub struct UpgradeMenu <'a> {
    background_rect: sdl2::rect::Rect,
    upgrades_vec: Vec<Upgrade <'a>>,
    menu_active: bool,
    grid_index: (usize, usize),
    building: BuildingType,
    upgrade: UpgradeType,
    damage: u8,
    health: u16,
    current_level: u8,
}

pub struct UpgradeManager <'a> {
    pub upgrade_vec: Vec<UpgradeMenu<'a>>,
    texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    font: &'a sdl2::ttf::Font<'a, 'a>,
}

impl<'a> UpgradeManager <'a> {
    pub fn new(game:&mut game_manager::GameManager, font: &'a sdl2::ttf::Font<'a, 'a>) -> UpgradeManager<'a> {
        let upgrades = UpgradeManager {
            texture_creator: game.canvas.texture_creator(),
            upgrade_vec: Vec::new(),
            font,
        };
        upgrades
    }
    pub fn check_upgrade(&mut self, game: &mut game_manager::GameManager, enemies: &mut enemy_manager::EnemyManager, towers: &mut tower_manager::TowerManager, buildings: &mut building_manager::BuildingManager, gui_manager: &mut gui_manager::GUIManager, temp_tile: &mut level_manager::LevelTile, grid_index: (usize, usize)) -> Result<(), String> {
        let mut upgrade_exists: bool = false;
        for upgrade in &mut self.upgrade_vec {
            if upgrade.grid_index == grid_index {
                upgrade.menu_active = true;
                upgrade_exists = true;
                break
            }
        }

        if !upgrade_exists {
            match temp_tile.tile_type {
                constants::TILE_TYPE_ARCHER_BOTTOM | constants::TILE_TYPE_FIREBALL_BOTTOM => {
                    for tower in &mut towers.tower_vec {
                        if tower.bottom_index == grid_index {
                            

                            Self::create_upgrade(tower, UpgradeType::Health);
                            let mut upgrade_menu = UpgradeMenu {
                                background_rect: sdl2::rect::Rect::new(0, 0, 100, 100),
                                upgrades_vec: Vec::new(),
                                menu_active: true,
                                grid_index,
                                building: BuildingType::Archer,
                                damage: tower.projectile_damage,
                                health: tower.health,
                                upgrade: UpgradeType::None,
                                current_level: 0,
                            };
                            let texture_surface = self.font.render(&format!("LEVEL: {}", 0).to_string())
                                .blended(constants::COLOR_BACKGROUND)
                                .map_err(|e| e.to_string())?;

                            let upgrade = Upgrade {
                                texture_surface,
                                upgrade_rect: sdl2::rect::Rect::new(0, 0, 50, 50),
                            };
                            upgrade_menu.upgrades_vec.push(upgrade);

                            self.upgrade_vec.push(upgrade_menu);
                            //push to upgrade vec and check if grid index already in upgrade vec

                            tower.health = 9999;
                            tower.max_health = 9999;
                            println!("UPGRADED");
                            gui_manager.create_message("upgrade started".to_string(), 256);
                        }
                    }
                },
                _ => {},
            }
        }
        Ok(())
    }
    pub fn render_upgrade_menu(&mut self, game: &mut game_manager::GameManager) {
        for upgrade in &mut self.upgrade_vec {
            if upgrade.menu_active {
                upgrade.background_rect.set_x((upgrade.grid_index.0 as i32 * constants::TILE_SIZE as i32) - game.cam_x); 
                upgrade.background_rect.set_y((upgrade.grid_index.1 as i32 * constants::TILE_SIZE as i32) - game.cam_y);

                for upgrades_index in 0..upgrade.upgrades_vec.len() {
                    let upgrades = &mut upgrade.upgrades_vec[upgrades_index];

                    upgrades.upgrade_rect.set_x((upgrade.grid_index.0 as i32 * constants::TILE_SIZE as i32) - game.cam_x); 
                    upgrades.upgrade_rect.set_y((upgrade.grid_index.1 as i32 * constants::TILE_SIZE as i32) - game.cam_y);
                    upgrades.upgrade_rect.set_width(upgrades.texture_surface.width());
                    upgrades.upgrade_rect.set_height(upgrades.texture_surface.height());

                    game.canvas.set_draw_color(sdl2::pixels::Color::WHITE);

                    game.canvas.fill_rect(upgrade.background_rect);

                    let texture_result = self.texture_creator.create_texture_from_surface(&upgrades.texture_surface);

                    let texture = match texture_result {
                        Ok(texture) => texture,
                        Err(err) => {
                            eprintln!("Failed to create texture from surface:\t{}", err);
                            continue; 
                        }
                    };

                    if let Err(err) = game.canvas.copy(&texture, None, upgrades.upgrade_rect) {
                        eprintln!("Failed to copy texture to canvas:\t{}", err);
                    }
                }

                println!("filled upgrade");


            }
        }
    }
    pub fn create_upgrade<T>(entity: &mut T, upgrade_type: UpgradeType) {
        match upgrade_type {
            UpgradeType::Health => {

            },
            UpgradeType::Damage => {

            },
            UpgradeType::None => {},
        }
    }
}
