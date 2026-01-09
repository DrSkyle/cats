use bevy::prelude::*;
use std::f32::consts::PI;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_cat_audio);
    }
}

// TODO: Integrate with Bevy Audio for real-time synthesis.
// For now, we simulate the internal state of the synthesis engines.

#[derive(Component)]
pub struct CatAudioState {
    pub meow_phase: f32, // 0.0 to 1.0 (meow progress)
    pub is_purring: bool,
    pub purr_comfort: f32,
    pub is_hissing: bool,
    pub adrenaline: f32,
}

#[derive(Component)]
pub struct VocalCords {
    pub size: f32, // Affects pitch (0.5 = small/high, 2.0 = large/low)
    pub snout_length: f32, // Affects formant filter
}

fn sawtooth(frequency: f32, time: f32) -> f32 {
    let period = 1.0 / frequency;
    let t = time % period;
    (2.0 * t / period) - 1.0
}

fn white_noise() -> f32 {
    rand::random::<f32>() * 2.0 - 1.0
}

// Simple Low Pass Filter simulation
struct LowPassFilter {
    cutoff: f32,
    prev_output: f32,
}

impl LowPassFilter {
    fn process(&mut self, input: f32, dt: f32) -> f32 {
        let rc = 1.0 / (2.0 * PI * self.cutoff);
        let alpha = dt / (rc + dt);
        let output = alpha * input + (1.0 - alpha) * self.prev_output;
        self.prev_output = output;
        output
    }
}

fn update_cat_audio(
    time: Res<Time>,
    mut query: Query<(&mut CatAudioState, &VocalCords)>,
) {
    for (mut state, cords) in query.iter_mut() {
        let dt = time.delta_secs();
        
        // 1. Meow Engine Logic
        if state.meow_phase > 0.0 && state.meow_phase < 1.0 {
            // Pitch Bend: Low -> High -> Low
            let pitch_mod = (state.meow_phase * PI).sin(); 
            let base_freq = 400.0 / cords.size; 
            let _current_freq = base_freq + (pitch_mod * 200.0);
            
            // Formant Filter Logic: "eee" (2kHz) -> "aaa" (1kHz) -> "ooo" (400Hz)
            // Simplified formant transition
            let _formant_cutoff = if state.meow_phase < 0.3 {
                 2000.0 // eee
            } else if state.meow_phase < 0.6 {
                 1000.0 // aaa
            } else {
                 400.0 // ooo
            };

            // Advance meow
            state.meow_phase += dt * 1.5; // Speed of meow
            if state.meow_phase >= 1.0 {
                state.meow_phase = 0.0; // Reset
            }
        }

        // 2. Purr Engine Logic
        if state.is_purring {
             let _purr_freq = 25.0; // Hz
             // Amplitude modulation logic would go here in the audio callback
        }

        // 3. Hiss Engine Logic
        if state.is_hissing {
             // White noise logic
        }
    }
}
