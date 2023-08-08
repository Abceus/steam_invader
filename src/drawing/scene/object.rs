use ggez::*;
use ggez::graphics::*;

pub mod component;

pub struct Object {
    pub drawable: ggez::graphics::Image,
    pub position: [f32; 2],
    pub rotation: f32,
    pub scale: f32,

    pub components: std::vec::Vec<Box<dyn component::Component>>
}

impl Object {
    pub fn new(drawable: ggez::graphics::Image) -> Object {
        return Object {
            drawable: drawable,
            position: [0.0, 0.0],
            rotation: 0.0,
            scale: 1.0,
            components: std::vec::Vec::new()
        };
    }

    pub fn update(&mut self, dt: f32) {
        // TODO: it's ok?
        let mut components = std::mem::replace(&mut self.components, Vec::new());
        for component in &mut components {
            component.update(self, dt);
        }
        self.components = components;
    }

    pub fn add_component(&mut self, new_component: Box<dyn component::Component>) {
        self.components.push(new_component);
    }
}