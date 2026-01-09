use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(VoxelGrid::new(100, 100, 1.0))
            .add_systems(Update, decay_pheromones);
    }
}

#[derive(Resource)]
pub struct VoxelGrid {
    pub width: usize,
    pub height: usize,
    pub cell_size: f32,
    pub pheromones: Vec<f32>,
}

impl VoxelGrid {
    pub fn new(width: usize, height: usize, cell_size: f32) -> Self {
        Self {
            width,
            height,
            cell_size,
            pheromones: vec![0.0; width * height],
        }
    }

    pub fn get_index(&self, x: usize, z: usize) -> Option<usize> {
        if x >= self.width || z >= self.height {
            None
        } else {
            Some(x + z * self.width)
        }
    }

    pub fn add_pheromone(&mut self, pos: Vec3, amount: f32) {
        // Convert world pos to grid coords
        // Assuming grid is centered at 0,0?? Or starts at 0,0?
        // Let's assume starts at -width/2, -height/2 for now or just 0 to width.
        // Simple mapping:
        let x = (pos.x / self.cell_size).floor() as i32 + (self.width as i32 / 2);
        let z = (pos.z / self.cell_size).floor() as i32 + (self.height as i32 / 2);

        if x >= 0 && x < self.width as i32 && z >= 0 && z < self.height as i32 {
            if let Some(idx) = self.get_index(x as usize, z as usize) {
                self.pheromones[idx] += amount;
                if self.pheromones[idx] > 1.0 {
                     self.pheromones[idx] = 1.0;
                }
            }
        }
    }
}

fn decay_pheromones(mut grid: ResMut<VoxelGrid>, time: Res<Time>) {
    let _dt = time.delta_secs();
    // Decay factor per frame? Or per second?
    // User said "value *= 0.99". Let's assume per tick for simplicity or adjust for dt.
    // 0.99 at 60fps is fast decay. Let's do 0.99 per second? 
    // Actually user said "value *= 0.99" which implies per tick usually in these specs.
    // But let's be frame-rate independent: P(t) = P(0) * decay^t
    // Let's simple multiply by 0.99 every frame for now as requested.
    
    for val in grid.pheromones.iter_mut() {
        if *val > 0.001 {
             *val *= 0.99;
        } else {
             *val = 0.0;
        }
    }
}
