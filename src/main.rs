use std::time::{Duration, Instant};
mod enemy;
use macroquad::prelude::*;

pub const SCREEN_SIZE: Vec2 = vec2(1000., 1000.);
pub const PLAYER_SPEED: f32 = 5.0;
pub const PLAYER_STARTING_POSITION: Vec2 = vec2(0., 0.);
pub const PLAYER_SIZE: Vec2 = vec2(32.0, 32.0);
pub const HEALTHBAR_SIZE: Vec2 = vec2(50.0, 10.0);

struct Resources {
    crab_sprite: Texture2D,
    shark_sprite: Texture2D,
    enemy_sprite: Texture2D,
}

impl Resources {
    async fn new() -> Result<Resources, FileError> {
        let crab_sprite = load_texture("assets/rustacean_happy.png").await?;
        let shark_sprite = load_texture("assets/sharky-single-boi.png").await?;
        let enemy_sprite = load_texture("assets/krab-verbeterd.png").await?;

        Ok(Resources{
            crab_sprite,
            shark_sprite,
            enemy_sprite,
        })
    }
}

struct Enemy {
    pos: Vec2,
    size: Vec2,
    speed: Vec2,
    health: f32,
    damage: f32,
}

struct Player {
    pos: Vec2,
    size: Vec2,
    speed: Vec2,
    health: f32,
}

struct Wall {
    pos: Vec2,
    size: Vec2,
    solid: bool,
}

struct Projectile {
    pos: Vec2,
    size: Vec2,
    speed: Vec2,
}

struct GameState {
    start_time: Instant,
    alive: bool,
    player: Player,
    max_enemies: i32,
    walls: Vec<Wall>,
    enemies: Vec<Enemy>,
    projectiles: Vec<Projectile>,
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
    let mut gs = GameState {
        start_time: Instant::now(),
        alive: true,
        player: Player {
            pos: PLAYER_STARTING_POSITION,
            size: PLAYER_SIZE,
            speed: vec2(PLAYER_SPEED, PLAYER_SPEED),
            health: 100.0,
        },
        max_enemies: 100,
        walls: vec![],
        enemies: vec![],
        projectiles: vec![],
    };

    gs.projectiles.push(Projectile{
        pos: vec2(64.0, 64.0),
        size: vec2(32.0, 32.0),
        speed: vec2(1.0, 0.0),
    });

    gs.enemies.push(Enemy{
        pos: vec2(48.0, 48.0),
        size: vec2(32.0, 32.0),
        speed: vec2(1.0, 0.0),
        health: 100.0,
        damage: 0.0
    });

    let x = screen_width() / 2.0;
    let y = screen_height() / 2.0;
    let width = 800.0;
    let height = width * (y / x);

    let resources = Resources::new().await.unwrap();

    while gs.alive {
        clear_background(WHITE);

        draw_fps();

        draw_time(&gs.start_time);

        process_inputs(&mut gs);

        let handle_enemy_hit = |enemy: &Enemy| {
            println!("Enemy got HITITITITITITITITIT")
        };
        collide_check(&gs.projectiles.first().unwrap(), &gs.enemies, handle_enemy_hit);

        let camera = Camera2D::from_display_rect(Rect::new(
            gs.player.pos.x - (width / 2.0),
            gs.player.pos.y - (height / 2.0),
            width,
            height)
        );
        set_camera(&camera);

        draw_enemies(&gs, &resources);
        draw_projectiles(&gs, &resources);
        draw_player(&mut gs, &resources);

        update_health(&mut gs);

        next_frame().await
    }

    // scoreboard and close game

}

fn collide_check(projectile: &Projectile, enemies: &Vec<Enemy>, callback: fn(enemy: &Enemy)) {
    let collide = |pos: Vec2, collider: Vec2, collider_size: Vec2| -> bool{
            pos.x > collider.x &&
            pos.x < collider.x + collider_size.x &&
            pos.y > collider.y &&
            pos.y < collider.y + collider_size.y
    };

    for enemy in enemies{
        let corners: [Vec2; 4] = [
            vec2(projectile.pos.x, projectile.pos.y),
            vec2(projectile.pos.x + projectile.size.x, projectile.pos.y),
            vec2(projectile.pos.x, projectile.pos.y + projectile.size.y),
            vec2(projectile.pos.x + projectile.size.x, projectile.pos.y + projectile.size.y),
        ];

        if  collide(corners[0], enemy.pos, enemy.size) ||
            collide(corners[1], enemy.pos, enemy.size) ||
            collide(corners[2], enemy.pos, enemy.size) ||
            collide(corners[3], enemy.pos, enemy.size) {
            callback(enemy);
        }
    }
}

fn draw_fps() -> () {
    draw_text(&format!("{} FPS", get_fps()).to_string(), 20.0, 20.0, 20.0, DARKGRAY);
}

fn draw_time(start_time: &Instant) -> () {
    let playtime = start_time.elapsed();
    let seconds = playtime.as_secs() % 60;
    let minutes = (playtime.as_secs() / 60) % 60;
    let hours = (playtime.as_secs() / 60) / 60;
    let time_string = format!("{}:{}:{}", hours, minutes, seconds);
    draw_text(&time_string, 0., 0., 20.0, DARKGRAY);
}

fn update_health(gs: &mut GameState) -> () {
    if gs.player.health <= 0.0 {
        gs.alive = false;
    }
}

fn process_inputs(gs: &mut GameState){
    if is_key_down(KeyCode::D) {
        gs.player.pos.x += PLAYER_SPEED;
    }
    if is_key_down(KeyCode::A) {
        gs.player.pos.x -= PLAYER_SPEED;
    }
    if is_key_down(KeyCode::S) {
        gs.player.pos.y += PLAYER_SPEED;
    }
    if is_key_down(KeyCode::W) {
        gs.player.pos.y -= PLAYER_SPEED;
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

fn draw_enemies(gs: &GameState, rs: &Resources) {
    for enemy in gs.enemies.iter() {
        draw_texture(rs.enemy_sprite, enemy.pos.x, enemy.pos.y, WHITE);
    }
}

fn draw_projectiles(gs: &GameState, rs: &Resources) {
    for projectile in gs.projectiles.iter() {
        draw_texture(rs.shark_sprite, projectile.pos.x, projectile.pos.y, WHITE);
    }
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
