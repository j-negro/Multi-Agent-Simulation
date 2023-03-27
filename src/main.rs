mod args;
mod particle;
mod simulation;

use args::Cli;
use clap::Parser;
use simulation::Simulation;

fn main() {
    let args = Cli::parse();

    let simulation = Simulation::new(
        args.area_size,
        args.interaction_range,
        args.noise_amplitude,
        args.number_of_particles,
        args.m,
    );

    println!("Hello, world!");
}
