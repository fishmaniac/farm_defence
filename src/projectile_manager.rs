use crate::constants;
use crate::event_manager;
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
    speed: f64,
    pub radius: u8,
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
    pub fn spawn_player_projectile (&mut self, player: &mut player_manager::PlayerManager, start: (i32, i32), position: (i32, i32), target: (i32, i32)) {
        let projectile = self::Projectile {
            time: 0,
            rect: sdl2::rect::Rect::new(position.0, position.1, constants::TILE_SIZE, constants::TILE_SIZE),
            texture_path: player.projectile_texture.clone(),
            start,
            position,
            target,
            hit_target: false,
            angle: Self::calculate_angle(start, target),
            speed: player.projectile_speed,
            radius: player.projectile_radius,
            damage: player.projectile_damage,
        };

        self.projectile_vec.push(projectile);
    }

    pub fn spawn_tower_projectile (&mut self, tower: &mut tower_manager::Tower, start: (i32, i32), position: (i32, i32), target: (i32, i32)) {
        let projectile = self::Projectile {
            time: 0,
            rect: sdl2::rect::Rect::new(position.0, position.1, constants::TILE_SIZE, constants::TILE_SIZE),
            texture_path: tower.projectile_texture.clone(),
            start,
            position,
            target,
            hit_target: false,
            angle: Self::calculate_angle(start, target),
            speed: tower.projectile_speed,
            radius: tower.projectile_radius,
            damage: tower.projectile_damage,
        };

        self.projectile_vec.push(projectile);
    }

    fn move_projectile (projectile: &mut Projectile, events: &mut event_manager::EventManager) {
        let dx = projectile.target.0 - projectile.position.0;
        let dy = projectile.target.1 - projectile.position.1 ;
        let distance = (((dx * dx) + (dy * dy)) as f32).sqrt();
        let direction_x = dx as f32 / distance;
        let direction_y = dy as f32 / distance;

        projectile.position.0 += (direction_x * (projectile.speed * events.delta_time.max(constants::MIN_GAME_RATE)) as f32) as i32;
        projectile.position.1 += (direction_y * (projectile.speed * events.delta_time.max(constants::MIN_GAME_RATE)) as f32) as i32;
    }

    pub fn check_projectile_hit(&mut self, game: &mut game_manager::GameManager, events: &mut event_manager::EventManager, player: &mut player_manager::PlayerManager, enemies: &mut enemy_manager::EnemyManager) {
        for enemy in &mut enemies.enemy_vec {
            let enemy_pos_pixel = (enemy.pixel_index.0 as i32, enemy.pixel_index.1 as i32);

            if player.is_attacking {
                let start = (player.rect.x() + player.x, player.rect.y() + player.y);
                self.spawn_player_projectile(player, start, start, (events.mouse_point.x + game.cam_x, events.mouse_point.y + game.cam_y)); 
                player.is_attacking = false;

            }
            for projectile in &mut self.projectile_vec {
                let projectile_hit: bool = tower_manager::TowerManager::is_within_area(projectile.position, enemy_pos_pixel, projectile.radius as i32);

                if projectile_hit && enemy.health != 0 && !projectile.hit_target {
                    if enemy.health > projectile.damage as u16 {
                        enemy.health -= projectile.damage as u16;
                    }
                    else {
                        enemy.health = 0;
                    }
                    projectile.hit_target = true;
                }
            }
        }
    }

    pub fn render_projectiles (&mut self, game: &mut game_manager::GameManager, tex_man: &mut texture_manager::TextureManager<sdl2::video::WindowContext>, events: &mut event_manager::EventManager) -> Result<(), String> {
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

            if !tower_manager::TowerManager::is_within_area(projectile.position, projectile.target, (projectile.speed * (events.delta_time).max(constants::MIN_GAME_RATE)) as i32) {
                Self::move_projectile(projectile, events);
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
