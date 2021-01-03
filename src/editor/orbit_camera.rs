use bevy::{
    prelude::*,
    input::{
        mouse::
        {
            MouseWheel,
            MouseMotion,
            MouseButtonInput,
        },
        ElementState,
    }
};

pub struct OrbitCamera
{
    target: Vec3,
    mouse_down: [bool;3],
}

impl OrbitCamera
{
    pub fn new(target: Vec3) -> Self
    {
        Self {
            target,
            mouse_down: [false,false,false],
        }
    }
}

#[derive(Default)]
struct State
{
    mouse_wheel_event_reader: EventReader<MouseWheel>,
    mouse_motion_event_reader: EventReader<MouseMotion>,
    mouse_button_event_reader: EventReader<MouseButtonInput>,
}

fn on_mouse_wheel(mut state: ResMut<State>, events: Res<Events<MouseWheel>>, mut query: Query<(&mut OrbitCamera, &mut Transform)>)
{
    let mut delta: f32 = 0.0;
    for event in state.mouse_wheel_event_reader.iter(&events) {
        delta += event.y;
    }

    if delta != 0.0 {
        for (camera, mut transform) in query.iter_mut() {
            let len = (transform.translation - camera.target).length();
            delta = -delta * len * 0.1;
            transform.translation = transform.translation + transform.forward() * delta;
        }
    }
}

fn on_mouse_button(mut state: ResMut<State>, events: Res<Events<MouseButtonInput>>,mut query: Query<&mut OrbitCamera>)
{
    for event in state.mouse_button_event_reader.iter(&events) {
        let button_down = event.state == ElementState::Pressed;

        for mut camera in query.iter_mut() {
            match event.button {
                MouseButton::Left => camera.mouse_down[0] = button_down,
                MouseButton::Right => camera.mouse_down[1] = button_down,
                MouseButton::Middle => camera.mouse_down[2] = button_down,
                _ => {},
            }
        }
    }
}

fn on_mouse_motion(mut state: ResMut<State>, events: Res<Events<MouseMotion>>, mut query: Query<(&mut OrbitCamera, &mut Transform)>)
{
    let mut delta = Vec2::zero();
    for event in state.mouse_motion_event_reader.iter(&events) {
        delta += event.delta;
    }
    for (mut camera, mut transform) in query.iter_mut() {
        if camera.mouse_down[0] {
            let ry = Quat::from_rotation_y(delta.x * 0.01);
            let rx = Quat::from_rotation_x(delta.y * -0.01);

            let t = {
                let dir = (rx * ry) * Vec3::new(1.0,1.0,1.0);
                let mut t = Transform::from_translation(3.0 * dir);
                t.look_at(Vec3::zero(), Vec3::unit_y());
                t
            };

            transform.translation = t.translation;
            transform.rotation = t.rotation;

            //let len = (transform.translation - camera.target).length();
            
            //transform.translation = transform.translation +  transform.rotation * (len * Vec3::unit_x());
        }

        if camera.mouse_down[2] {
            let len = (transform.translation - camera.target).length();
            let dir = transform.rotation * (len * 0.001 * Vec3::new(-delta.x, delta.y, 0.0));
            camera.target += dir; 
            transform.translation = transform.translation + dir;
        }
    }
}

pub struct OrbitCameraPlugin;

impl Plugin for OrbitCameraPlugin
{
    fn build(&self, app: &mut AppBuilder) {
        app
            .init_resource::<State>()
            .add_system(on_mouse_wheel.system())
            .add_system(on_mouse_button.system())
            .add_system(on_mouse_motion.system());
    }
}