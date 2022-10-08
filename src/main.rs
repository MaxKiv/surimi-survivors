use macroquad::prelude::*;
use macroquad::prelude::animation::AnimatedSprite;
use macroquad::prelude::collections::storage;
use macroquad_platformer::{Actor, Solid, World};

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
    speed: Vec2,
    health: f32,
}

#[derive(Clone)]
struct Player {
    pos: Vec2,
    speed: Vec2,
    health: f32,
}

#[derive(Clone)]
struct Wall {
    collider: Solid,
    speed: f32
}

#[derive(Clone)]
struct Projectile {
    pos: Vec2,
    velocity: Vec2,
}

#[derive(Clone)]
struct GameState {
    player: Player,
    projectiles: Vec<Projectile>,
    enemies: Vec<Wall>,
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
    let mut gamestate = GameState{
        player: Player{
            pos: vec2(50.0, 80.0),
            speed: vec2(0., 0.),
            health: 100.0,
        },
        projectiles: Vec::new(),
        enemies: Vec::new(),
    };

    gamestate.projectiles.push(Projectile{
        pos: vec2(0., 0.),
        velocity: vec2(1., 1.),
    });

    let mut player = CrabPlayer {
        collider: world.add_actor(vec2(50.0, 80.0), 8, 8),
        speed: vec2(0., 0.),
        health: 100.0,
    };

    let wall = Wall{
        collider: world.add_solid(vec2(400.0, 130.0), 32, 8),
        speed: 50.,
    };

    while alive && game_running {
        clear_background(WHITE);

        // Draw "wall"
        {
            let pos = world.solid_pos(wall.collider);
            draw_texture(resources.crab_sprite, pos.x, pos.y, YELLOW);
        }

        draw_projectiles(&gamestate, &resources);

        draw_new_player(&gamestate, &resources);


        let mut player_pos = world.actor_pos(player.collider);
        process_inputs(&mut world, &mut player, &mut player_pos, &mut game_running);

        let collided_with_definitely_wall =  world.collide_check(player.collider, player_pos);

        if collided_with_definitely_wall {
            player.health -= 1.0;
        }

        draw_text(&format!("{} FPS", get_fps()).to_string(), 20.0, 20.0, 20.0, DARKGRAY);

        let camera = Camera2D::from_display_rect(Rect::new(
            player_pos.x - (width / 2.0),
            player_pos.y - (height / 2.0),
            width,
            height)
        );
        set_camera(&camera);

        //draw_player(&player, &player_pos, &resources);

        if player.health <= 0.0 {
            alive = false;
        }

        next_frame().await
    }

    // scoreboard and close game

}

fn draw_new_player(gs: &GameState, rs: &Resources) {
    draw_texture(rs.crab_sprite, gs.player.pos.x, gs.player.pos.y, YELLOW);
    draw_rectangle_lines(gs.player.pos.x, gs.player.pos.y, PLAYER_SIZE.x, PLAYER_SIZE.y, 5.0, BLACK);
}

fn draw_projectiles(gs: &GameState, rs: &Resources) {
    for p in gs.projectiles {
        draw_texture(rs.crab_sprite, gs.player.pos.x, gs.player.pos.y, RED);
    }
}

fn process_inputs(world: &mut World, player: &mut CrabPlayer, player_pos: &mut Vec2, game_running: &mut bool) {
    if is_key_down(KeyCode::D) {
        player_pos.x += PLAYER_SPEED;
        world.set_actor_position(player.collider, *player_pos);
    }
    if is_key_down(KeyCode::A) {
        player_pos.x -= PLAYER_SPEED; 
        world.set_actor_position(player.collider, *player_pos);
    }
    if is_key_down(KeyCode::S) {
        player_pos.y += PLAYER_SPEED;
        world.set_actor_position(player.collider, *player_pos);
    }
    if is_key_down(KeyCode::W) {
        player_pos.y -= PLAYER_SPEED;
        world.set_actor_position(player.collider, *player_pos);
    }
    if is_key_down(KeyCode::Escape) {
        *game_running = false;
    }
}

fn draw_player(player: &CrabPlayer, pos: &Vec2, resources: &Resources) {
    draw_texture(resources.crab_sprite, pos.x, pos.y, YELLOW);
    draw_rectangle_lines(pos.x, pos.y, PLAYER_SIZE.x, PLAYER_SIZE.y, 5.0, BLACK);
    draw_healthbar(&player, &pos);
}

fn draw_healthbar(player: &CrabPlayer, pos: &Vec2) {
    let health_scaling_factor = player.health / 100.0;
    let offset = vec2(PLAYER_SIZE.x / 2.0 - HEALTHBAR_SIZE.x / 2.0, PLAYER_SIZE.y + HEALTHBAR_SIZE.y / 2.0);
    draw_rectangle(pos.x + offset.x, pos.y + offset.y, health_scaling_factor * HEALTHBAR_SIZE.x, HEALTHBAR_SIZE.y, RED);
    draw_rectangle_lines(pos.x + offset.x, pos.y + offset.y, health_scaling_factor * HEALTHBAR_SIZE.x, HEALTHBAR_SIZE.y, 5.0, BLACK);
}

fn does_projectile_hit(projectile: &Projectile, target: &CrabPlayer){

}