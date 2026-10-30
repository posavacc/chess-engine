use macroquad::prelude::*;

#[macroquad::main(window_conf())]
async fn main() {
    loop {
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() { break; }

        clear_background(BLACK);

        let sqr_num = if screen_width() < screen_height() { screen_width() } else { screen_height() } / 8.0;
        for i in 0..8  {
            for j in 0..8 {
                if (i + j) % 2 == 0 {
                    draw_rectangle(sqr_num * j as f32, sqr_num * i as f32, sqr_num, sqr_num, WHITE);
                } else {
                    draw_rectangle(sqr_num * j as f32, sqr_num * i as f32, sqr_num, sqr_num, GRAY);
                }
            }
        }

        next_frame().await;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "particle simulation".to_string(),
        window_width: 600,
        window_height: 600,
        window_resizable: false,
        platform: miniquad::conf::Platform {
           swap_interval: Some(1),
            ..Default::default()
        },
        ..Default::default()
    }
}
