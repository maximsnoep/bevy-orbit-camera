use crate::automatic;
use crate::control;
use crate::transform;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use std::time::Duration;

pub struct OrbitCameraPlugin;

impl Plugin for OrbitCameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<automatic::AutomaticRotation>()
            .add_systems(Update, control::system)
            .add_systems(Update, transform::system)
            .add_systems(
                FixedUpdate,
                automatic::update.run_if(on_timer(Duration::from_millis(10))),
            );
    }
}

#[derive(Bundle)]
pub struct OrbitCameraBundle {
    controller: control::Controller,
    look_transform: transform::LookTransform,
    transform: Transform,
}

impl OrbitCameraBundle {
    pub fn new(controller: control::Controller, eye: Vec3, target: Vec3, up: Vec3) -> Self {
        Self {
            controller,
            look_transform: transform::LookTransform::new(eye, target, up),
            transform: Transform::from_translation(eye).looking_at(target, up),
        }
    }
}
