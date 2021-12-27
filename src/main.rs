use bevy::prelude::*;
use names::Generator;

const WORLD_POPULATION: u32 = 100;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldSetupPlugin)
        .add_plugin(RegisterSystemsPlugins)
        .run();
}

/* START Plugins */
struct WorldSetupPlugin;
impl Plugin for WorldSetupPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(add_people.system());
    }
}

struct RegisterSystemsPlugins;
impl Plugin for RegisterSystemsPlugins {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_system(print_people.system());
    }
}

/* START Systems */
fn add_people(mut commands: Commands) {
    let mut generator = Generator::default();
    for n in 0..WORLD_POPULATION {
        commands
            .spawn()
            .insert(PersonComponent)
            .insert(NameComponent(generator.next().unwrap()))
            .insert(AgeComponent(n))
            .insert(LevelComponent(n));
    }
}

fn print_people(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    query: Query<(&NameComponent, &AgeComponent, &LevelComponent), With<PersonComponent>>,
) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    if timer.0.tick(time.delta()).just_finished() {
        for (name, age, level) in query.iter() {
            println!("hello {}!", name.0);
            println!("age: {}", age.0);
            println!("level: {}", level.0);
        }
    }
}

/* START Resources */
struct GreetTimer(Timer);

/* START Components */
struct PersonComponent;
struct NameComponent(String);
struct AgeComponent(u32);
struct LevelComponent(u32);
