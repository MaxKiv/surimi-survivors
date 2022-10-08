use macroquad::prelude::*;
use macroquad::prelude::animation::AnimatedSprite;
use macroquad::prelude::collections::storage;
use macroquad_platformer::{Actor, Solid, World};

pub const PLAYER_SIZE: Vec2 = vec2(32.0, 32.0);
pub const HEALTHBAR_SIZE: Vec2 = vec2(50.0, 10.0);

struct Resources {
    crab_sprite: Texture2D,
}

impl Resources {
    async fn new() -> Result<Resources, FileError> {
        let crab_sprite = load_texture("assets/rustacean_happy.png").await?;

        Ok(Resources{
            crab_sprite
        })
    }
}

struct CrabPlayer {
    collider: Actor,
    speed: Vec2,
    health: f32,
}

struct Wall {
    collider: Solid,
    speed: f32
}

#[macroquad::main("InputKeys")]
async fn main() {
    let x = screen_width() / 2.0;
    let y = screen_height() / 2.0;
    let width = 800.0;
    let height = width * (y / x);
    let mut game_running = true;
    let mut alive = true;

    let resources = Resources::new().await.unwrap();

    let mut world = World::new();

    let player = CrabPlayer {
        collider: world.add_actor(vec2(50.0, 80.0), 8, 8),
        speed: vec2(0., 0.),
        health: 100.0,
    };

    let wall = Wall{
        collider: world.add_solid(vec2(400.0, 130.0), 32, 8),
        speed: 50.,
    };

    while alive && game_running {
        clear_background(LIGHTGRAY);

        {
            let pos = world.solid_pos(wall.collider);
            draw_texture(resources.crab_sprite, pos.x, pos.y, YELLOW);
        }

        let mut player_pos = world.actor_pos(player.collider);
        if is_key_down(KeyCode::D) {
            player_pos.x += 1.0;
            world.set_actor_position(player.collider, player_pos);
        }
        if is_key_down(KeyCode::A) {
            player_pos.x -= 1.0;
            world.set_actor_position(player.collider, player_pos);
        }
        if is_key_down(KeyCode::S) {
            player_pos.y += 1.0;
            world.set_actor_position(player.collider, player_pos);
        }
        if is_key_down(KeyCode::W) {
            player_pos.y -= 1.0;
            world.set_actor_position(player.collider, player_pos);
        }
        if is_key_down(KeyCode::Escape) {
            game_running = false;
        }

        draw_text(&format!("{} FPS", get_fps()).to_string(), 20.0, 20.0, 20.0, DARKGRAY);

        let camera = Camera2D::from_display_rect(Rect::new(
            player_pos.x - (width / 2.0),
            player_pos.y - (height / 2.0),
            width,
            height)
        );
        set_camera(&camera);

        draw_player(&player, &player_pos, &resources);

        next_frame().await
    }

    // scoreboard and close game

}

fn draw_player(player: &CrabPlayer, pos: &Vec2, resources: &Resources) {
    draw_texture(resources.crab_sprite, pos.x, pos.y, YELLOW);
    draw_rectangle_lines(pos.x, pos.y, PLAYER_SIZE.x, PLAYER_SIZE.y, 5.0, BLACK);
    draw_healthbar(&player, &pos);
}

fn draw_healthbar(player: &CrabPlayer, pos: &Vec2) {
    let health_scaling_factor = player.health / 100.0;
    // let offset = vec2(-PLAYER_SIZE.x-HEALTHBAR_SIZE.x/2.0, PLAYER_SIZE.y);
    let offset = vec2(PLAYER_SIZE.x / 2.0 - HEALTHBAR_SIZE.x / 2.0, PLAYER_SIZE.y + HEALTHBAR_SIZE.y / 2.0);
    draw_rectangle_lines(pos.x + offset.x + 2.0, pos.y + offset.y + 2.0, HEALTHBAR_SIZE.x, HEALTHBAR_SIZE.y, 5.0, BLACK);
    draw_rectangle(pos.x + offset.x, pos.y + offset.y, health_scaling_factor * HEALTHBAR_SIZE.x, health_scaling_factor * HEALTHBAR_SIZE.y, RED);
}
