use crate::prelude::*;

// @CONSIDER: Merge render_name / render_healthbar into one system?

#[system]
#[read_component(Point)]
#[read_component(Name)]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn render_name(ecs: &SubWorld, #[resource] mouse_pos: &Point, #[resource] camera: &Camera) {
    let mut positions = <(&Point, &Name)>::query();
    let mut player_fov = <&FieldOfView>::query().filter(component::<Player>());
    let offset = Point::new(camera.left_x, camera.top_y);
    let map_pos = *mouse_pos + offset;
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(CONSOLE_LAYER_HUD);
    let player_fov = player_fov.iter(ecs).next().unwrap();
    positions
        .iter(ecs)
        .filter(|(pos, _)| **pos == map_pos && player_fov.visible_tiles.contains(pos))
        .for_each(|(_, name)| {
            let screen_pos = Point {
                x: (mouse_pos.x * 4) + 4,
                y: mouse_pos.y * 4,
            };
            let display = name.0.clone();
            draw_batch.print(screen_pos, &display);
        });
    draw_batch.submit(Z_INDEX_TOOLTIPS).expect("Batch error");
}

#[system]
#[read_component(Point)]
#[read_component(Health)]
#[read_component(FieldOfView)]
#[read_component(Player)]
// @TODO: Filter out player or display something as name for player
// @TODO: Display health as healthbar
pub fn render_health(ecs: &SubWorld, #[resource] mouse_pos: &Point, #[resource] camera: &Camera) {
    let mut positions = <(&Point, &Health)>::query();
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let offset = Point::new(camera.left_x, camera.top_y);
    let map_pos = *mouse_pos + offset;
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(CONSOLE_LAYER_HUD);
    let player_fov = fov.iter(ecs).next().unwrap();
    positions
        .iter(ecs)
        .filter(|(pos, _)| **pos == map_pos && player_fov.visible_tiles.contains(pos))
        .for_each(|(_, health)| {
            let screen_pos = Point {
                x: (mouse_pos.x * 4) + 4,
                y: (mouse_pos.y * 4) + 1,
            };
            let display = format!("Health: {} / {}", health.current, health.max);
            draw_batch.print(screen_pos, &display);
        });
    draw_batch.submit(Z_INDEX_TOOLTIPS).expect("Batch error");
}
