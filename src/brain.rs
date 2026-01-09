use bevy::prelude::*;
use crate::genome::{Motor, Sensor, SensorType};

#[derive(Component)]
pub struct Brain {
    pub input_neurons: Vec<f32>,
    pub hidden_neurons: Vec<f32>,
    pub output_neurons: Vec<f32>,
    pub synapses: Vec<Synapse>,
}

#[derive(Clone)]
pub struct Synapse {
    pub from_layer: LayerType,
    pub from_index: usize,
    pub to_layer: LayerType,
    pub to_index: usize,
    pub weight: f32,
}

#[derive(Clone, PartialEq)]
pub enum LayerType {
    Input,
    Hidden,
    Output,
}

pub fn brain_system(
    mut brain_query: Query<(Entity, &mut Brain, &Children)>,
    mut sensor_query: Query<&mut Sensor>,
    mut motor_query: Query<&mut Motor>,
) {
    for (_entity, mut brain, children) in brain_query.iter_mut() {
        // 1. READ SENSORS -> INPUT LAYER
        // We need to map global sensors belonging to this creature to inputs
        // For simplicity, we assume 'children' contains body parts with Sensors.
        
        let mut sensor_idx = 0;
        for child in children.iter() {
           if let Ok(sensor) = sensor_query.get(*child) {
               if sensor_idx < brain.input_neurons.len() {
                   brain.input_neurons[sensor_idx] = sensor.value;
                   sensor_idx += 1;
               }
           }
           // TODO: recursivity for children of children? 
           // Real impl needs flattened list of sensors for the creature.
        }

        // 2. FORWARD PASS
        // Reset hidden/output
        brain.hidden_neurons.fill(0.0);
        brain.output_neurons.fill(0.0);

        // Reserving this for potential performance logic
        // let synapses = &brain.synapses; 
        
        // Fix: Clone synapses for iteration to avoid borrow checker errors with brain.hidden_neurons
        let synapses = brain.synapses.clone();

        // Propagate signals
        for synapse in &synapses {
            let input_val = match synapse.from_layer {
                LayerType::Input => brain.input_neurons.get(synapse.from_index).cloned().unwrap_or(0.0),
                LayerType::Hidden => brain.hidden_neurons.get(synapse.from_index).cloned().unwrap_or(0.0),
                LayerType::Output => 0.0,
            };

            let weighted = input_val * synapse.weight;
            
            match synapse.to_layer {
                LayerType::Hidden => {
                     if let Some(n) = brain.hidden_neurons.get_mut(synapse.to_index) { *n += weighted; }
                }
                LayerType::Output => {
                     if let Some(n) = brain.output_neurons.get_mut(synapse.to_index) { *n += weighted; }
                }
                _ => {}
            }
        }
        
        // Activation (Tanh)
        for n in brain.hidden_neurons.iter_mut() { *n = n.tanh(); }
        for n in brain.output_neurons.iter_mut() { *n = n.tanh(); }

        // 3. WRITE OUTPUT -> MOTORS
        let mut motor_idx = 0;
         for child in children.iter() {
           if let Ok(mut motor) = motor_query.get_mut(*child) {
               if motor_idx < brain.output_neurons.len() {
                   motor.target_velocity = brain.output_neurons[motor_idx] * 10.0; // Scale output to velocity
                   motor_idx += 1;
               }
           }
        }
    }
}

pub fn sensor_reduction_system(
    mut sensors: Query<(&mut Sensor, &GlobalTransform)>,
    // Add Resources for World, etc.
) {
    for (mut sensor, transform) in sensors.iter_mut() {
        match sensor.sensor_type {
            SensorType::Eye { range: _, fov: _ } => {
                // Raycast logic here. For now, just height from ground?
                sensor.value = transform.translation().y;
            }
            SensorType::Touch => {
                // Collision logic
                sensor.value = 0.0; 
            }
            SensorType::Accelerometer => {
                sensor.value = transform.translation().y; // Placeholder
            }
            SensorType::None => {}
        }
    }
}
