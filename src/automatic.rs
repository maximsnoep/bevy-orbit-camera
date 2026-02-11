use crate::transform::LookTransform;
use bevy::prelude::*;

#[derive(Component)]
pub struct Marker;

#[derive(Resource)]
pub struct AutomaticRotation {
    pub enabled: bool,
    pub sensitivity: f32, // amount of radians to be rotated per second
}

impl Default for AutomaticRotation {
    fn default() -> Self {
        Self {
            enabled: false,
            sensitivity: 1.0,
        }
    }
}

pub fn update(
    mut cameras: Query<&mut LookTransform, With<Marker>>,
    automatic_rotation: Res<AutomaticRotation>,
    time: Res<Time>,
) {
    if !automatic_rotation.enabled {
        return;
    }

    for mut t in cameras.iter_mut() {
        let up = t.up.normalize();
        let mut forward = (t.target - t.eye).normalize();
        let radius = t.radius();

        // radians per second * dt
        let yaw = automatic_rotation.sensitivity * time.delta_secs();

        forward = (Quat::from_axis_angle(up, yaw) * forward).normalize();

        // eye = target - forward * radius  (forward is eye->target)
        t.eye = t.target - forward * radius;
    }
}
