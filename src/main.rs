use ggez::*;
use ggez::graphics::*;
use ggez::event::*;

mod drawing;
use drawing::scene::*;
use std::{env, path};

struct State {
    dt: std::time::Duration,
    rotation_speed: f32,
    scene: drawing::scene::Scene
}

impl ggez::event::EventHandler<GameError> for State {
  fn update(&mut self, ctx: &mut Context) -> GameResult {
    self.dt = timer::delta(ctx);
    self.scene.update(self.dt.as_millis() as f32);
    Ok(())
  }
  fn draw(&mut self, ctx: &mut Context) -> GameResult {
    let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);
    self.scene.draw(&mut canvas, graphics::DrawParam::new());
    //self.egui_backend.draw(&mut canvas, graphics::DrawParam::new());
    canvas.finish(ctx)?;
    Ok(())
  }
}

fn main() {

  let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
      let mut path = path::PathBuf::from(manifest_dir);
      path.push("resources");
      path
  } else {
      path::PathBuf::from("./resources")
  }; 

    let c = conf::Conf::new();
    let (mut ctx, event_loop) = ContextBuilder::new("hello_ggez", "awesome_person")
        .default_conf(c)
        .add_resource_path(resource_dir)
        .build()
        .unwrap();

    let mut state = State {
        dt: std::time::Duration::new(0, 0),
        rotation_speed: 5.0,
        scene: drawing::scene::Scene::new()
    };

    let mut object1 = crate::drawing::scene::object::Object::new(Image::from_path(
            &mut ctx, "/logo.png").unwrap());

    object1.add_component(Box::new(drawing::scene::object::component::test_component::TestComponent::new(5.0)));
    object1.position = [ctx.gfx.size().0/2.0, ctx.gfx.size().1/2.0];
    object1.scale = 0.4;
    state.scene.add_object(object1);

    let mut object2 = crate::drawing::scene::object::Object::new(Image::from_path(
            &mut ctx, "/logo.png").unwrap());

    object2.add_component(Box::new(drawing::scene::object::component::test_component::TestComponent::new(-2.0)));
    object2.position = [ctx.gfx.size().0/4.0, ctx.gfx.size().1/4.0];
    object2.scale = 0.2;
    state.scene.add_object(object2);

    event::run(ctx, event_loop, state);
}
