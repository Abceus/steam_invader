use ggez::*;
use ggez::graphics::*;
use ggez::event::*;
use ggez_egui::*;

struct State {
    dt: std::time::Duration,
    logo: Image,
    position: [f32; 2],
    rotation: f32,
    rotation_speed: f32,
    scale: f32,
    egui_backend: EguiBackend
}

impl ggez::event::EventHandler<GameError> for State {
  fn update(&mut self, ctx: &mut Context) -> GameResult {
    self.dt = timer::delta(ctx);
    self.rotation += (self.dt.as_millis() as f32) *self.rotation_speed/1000.0;
		let egui_ctx = self.egui_backend.ctx();
		egui::Window::new("egui-window").show(&egui_ctx, |ui| {
			ui.label("a very nice gui :3");
			if ui.button("print \"hello world\"").clicked() {
				println!("hello world");
			}
			if ui.button("quit").clicked() {
				quit(ctx);
			}
		});
    Ok(())
  }
  fn draw(&mut self, ctx: &mut Context) -> GameResult {
    graphics::clear(ctx, graphics::Color::BLACK);
    graphics::draw(ctx, &self.logo, graphics::DrawParam::default().dest(self.position).rotation(self.rotation).scale([self.scale, self.scale]).offset([0.5, 0.5]))?;
		graphics::draw(ctx, &self.egui_backend, ([0.0, 0.0],))?;
    graphics::present(ctx)?;
    Ok(())
  }

  fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, _x: f32, _y: f32) {
		self.egui_backend.input.mouse_button_down_event(button);
	}

	fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, _x: f32, _y: f32) {
		self.egui_backend.input.mouse_button_up_event(button);
	}

	fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
		self.egui_backend.input.mouse_motion_event(x, y);
	}
}

fn main() {
    let c = conf::Conf::new();
    let (mut ctx, event_loop) = ContextBuilder::new("hello_ggez", "awesome_person")
        .default_conf(c)
        .build()
        .unwrap();

    let state = State {
        dt: std::time::Duration::new(0, 0),
        logo: Image::new(
            &mut ctx,
            std::path::Path::new("/logo.png")
        ).unwrap(),
        position: [size(&ctx).0/2.0, size(&ctx).1/2.0],
        rotation: 0.0,
        rotation_speed: 5.0,
        scale: 0.4,
        egui_backend: EguiBackend::default(),
    };

    event::run(ctx, event_loop, state);
}
