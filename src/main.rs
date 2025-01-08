use bevy::app::App;
use geometry_wars::GamePlugin;

fn main() {
    App::new().add_plugins(GamePlugin).run();
}
