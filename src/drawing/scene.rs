use ggez::*;
use ggez::graphics::*;
use ggez::context::*;

pub mod object;

pub struct Scene {
    objects: std::vec::Vec<object::Object>
}

impl Scene {
    pub fn new() -> Scene {
        Scene { 
            objects: std::vec::Vec::new() 
        }
    }

    pub fn add_object(&mut self, new_object: object::Object) {
        self.objects.push(new_object);
    }

    pub fn update(&mut self, dt: f32) {
        for object in &mut self.objects {
            object.update(dt);
        }
    }
}

impl ggez::graphics::Drawable for Scene {
    fn draw(&self, ctx: &mut Canvas, param: impl Into<DrawParam>) {
        for object in &self.objects {
            object.drawable.draw(ctx, graphics::DrawParam::default().dest(object.position).rotation(object.rotation).scale([object.scale, object.scale]).offset([0.5, 0.5]));
        }
    }

    fn dimensions(&self, gfx: &impl Has<GraphicsContext>) -> Option<Rect> {
        return None;
    }
}