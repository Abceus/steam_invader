pub mod test_component;

pub trait Component {
    fn update(&mut self, owner: &mut crate::drawing::scene::object::Object, dt: f32);
}