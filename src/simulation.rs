use neighbors::{cell_index_method::CellIndexMethod, NeighborMethod, Particle as MethodParticle};
use rand::Rng;
use std::f64::consts::PI;

use crate::particle::Particle;

const PARTICLE_SPEED: f64 = 0.3;
const PARTICLE_RADIUS: f64 = 0.0;

struct Simulation {
    length: f64,
    interaction_range: f64,
    particles: Vec<Particle>,
    noise_amplitude: f64,
    time: u32,
    neighbors_method: CellIndexMethod<Particle>,
}

impl Simulation {
    pub fn new(
        length: f64,
        interaction_range: f64,
        noise_amplitude: f64,
        particle_count: usize,
        m: usize,
    ) -> Self {
        let mut particles = Vec::with_capacity(particle_count);

        let mut rng = rand::thread_rng();

        let mut neighbors_method = CellIndexMethod::new(length, Some(m), interaction_range, true);

        for id in 0..particle_count {
            let x = rng.gen_range(0.0..length);
            let y = rng.gen_range(0.0..length);
            let theta = rng.gen_range(0.0..2.0 * PI);
            let particle = Particle::new(id as u32, x, y, PARTICLE_SPEED, theta, PARTICLE_RADIUS);
            particles.push(particle);
        }

        Simulation {
            length,
            interaction_range,
            particles,
            noise_amplitude,
            time: 0,
            neighbors_method,
        }
    }

    pub fn advance_time(&mut self) {
        let particles_before = self.particles.clone();

        self.neighbors_method.set_particles(particles_before);

        let neighbors = self.neighbors_method.calculate_neighbors();

        for particle in self.particles.iter_mut() {
            let (x, y) = particle.get_coordinates();
            let (v_x, v_y) = particle.get_velocity_coordinates();

            let particle_neighbors = &neighbors[particle.get_id() as usize];

            let mut sin_sum = particle.get_angle().sin();
            let mut cos_sum = particle.get_angle().cos();

            for neighbor in particle_neighbors.iter() {
                sin_sum += neighbor.get_angle().sin();
                cos_sum += neighbor.get_angle().cos();
            }

            let sin_avg = sin_sum / (particle_neighbors.len() + 1) as f64;
            let cos_avg = cos_sum / (particle_neighbors.len() + 1) as f64;

            particle.update_angle((sin_avg).atan2(cos_avg));

            particle.update_position(x + v_x * self.time as f64, y + v_y * self.time as f64);
        }

        self.time += 1;
    }
}
