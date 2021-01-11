use bevy::{
    prelude::*,
};

mod editor;
mod sky;


fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(editor::Editor)
        .add_plugin(sky::SkyPlugin)
        .add_startup_system(setup.system())
        .run();
}


fn setup(commands: &mut Commands, asset_server: Res<AssetServer>) 
{
    commands
        .spawn_scene(asset_server.load("FlightHelmet/FlightHelmet.gltf"))
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 5.0, 4.0)),
            ..Default::default()
        });
}