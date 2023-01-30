use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Name)]
#[read_component(Health)]
#[read_component(FieldOfView)]
#[read_component(Player)]
#[allow(clippy::trivially_copy_pass_by_ref)]
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

            if let Some(health) = health {
                let healthbar_width = 4;
                let screen_pos_health_offset = Point { x: 0, y: 1 };
                draw_batch.print_color(
                    screen_pos + screen_pos_health_offset,
                    format!("Health: {} / {}", health.current, health.max),
                    ColorPair::new(WHITE, RED),
                );
                draw_batch.bar_horizontal(
                    screen_pos + Point { x: -4, y: 4 },
                    healthbar_width,
                    health.current * healthbar_width - 1,
                    health.max * healthbar_width,
                    ColorPair::new(RED, BLACK),
                );
            }
        });
    draw_batch.submit(Z_INDEX_TOOLTIPS).expect("Batch error");
}
