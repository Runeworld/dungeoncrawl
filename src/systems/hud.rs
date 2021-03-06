// @TODO: Stack items in inventory and rework player input for using them
// @TODO: Visually and logically separate inventory from current equipment

use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Item)]
#[read_component(Carried)]
#[read_component(Name)]
pub fn hud(ecs: &SubWorld) {
    let mut health_query = <&Health>::query().filter(component::<Player>());
    let player_health = health_query.iter(ecs).next().unwrap();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(CONSOLE_LAYER_HUD);
    draw_batch.print_centered(
        1,
        "Find the amulet. Arrow keys to move. G to pick up items. Number keys to use items.", // @TODO: Move tutorial before game start
    );
    draw_batch.bar_horizontal(
        Point::zero(),
        WORLD_WIDTH_IN_TILES * 2,
        player_health.current,
        player_health.max,
        ColorPair::new(RED, BLACK),
    );
    draw_batch.print_color_centered(
        0,
        format!(
            " Health: {} / {} ",
            player_health.current, player_health.max
        ),
        ColorPair::new(WHITE, RED),
    );

    let (player, map_level) = <(Entity, &Player)>::query()
        .iter(ecs)
        .map(|(entity, player)| (*entity, player.map_level))
        .next()
        .unwrap();

    draw_batch.print_color_right(
        Point::new(WORLD_WIDTH_IN_TILES * 2, 1),
        format!("Dungeon Level: {}", map_level + 1),
        ColorPair::new(YELLOW, BLACK),
    );

    let mut item_query = <(&Item, &Name, &Carried)>::query();
    let mut y = 3;
    item_query
        .iter(ecs)
        .filter(|(_, _, carried)| carried.0 == player)
        .for_each(|(_, name, _)| {
            draw_batch.print(Point::new(3, y), format!("{} : {}", y - 2, &name.0));
            y += 1;
        });
    if y > 3 {
        draw_batch.print_color(
            Point::new(3, 2),
            "Items carried",
            ColorPair::new(YELLOW, BLACK),
        );
    }

    draw_batch.submit(Z_INDEX_HUD).expect("Batch error");
}
