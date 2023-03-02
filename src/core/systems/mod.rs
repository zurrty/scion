use crate::core::components::material::Material;
use crate::core::components::ui::ui_image::UiImage;
use crate::core::components::ui::ui_text::{UiText, UiTextImage};
use crate::core::package::Package;

use crate::core::resources::events::topic::TopicConfiguration;
use crate::core::resources::events::Events;

use crate::core::resources::time::{Time, TimerType, Timers};

use crate::ScionBuilder;

use crate::core::resources::asset_manager::AssetManager;
use crate::core::resources::audio::Audio;
use crate::core::resources::font_atlas::FontAtlas;
use crate::core::resources::inputs::inputs_controller::InputsController;
use crate::core::scene::SceneController;
use crate::core::state::GameState;
use crate::core::systems::animations_system::animation_executer_system;
use crate::core::systems::asset_ref_resolver_system::asset_ref_resolver_system;
use crate::core::systems::asset_ref_resolver_system::MaterialAssetResolverFn;
use crate::core::systems::collider_systems::{compute_collisions_system, debug_colliders_system};
use crate::core::systems::default_camera_system::camera_dpi_system;
use crate::core::systems::default_camera_system::default_camera_system;
use crate::core::systems::developer_console_system::developer_console_system;
use crate::core::systems::hide_propagation_system::{
    hide_propagated_deletion_system, hide_propagation_system,
};
use crate::core::systems::hierarchy_system::children_manager_system;
use crate::core::systems::missing_ui_component_system::missing_ui_component_system;
use crate::core::systems::parent_transform_system::{dirty_child_system, dirty_transform_system};
use crate::core::systems::ui_text_system::{sync_text_value_system, ui_text_bitmap_update_system};
use crate::core::world::GameData;

pub(crate) mod animations_system;
pub(crate) mod asset_ref_resolver_system;
pub(crate) mod collider_systems;
pub(crate) mod default_camera_system;
pub(crate) mod hide_propagation_system;
pub(crate) mod hierarchy_system;
pub(crate) mod missing_ui_component_system;
pub(crate) mod parent_transform_system;
pub(crate) mod ui_text_system;
pub(crate) mod developer_console_system;

pub(crate) struct InternalPackage;
impl Package for InternalPackage {
    fn prepare(&self, data: &mut GameData) {
        let mut events = Events::default();
        events
            .create_topic("Inputs", TopicConfiguration::default())
            .expect("Error while creating topic for inputs event");

        let mut timers = Timers::default();

        if cfg!(feature = "hot-reload") {
            let _res = timers.add_timer("hot-reload-timer", TimerType::Cyclic, 5.);
        }

        data.insert_resource(Time::default());
        data.insert_resource(events);
        data.insert_resource(timers);
        data.insert_resource(AssetManager::default());
        data.insert_resource(InputsController::default());
        data.insert_resource(GameState::default());
        data.insert_resource(SceneController::default());
        data.insert_resource(Audio::default());
        data.insert_resource(FontAtlas::default());
    }

    fn load(self, builder: ScionBuilder) -> ScionBuilder {
        builder
            .with_system(default_camera_system)
            .with_system(camera_dpi_system)
            .with_system(sync_text_value_system)
            .with_system(ui_text_bitmap_update_system)
            .with_system(debug_colliders_system)
            .with_system(children_manager_system)
            .with_system(hide_propagated_deletion_system)
            .with_system(hide_propagation_system)
            .with_system(missing_ui_component_system::<UiImage>)
            .with_system(missing_ui_component_system::<UiTextImage>)
            .with_system(missing_ui_component_system::<UiText>)
            .with_system(asset_ref_resolver_system::<Material, MaterialAssetResolverFn>)
            .with_system(animation_executer_system)
            .with_system(dirty_child_system)
            .with_system(dirty_transform_system)
            .with_system(compute_collisions_system)
            .with_system(developer_console_system)
    }
}
