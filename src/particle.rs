use std::hash::Hash;

use neighbors::Particle as MethodParticle;

#[derive(Debug, Clone)]
pub struct Particle {
    id: u32,
    x: f64,
    y: f64,
    v: f64,
    theta: f64,
    radius: f64,
}

impl Particle {
    pub fn new(id: u32, x: f64, y: f64, v: f64, theta: f64, radius: f64) -> Particle {
        Particle {
            id,
            x,
            y,
            v,
            theta,
            radius,
        }
    }

    pub fn update_position(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }

    pub fn update_angle(&mut self, theta: f64) {
        self.theta = theta;
    }

    pub fn get_velocity_coordinates(&self) -> (f64, f64) {
        (self.v * self.theta.cos(), self.v * self.theta.sin())
    }

    pub fn get_angle(&self) -> f64 {
        self.theta
    }
}

impl MethodParticle for Particle {
    fn get_id(&self) -> u32 {
        self.id
    }

    fn get_coordinates(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    fn get_radius(&self) -> f64 {
        self.radius
    }
}

impl PartialEq for Particle {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Hash for Particle {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Eq for Particle {}
