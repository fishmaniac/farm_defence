use crate::constants;
use crate::player_manager;
use crate::game_manager;
use crate::level_manager;
use crate::enemy_manager;

pub struct Projectile {
    pub rect: sdl2::rect::Rect,
    pub texture_path: String,
}

pub struct ProjectileManager {
    pub project_vec: Vec<Projectile>,
}

impl ProjectileManager {
    pub fn new () -> Self {
        let projectiles = ProjectileManager {
            project_vec: Vec::new(),
        };
        projectiles
    }
}
