use bevy::prelude::*;
use crate::genome::{Genotype, spawn_creature};
use rand::prelude::*;

pub struct PopulationPlugin;

impl Plugin for PopulationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PopulationConfig {
            target_population: 50,
            mutation_rate: 0.01,
        })
        .add_systems(Startup, initial_spawn)
        .add_systems(Update, (population_control, handle_meteor_strike));
    }
}

// ... (existing helper structs)

fn handle_meteor_strike(
    mut commands: Commands,
    mut events: EventReader<crate::ui::MeteorStrikeEvent>,
    query: Query<Entity, With<Creature>>,
) {
    for _ in events.read() {
        let mut rng = rand::thread_rng();
        for entity in query.iter() {
            if rng.gen_bool(0.9) { 
                 commands.entity(entity).despawn_recursive();
            }
        }
    }
}

#[derive(Resource)]
pub struct PopulationConfig {
    pub target_population: usize,
    pub mutation_rate: f32, // Controlled by UI
}

#[derive(Component)]
pub struct Creature;

fn initial_spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    config: Res<PopulationConfig>,
) {
    // Spawn initial random creatures
    for _ in 0..config.target_population {
        // Create random genotype (TODO: dedicated generator)
        let genotype = Genotype { 
             nodes: vec![
                 crate::genome::GeneNode {
                     id: 0,
                     dimensions: Vec3::new(0.5, 0.5, 1.0),
                     joint_type: crate::genome::JointType::Fixed,
                     sensor_type: crate::genome::SensorType::Eye { range: 10.0, fov: 1.0 },
                     stiffness: 100.0,
                     children: vec![],
                 }
             ], 
             root_node: 0 
        };
        
        let x = (rand::random::<f32>() * 40.0) - 20.0;
        let z = (rand::random::<f32>() * 40.0) - 20.0;
        
        spawn_creature(&mut commands, &mut meshes, &mut materials, &genotype, Vec3::new(x, 2.0, z));
    }
}

fn population_control(
    mut commands: Commands,
    query: Query<Entity, With<Creature>>,
    config: Res<PopulationConfig>,
) {
    // If population is too low, spawn more? 
    // Or reproduction logic?
    // For now, placeholder.
}

pub fn meteor_strike(
    commands: &mut Commands,
    query: &Query<Entity, With<Creature>>,
) {
    let mut rng = rand::thread_rng();
    for entity in query.iter() {
        if rng.gen_bool(0.9) { // 90% chance to die
             commands.entity(entity).despawn_recursive();
        }
    }
}
