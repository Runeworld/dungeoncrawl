use bevy::prelude::*;

// https://bevyengine.org/learn/book/getting-started/ecs/
fn main() {
    App::build().add_system(hello_world_system.system()).run();
}

fn hello_world_system() {
    println!("hello world");
}
