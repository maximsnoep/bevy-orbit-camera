use crate::transform::LookTransform;
use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::prelude::*;

const MAX_PITCH_ANGLE: f32 = 1.5260712;
const ROTATION_RADIANS_PER_PIXEL: f32 = 0.0075;
const TRANSLATION_UNITS_PER_PIXEL: f32 = 0.01;

#[derive(Clone, Component, Copy, Debug, Reflect)]
#[reflect(Component, Debug)]
pub(crate) struct OrbitControllerState {
    pub(crate) yaw: f32,
    pitch: f32,
    yaw_zero_forward: Vec3,
}

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
            mouse_rotate_sensitivity: Vec2::splat(0.2),
            mouse_translate_sensitivity: Vec2::splat(2.0),
            mouse_wheel_zoom_sensitivity: 0.2,
            pixels_per_line: 53.0,
        }
    }
}

pub fn system(
    mut commands: Commands,
    mut mouse_wheel_reader: MessageReader<MouseWheel>,
    mut mouse_motion_events: MessageReader<MouseMotion>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut cameras: Query<(
        Entity,
        &Controller,
        Option<&mut OrbitControllerState>,
        &mut LookTransform,
    )>,
) {
    // Can only control one camera at a time.
    let (
        entity,
        Controller {
            mouse_rotate_sensitivity,
            mouse_translate_sensitivity,
            mouse_wheel_zoom_sensitivity,
            pixels_per_line,
            ..
        },
        controller_state,
        mut transform,
    ) = match cameras.single_mut() {
        Ok((entity, controller, controller_state, look_transform)) => {
            (entity, controller, controller_state, look_transform)
        }
        Err(_) => return,
    };

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
    let up = transform.up.try_normalize().unwrap_or(Vec3::Y);

    // ROTATE / ORBIT
    // changes the FORWARD vector.
    let Some(initial_forward) = (transform.target - transform.eye).try_normalize() else {
        return;
    }; // eye -> target
    let mut state = controller_state
        .as_deref()
        .copied()
        .unwrap_or_else(|| OrbitControllerState::from_forward(initial_forward, up));
    if keyboard.pressed(KeyCode::ControlLeft) || mouse_buttons.pressed(MouseButton::Middle) {
        let delta = mouse_rotate_sensitivity * cursor_delta * ROTATION_RADIANS_PER_PIXEL;

        state.yaw -= delta.x;
        state.pitch = (state.pitch - delta.y).clamp(-MAX_PITCH_ANGLE, MAX_PITCH_ANGLE);
    }
    let forward = state.forward(up);

    // TRANSLATE
    // changes the TARGET vector.
    let mut target = transform.target;
    if mouse_buttons.pressed(MouseButton::Right) {
        let delta = mouse_translate_sensitivity * cursor_delta * TRANSLATION_UNITS_PER_PIXEL;
        if let Some(right) = up.cross(forward).try_normalize() {
            let upish = forward.cross(right).normalize();
            target += delta.x * right + delta.y * upish;
        }
    }

    match controller_state {
        Some(mut controller_state) => *controller_state = state,
        None => {
            commands.entity(entity).insert(state);
        }
    }

    // ZOOM
    // changes the RADIUS.
    let radius = (transform.radius() * scroll_delta).clamp(0.001, 1000000.0);

    // Do the transformations
    transform.target = target;
    transform.eye = target - forward * radius;
}

impl OrbitControllerState {
    pub(crate) fn from_forward(forward: Vec3, up: Vec3) -> Self {
        let pitch = forward.dot(up).clamp(-1.0, 1.0).asin();
        let yaw_zero_forward = (forward - up * forward.dot(up))
            .try_normalize()
            .unwrap_or_else(|| orthonormal_vector(up));

        Self {
            yaw: 0.0,
            pitch: pitch.clamp(-MAX_PITCH_ANGLE, MAX_PITCH_ANGLE),
            yaw_zero_forward,
        }
    }

    fn forward(&self, up: Vec3) -> Vec3 {
        let horizontal = Quat::from_axis_angle(up, self.yaw) * self.yaw_zero_forward;
        (horizontal * self.pitch.cos() + up * self.pitch.sin()).normalize()
    }
}

fn orthonormal_vector(up: Vec3) -> Vec3 {
    up.any_orthonormal_vector()
}
