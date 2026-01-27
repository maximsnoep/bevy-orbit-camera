use crate::control::LookAngles;
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
) {
    if !automatic_rotation.enabled {
        return;
    }
    for mut transform in cameras.iter_mut() {
        let mut look_angles = LookAngles::from_vector(-transform.look_direction().unwrap());
        let rotation = automatic_rotation.sensitivity / 100.;
        look_angles.add_yaw(-rotation);
        transform.eye = transform.target + transform.radius() * look_angles.unit_vector();
    }
}
