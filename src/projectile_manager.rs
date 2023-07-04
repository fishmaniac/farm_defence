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
    start: (i32, i32),
    position: (i32, i32),
    target: (i32, i32),
    angle: f64,
    speed: u8,
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
    pub fn spawn_projectile (&mut self, start: (i32, i32), position: (i32, i32), target: (i32, i32)) {
        //TODO: projectile
/*         if self.projectile_vec.len() < 3 { */
            let projectile = self::Projectile {
                rect: sdl2::rect::Rect::new(position.0, position.1, constants::TILE_SIZE, constants::TILE_SIZE),
                texture_path: constants::TEXTURE_PROJECTILE_ARROW.to_string(),
                start,
                position,
                target,
                speed: constants::PROJECTILE_ARROW_SPEED,
                angle: Self::calculate_angle(start, target),
        };


            self.projectile_vec.push(projectile);
/*         } */
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
        for projectile_index in 0..self.projectile_vec.len() {
            if projectile_index < self.projectile_vec.len() {
                //TODO: ADD CAM TRANSFORM?
                let position: (i32, i32) = self.projectile_vec[projectile_index].position;

                self.projectile_vec[projectile_index].rect.set_x(position.0 - game.cam_x);
                self.projectile_vec[projectile_index].rect.set_y(position.1 - game.cam_y);

                let texture = tex_man.load(&self.projectile_vec[projectile_index].texture_path)?;
                game.canvas.copy_ex(
                    &texture, // Texture object
                    None,      // source rect
                    self.projectile_vec[projectile_index].rect,     // destination rect
                    self.projectile_vec[projectile_index].angle,      // angle (degrees)
                    None,   // center
                    false,    // flip horizontal
                    false,     // flip vertical
                )?;

                if !tower_manager::TowerManager::is_within_area(self.projectile_vec[projectile_index].position, self.projectile_vec[projectile_index].target, self.projectile_vec[projectile_index].speed as i32) {
                    Self::move_projectile(&mut self.projectile_vec[projectile_index], game);
                }
                else {
                    self.projectile_vec.remove(projectile_index);
                }
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
