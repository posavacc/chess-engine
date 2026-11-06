use macroquad::prelude::*;
use Piece::*;

#[derive(Copy, Clone)]
enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[macroquad::main(window_conf())]
async fn main() {
    let board: [Option<Piece>; 64] = [None; 64];

    let pieces_sprite = load_texture("sprites/pieces.png").await.unwrap();

    loop {
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() { break; }

        clear_background(BLACK);

        let sqr_width = if screen_width() < screen_height() { screen_width() } else { screen_height() } / 8.0;
        for i in 0..8  {
            for j in 0..8 {
                if (i + j) % 2 == 0 {
                    draw_rectangle(sqr_width * j as f32, sqr_width * i as f32, sqr_width, sqr_width, WHITE);
                } else {
                    draw_rectangle(sqr_width * j as f32, sqr_width * i as f32, sqr_width, sqr_width, GRAY);
                }
            }
        }

        draw_piece(&pieces_sprite, King, vec2(0.0, 0.0), false);

        next_frame().await;
    }
}

fn draw_piece(sprite_sheet: &Texture2D, piece: Piece, pos: Vec2, white: bool) {
    let cell_w = 2000.0 / 6.0;
    let cell_h = 667.0  / 2.0;

    let col = match piece {
        Pawn   => 5,
        Knight => 3,
        Bishop => 2,
        Rook   => 4,
        Queen  => 1,
        King   => 0,
    };

    let row = if white { 0 } else { 1 };

    let size = screen_width() / 8.0;

    draw_texture_ex(sprite_sheet, pos.x, pos.y, WHITE, DrawTextureParams {
        dest_size: Some(vec2(size, size)),
        source: Some(Rect::new(col as f32 * cell_w, row as f32 * cell_h, cell_w, cell_h)),
        ..Default::default()
    })
}

fn window_conf() -> Conf {
    Conf {
        window_title: "particle simulation".to_string(),
        window_width: 700,
        window_height: 700,
        window_resizable: false,
        platform: miniquad::conf::Platform {
           swap_interval: Some(1),
            ..Default::default()
        },
        ..Default::default()
    }
}
