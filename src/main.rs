use macroquad::prelude::*;

#[macroquad::main("Rust Pong")]
async fn main() {

    // プレイヤー位置
    let mut player_y = 200.0;

    // ボール位置
    let mut ball_x = 400.0;
    let mut ball_y = 200.0;

    // ボール速度
    let mut vel_x = -1.0;
    let mut vel_y = 1.0;

    loop {
        clear_background(BLACK);

        if is_key_down(KeyCode::Up) {
            player_y -= 5.0;
        }

        if is_key_down(KeyCode::Down) {
            player_y += 5.0;
        }

        ball_x += vel_x;
        ball_y += vel_y;

        // 壁バウンド
        if ball_y < 0.0 || ball_y > screen_height() {
            vel_y *= -1.0;
        }

        // ラケット衝突
        if ball_x < 40.0 && ball_y > player_y && ball_y < player_y + 100.0 {
          vel_x *= -1.0;
        }

        draw_rectangle(20.0, player_y, 20.0, 100.0, WHITE);

        draw_circle(ball_x, ball_y, 10.0, WHITE);

        next_frame().await;
    }
}
