use bevy::prelude::*;
use bevy::input::mouse::{MouseMotion, MouseWheel};

#[derive(Component)]
pub struct PanOrbitCamera {
    pub focus: Vec3,
    pub radius: f32,
    pub upside_down: bool,
}

impl Default for PanOrbitCamera {
    fn default() -> Self {
        PanOrbitCamera {
            focus: Vec3::ZERO,
            radius: 15.0,
            upside_down: false,
        }
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, camera_orbit_system);
    }
}

fn camera_orbit_system(
    mut ev_motion: EventReader<MouseMotion>,
    mut ev_scroll: EventReader<MouseWheel>,
    input_mouse: Res<ButtonInput<MouseButton>>,
    mut query: Query<(&mut PanOrbitCamera, &mut Transform, &Projection)>,
) {
    // Rotation sensitivity
    let sensitivity = 0.005; 
    let zoom_sensitivity = 1.0;

    for (mut pan_orbit, mut transform, projection) in query.iter_mut() {
        let mut rotation_move = Vec2::ZERO;
        let mut scroll = 0.0;
        let mut orbit_button_changed = false;

        // Right mouse button to rotate
        if input_mouse.pressed(MouseButton::Right) {
            for ev in ev_motion.read() {
                rotation_move += ev.delta;
            }
        }

        for ev in ev_scroll.read() {
            scroll += ev.y;
        }

        if rotation_move.length_squared() > 0.0 {
            // Apply rotation
             // Yaw (around Y axis)
            let yaw = Quat::from_rotation_y(-rotation_move.x * sensitivity);
            // Pitch (around local X axis)
            let pitch = Quat::from_rotation_x(-rotation_move.y * sensitivity);
            
            transform.rotation = yaw * transform.rotation; // global yaw
            transform.rotation = transform.rotation * pitch; // local pitch
        } else if scroll.abs() > 0.0 {
            // Zoom
            pan_orbit.radius -= scroll * zoom_sensitivity;
            // Clamp radius
            pan_orbit.radius = pan_orbit.radius.max(2.0).min(50.0);
            
            // Re-calculate position based on radius and current rotation
            let rot_matrix = Mat3::from_quat(transform.rotation);
            transform.translation = pan_orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, pan_orbit.radius));
        }
    }
}
