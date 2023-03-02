use hecs::Entity;
use crate::core::resources::events::SubscriberId;

#[derive(Default)]
pub struct DeveloperConsole {
    pub(crate) messages_history: Vec<String>,
    pub(crate) subscriber_id: Option<SubscriberId>,
    pub(crate) displayed: bool,
    pub(crate) container: Option<Entity>
}

pub struct DeveloperConsoleContainer;