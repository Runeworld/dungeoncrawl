use crate::prelude::*;

#[system]
#[read_component(EventLogMessage)]
pub fn event_log_clear(ecs: &SubWorld, commands: &mut CommandBuffer) {
    let mut message_query = <Entity>::query().filter(component::<EventLogMessage>());

    message_query.iter(ecs).for_each(|entity| {
        commands.remove(*entity);
    });
}
