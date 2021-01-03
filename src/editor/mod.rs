use bevy::prelude::*;


mod orbit_camera;
use orbit_camera::*;

pub struct Editor;


impl Plugin for Editor
{
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_plugin(OrbitCameraPlugin)
            .add_startup_system(setup.system());
    }

}

fn setup(command: &mut Commands)
{
    let eye = Vec3::new(3.0,0.0,3.0);
    let target = Vec3::zero();
    let up = {
        let forward = Vec3::normalize(eye - target);
        let right = Vec3::unit_y().cross(forward).normalize();
        forward.cross(right)
    };

    let camera = {
        let mut bundle = Camera3dBundle::default();
        bundle.perspective_projection.near = 0.1;
        bundle.transform.translation = eye;
        bundle.transform.look_at(target, up);
        bundle
    };

    command
        .spawn(camera)
        .with(OrbitCamera::new(target));
}