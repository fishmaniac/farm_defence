use crate::constants;
use crate::game_manager;
use crate::enemy_manager;
use crate::tower_manager;

pub struct GUI {
    pub index: (usize, usize),
    pub rect: sdl2::rect::Rect,
}

pub struct GUIManager {
    pub gui_vec: Vec<GUI>,
}

impl GUIManager {
    pub fn new () -> Self {
        let gui = GUIManager {
            gui_vec: Vec::new(),
        };
        gui
    }
    pub fn render_health_bar_enemy (&mut self, game: &mut game_manager::GameManager, enemy: &enemy_manager::Enemy) {
        //TODO: MATCH TO ENEMY TYPE
        let back_rect = sdl2::rect::Rect::new(enemy.rect.x() + (enemy.rect.width() - constants::ENEMY_GOBLIN_HEALTH_BAR_WIDTH) as i32 / 2, enemy.rect.y() - constants::ENEMY_GOBLIN_HEALTH_BAR_HEIGHT as i32, constants::ENEMY_GOBLIN_HEALTH_BAR_WIDTH, constants::ENEMY_GOBLIN_HEALTH_BAR_HEIGHT);
        let health_percentage = enemy.health as f64 / constants::ENEMY_GOBLIN_HEALTH as f64;

        let temp_gui = self::GUI {
            index: enemy.index,
            rect: sdl2::rect::Rect::new(back_rect.x(), back_rect.y(), (back_rect.width() as f64 * health_percentage) as u32, back_rect.height()),
        };
        /*  self.gui_vec.push(temp_gui); */
        game.canvas.set_draw_color(constants::COLOR_RED);
        game.canvas.fill_rect(back_rect);
        game.canvas.set_draw_color(constants::COLOR_GREEN);
        game.canvas.fill_rect(temp_gui.rect);
    }
    pub fn render_health_bar_tower (&mut self, game: &mut game_manager::GameManager, tower: &tower_manager::Tower) {
        //TODO: MATCH TO TOWER TYPE
        let back_rect = sdl2::rect::Rect::new(tower.top_rect.x() + (tower.top_rect.width() - constants::TOWER_ARCHER_HEALTH_BAR_WIDTH) as i32 / 2, tower.top_rect.y() - constants::TOWER_ARCHER_HEALTH_BAR_HEIGHT as i32, constants::TOWER_ARCHER_HEALTH_BAR_WIDTH, constants::TOWER_ARCHER_HEALTH_BAR_HEIGHT);

        let health_percentage = tower.health as f64 / constants::TOWER_ARCHER_HEALTH as f64;

        let temp_gui = self::GUI {
            index: (tower.top_index.0, tower.top_index.1),
            rect: sdl2::rect::Rect::new(back_rect.x(), back_rect.y(), (back_rect.width() as f64 * health_percentage) as u32, back_rect.height()),
        };
        /*  self.gui_vec.push(temp_gui); */
        game.canvas.set_draw_color(constants::COLOR_RED);
        game.canvas.fill_rect(back_rect);
        game.canvas.set_draw_color(constants::COLOR_GREEN);
        game.canvas.fill_rect(temp_gui.rect);
    }
}
