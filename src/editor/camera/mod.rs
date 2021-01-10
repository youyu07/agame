use bevy::{
    prelude::*,
};

mod orbit;
pub use orbit::OrbitCamera;

pub struct OrbitCameraPlugin;

impl Plugin for OrbitCameraPlugin
{
    fn build(&self, app: &mut AppBuilder) {
        app
            .init_resource::<orbit::State>()
            .add_system(orbit::on_mouse_wheel.system())
            .add_system(orbit::on_mouse_down.system())
            .add_system(orbit::on_mouse_motion.system());
    }
}