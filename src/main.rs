use bevy::{
    prelude::*,
};

mod shape;
mod editor;


fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(editor::Editor)
        .add_startup_system(setup.system())
        .run();
}


fn setup(commands: &mut Commands, 
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {

    commands
        .spawn_scene(asset_server.load("FlightHelmet/FlightHelmet.gltf"))
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 5.0, 4.0)),
            ..Default::default()
        })
        .spawn(PbrBundle{
            mesh: meshes.add(Mesh::from(shape::Cube::default())),
            material: materials.add(Color::rgb(0.5, 0.4, 0.3).into()),
            ..Default::default()
        });
}