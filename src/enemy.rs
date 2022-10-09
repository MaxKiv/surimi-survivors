use rand::*;
use macroquad::prelude::{Vec2, vec2};
use macroquad_platformer::Actor;
use crate::{GameState, SCREEN_SIZE};

#[derive(Debug)]
enum EnemyType {
    Shark,
}

#[derive(Debug)]
struct Enemy {
    r#type: EnemyType,
    collider: Actor,
    pos: Vec2,
    speed: Vec2,
    health: f32,
    damage: f32,
}

impl Enemy {
    fn new(collider: Actor, pos: Vec2, speed: Vec2, health: f32, damage: f32) -> Self { Self { r#type: EnemyType::Shark, collider, pos, speed, health, damage } }
}

fn spawn_enemy(gs: &mut GameState, enemy: &Enemy) -> () {
    draw_enemy(spawn_location_factory(gs), enemy);
}

fn draw_enemy(pos: Vec2, enemy: &Enemy) -> () {

}

fn spawn_location_factory(gs: &mut GameState) -> Vec2 {
    let angle = rand::thread_rng().gen_range(0.0..2.0*std::f64::consts::PI);
    let distance: f32 = 1.5 * (SCREEN_SIZE.x.powi(2) + SCREEN_SIZE.y.powi(2)).sqrt();
    let x = gs.player.pos.x + (distance * angle.acos()as f32);
    let y = gs.player.pos.y + (distance * angle.asin()as f32);
    return vec2(x,y);
}

