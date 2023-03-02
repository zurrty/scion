use hecs::Entity;
use scion::core::world::{GameData, World};
use scion::core::{
    components::{
        maths::transform::Transform,
        tiles::tileset::Tileset,
        ui::{font::Font, ui_image::UiImage, ui_text::UiText},
    },
    resources::time::TimerType,
    scene::Scene,
};
use scion::core::components::color::Color;
use scion::core::resources::asset_manager::AssetManager;

use crate::{asset_path, resources::TetrisResource};

#[derive(Default)]
pub struct MainScene {
    score: Option<Entity>,
}

impl Scene for MainScene {
    fn on_start(&mut self, data: &mut GameData) {
        add_main_ui_mask(data);
        add_ui_top_overflow(data);
        self.score = Some(add_score_ui(data));
        data.add_default_camera();
        let _r = data.timers().add_timer("piece", TimerType::Cyclic, 0.5);
        let _r = data.timers().add_timer("action_reset_timer", TimerType::Manual, 0.2);
        let mut tetris = TetrisResource::default();
        tetris.asset = Some(data.assets_mut().register_tileset(Tileset::new(
            asset_path().join("blocs.png").get(),
            8,
            1,
            32,
        )));
        data.insert_resource(tetris);
    }
}

fn add_score_ui(data: &mut GameData) -> Entity {
    // First we add an UiText to the world
    let font = Font::TrueType {
        font_path: asset_path().join("sourcecodepro.ttf").get(),
    };
    let font_asset = data.assets_mut().register_font(font);

    let txt = UiText::new("Whereas recognition of the inherent dignity".to_string(), font_asset.clone()).with_font_size(16).with_font_color(Color::new_rgb(255,255,255));
    let mut transform = Transform::from_xyz(394., 250., 2);

    data.push((txt, transform));

    let txt = UiText::new("".to_string(), font_asset)
        .sync_value(|res| res.get_resource::<TetrisResource>().unwrap().get_score()).with_font_size(16);

    let mut transform = Transform::from_xyz(394., 290., 2);
    data.push((txt, transform))
}

fn add_main_ui_mask(data: &mut GameData) {
    let path = asset_path().join("ui.png").get();
    let image = UiImage::new(544., 704., path);

    let mut t = Transform::default();
    t.set_z(0);
    data.push((image, t));
}

fn add_ui_top_overflow(data: &mut GameData) {
    let path = asset_path().join("ui_overflow_top.png").get();
    let image = UiImage::new(324., 32., path);

    let mut t = Transform::default();
    t.set_z(2);
    t.append_translation(32., 0.);
    data.push((image, t));
}
