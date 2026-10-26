[![Crates.io](https://img.shields.io/crates/v/bevy_orbit_camera)](https://crates.io/crates/bevy_orbit_camera)

# bevy-orbit-camera

An orbit camera for [Bevy](https://bevyengine.org): rotate, pan and zoom around a target with the mouse, and optionally
rotate automatically. Based on the orbit camera of [smooth-bevy-cameras](https://github.com/bonsairobo/smooth-bevy-cameras).

## How to use

Add the `OrbitCameraPlugin`, and give your camera an `OrbitCameraBundle` (its controller, eye, target and up):
```rust
App::new().add_plugins(OrbitCameraPlugin);

commands.spawn((
  Camera3d::default(),
  OrbitCameraBundle::new(Controller::default(), Vec3::new(0., 0., 10.), Vec3::ZERO, Vec3::Y),
));
```

## Controls

| Input | Action |
|-------|--------|
| Middle mouse drag, or Left Ctrl + mouse move | Rotate around the target |
| Right mouse drag | Pan |
| Scroll wheel | Zoom |

For automatic rotation, add the `automatic::Marker` component to the camera and enable the `AutomaticRotation` resource
(its `sensitivity` is in radians per second).

## Version Compatibility

| bevy | bevy_orbit_camera |
|------|-------------------|
| 0.20.0 | 0.4.x |
| 0.18.0 | 0.2.x, 0.3.x |
| 0.17.3 | 0.1.x |
