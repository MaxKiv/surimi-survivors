use macroquad::prelude::{Vec2, vec2, collections::storage, WHITE};
use macroquad_platformer::Actor;
use rand::Rng;
use crate::{SCREEN_SIZE, Resources, draw_texture};

#[derive(Debug)]
enum EnemyType {
    Shark,
}

#[derive(Debug)]
pub struct Enemy {
    r#type: EnemyType,
    pos: Vec2,
    speed: Vec2,
    health: f32,
    damage: f32,
}

pub fn spawn_enemy(pos: Vec2, enemy: &Enemy) -> () {
    let enemy = Enemy {
        r#type: EnemyType::Shark,
        pos: spawn_location_factory(&pos),
        speed: vec2(1.,1.),
        health: 100.,
        damage: 1.,
    };
    draw_enemy(&enemy);
}

pub fn draw_enemy(enemy: &Enemy) -> () {
    let rs = storage::get::<Resources>();
    draw_texture(rs.enemy_sprite, enemy.pos.x, enemy.pos.y, WHITE);
}

pub fn spawn_location_factory(player_pos: &Vec2) -> Vec2 {
    let angle = rand::thread_rng().gen_range(0.0..2.0*std::f64::consts::PI);
    let distance: f32 = 1.5 * (SCREEN_SIZE.x.powi(2) + SCREEN_SIZE.y.powi(2)).sqrt();
    let x = player_pos.x + (distance * angle.acos()as f32);
    let y = player_pos.y + (distance * angle.asin()as f32);
    return vec2(x,y);
}
