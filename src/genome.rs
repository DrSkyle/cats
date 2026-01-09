use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JointType {
    Fixed,
    Hinge { limit_min: f32, limit_max: f32 },
    Spherical,
}

#[derive(Component)]
pub struct Motor {
    pub joint_entity: Entity,
    pub target_velocity: f32,
    pub max_force: f32,
}

#[derive(Component)]
pub struct Sensor {
    pub sensor_type: SensorType,
    pub value: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SensorType {
    None,
    Eye { range: f32, fov: f32 },
    Touch,
    Accelerometer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneNode {
    pub id: u32,
    pub dimensions: Vec3,
    pub joint_type: JointType,
    pub sensor_type: SensorType,
    pub stiffness: f32,
    pub children: Vec<u32>, // IDs of connected nodes
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Genotype {
    pub nodes: Vec<GeneNode>,
    pub root_node: u32,
}

// Helper for recursive spawning
fn spawn_node(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    nodes: &Vec<GeneNode>,
    current_node_id: u32,
    parent_entity: Option<Entity>,
    anchor_pos: Vec3,
) {
    let node = nodes.iter().find(|n| n.id == current_node_id);
    if node.is_none() { return; }
    let node = node.unwrap();

    // Create the bone (Physics)
    // For simplicity, we just spawn at anchor_pos for now.
    // Real implementation needs to calculate relative transform.
    
    let half_extents = node.dimensions / 2.0;
    
    let mut entity_cmd = commands.spawn((
        PbrBundle {
             // Debug mesh for bone
            mesh: meshes.add(Cuboid::from_size(node.dimensions).mesh()),
            material: materials.add(Color::srgb(0.8, 0.2, 0.2)),
            transform: Transform::from_translation(anchor_pos), 
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(half_extents.x, half_extents.y, half_extents.z),
        crate::physic_sim::Bone,
    ));
    
    if let Some(parent) = parent_entity {
        // Add joint
        // For now, FixedJoint. Real imple would use node.joint_type
        let joint = ImpulseJoint::new(parent, RapierImpulseJointHandle::new(GenericJoint::fixed()));
        let joint_entity = entity_cmd.insert(joint).id();
        
        // If has joint type that isn't fixed, add Motor
        match node.joint_type {
            JointType::Hinge { .. } => {
                 // For now all are fixed, but logic:
                 // commands.entity(joint_entity).insert(Motor { joint_entity: parent, ... });
            }
            _ => {}
        }
    } else {
        // Root node
    }
    
    let entity = entity_cmd.id();
    
    // Attach Sensor
    if let SensorType::None = node.sensor_type {
        // No sensor
    } else {
        commands.entity(entity).insert(crate::genome::Sensor {
            sensor_type: node.sensor_type.clone(),
            value: 0.0,
        });
    }

    // Spawn "Soft Body Flesh" visual child
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::from_size(node.dimensions * 1.2).mesh()), // Slightly larger
            material: materials.add(Color::srgba(0.8, 0.5, 0.5, 0.8)), // Semi-transparent flesh?
            ..default()
        },
        crate::physic_sim::SoftBodyFlesh {
            stiffness: node.stiffness,
            damping: 0.5,
            original_offset: Vec3::ZERO, // Local to bone
            current_offset: Vec3::ZERO,
            velocity: Vec3::ZERO,
        },
    )).set_parent(entity);

    // Recursion
    for &child_id in &node.children {
        // Calculate interaction/offset for child? 
        // For simplicity, just offset by dimension.x
        let child_offset = anchor_pos + Vec3::new(node.dimensions.x, 0.0, 0.0); 
        spawn_node(commands, meshes, materials, nodes, child_id, Some(entity), child_offset);
    }
}

pub fn spawn_creature(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    genotype: &Genotype,
    position: Vec3,
) {
    spawn_node(commands, meshes, materials, &genotype.nodes, genotype.root_node, None, position);
}
