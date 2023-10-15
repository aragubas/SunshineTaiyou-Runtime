use ggez::context::Has;
use ggez::graphics::{Canvas, Color, Drawable, DrawMode, DrawParam, FillOptions, GraphicsContext, Mesh, Rect, Transform};
use ggez::mint::Point2;

pub fn draw_shaded_rectangle(context: &impl Has<GraphicsContext>, canvas: &mut Canvas, dest_rect: Rect) {
  let draw_params = DrawParam::new()
    .dest(Point2{ x: dest_rect.x, y: dest_rect.y });
  
  // Background
  let background = Mesh::new_rectangle(
    context,
    DrawMode::Fill(FillOptions::default()),
    Rect::new(0.0, 0.0, dest_rect.w, dest_rect.h),
    Color::from_rgb(231, 231, 231 )).expect("Could not create rectangle");
  background.draw(canvas, draw_params);
    
  let (mut origin, mut destination) = (Point2{ x: 0.0, y: 1.0 }, Point2{ x: dest_rect.w, y: 1.0 });
  
  // Top Border
  let top_line = Mesh::new_line(
    context,
    &[origin, destination],
    1.0,
    Color::WHITE
  ).expect("Could not create line");
  top_line.draw(canvas, draw_params);

  // Left Border
  origin = Point2{ x: 1.0, y: 1.0 };
  destination = Point2 { x: 1.0, y: dest_rect.h };
  let left_line = Mesh::new_line(
    context,
    &[origin, destination],
    1.0,
    Color::WHITE
  ).expect("Could not create line");
  left_line.draw(canvas, draw_params);

  // Right Border
  origin = Point2{ x: dest_rect.w, y: 0.0 };
  destination = Point2 { x: dest_rect.w, y: dest_rect.h };
  let right_line = Mesh::new_line(
    context,
    &[origin, destination],
    1.0,
    Color::from_rgb(179, 179, 179)
  ).expect("Could not create line");
  right_line.draw(canvas, draw_params);

  // Bottom Border
  origin = Point2{ x: 0.0, y: dest_rect.h - 1.0 };
  destination = Point2 { x: dest_rect.w, y: dest_rect.h - 1.0 };
  let bottom_line = Mesh::new_line(
    context,
    &[origin, destination],
    1.0,
    Color::from_rgb(179, 179, 179)
  ).expect("Could not create line");
  bottom_line.draw(canvas, draw_params);

  // Bottom Shadow
  origin = Point2{ x: 0.0, y: dest_rect.h };
  destination = Point2 { x: dest_rect.w, y: dest_rect.h };
  let bottom_shadow = Mesh::new_line(
    context,
    &[origin, destination],
    1.0,
    Color::BLACK
  ).expect("Could not create line");
  bottom_shadow.draw(canvas, draw_params);

}
