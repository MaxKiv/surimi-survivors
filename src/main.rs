use macroquad::prelude::*;
use macroquad::prelude::animation::AnimatedSprite;
use macroquad::prelude::collections::storage;
use macroquad_platformer::{Actor, Solid, World};

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
}

struct Wall {
    collider: Solid,
    speed: f32
}

#[macroquad::main("InputKeys")]
async fn main() {
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;
    let mut width = 800.0;
    let mut height = width * (y / x);
    let mut game_running = true;

    let resources = Resources::new().await.unwrap();

    let mut world = World::new();

    let mut player = CrabPlayer {
        collider: world.add_actor(vec2(50.0, 80.0), 8, 8),
        speed: vec2(0., 0.),
    };

    let mut wall = Wall{
        collider: world.add_solid(vec2(400.0, 130.0), 32, 8),
        speed: 50.,
    };

    // let camera = Camera2D::from_display_rect(Rect::new(
    //     (x / 2.0) - (width / 2.0),
    //     (y / 2.0) - (height / 2.0),
    //     width,
    //     height)
    // );

    while game_running {
        clear_background(LIGHTGRAY);




        {
            let pos = world.solid_pos(wall.collider);
            draw_texture(resources.crab_sprite, pos.x, pos.y, YELLOW);
        }

        if is_key_down(KeyCode::D) {
            x += 1.0;
        }
        if is_key_down(KeyCode::A) {
            x -= 1.0;
        }
        if is_key_down(KeyCode::S) {
            y += 1.0;
        }
        if is_key_down(KeyCode::W) {
            y -= 1.0;
        }
        if is_key_down(KeyCode::Escape) {
            game_running = false;
        }

        draw_text(&format!("{} FPS", get_fps()).to_string(), 20.0, 20.0, 20.0, DARKGRAY);


        let camera = Camera2D::from_display_rect(Rect::new(
            x - (width / 2.0),
            y - (height / 2.0),
            width,
            height)
        );
        set_camera(&camera);
        draw_texture(resources.crab_sprite, x, y, YELLOW);

        next_frame().await
    }
}
