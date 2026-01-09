use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::genome::GeneNode;

pub struct PhysicSimPlugin;

impl Plugin for PhysicSimPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (soft_body_jiggle, metabolic_tick));
    }
}

#[derive(Component)]
pub struct Bone;

#[derive(Component)]
pub struct SoftBodyFlesh {
    pub stiffness: f32,
    pub damping: f32,
    pub original_offset: Vec3,
    pub current_offset: Vec3,
    pub velocity: Vec3,
}

#[derive(Component)]
pub struct Metabolism {
    pub energy: f32,
    pub mass: f32,
    pub active_synapses: usize,
}

// "Jiggle Physics" - The flesh lags behind the bone
fn soft_body_jiggle(
    time: Res<Time>,
    // Parenting: Bone -> Flesh
    mut query: Query<(&GlobalTransform, &Children)>, 
    mut flesh_query: Query<(&mut Transform, &mut SoftBodyFlesh), Without<Bone>>,
) {
    let dt = time.delta_secs();

    for (_bone_transform, children) in query.iter_mut() {
        for child in children.iter() {
            if let Ok((mut flesh_transform, mut flesh)) = flesh_query.get_mut(*child) {
                // In a local space simulation, we want the flesh to spring towards 'original_offset'
                // But purely local transform doesn't capture "inertia" from world movement well.
                // A better fake: modify the local translation based on the parent's acceleration?
                // Or simply: Run a spring simulation on local translation.
                
                // Target is 0.0 (relative to parent)
                let target = Vec3::ZERO; 
                let current = flesh_transform.translation;
                
                let displacement = current - target;
                let spring_force = -flesh.stiffness * displacement;
                let damping_force = -flesh.damping * flesh.velocity;
                
                let acceleration = spring_force + damping_force;
                flesh.velocity += acceleration * dt;
                flesh_transform.translation += flesh.velocity * dt;
            }
        }
    }
}

fn metabolic_tick(
    time: Res<Time>,
    mut query: Query<(&mut Metabolism, &Velocity)>,
) {
    let dt = time.delta_secs();
    
    for (mut metabolism, velocity) in query.iter_mut() {
        let speed = velocity.linvel.length();
        let torque = velocity.angvel.length();
        
        let base_loss = metabolism.mass * 0.01;
        let motor_loss = torque * 0.05;
        let brain_loss = (metabolism.active_synapses as f32) * 0.001;
        let movement_loss = speed * 0.02; // Extra cost for dragging mass
        
        let total_loss = (base_loss + motor_loss + brain_loss + movement_loss) * dt;
        
        metabolism.energy -= total_loss;
        
        if metabolism.energy <= 0.0 {
            // Die
            // commands.entity(entity).despawn_recursive();
        }
    }
}
