use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Name)]
#[read_component(Health)]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn tooltips(ecs: &SubWorld, #[resource] mouse_pos: &Point, #[resource] camera: &Camera) {
    let mut positions = <(&Point, &Name, Option<&Health>)>::query();
    let mut player_fov = <&FieldOfView>::query().filter(component::<Player>());
    let offset = Point::new(camera.left_x, camera.top_y);
    let map_pos = *mouse_pos + offset;
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(CONSOLE_LAYER_HUD);
    let player_fov = player_fov.iter(ecs).next().unwrap();
    positions
        .iter(ecs)
        .filter(|(pos, _, _)| **pos == map_pos && player_fov.visible_tiles.contains(pos))
        .for_each(|(_, name, health)| {
            let screen_pos = Point {
                x: (mouse_pos.x * 4) + 4,
                y: mouse_pos.y * 4,
            };
            let name_display = name.0.clone();
            draw_batch.print(screen_pos, &name_display);

            if health.is_some() {
                // @TODO: Display health as healthbar
                let screen_pos_health_offset = Point { x: 0, y: 1 };
                let health_display = format!(
                    "Health: {} / {}",
                    health.unwrap().current,
                    health.unwrap().max
                );
                draw_batch.print(screen_pos + screen_pos_health_offset, health_display);
            }
        });
    draw_batch.submit(Z_INDEX_TOOLTIPS).expect("Batch error");
}
