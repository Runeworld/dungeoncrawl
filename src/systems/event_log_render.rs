use crate::prelude::*;

#[system]
#[read_component(EventLogMessage)]
pub fn event_log_render(ecs: &SubWorld, commands: &mut CommandBuffer) {
    let mut message_query = <(Entity, &EventLogMessage)>::query();
    let message_count = message_query.iter(ecs).count() as i32;

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(CONSOLE_LAYER_HUD);

    let mut y = (WINDOW_HEIGHT_IN_TILES * 4) - message_count;
    message_query.iter(ecs).for_each(|(_, message)| {
        draw_batch.print(Point::new(3, y), &message.content);
        y += 1;
    });

    draw_batch.submit(Z_INDEX_HUD).expect("Batch error");
}
