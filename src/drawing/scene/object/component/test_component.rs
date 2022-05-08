pub struct TestComponent {
    rotation_speed: f32
}

impl TestComponent {
    pub fn new(rotation_speed: f32) -> TestComponent {
        return TestComponent {
            rotation_speed: rotation_speed
        };
    }
}

impl crate::drawing::scene::object::component::Component for TestComponent {
    fn update(&mut self, owner: &mut crate::drawing::scene::object::Object, dt: f32) {
        owner.rotation += dt *self.rotation_speed/1000.0;
    }
}