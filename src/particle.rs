pub struct Particle {
    id: u64,
    x: f64,
    y: f64,
    v: f64,
    theta: f64,
    radius: f64,
}

impl Particle {
    pub fn new(id: u64, x: f64, y: f64, v: f64, theta: f64, radius: f64) -> Particle {
        Particle {
            id,
            x,
            y,
            v,
            theta,
            radius,
        }
    }

    fn get_id(&self) -> u64 {
        self.id
    }

    fn get_coordinates(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    fn get_radius(&self) -> f64 {
        self.radius
    }
}
