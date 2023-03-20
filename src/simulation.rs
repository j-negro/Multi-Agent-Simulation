use crate::particle::Particle;

struct Simulation {
    lenght: f64,
    interaction_range: f64,
    particles: Vec<Particle>,
    noise_amplitude: f64,
}

impl Simulation {
    fn new(
        lenght: f64,
        interaction_range: f64,
        particles: Vec<Particle>,
        noise_amplitude: f64,
    ) -> Simulation {
        Simulation {
            lenght,
            interaction_range,
            particles,
            noise_amplitude,
        }
    }
}
