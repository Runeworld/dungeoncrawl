use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}

/* START Plugins */
struct HelloPlugin;
impl Plugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(add_people.system())
            .add_system(greet_people.system());
    }
}

/* START Systems */

fn add_people(mut commands: Commands) {
    commands
        .spawn()
        .insert(PersonComponent)
        .insert(FriendlyComponent)
        .insert(NameComponent("Kyle".to_string()));

    commands
        .spawn()
        .insert(PersonComponent)
        .insert(NameComponent("Simon".to_string()));
}

fn greet_people(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    query: Query<&NameComponent, (With<PersonComponent>, With<FriendlyComponent>)>,
) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("hello {}!", name.0);
        }
    }
}

/* START Resources */
struct GreetTimer(Timer);

/* START Components */
struct PersonComponent;
struct NameComponent(String);
struct FriendlyComponent;
