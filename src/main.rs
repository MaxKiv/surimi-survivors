use std::time::{Duration, Instant};
mod enemy;
use macroquad::prelude::*;

pub const SCREEN_SIZE: Vec2 = vec2(1000., 1000.);
pub const PLAYER_SPEED: f32 = 5.0;
pub const PLAYER_STARTING_POSITION: Vec2 = vec2(0., 0.);
pub const PLAYER_SIZE: Vec2 = vec2(32.0, 32.0);
pub const HEALTHBAR_SIZE: Vec2 = vec2(50.0, 10.0);
pub const ENEMY_HEALTHBAR_ENABLED: bool = true;

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
    alive: bool,
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
    weapon_cooldown: f32,
    weapon_last_fired: f32,
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
        weapon_cooldown: 1.0,
        weapon_last_fired: 0.0,
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

    // gs.projectiles.push(Projectile{
    //     pos: vec2(64.0, 64.0),
    //     size: vec2(32.0, 32.0),
    //     speed: vec2(1.5, 0.75),
    // });

    for i in 1..1000 {
        gs.enemies.push(Enemy{
            pos: vec2(48.0 * ((i % 10) as f32), 128.0 + ((i / 10) as f32) * 48.0),
            size: vec2(32.0, 32.0),
            speed: vec2(1.0, 0.0),
            health: 100.0,
            damage: 0.0,
            alive: true,
        });
    }



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

        let handle_enemy_hit = |enemy: &mut Enemy| {
            enemy.alive = false;
        };

        //collide_check_player(&gs.player, &mut gs.enemies, handle_enemy_hit);
        for projectile in gs.projectiles.iter() {
            collide_check_projectile(projectile, &mut gs.enemies, handle_enemy_hit);
        }

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
        update_projectiles(&mut gs.projectiles);

        next_frame().await
    }
}

fn update_projectiles(ps: &mut Vec<Projectile>) {
    for projectile in ps {
        projectile.pos += projectile.speed
    }
}

fn collide_check_player(player: &Player, enemies: &mut Vec<Enemy>, callback: fn(&mut Enemy)) {
    for enemy in enemies{
        if collide_check(player.pos, player.size, enemy.pos, enemy.size) {
            callback(enemy)
        }
    }
}

fn collide_check_projectile(projectile: &Projectile, enemies: &mut Vec<Enemy>, callback: fn(&mut Enemy)) {
    for enemy in enemies{
        if collide_check(projectile.pos, projectile.size, enemy.pos, enemy.size) {
            callback(enemy)
        }
    }
}

fn collide_check(pos: Vec2, size: Vec2, other_pos: Vec2, other_size: Vec2) -> bool {
    let collide = |pos: Vec2, collider: Vec2, collider_size: Vec2| -> bool{
        pos.x > collider.x &&
            pos.x < collider.x + collider_size.x &&
            pos.y > collider.y &&
            pos.y < collider.y + collider_size.y
    };

    let corners: [Vec2; 4] = [
        vec2(pos.x, pos.y),
        vec2(pos.x + size.x, pos.y),
        vec2(pos.x, pos.y + size.y),
        vec2(pos.x + size.x, pos.y + size.y),
    ];

    if  collide(corners[0], other_pos, other_size) ||
        collide(corners[1], other_pos, other_size) ||
        collide(corners[2], other_pos, other_size) ||
        collide(corners[3], other_pos, other_size) {
        return true;
    }
    return false;
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
    if is_key_down(KeyCode::Space)  {
        gs.projectiles.push(Projectile{
            pos: gs.player.pos,
            size: vec2(32.0, 32.0),
            speed: vec2(10., 0.),
        })
    }
}

fn draw_player(gs: &mut GameState, resources: &Resources) {
    draw_texture(resources.crab_sprite, gs.player.pos.x, gs.player.pos.y, YELLOW);
    // draw collision box
    draw_collision_box(gs);
    draw_healthbar_player(gs);
}

fn draw_enemies(gs: &GameState, rs: &Resources) {
    for enemy in gs.enemies.iter() {
        if enemy.alive {
            draw_texture(rs.enemy_sprite, enemy.pos.x, enemy.pos.y, WHITE);
            if ENEMY_HEALTHBAR_ENABLED { draw_healthbar_enemy(enemy); }
        }
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
        GRAY,
    );
}

fn draw_healthbar(pos: Vec2, offset: Vec2, percentage: f32) {
    draw_rectangle(
        pos.x + offset.x,
        pos.y + offset.y,
        percentage * HEALTHBAR_SIZE.x,
        HEALTHBAR_SIZE.y,
        RED
    );
    draw_rectangle_lines(
        pos.x + offset.x,
        pos.y + offset.y,
        percentage * HEALTHBAR_SIZE.x,
        HEALTHBAR_SIZE.y,
        5.0,
        BLACK
    );
}

fn draw_healthbar_player(gs: &mut GameState) {
    let health_scaling_factor = gs.player.health / 100.0;
    let offset = vec2(
        PLAYER_SIZE.x / 2.0 - HEALTHBAR_SIZE.x / 2.0,
        PLAYER_SIZE.y + HEALTHBAR_SIZE.y / 2.0
    );
    draw_healthbar(gs.player.pos, offset, health_scaling_factor);
}

fn draw_healthbar_enemy(enemy: &Enemy) {
        let health_scaling_factor = enemy.health / 100.0;
        let offset = vec2(
            PLAYER_SIZE.x / 2.0 - HEALTHBAR_SIZE.x / 2.0,
            PLAYER_SIZE.y + HEALTHBAR_SIZE.y / 2.0,
        );
        draw_healthbar(enemy.pos, offset, health_scaling_factor);
}
