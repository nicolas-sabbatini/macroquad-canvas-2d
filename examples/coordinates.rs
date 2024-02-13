#![allow(clippy::cast_precision_loss)]
use macroquad::prelude::*;
use macroquad_canvas_2d::Canvas2D;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

fn win_config() -> Conf {
    Conf {
        window_title: "Coordinates usage".to_string(),
        window_width: WIDTH,
        window_height: HEIGHT,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(win_config)]
async fn main() {
    let canvas = Canvas2D::new(WIDTH as f32, HEIGHT as f32);

    loop {
        // Get canvas dimensions and padding
        let (left_padding, top_padding, dimensions) =
            canvas.calculate_size_and_padding(screen_width(), screen_height());

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

        // Draw inside canvas
        canvas.set_camera();
        {
            // Clear background
            clear_background(WHITE);
            draw_line(canvas_mouse_x, 0.0, canvas_mouse_x, HEIGHT as f32, 5.0, RED);

            draw_line(0.0, canvas_mouse_y, WIDTH as f32, canvas_mouse_y, 5.0, RED);

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
        }
        set_default_camera();

        clear_background(BLACK);

        // Draw canvas on screen
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

        next_frame().await;
    }
}
