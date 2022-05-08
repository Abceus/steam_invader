use ggez::*;
use ggez::graphics::*;
use ggez::event::*;
use ggez_egui::*;

mod drawing;
use drawing::scene::*;

struct State {
    dt: std::time::Duration,
    egui_backend: EguiBackend,
    scene: drawing::scene::Scene
}

impl ggez::event::EventHandler<GameError> for State {
  fn update(&mut self, ctx: &mut Context) -> GameResult {
    self.dt = timer::delta(ctx);
    self.scene.update(self.dt.as_millis() as f32);
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
    graphics::draw(ctx, &self.scene, ([0.0, 0.0],))?;
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

    let mut state = State {
        dt: std::time::Duration::new(0, 0),
        rotation_speed: 5.0,
        egui_backend: EguiBackend::default(),

        scene: drawing::scene::Scene::new()
    };

    let mut object1 = crate::drawing::scene::object::Object::new(Box::new(Image::new(
            &mut ctx,
            std::path::Path::new("/logo.png")
        ).unwrap()));

    object1.add_component(Box::new(drawing::scene::object::component::test_component::TestComponent::new(5.0)));
    object1.position = [size(&ctx).0/2.0, size(&ctx).1/2.0];
    object1.scale = 0.4;
    state.scene.add_object(object1);

    let mut object2 = crate::drawing::scene::object::Object::new(Box::new(Image::new(
            &mut ctx,
            std::path::Path::new("/logo.png")
        ).unwrap()));

    object2.add_component(Box::new(drawing::scene::object::component::test_component::TestComponent::new(-2.0)));
    object2.position = [size(&ctx).0/4.0, size(&ctx).1/4.0];
    object2.scale = 0.2;
    state.scene.add_object(object2);

    event::run(ctx, event_loop, state);
}
