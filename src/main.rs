mod taiyou_draw;
mod taiyou_fs;
mod taiyou_package;
mod taiyou_process;
use ggez::mint::Point2;
use ggez::{Context, ContextBuilder, GameError};
use ggez::conf::{FullscreenType, WindowMode};
use ggez::graphics::{self, Color, Rect};
use ggez::event::{self, EventHandler};
use taiyou_draw::{draw_shaded_rectangle};
use taiyou_process::ProcessManager;

fn main() {
  let (mut context, event_loop) = ContextBuilder::new("taiyou_runtime", "aragubas and taiyou contributors")
    .build()
    .expect("Could not create ggez context.");

  context.gfx.set_mode(WindowMode { width: 800.0, height: 600.0, maximized: false, fullscreen_type: FullscreenType::Windowed, borderless: false, transparent: false, min_width: 800.0, min_height: 600.0, max_width: 800.0, max_height: 600.0, resizable: false, visible: true, resize_on_scale_factor_change: false, logical_size: None } )
    .expect("Could not set window mode");
  context.gfx.set_window_title("Sunshine Taiyou");

  let app = App::new(&mut context);
  event::run(context, event_loop, app);

}

// App State
struct App {
  work_area: Point2<f32>,
  menu_bar_height: f32,
  process_manager: ProcessManager
}

impl App {
  pub fn new(context: &mut Context) -> App {
    // Initialization Logic
    println!("SunshineTaiyou Initialization");
    
    let menubar_height = 25.0;
    
    let mut process_manager = ProcessManager::new();
    process_manager.create_process(String::from("./TaiyouFS/Packages/taiyou.testapp/"));

    App { 
      menu_bar_height: 25.0, 
      work_area: Point2{ x: 800.0, y: (600.0 - 25.0) },
      process_manager: process_manager
    }
  }
}

impl EventHandler for App {
  fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
    // Update Loop
    Ok(())
  }

  fn draw(&mut self, context: &mut Context) -> Result<(), GameError> {
    let mut canvas = graphics::Canvas::from_frame(context, Color::from_rgb(179, 179, 218));
    
    // Draw Menubar
    draw_shaded_rectangle(context, &mut canvas, Rect{ x: 0.0, y: 0.0, w: 800.0, h: 25.0 });

    canvas.finish(context)
  }
}
