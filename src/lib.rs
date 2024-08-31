/*
  Copyright 2024 Nicolas Cesar Sabbatini Vrech

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

use macroquad::prelude::*;

pub struct Canvas2D {
    camera: Camera2D,
    width: f32,
    height: f32,
}

impl Canvas2D {
    /// Create a new canvas with the given width and height.
    #[must_use]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn new(width: f32, height: f32) -> Self {
        let mut camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, width, height));
        camera.render_target = Some(render_target(width as u32, height as u32));
        // Temp fix or maybe I am doing something wrong
        // https://github.com/not-fl3/macroquad/issues/171#issuecomment-880601087
        camera.zoom.y = -camera.zoom.y;
        Canvas2D {
            camera,
            width,
            height,
        }
    }

    /// Get width.
    #[must_use]
    pub fn width(&self) -> f32 {
        self.width
    }

    /// Get height.
    #[must_use]
    pub fn height(&self) -> f32 {
        self.height
    }

    /// Get width and height.
    #[must_use]
    pub fn width_height(&self) -> (f32, f32) {
        (self.width, self.height)
    }

    /// Get a reference of the canvas texture.
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn get_texture(&self) -> &Texture2D {
        &self.camera.render_target.as_ref().unwrap().texture
    }

    /// Get a mutable reference of the canvas texture.
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn get_texture_mut(&mut self) -> &mut Texture2D {
        &mut self.camera.render_target.as_mut().unwrap().texture
    }

    /// Set the canvas as te default camera to draw
    /// if you want to draw to the screen you should call.
    /// `macroquad::camera::set_default_camera()`
    pub fn set_camera(&self) {
        set_camera(&self.camera);
    }

    /// Calculate size and padding of the canvas so it can fit inside
    /// of the target and its position is in the center.
    #[must_use]
    pub fn calculate_size_and_padding(
        &self,
        target_width: f32,
        target_height: f32,
    ) -> (f32, f32, Vec2) {
        let new_size: Vec2 = self.calculate_size(target_width, target_height);

        // Calculate padding
        let left_padding: f32 = (target_width - new_size.x) / 2.0;
        let top_padding: f32 = (target_height - new_size.y) / 2.0;

        (left_padding, top_padding, new_size)
    }

    /// Calculate size of the canvas so it can fit inside of the target
    /// respecting the aspect ratio.
    #[must_use]
    pub fn calculate_size(&self, target_width: f32, target_height: f32) -> Vec2 {
        let min_scale_factor: f32 = self.calculate_min_scale_factor(target_width, target_height);

        // Calculate windows new size
        let new_width: f32 = self.width * min_scale_factor;
        let new_height: f32 = self.height * min_scale_factor;

        Vec2::new(new_width, new_height)
    }

    /// Calculate the minimum scale factor so the canvas can fit inside of the target
    /// respecting the aspect ratio of the canvas.
    #[must_use]
    pub fn calculate_min_scale_factor(&self, target_width: f32, target_height: f32) -> f32 {
        let (scale_factor_w, scale_factor_h) =
            self.calculate_scale_factor(target_width, target_height);
        f32::min(scale_factor_w, scale_factor_h)
    }

    /// Calculate scale factor so the canvas can fit inside of the target.
    #[must_use]
    pub fn calculate_scale_factor(&self, target_width: f32, target_height: f32) -> (f32, f32) {
        (target_width / self.width, target_height / self.height)
    }

    /// Convert from the parent coordinates to canvas coordinates.
    ///
    /// Warning it can return negative numbers or values grater than the canvas
    /// when the mouse is outside of the canvas.
    #[must_use]
    pub fn parent_coordinates_to_canvas_coordinates(
        &self,
        parent_width: f32,
        parent_height: f32,
        screen_x: f32,
        screen_y: f32,
        offset_x: f32,
        offset_y: f32,
    ) -> (f32, f32) {
        let scale_factor = self.calculate_min_scale_factor(parent_width, parent_height);

        let camera_offset = self.camera.offset / self.camera.zoom;

        let x = (screen_x - offset_x) / scale_factor - camera_offset.x;
        let y = (screen_y - offset_y) / scale_factor - camera_offset.y;
        (x, y)
    }

    /// Convert from the canvas coordinates to parent coordinates.
    ///
    /// Warning do to float division it can be a small margin of error.
    #[must_use]
    pub fn canvas_coordinates_to_parent_coordinates(
        &self,
        parent_width: f32,
        parent_height: f32,
        canvas_x: f32,
        canvas_y: f32,
        offset_x: f32,
        offset_y: f32,
    ) -> (f32, f32) {
        let scale_factor = self.calculate_min_scale_factor(parent_width, parent_height);
        let camera_offset = self.camera.offset / self.camera.zoom;

        let x = ((canvas_x + camera_offset.x) * scale_factor) + offset_x;
        let y = ((canvas_y + camera_offset.y) * scale_factor) + offset_y;
        (x, y)
    }

    /// A wrapper around the `parent_to_canvas` for better ergonomic.
    /// Convert from the screen coordinates to canvas coordinates.
    ///
    /// Warning it can return negative numbers or values grater than the canvas
    /// when the mouse is outside of the canvas.
    #[must_use]
    pub fn screen_coordinates_to_canvas_coordinates(
        &self,
        screen_x: f32,
        screen_y: f32,
        offset_x: f32,
        offset_y: f32,
    ) -> (f32, f32) {
        self.parent_coordinates_to_canvas_coordinates(
            screen_width(),
            screen_height(),
            screen_x,
            screen_y,
            offset_x,
            offset_y,
        )
    }

    /// A wrapper around the `canvas_to_parent` for better ergonomic.
    /// Convert from the canvas coordinates to screen coordinates.
    ///
    /// Warning do to float division it can be a small margin of error.
    #[must_use]
    pub fn canvas_coordinates_to_screen_coordinates(
        &self,
        canvas_x: f32,
        canvas_y: f32,
        offset_x: f32,
        offset_y: f32,
    ) -> (f32, f32) {
        self.canvas_coordinates_to_parent_coordinates(
            screen_width(),
            screen_height(),
            canvas_x,
            canvas_y,
            offset_x,
            offset_y,
        )
    }

    /// Get the mouse position in canvas coordinates.
    ///
    /// Warning it can return negative numbers or values grater than the canvas
    #[must_use]
    pub fn screen_mouse_position_to_canvas(&self, offset_x: f32, offset_y: f32) -> (f32, f32) {
        let (x, y) = mouse_position();
        self.screen_coordinates_to_canvas_coordinates(x, y, offset_x, offset_y)
    }

    /// Draws the canvas to the middle of the screen, keeping the aspect ratio.
    /// It calls `set_default_camera` before drawing.
    pub fn draw_to_screen(&self) {
        set_default_camera();
        // Get canvas dimensions and padding
        let (left_padding, top_padding, dimensions) =
            self.calculate_size_and_padding(screen_width(), screen_height());

        // Draw canvas on screen
        draw_texture_ex(
            self.get_texture(),
            left_padding,
            top_padding,
            WHITE,
            DrawTextureParams {
                dest_size: Some(dimensions),
                ..Default::default()
            },
        );
    }

    /// Zoom in/out the camera.
    pub fn zoom(&mut self, zoom: f32) {
        self.camera.zoom += zoom;
    }

    /// Set the camara Zoom.
    pub fn set_zoom(&mut self, zoom: f32) {
        self.camera.zoom = vec2(zoom, zoom);
    }

    /// Rotate the camera by a factor.
    /// The angle is in degrees.
    pub fn rotate(&mut self, angle: f32) {
        self.camera.rotation += angle;
    }

    /// Set the camara rotation.
    /// The angle is in degrees.
    pub fn set_rotation(&mut self, angle: f32) {
        self.camera.rotation = angle;
    }

    /// Move the camera by a amount in pixels.
    pub fn move_camera_by(&mut self, x_amount: f32, y_amount: f32) {
        let offset = vec2(x_amount * self.camera.zoom.x, y_amount * self.camera.zoom.y);
        self.camera.offset += offset;
    }

    /// Move the camera to the given position.
    pub fn move_camera_to(&mut self, x_position: f32, y_position: f32) {
        let offset = vec2(
            x_position * self.camera.zoom.x,
            y_position * self.camera.zoom.y,
        );
        self.camera.offset = offset;
    }
}
