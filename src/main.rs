mod enemy;

use std::time::Duration;
use macroquad::prelude::*;
use macroquad_platformer::{Actor, World};

pub const SCREEN_SIZE: Vec2 = vec2(1000., 1000.); 
pub const PLAYER_SPEED: f32 = 5.0;
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
    pos: Vec2,
    speed: Vec2,
    health: f32,
}

struct GameState {
    alive: bool,
    player: CrabPlayer,
    max_enemies: i32,
    enemies: Vec<Enemy>,
    playtime: Duration,
}

fn conf() -> Conf {
    Conf {
        window_title: String::from("Surimi Survivors"),
        window_width: 1260,
        window_height: 768,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut world = World::new();

    let mut gs = GameState {
        alive: true,
        player: CrabPlayer {
            collider: world.add_actor(vec2(0., 0.), PLAYER_SIZE.x.round() as i32, PLAYER_SIZE.y.round() as i32),
            pos: vec2(0., 0.),
            speed: vec2(PLAYER_SPEED, PLAYER_SPEED),
            health: 100.0,
        },
        max_enemies: 100,
        enemies: Vec::new(),
        playtime: Duration::new(0, 0),
    };

    let x = screen_width() / 2.0;
    let y = screen_height() / 2.0;
    let width = 800.0;
    let height = width * (y / x);

    let resources = Resources::new().await.unwrap();

    let enemy = Enemy {
        collider: world.add_actor(vec2(200.0, 200.0), 200, 200),
        pos: vec2(200., 200.),
        speed: vec2(1.,1.),
        health: 10.,
        damage: 1.,
    };
    // let wall = Wall{
    //     collider: world.add_solid(vec2(400.0, 130.0), 32, 8),
    // };

    while gs.alive {
        clear_background(WHITE);

        draw_fps();
        draw_time(&gs);

        // draw_projectiles(&gs, &resources);

        process_inputs(&mut world, &mut gs);

        // let collided_with_definitely_wall =  world.collide_check(player.collider, player_pos);

        if world.collide_check(gs.player.collider, gs.player.pos) {
            gs.player.health -= 1.0;
        }

        let camera = Camera2D::from_display_rect(Rect::new(
            // player_pos.x - (width / 2.0),
            // player_pos.y - (height / 2.0),
            gs.player.pos.x - (width / 2.0),
            gs.player.pos.y - (height / 2.0),
            width,
            height)
        );
        set_camera(&camera);

        // draw_enemies(&world, &resources);
        // draw single enemy
        // draw_enemy();

        draw_player(&mut gs, &resources);

        update_health(&mut gs);

        next_frame().await
    }

    // scoreboard and close game

}

fn draw_fps() -> () {
    draw_text(&format!("{} FPS", get_fps()).to_string(), 20.0, 20.0, 20.0, DARKGRAY);
}

fn draw_time(gs: &GameState) -> () {
    let seconds = gs.playtime.as_secs() % 60;
    let minutes = (gs.playtime.as_secs() / 60) % 60;
    let hours = (gs.playtime.as_secs() / 60) / 60;
    let time_string = format!("{}:{}:{}", hours, minutes, seconds);
    draw_text(&time_string, 0., 0., 20.0, DARKGRAY);
}

fn update_health(gs: &mut GameState) -> () {
    if gs.player.health <= 0.0 {
        gs.alive = false;
    }
}

// fn draw_enemies(enemies: &mut [Enemy], resources: &Resources) {
    // for enemy in &enemies {
    //     draw_enemy(&enemy, &resources);
    // }
// }

//
// fn draw_projectiles(gs: &GameState, rs: &Resources) {
//     for p in gs.projectiles {
//         draw_texture(rs.crab_sprite, gs.player.pos.x, gs.player.pos.y, RED);
//     }
// }

fn process_inputs(world: &mut World, gs: &mut GameState){
    if is_key_down(KeyCode::D) {
        gs.player.pos.x += PLAYER_SPEED;
        world.set_actor_position(gs.player.collider, gs.player.pos);
    }
    if is_key_down(KeyCode::A) {
        gs.player.pos.x -= PLAYER_SPEED;
        world.set_actor_position(gs.player.collider, gs.player.pos);
    }
    if is_key_down(KeyCode::S) {
        gs.player.pos.y += PLAYER_SPEED;
        world.set_actor_position(gs.player.collider, gs.player.pos);
    }
    if is_key_down(KeyCode::W) {
        gs.player.pos.y -= PLAYER_SPEED;
        world.set_actor_position(gs.player.collider, gs.player.pos);
    }
    if is_key_down(KeyCode::Escape) {
        gs.alive = false;
    }
}

fn draw_player(gs: &mut GameState, resources: &Resources) {
    draw_texture(resources.crab_sprite, gs.player.pos.x, gs.player.pos.y, YELLOW);
    // draw collision box
    draw_collision_box(gs);
    draw_healthbar(gs);
}

fn draw_collision_box(gs: &mut GameState) -> () {
    draw_rectangle_lines(
        gs.player.pos.x,
        gs.player.pos.y,
        PLAYER_SIZE.x,
        PLAYER_SIZE.y,
        5.0,
        GRAY
    );
}

fn draw_healthbar(gs: &mut GameState) {
    let health_scaling_factor = gs.player.health / 100.0;
    let offset = vec2(
        PLAYER_SIZE.x / 2.0 - HEALTHBAR_SIZE.x / 2.0,
        PLAYER_SIZE.y + HEALTHBAR_SIZE.y / 2.0
    );
    draw_rectangle(
        gs.player.pos.x + offset.x,
        gs.player.pos.y + offset.y,
        health_scaling_factor * HEALTHBAR_SIZE.x,
        HEALTHBAR_SIZE.y,
        RED
    );
    draw_rectangle_lines(
        gs.player.pos.x + offset.x,
        gs.player.pos.y + offset.y,
        health_scaling_factor * HEALTHBAR_SIZE.x,
        HEALTHBAR_SIZE.y,
        5.0,
        BLACK
    );
}


// fn does_projectile_hit(projectile: &Projectile, target: &CrabPlayer){
//
// }
