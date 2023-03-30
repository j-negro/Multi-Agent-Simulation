use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "Multi Agent Simulation", author, version, about)]
pub struct Cli {
    #[arg(short, long)]
    pub particle_count: usize,
    #[arg(short, long)]
    pub noise_amplitude: f64,
    #[arg(short, long)]
    pub length: f64,
    #[arg(short, long, default_value_t = 1000)]
    pub max_iterations: u32,
    #[arg(short, long, default_value_t = String::from("./output.xyz"))]
    pub output_path: String,
    #[arg(short, long)]
    pub graph_path: Option<String>,
}
