use scion::core::scene::Scene;
use scion::core::world::{GameData, World};

#[derive(Default)]
struct MyScene;

impl Scene for MyScene{
    fn on_start(&mut self, data: &mut GameData) {
        data.add_default_camera();
        data.add_developer_console();
    }
}

fn main() {
    scion::Scion::app()
        .with_scene::<MyScene>()
        .run();
}