use crate::transform::LookTransform;
use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use std::f32::consts::PI;

/// A 3rd person camera that orbits around the target.
#[derive(Clone, Component, Copy, Debug, Reflect)]
#[reflect(Component, Default, Debug)]
pub struct Controller {
    pub mouse_rotate_sensitivity: Vec2,
    pub mouse_translate_sensitivity: Vec2,
    pub mouse_wheel_zoom_sensitivity: f32,
    pub pixels_per_line: f32,
}

impl Default for Controller {
    fn default() -> Self {
        Self {
            mouse_rotate_sensitivity: Vec2::splat(0.08),
            mouse_translate_sensitivity: Vec2::splat(0.1),
            mouse_wheel_zoom_sensitivity: 0.2,
            pixels_per_line: 53.0,
        }
    }
}

pub fn system(
    time: Res<Time>,
    mut mouse_wheel_reader: MessageReader<MouseWheel>,
    mut mouse_motion_events: MessageReader<MouseMotion>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut cameras: Query<(&Controller, &mut LookTransform)>,
) {
    // Can only control one camera at a time.
    let (
        Controller {
            mouse_rotate_sensitivity,
            mouse_translate_sensitivity,
            mouse_wheel_zoom_sensitivity,
            pixels_per_line,
            ..
        },
        mut transform,
    ) = match cameras.single_mut() {
        Ok((controller, look_transform)) => (controller, look_transform),
        Err(e) => {
            println!("Error handling bevy-orbit-camera controller: {}", e);
            return;
        }
    };

    // Amount of time since last update
    let time_delta = time.delta_secs();
    // Mouse movement since last update
    let cursor_delta = mouse_motion_events
        .read()
        .map(|event| event.delta)
        .sum::<Vec2>();

    // Amount of scroll since last update
    let scroll_delta = mouse_wheel_reader.read().fold(1.0, |acc, event| {
        let scroll_amount = match event.unit {
            MouseScrollUnit::Line => event.y,
            MouseScrollUnit::Pixel => event.y / *pixels_per_line,
        };
        acc * (1.0 - scroll_amount * mouse_wheel_zoom_sensitivity)
    });

    // World up vector (does not change).
    let up = transform.up.normalize();

    // ROTATE / ORBIT
    // changes the FORWARD vector.
    let mut forward = (transform.target - transform.eye).normalize(); // eye -> target
    if keyboard.pressed(KeyCode::ControlLeft) || mouse_buttons.pressed(MouseButton::Middle) {
        let delta = mouse_rotate_sensitivity * cursor_delta;

        // yaw rotates around "up"
        let yaw = time_delta * -delta.x;
        forward = Quat::from_axis_angle(up, yaw) * forward;

        // pitch rotates around "right"
        let pitch = time_delta * delta.y;
        forward = Quat::from_axis_angle(up.cross(forward).normalize(), pitch) * forward;
    }

    // TRANSLATE
    // changes the TARGET vector.
    let mut target = transform.target;
    if mouse_buttons.pressed(MouseButton::Right) {
        let delta = mouse_translate_sensitivity * cursor_delta;
        target += time_delta
            * (delta.x * up.cross(forward).normalize()
                + delta.y * forward.cross(up.cross(forward)).normalize());
    }

    // ZOOM
    // changes the RADIUS.
    let radius = (transform.radius() * scroll_delta).clamp(0.001, 1000000.0);

    // Do the transformations
    transform.target = target;
    transform.eye = target - forward * radius;
}
