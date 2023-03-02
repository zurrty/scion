use atomic_refcell::AtomicRefMut;
use crate::core::components::color::Color;
use crate::core::components::Hide;
use crate::core::components::material::Material;
use crate::core::components::maths::transform::Transform;
use crate::core::components::shapes::rectangle::Rectangle;
use crate::core::resources::developer_console::{DeveloperConsole, DeveloperConsoleContainer};
use crate::core::resources::events::PollConfiguration;
use crate::core::resources::events::topic::TopicConfiguration;
use crate::core::world::{GameData, Resources, SubWorld, World};

const DEVELOPER_CONSOLE_TOPIC: &str = "developer_console_topic";

pub(crate) fn developer_console_system(data: &mut GameData) {
    if data.get_resource_mut::<DeveloperConsole>().is_some() {
        handle_events_for_developer_console(data);

        let window_width = data.window().width();
        let (world, resource) = data.split();

        add_console_if_missing(window_width, world, resource)

        resource.inputs().key_pressed()


    }
}

fn add_console_if_missing(window_width: u32, world: &mut SubWorld, resource: &mut Resources) {
    let mut developer_console = resource.get_resource_mut::<DeveloperConsole>().unwrap();

    if developer_console.container.is_none() {
        developer_console.container = Some(world.push((DeveloperConsoleContainer,
                                                       Hide,
                                                       Transform::from_xyz(0., 0., 100),
                                                       Rectangle::new(window_width as f32, 200., None),
                                                       Material::Color(Color::new_rgb(30, 30, 30)),
        )));
    }
}

fn handle_events_for_developer_console(data: &mut GameData) {
    let mut events_resource = data.events();
    let mut developer_console = data.get_resource_mut::<DeveloperConsole>().unwrap();

    if !events_resource.topic_exists(DEVELOPER_CONSOLE_TOPIC) {
        events_resource.create_topic(DEVELOPER_CONSOLE_TOPIC,
                                     TopicConfiguration::default()).expect("Error while creating topic for developer console.");
    }

    if developer_console.subscriber_id.is_none() {
        let subscriber_id = events_resource.subscribe(DEVELOPER_CONSOLE_TOPIC, PollConfiguration::default()).expect("Error while subscribing to a suposed existing topic");
        developer_console.subscriber_id = Some(subscriber_id);
    }

    match events_resource.poll::<String>(developer_console.subscriber_id.as_ref().unwrap()) {
        Ok(mut messages) => {
            for message in messages.drain(0..messages.len()) {
                developer_console.messages_history.push(message);
            }
        }
        Err(_) => developer_console.subscriber_id = None
    };
}