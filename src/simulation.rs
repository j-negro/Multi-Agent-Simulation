use neighbors::{cell_index_method::CellIndexMethod, NeighborMethod, Particle as MethodParticle};
use rand::{rngs::ThreadRng, Rng};
use std::f64::consts::PI;

use crate::particle::Particle;

const PARTICLE_SPEED: f64 = 0.03;
const PARTICLE_RADIUS: f64 = 0.0;
const DELTA_TIME: f64 = 1.0;
const INTERACTION_RANGE: f64 = 1.0;

pub struct Simulation {
    length: f64,
    particles: Vec<Particle>,
    noise_amplitude: f64,
    neighbors_method: CellIndexMethod<Particle>,
    rng: ThreadRng,
}

impl Simulation {
    pub fn new(length: f64, noise_amplitude: f64, particle_count: usize) -> Self {
        let mut particles = Vec::with_capacity(particle_count);

        let mut rng = rand::thread_rng();

        // TODO: Calculate m
        let neighbors_method = CellIndexMethod::new(length, None, INTERACTION_RANGE, true);

        for id in 0..particle_count {
            let x = rng.gen_range(0.0..length);
            let y = rng.gen_range(0.0..length);
            let theta = rng.gen_range(-PI..PI);
            let particle = Particle::new(id as u32, x, y, PARTICLE_SPEED, theta, PARTICLE_RADIUS);
            particles.push(particle);
        }

        Simulation {
            length,
            particles,
            noise_amplitude,
            neighbors_method,
            rng,
        }
    }

    pub fn get_particles(&self) -> &[Particle] {
        &self.particles
    }

    pub fn get_length(&self) -> f64 {
        self.length
    }

    pub fn run_cycle(&mut self) {
        let particles_before = self.particles.clone();

        self.neighbors_method.set_particles(particles_before);

        let neighbors = self.neighbors_method.calculate_neighbors();

        for particle in &mut self.particles {
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

            let noise = if self.noise_amplitude != 0.0 {
                self.rng
                    .gen_range(-self.noise_amplitude / 2f64..self.noise_amplitude / 2f64)
            } else {
                0.0
            };

            let theta = (sin_avg).atan2(cos_avg) + noise;

            particle.update_angle(theta);

            let x = (x + v_x * DELTA_TIME).rem_euclid(self.length);
            let y = (y + v_y * DELTA_TIME).rem_euclid(self.length);
            particle.update_position(x, y);
        }
    }

    pub fn calculate_order_parameter(&mut self) -> f64 {
        let velocity_sum = self
            .particles
            .iter()
            .map(Particle::get_velocity_coordinates)
            .reduce(|(vx_acc, vy_acc), (vx, vy)| (vx_acc + vx, vy_acc + vy))
            .expect("particles should not be empty");

        let velocity_sum_module = (velocity_sum.0.powi(2) + velocity_sum.1.powi(2)).sqrt();

        velocity_sum_module / (self.particles.len() as f64 * PARTICLE_SPEED)
    }
}
