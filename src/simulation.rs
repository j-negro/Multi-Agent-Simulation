use rand::Rng;
use std::f64::consts::PI;

use crate::particle::Particle;

struct Simulation {
    lenght: f64,
    interaction_range: f64,
    particles: Vec<Particle>,
    noise_amplitude: f64,
}

const PARTICLE_SPEED: f64 = 0.3;
const PARTICLE_RADIUS: f64 = 0.0;

impl Simulation {
    fn new(
        lenght: f64,
        interaction_range: f64,
        noise_amplitude: f64,
        particle_count: usize,
    ) -> Simulation {
        let mut particles = Vec::with_capacity(particle_count);

        let mut rng = rand::thread_rng();

        for id in 0..particle_count {
            let x = rng.gen_range(0.0..lenght);
            let y = rng.gen_range(0.0..lenght);
            let theta = rng.gen_range(0.0..2.0 * PI);
            let particle = Particle::new(id as u32, x, y, PARTICLE_SPEED, theta, PARTICLE_RADIUS);
            particles.push(particle);
        }

        Simulation {
            lenght,
            interaction_range,
            particles,
            noise_amplitude,
        }
    }
}
