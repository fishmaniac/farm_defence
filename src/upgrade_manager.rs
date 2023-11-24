use crate::building_manager;
use crate::constants;
use crate::event_manager;
use crate::gui_manager;
use crate::level_manager;
use crate::tower_manager;
use crate::game_manager;

// pub enum UpgradePath {
//     First,
//     Second,
// }

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
    cost: u32,
    damage: u8,
    health: u16,
    radius: i32,
    last_clicked: i32,
}

pub struct UpgradeMenu <'a> {
    background_rect: sdl2::rect::Rect,
    upgrades_first_path: std::collections::LinkedList<Upgrade <'a>>,
    upgrades_second_path: std::collections::LinkedList<Upgrade <'a>>,
    current_first_path: Option<Upgrade<'a>>,
    current_second_path: Option<Upgrade<'a>>,
    menu_active: bool,
    grid_index: (usize, usize),
    building_index: usize,
}

pub struct UpgradeManager <'a> {
    pub upgrade_menu_vec: Vec<UpgradeMenu<'a>>,
    texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    font: &'a sdl2::ttf::Font<'a, 'a>,
    /*     empty_surface: sdl2::surface::Surface<'a>, */
}

impl<'a> UpgradeManager <'a> {
    pub fn new(game:&mut game_manager::GameManager, font: &'a sdl2::ttf::Font<'a, 'a>) -> UpgradeManager<'a> {
        let upgrades = UpgradeManager {
            texture_creator: game.canvas.texture_creator(),
            upgrade_menu_vec: Vec::new(),
            font,
        };
        upgrades
    }
    pub fn check_upgrade(&mut self, 
        game: &mut game_manager::GameManager,
        towers: &mut tower_manager::TowerManager, 
        buildings: &mut building_manager::BuildingManager, 
        gui_manager: &mut gui_manager::GUIManager, 
        temp_tile: &mut level_manager::LevelTile, 
        grid_index: (usize, usize)) 
    -> Result<(), String> {
        let mut upgrade_exists: bool = false;
        for upgrade in &mut self.upgrade_menu_vec {
            if upgrade.grid_index == grid_index {
                upgrade.menu_active = true;
                upgrade_exists = true;
            }
            else if upgrade.grid_index != grid_index {
                upgrade.menu_active = false;
            }
        }

        if !upgrade_exists {
            match temp_tile.tile_type {
                constants::TILE_TYPE_ARCHER_BOTTOM => {
                    self.create_upgrades(
                        gui_manager,
                        towers,
                        BuildingType::Archer,
                        grid_index
                    );
                },
                constants::TILE_TYPE_FIREBALL_BOTTOM => {
                    self.create_upgrades(
                        gui_manager,
                        towers,
                        BuildingType::Fireball,
                        grid_index
                    );
                },
                _ => {},
            }
        }
        Ok(())
    }
    pub fn create_upgrades(&mut self, 
        gui_manager: &mut gui_manager::GUIManager, 
        towers: &mut tower_manager::TowerManager, 
        building_type: BuildingType, 
        grid_index: (usize, usize)) 
    -> Result<(), String> {
        match building_type {
            BuildingType::Archer => {
                for tower_index in 0..towers.tower_vec.len() {
                    let tower = &mut towers.tower_vec[tower_index];
                    if tower.bottom_index == grid_index {
                        let upgrade_menu: Option<UpgradeMenu> 
                        = match Self::initialize_upgrade_menu(grid_index, tower_index) {
                            Ok(upgrade_menu) => Some(upgrade_menu),
                            Err(e) => None,
                        };
                        match upgrade_menu {
                            Some(mut upgrade_menu) => {
                                if let Ok(upgrade_first_0) 
                                = self.create_upgrade_button(
                                    "broadheads",
                                    0,
                                    constants::TOWER_ARCHER_DAMAGE * 2,
                                    0,
                                    5
                                ) {
                                    upgrade_menu.upgrades_first_path.push_back(upgrade_first_0);
                                }
                                if let Ok(upgrade_first_1) 
                                = self.create_upgrade_button(
                                    "extra training",
                                    0,
                                    constants::TOWER_ARCHER_DAMAGE * 3,
                                    5,
                                    1
                                ) {
                                    upgrade_menu.upgrades_first_path.push_back(upgrade_first_1);
                                }
                                if let Ok(upgrade_second_0) 
                                = self.create_upgrade_button(
                                    "reinforced tower",
                                    constants::TOWER_ARCHER_HEALTH * 2,
                                    0,
                                    0,
                                    5
                                ) {
                                    upgrade_menu.upgrades_second_path.push_back(upgrade_second_0);
                                }
                                if let Ok(upgrade_second_1) 
                                = self.create_upgrade_button(
                                    "more reinforceteststststststststststst",
                                    constants::TOWER_ARCHER_HEALTH * 3,
                                    0,
                                    0,
                                    15
                                ) {
                                    upgrade_menu.upgrades_second_path.push_back(upgrade_second_1);
                                }

                                self.upgrade_menu_vec.push(upgrade_menu);
                                gui_manager.create_message("upgrade started".to_string(), 256);
                            }
                            None => gui_manager.create_message(
                                "error creating upgrade... why".to_string(),
                                1028
                            ),
                        }
                    }
                }
            },
            BuildingType::Fireball => {
                for tower_index in 0..towers.tower_vec.len() {
                    let tower = &mut towers.tower_vec[tower_index];
                    if tower.bottom_index == grid_index {
                        let upgrade_menu: Option<UpgradeMenu> 
                        = match Self::initialize_upgrade_menu(grid_index, tower_index) {
                            Ok(upgrade_menu) => Some(upgrade_menu),
                            Err(e) => None,
                        };
                        match upgrade_menu {
                            Some(mut upgrade_menu) => { 
                                if let Ok(upgrade_first_0) = self.create_upgrade_button(
                                    "bigger balls",
                                    0,
                                    constants::TOWER_FIREBALL_DAMAGE * 2,
                                    5,
                                    5
                                ) {
                                    upgrade_menu.upgrades_first_path.push_back(upgrade_first_0);
                                }
                                if let Ok(upgrade_first_1) = self.create_upgrade_button(
                                    "extra hot balls",
                                    0,
                                    constants::TOWER_FIREBALL_DAMAGE * 3,
                                    0,
                                    1
                                ) {
                                    upgrade_menu.upgrades_first_path.push_back(upgrade_first_1);
                                }
                                if let Ok(upgrade_second_0) = self.create_upgrade_button(
                                    "reinforced tower",
                                    constants::TOWER_FIREBALL_HEALTH * 2,
                                    0,
                                    0,
                                    5
                                ) {
                                    upgrade_menu.upgrades_second_path.push_back(upgrade_second_0);
                                }
                                if let Ok(upgrade_second_1) = self.create_upgrade_button(
                                    "m",
                                    constants::TOWER_FIREBALL_HEALTH * 3,
                                    0,
                                    0,
                                    15
                                ) {
                                    upgrade_menu.upgrades_second_path.push_back(upgrade_second_1);
                                }

                                self.upgrade_menu_vec.push(upgrade_menu);
                                gui_manager.create_message("upgrade started".to_string(), 256);
                            }
                            None => gui_manager.create_message(
                                "error creating upgrade... why".to_string(),
                                1028
                            ),
                        }
                    }
                }

            },
            BuildingType::Base => {},
            BuildingType::None => {},
        }
        Ok(())
    }
    fn initialize_upgrade_menu(grid_index: (usize, usize), tower_index: usize) 
    -> Result<UpgradeMenu<'a>, String> {
        let upgrade_menu = UpgradeMenu {
            background_rect: sdl2::rect::Rect::new(
                0,
                0,
                constants::TILE_SIZE,
                constants::TILE_SIZE * 2
            ),
            upgrades_first_path: std::collections::LinkedList::new(),
            upgrades_second_path: std::collections::LinkedList::new(),
            current_first_path: None,
            current_second_path: None,
            menu_active: true,
            grid_index,
            building_index: tower_index,
        };

        Ok(upgrade_menu)
    }

    fn create_upgrade_button(&mut self, 
        text: &str, 
        health: u16, 
        damage: u8, 
        radius: i32, 
        cost: u32) 
    -> Result<Upgrade<'a>, String> {
        let texture_surface = self.font.render(
            &format!(
            "{} {}",
            cost,
            text
        ).to_string()
        )
            .blended(constants::COLOR_BACKGROUND)
            .map_err(|e| e.to_string())?;
        let upgrade = Upgrade {
            texture_surface,
            upgrade_rect: sdl2::rect::Rect::new(0, 0, 0, 0),
            last_clicked: 0,
            cost,
            damage,
            health,
            radius,
        };
        Ok(upgrade)
    }
    pub fn update_upgrade_menus(&mut self, 
        game: &mut game_manager::GameManager, 
        events: &mut event_manager::EventManager, 
        towers: &mut tower_manager::TowerManager) {
        if !game.build_mode && !game.seed_mode {
            for upgrade_index in  0..self.upgrade_menu_vec.len() {
                let upgrade = &mut self.upgrade_menu_vec[upgrade_index];
                if upgrade.current_first_path.is_none() {
                    if let Some(current_first) 
                    = upgrade.upgrades_first_path.pop_front() {
                        upgrade.current_first_path = Some(current_first);
                    }
                }
                if upgrade.current_second_path.is_none() {
                    if let Some(current_second) 
                    = upgrade.upgrades_second_path.pop_front() {
                        upgrade.current_second_path = Some(current_second);
                    }
                }
                if upgrade.menu_active {
                    if let Some(current_first) = &mut upgrade.current_first_path {
                        if current_first.upgrade_rect.contains_point(game.mouse_point) 
                        && game.mouse_button == sdl2::mouse::MouseButton::Left 
                        && current_first.last_clicked > 64 
                        && game.gold_amount >= current_first.cost as u32 {
                            Self::upgrade_current_upgrade(
                                game,
                                towers,
                                current_first,
                                upgrade_index
                            );
                            upgrade.background_rect.set_width(0);
                            /*                             println!("PREV DAMAGE: {}, PREV RADIUS: {}, UPGRADE DAMAGE: {}, UPGRADE_RADIUS: {}", towers.tower_vec[upgrade_index].projectile_damage, towers.tower_vec[upgrade.building_index].attack_radius, current_first.damage, current_first.radius);  */
                            upgrade.current_first_path = None;
                        }
                        else {
                            current_first.last_clicked += 1;
                        }

                    }
                    if let Some(current_second) = &mut upgrade.current_second_path {
                        if current_second.upgrade_rect.contains_point(game.mouse_point) 
                        && game.mouse_button == sdl2::mouse::MouseButton::Left 
                        && current_second.last_clicked > 64 
                        && game.gold_amount >= current_second.cost as u32 {
                            Self::upgrade_current_upgrade(
                                game,
                                towers,
                                current_second,
                                upgrade_index
                            );
                            upgrade.background_rect.set_width(0);
                            /*                             println!("PREV HEALTH: {}, PREV RADIUS: {}, UPGRADE HEALTH: {}, UPGRADE_RADIUS: {}", towers.tower_vec[upgrade_index].health, towers.tower_vec[upgrade.building_index].attack_radius, current_second.damage, current_second.radius);  */
                            upgrade.current_second_path = None;
                        }
                        else {
                            current_second.last_clicked += 1;
                        }
                    }
                }
                else {
                    if let Some(current_first) = &mut upgrade.current_first_path {
                        current_first.last_clicked = 0;
                    }
                    if let Some(current_second) = &mut upgrade.current_second_path {
                        current_second.last_clicked = 0;
                    }
                }
            }
        }
        else {

        }
    }
    fn upgrade_current_upgrade(
        game: &mut game_manager::GameManager, 
        towers: &mut tower_manager::TowerManager, 
        current: &mut Upgrade<'a>, 
        upgrade_index: usize) {
        if current.upgrade_rect.contains_point(game.mouse_point) 
        && game.mouse_button == sdl2::mouse::MouseButton::Left 
        && current.last_clicked > 32 
        && game.gold_amount >= current.cost as u32 {
            game.gold_amount -= current.cost as u32;
            towers.tower_vec[upgrade_index].health += current.health;
            towers.tower_vec[upgrade_index].max_health += current.health;
            towers.tower_vec[upgrade_index].projectile_damage += current.damage;
            towers.tower_vec[upgrade_index].attack_radius += current.radius;
            println!("NEXT DAMAGE: {}", towers.tower_vec[upgrade_index].projectile_damage);
        }
        else {
            current.last_clicked += 1;
        }

    }


    pub fn render_upgrade_menus(&mut self, game: &mut game_manager::GameManager) {
        if !game.build_mode && !game.seed_mode {
            for upgrade in &mut self.upgrade_menu_vec {
                if upgrade.menu_active {
                    upgrade.background_rect.set_x((
                        upgrade.grid_index.0 as i32 * constants::TILE_SIZE as i32
                    ) - game.cam_x); 
                    upgrade.background_rect.set_y((
                        upgrade.grid_index.1 as i32 * constants::TILE_SIZE as i32
                    ) - game.cam_y);

                    game.canvas.set_draw_color(sdl2::pixels::Color::WHITE);
                    game.canvas.fill_rect(upgrade.background_rect);

                    if let Some(upgrades) = &mut upgrade.current_first_path {
                        upgrades.upgrade_rect.set_x(
                            upgrade.grid_index.0 as i32 
                            * constants::TILE_SIZE as i32 
                            - game.cam_x
                        ); 
                        upgrades.upgrade_rect.set_y(
                            upgrade.grid_index.1 as i32 
                            * constants::TILE_SIZE as i32 
                            - game.cam_y
                        );
                        upgrades.upgrade_rect.set_width(upgrades.texture_surface.width());
                        upgrades.upgrade_rect.set_height(upgrades.texture_surface.height());
                        if upgrades.upgrade_rect.width() > upgrade.background_rect.width() {
                            upgrade.background_rect.set_width(upgrades.upgrade_rect.width());
                        }
                        let texture_result 
                        = self.texture_creator.create_texture_from_surface(&upgrades.texture_surface);

                        let texture = match texture_result {
                            Ok(texture) => texture,
                            Err(err) => {
                                eprintln!("Failed to create texture from surface:\t{}", err);
                                continue; 
                            }
                        };

                        if let Err(err) 
                        = game.canvas.copy(&texture, None, upgrades.upgrade_rect) {
                            eprintln!("Failed to copy texture to canvas:\t{}", err);
                        }
                    }
                    else {
                        println!("first empty");

                        game.canvas.set_draw_color(sdl2::pixels::Color::RED);
                        game.canvas.fill_rect(upgrade.background_rect);
                        //~!~!~GET RID OF UNWRAP~!~!~
                        // let texture_surface = self.font.render("empty")
                        //     .blended(constants::COLOR_BACKGROUND)
                        //     .map_err(|e| e.to_string()).unwrap();
                        // upgrade.texture_surface = texture_surface;


                    }

                    if let Some(upgrades) = &mut upgrade.current_second_path {
                        upgrades.upgrade_rect.set_x(
                            upgrade.grid_index.0 as i32 
                            * constants::TILE_SIZE as i32 
                            - game.cam_x
                        ); 
                        upgrades.upgrade_rect.set_y(
                            upgrade.grid_index.1 as i32 
                            * constants::TILE_SIZE as i32 
                            + upgrades.texture_surface.height() as i32 
                            - game.cam_y);
                        upgrades.upgrade_rect.set_width(upgrades.texture_surface.width());
                        upgrades.upgrade_rect.set_height(upgrades.texture_surface.height());
                        if upgrades.upgrade_rect.width() > upgrade.background_rect.width() {
                            upgrade.background_rect.set_width(upgrades.upgrade_rect.width());
                        }

                        let texture_result 
                        = self.texture_creator.create_texture_from_surface(&upgrades.texture_surface);

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
                }
            }
        }
    }
}
