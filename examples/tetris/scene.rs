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
use scion::core::components::material::Material;
use scion::core::components::ui::ui_input::UiInput;

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
        font_path: asset_path().join("rainyhearts.ttf").get(),
    };
    let font_asset = data.assets_mut().register_font(font);

    let mut input = UiInput::new(200, 200, font_asset.clone())
        .with_font_size(16)
        .with_font_color(Color::new_rgb(0,0,0));
    input.set_text("Coucou".to_string());

    data.push((
        input,
        Transform::from_xyz(394., 330., 2)
    ));

    let mut input2 = UiInput::new(200, 200, font_asset.clone())
        .with_font_size(16)
        .with_tab_index(13)
        .with_font_color(Color::new_rgb(0,0,0));
    input2.set_text("Coucou2".to_string());

    data.push((
        input2,
        Transform::from_xyz(394., 430., 2)
    ));

    let font2 = Font::Bitmap {
        texture_path: asset_path().join("font.png").get(),
        chars: "0123456789ACEOPRSULI".to_string(),
        texture_columns: 20.,
        texture_lines: 1.,
        width: 21.,
        height: 27.,
    };
    let font_asset_2 = data.assets_mut().register_font(font2);

    let txt = UiText::new("SCORE".to_string(), font_asset_2.clone());
    let transform = Transform::from_xyz(394., 250., 2);

    data.push((txt, transform));

    let txt = UiText::new("".to_string(), font_asset.clone())
        .sync_value(|res| res.get_resource::<TetrisResource>().unwrap().get_score()).with_font_size(32);

    let transform = Transform::from_xyz(394., 290., 2);
    data.push((txt, transform))
}

fn add_main_ui_mask(data: &mut GameData) {
    let path = asset_path().join("ui.png").get();
    let image = UiImage::new(544., 704.);

    let mut t = Transform::default();
    t.set_z(0);
    data.push((image, t, Material::Texture(path)));
}

fn add_ui_top_overflow(data: &mut GameData) {
    let path = asset_path().join("ui_overflow_top.png").get();
    let image = UiImage::new(324., 32.);

    let mut t = Transform::default();
    t.set_z(2);
    t.append_translation(32., 0.);
    data.push((image, t, Material::Texture(path)));
}
