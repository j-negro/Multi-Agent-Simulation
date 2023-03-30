use neighbors::Particle as MethodParticle;
use std::io::Write;

use super::Result;
use crate::particle::Particle;
use std::{fs::File, io::BufWriter};

pub fn output_snapshot(
    file: &mut File,
    particles: &[Particle],
    lenght: f64,
    time: u32,
) -> Result<()> {
    let mut writer = BufWriter::new(file);

    let particles_count = particles.len();

    // Print the number of particles
    writeln!(writer, "{}", particles_count + 4)?;

    // Print simulation properties
    writeln!(
        writer,
        "Properties=id:I:1:pos:R:2:velo:R:2:theta:R:1 pbc=\"T T\" Time={time}",
    )?;

    for particle in particles {
        let (x, y) = particle.get_coordinates();
        let (v_x, v_y) = particle.get_velocity_coordinates();

        writeln!(
            writer,
            "{} {:.6} {:.6} {:.6} {:.6} {:.6}",
            particle.get_id(),
            x,
            y,
            v_x,
            v_y,
            particle.get_angle()
        )?;
    }

    // Print particles at the corners
    writeln!(
        writer,
        "{} {:.6} {:.6} {:.6} {:.6} {:.6}",
        particles_count, 0.0, 0.0, 0.0, 0.0, 0
    )?;
    writeln!(
        writer,
        "{} {:.6} {:.6} {:.6} {:.6} {:.6}",
        particles_count + 1,
        0.0,
        lenght,
        0.0,
        0.0,
        0
    )?;
    writeln!(
        writer,
        "{} {:.6} {:.6} {:.6} {:.6} {:.6}",
        particles_count + 2,
        lenght,
        0.0,
        0.0,
        0.0,
        0
    )?;
    writeln!(
        writer,
        "{} {:.6} {:.6} {:.6} {:.6} {:.6}",
        particles_count + 3,
        lenght,
        lenght,
        0.0,
        0.0,
        0
    )?;

    Ok(())
}
