use bevy::prelude::*;
use bevy::window::WindowPlugin;
use bevy::DefaultPlugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Bevy Game".to_string(),
                width: 800.,
                height: 600.,
                canvas: Some("#bevy".to_owned()),
                ..WindowDescriptor::default()
            },
            ..WindowPlugin::default()
        }))
        .run();

    // .insert_resource(Msaa { samples: 1 })
    // .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
    // .insert_resource(WindowDescriptor {
    //     width: 800.,
    //     height: 600.,
    //     title: "Bevy game".to_string(), // ToDo
    //     canvas: Some("#bevy".to_owned()),
    //     ..Default::default()
    // })
    // .add_plugins(DefaultPlugins)
    // .add_plugin(GamePlugin)
}
