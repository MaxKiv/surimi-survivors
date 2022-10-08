use macroquad::prelude::*;
use macroquad::prelude::animation::AnimatedSprite;
use macroquad::prelude::collections::storage;
use macroquad_platformer::Actor;

struct Resources {
    crab_sprite: Texture2D,
}

impl Resources {
    async fn new() -> Result<Resources, macroquad::prelude::FileError> {
        let crab_sprite = load_texture("assets/rustacean_happy.png").await?;

        Ok(Resources{
            crab_sprite
        })
    }
}

// struct CrabPlayer {
//     collider: Actor,
//     speed: Vec2,
// }
//
// impl CrabPlayer {
//     pub const MOVE_SPEED: f32 = 300.0;
//
//     fn new() -> CrabPlayer {
//         let mut resources = storage::get_mut::<Resources>().unwrap();
//
//         CrabPlayer {
//             collider: resources.physics.add
//         }
//     }
// }

#[macroquad::main("InputKeys")]
async fn main() {
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;
    let mut game_running = true;

    let resources = Resources::new().await.unwrap();
    // storage::store(resources);

    while game_running {
        clear_background(LIGHTGRAY);

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

        draw_texture(resources.crab_sprite, x, y, YELLOW);

        next_frame().await
    }
}
