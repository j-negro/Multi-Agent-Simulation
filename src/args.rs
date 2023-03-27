use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "Multi Agent Simulation", author, version, about)]
pub struct Cli {
    #[arg()]
    pub number_of_particles: usize,
    #[arg()]
    pub interaction_range: f64,
    #[arg()]
    pub m: usize,
    #[arg()]
    pub noise_amplitude: f64,
    #[arg()]
    pub area_size: f64,
    #[arg(short, long, default_value_t = 1000)]
    pub max_iterations: u32,
    #[arg(short, long, default_value_t = String::from("./output.xyz"))]
    pub output_path: String,
}
