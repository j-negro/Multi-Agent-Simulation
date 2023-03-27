mod args;
mod io;
mod particle;
mod simulation;

use anyhow::Result;
use args::Cli;
use clap::Parser;
use io::output_snapshot;
use simulation::Simulation;
use std::fs::File;

fn main() -> Result<()> {
    let args = Cli::parse();
    let mut file = File::create(args.output_path)?;

    let mut simulation = Simulation::new(
        args.area_size,
        args.interaction_range,
        args.noise_amplitude,
        args.number_of_particles,
        args.m,
    );
    output_snapshot(
        &mut file,
        simulation.get_particles(),
        simulation.get_length(),
        0,
    )?;

    for i in 0..args.max_iterations {
        simulation.advance_time();
        output_snapshot(
            &mut file,
            simulation.get_particles(),
            simulation.get_length(),
            i + 1,
        )?;
    }

    Ok(())
}
