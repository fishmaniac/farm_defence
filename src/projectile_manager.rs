use crate::constants;
use crate::player_manager;
use crate::game_manager;
use crate::level_manager;
use crate::enemy_manager;
use crate::texture_manager;
use crate::tower_manager;

pub struct Projectile {
    pub rect: sdl2::rect::Rect,
    pub texture_path: String,
    pub time: u8,
    pub start: (i32, i32),
    pub position: (i32, i32),
    pub target: (i32, i32),
    pub hit_target: bool,
    angle: f64,
    speed: u8,
    pub radius: u16,
    pub damage: u8,
}

pub struct ProjectileManager {
    pub projectile_vec: Vec<Projectile>,
}

impl ProjectileManager {
    pub fn new () -> Self {
        let projectiles = ProjectileManager {
            projectile_vec: Vec::new(),
        };
        projectiles
    }
    pub fn spawn_projectile (&mut self, tower: &mut tower_manager::Tower, start: (i32, i32), position: (i32, i32), target: (i32, i32)) {
        let projectile = self::Projectile {
            time: 0,
            rect: sdl2::rect::Rect::new(position.0, position.1, constants::TILE_SIZE, constants::TILE_SIZE),
            texture_path: constants::TEXTURE_PROJECTILE_ARROW.to_string(),
            start,
            position,
            target,
            hit_target: false,
            angle: Self::calculate_angle(start, target),
            speed: constants::PROJECTILE_ARROW_SPEED,
            radius: 128,
            damage: tower.attack_damage,
        };

        self.projectile_vec.push(projectile);
    }

    fn move_projectile (projectile: &mut Projectile, game: &mut game_manager::GameManager) {
        let dx = projectile.target.0 - projectile.position.0;
        let dy = projectile.target.1 - projectile.position.1 ;
        let distance = (((dx * dx) + (dy * dy)) as f32).sqrt();
        let direction_x = dx as f32 / distance;
        let direction_y = dy as f32 / distance;

        projectile.position.0 += (direction_x * projectile.speed as f32) as i32;
        projectile.position.1 += (direction_y * projectile.speed as f32) as i32;
    }

    pub fn render_projectiles (&mut self, game: &mut game_manager::GameManager, tex_man: &mut texture_manager::TextureManager<sdl2::video::WindowContext>) -> Result<(), String> {
        for projectile in &mut self.projectile_vec {
            projectile.rect.set_x(projectile.position.0 - game.cam_x);
            projectile.rect.set_y(projectile.position.1 - game.cam_y);

            let texture = tex_man.load(&projectile.texture_path)?;
            game.canvas.copy_ex(
                &texture, // Texture object
                None,      // source rect
                projectile.rect,     // destination rect
                projectile.angle,      // angle (degrees)
                None,   // center
                false,    // flip horizontal
                false,     // flip vertical
            )?;

            if !tower_manager::TowerManager::is_within_area(projectile.position, projectile.target, projectile.speed as i32) {
                Self::move_projectile(projectile, game);
            }
            else {
                projectile.time += 1;
            }
        }
        Ok(())
    }
    fn calculate_angle(position: (i32, i32), target: (i32, i32)) -> f64 {
        let dx = target.0 - position.0;
        let dy = target.1 - position.1;

        let angle = (dy as f64).atan2(dx as f64).to_degrees() + 90.0;
        angle
    }
}
