#![allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]

use macroquad::prelude::*;
use macroquad_canvas_2d::Canvas2D;

const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 600.0;

fn win_config() -> Conf {
    Conf {
        window_title: "Coordinates usage".to_string(),
        window_width: WIDTH as i32,
        window_height: HEIGHT as i32,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(win_config)]
async fn main() {
    let mut canvas = Canvas2D::new(WIDTH, HEIGHT);

    loop {
        // Get canvas dimensions and padding
        let (left_padding, top_padding, dimensions) =
            canvas.calculate_size_and_padding(screen_width(), screen_height());

        // Move the camera with 'w' and 's'
        if is_key_down(KeyCode::W) {
            canvas.move_camera(0.0, 50.0 * get_frame_time());
        }
        if is_key_down(KeyCode::S) {
            canvas.move_camera(0.0, -50.0 * get_frame_time());
        }

        // Mouse position
        let (screen_mouse_x, screen_mouse_y) = mouse_position();
        let (canvas_mouse_x, canvas_mouse_y) = canvas.screen_coordinates_to_canvas_coordinates(
            screen_mouse_x,
            screen_mouse_y,
            left_padding,
            top_padding,
        );
        let (undo_mouse_x, undo_mouse_y) = canvas.canvas_coordinates_to_screen_coordinates(
            canvas_mouse_x,
            canvas_mouse_y,
            left_padding,
            top_padding,
        );

        // A usless code block for linting purposes
        {
            // Draw inside canvas
            canvas.set_camera();

            // Clear background
            clear_background(WHITE);

            for y in -10..20 {
                if y % 2 == 0 {
                    draw_rectangle(WIDTH / 2.0 - 25.0, y as f32 * 50.0, 50.0, 50.0, BLUE);
                } else {
                    draw_rectangle(WIDTH / 2.0 - 25.0, y as f32 * 50.0, 50.0, 50.0, GREEN);
                }
            }

            draw_circle(WIDTH / 2.0, HEIGHT / 2.0, 10.0, YELLOW);

            // Draw red lines
            draw_line(
                canvas_mouse_x,
                canvas_mouse_y - HEIGHT,
                canvas_mouse_x,
                canvas_mouse_y + HEIGHT,
                5.0,
                RED,
            );
            draw_line(0.0, canvas_mouse_y, WIDTH, canvas_mouse_y, 5.0, RED);
        }

        // Set default camera and clear background
        set_default_camera();
        clear_background(LIME);
        // Draw canvas on screen
        // You can use `canvas.draw_to_screen()` insted
        draw_texture_ex(
            canvas.get_texture(),
            left_padding,
            top_padding,
            WHITE,
            DrawTextureParams {
                dest_size: Some(dimensions),
                ..Default::default()
            },
        );

        // Draw text
        draw_text(
            &format!("Screen mouse: {screen_mouse_x} : {screen_mouse_y}"),
            80.0,
            50.0,
            24.0,
            BLACK,
        );
        draw_text(
            &format!("Canvas mouse: {canvas_mouse_x} : {canvas_mouse_y}"),
            80.0,
            100.0,
            24.0,
            BLACK,
        );
        draw_text(
            &format!("Undo mouse: {undo_mouse_x} : {undo_mouse_y}"),
            80.0,
            150.0,
            24.0,
            BLACK,
        );
        draw_text(
            "Press 'w' or 's' to move up or down",
            80.0,
            200.0,
            24.0,
            BLACK,
        );

        next_frame().await;
    }
}
