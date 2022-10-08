use std::time::{Duration, Instant};
use macroquad::prelude::*;
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

struct Enemy {
    collider: Actor,
    speed: Vec2,
    health: f32,
    damage: f32,
}

struct CrabPlayer {
    collider: Actor,
    pos: Vec2,
    speed: Vec2,
    health: f32,
}

struct Wall {
    collider: Solid,
}

struct GameState {
    alive: bool,
    player: CrabPlayer,
    max_enemies: i32,
    enemies: Vec<Enemy>,
    playtime: Duration,
}

#[macroquad::main("InputKeys")]
async fn main() {
    let mut world = World::new();

    let mut gamestate = GameState {
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

    // let wall = Wall{
    //     collider: world.add_solid(vec2(400.0, 130.0), 32, 8),
    // };

    while gamestate.alive {
        clear_background(WHITE);

        draw_fps();
        draw_time(&gamestate);

        // // Draw "wall"
        // {
        //     let pos = world.solid_pos(wall.collider);
        //     draw_texture(resources.crab_sprite, pos.x, pos.y, YELLOW);
        // }
        // draw_projectiles(&gamestate, &resources);



        let player_pos = world.actor_pos(gamestate.player.collider);
        process_inputs(&mut world, &mut gamestate);

        // let collided_with_definitely_wall =  world.collide_check(player.collider, player_pos);

        // if collided_with_definitely_wall {
        //     player.health -= 1.0;
        // }

        let camera = Camera2D::from_display_rect(Rect::new(
            player_pos.x - (width / 2.0),
            player_pos.y - (height / 2.0),
            width,
            height)
        );
        set_camera(&camera);

        // draw_enemies(&world, &resources);
        // draw single enemy

        draw_player(&mut gamestate, &resources);

        update_health(&mut gamestate);

        next_frame().await
    }

    // scoreboard and close game

}

fn draw_fps() -> () {
    draw_text(&format!("{} FPS", get_fps()).to_string(), 20.0, 20.0, 20.0, DARKGRAY);
}

fn draw_time(gamestate: &GameState) -> () {
    let seconds = gamestate.playtime.as_secs() % 60;
    let minutes = (gamestate.playtime.as_secs() / 60) % 60;
    let hours = (gamestate.playtime.as_secs() / 60) / 60;
    let time_string = format!("{}:{}:{}", hours, minutes, seconds);
    draw_text(&time_string, 0., 0., 20.0, DARKGRAY);
}

fn update_health(gamestate: &mut GameState) -> () {
    if gamestate.player.health <= 0.0 {
        gamestate.alive = false;
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

fn process_inputs(world: &mut World, gamestate: &mut GameState){
    if is_key_down(KeyCode::D) {
        gamestate.player.pos.x += PLAYER_SPEED;
        world.set_actor_position(gamestate.player.collider, gamestate.player.pos);
    }
    if is_key_down(KeyCode::A) {
        gamestate.player.pos.x -= PLAYER_SPEED;
        world.set_actor_position(gamestate.player.collider, gamestate.player.pos);
    }
    if is_key_down(KeyCode::S) {
        gamestate.player.pos.y += PLAYER_SPEED;
        world.set_actor_position(gamestate.player.collider, gamestate.player.pos);
    }
    if is_key_down(KeyCode::W) {
        gamestate.player.pos.y -= PLAYER_SPEED;
        world.set_actor_position(gamestate.player.collider, gamestate.player.pos);
    }
    if is_key_down(KeyCode::Escape) {
        gamestate.alive = false;
    }
}

fn draw_player(gamestate: &mut GameState, resources: &Resources) {
    draw_texture(resources.crab_sprite, gamestate.player.pos.x, gamestate.player.pos.y, YELLOW);
    // draw collision box
    draw_collision_box(gamestate);
    draw_healthbar(gamestate);
}

fn draw_collision_box(gamestate: &mut GameState) -> () {
    draw_rectangle_lines(
        gamestate.player.pos.x,
        gamestate.player.pos.y,
        PLAYER_SIZE.x,
        PLAYER_SIZE.y,
        5.0,
        GRAY
    );
}

fn draw_healthbar(gamestate: &mut GameState) {
    let health_scaling_factor = gamestate.player.health / 100.0;
    let offset = vec2(
        PLAYER_SIZE.x / 2.0 - HEALTHBAR_SIZE.x / 2.0,
        PLAYER_SIZE.y + HEALTHBAR_SIZE.y / 2.0
    );
    draw_rectangle(
        gamestate.player.pos.x + offset.x,
        gamestate.player.pos.y + offset.y,
        health_scaling_factor * HEALTHBAR_SIZE.x,
        HEALTHBAR_SIZE.y,
        RED
    );
    draw_rectangle_lines(
        gamestate.player.pos.x + offset.x,
        gamestate.player.pos.y + offset.y,
        health_scaling_factor * HEALTHBAR_SIZE.x,
        HEALTHBAR_SIZE.y,
        5.0,
        BLACK
    );
}
















// fn spawn_location_factory(()) -> Vec2 {
// }

// fn does_projectile_hit(projectile: &Projectile, target: &CrabPlayer){
//
// }