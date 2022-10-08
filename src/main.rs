use macroquad::prelude::*;

#[macroquad::main("InputKeys")]
async fn main() {
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;


    let rustacean_tex = load_texture("assets/rustacean_happy.png").await.unwrap();
    rustacean_tex.set_filter(FilterMode::Nearest);

    loop {
        clear_background(LIGHTGRAY);

        if is_key_down(KeyCode::Right) {
            x += 1.0;
        }
        if is_key_down(KeyCode::Left) {
            x -= 1.0;
        }
        if is_key_down(KeyCode::Down) {
            y += 1.0;
        }
        if is_key_down(KeyCode::Up) {
            y -= 1.0;
        }


        draw_texture(rustacean_tex, x, y, YELLOW);

        // draw_circle(x, y, 15.0, YELLOW);
        draw_text("move the ball with arrow keys", 20.0, 20.0, 20.0, DARKGRAY);
        next_frame().await
    }
}
