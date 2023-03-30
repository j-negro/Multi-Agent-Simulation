mod args;
mod io;
mod particle;
mod plot;
mod simulation;

use anyhow::Result;
use args::Cli;
use clap::Parser;
use simulation::Simulation;
use std::fs::File;

fn main() -> Result<()> {
    let args = Cli::parse();
    let mut file = File::create(args.simulation_output_path)?;

    let mut simulation = Simulation::new(args.length, args.noise_amplitude, args.particle_count);
    io::output_snapshot(
        &mut file,
        simulation.get_particles(),
        simulation.get_length(),
        0,
    )?;

    let mut orders_list = Vec::with_capacity(args.max_iterations as usize + 1);
    orders_list.push(simulation.get_order_parameter());

    for i in 0..args.max_iterations {
        simulation.run_cycle();

        let order = simulation.get_order_parameter();
        orders_list.push(order);

        io::output_snapshot(
            &mut file,
            simulation.get_particles(),
            simulation.get_length(),
            i + 1,
        )?;
    }


    if let Some(path) = args.graph_path {
        plot::order_time_graph(&path, orders_list)?;
    }

    Ok(())
}
