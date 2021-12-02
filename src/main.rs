use bevy::{
    prelude::{App, ClearColor, Color},
    window::WindowDescriptor,
    DefaultPlugins,
};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .insert_resource(WindowDescriptor {
            width: 800.,
            height: 600.,
            title: "Bevy game".to_string(),
            ..Default::default()
        })
        .run()
}
