use neighbors::Particle as MethodParticle;
use std::io::Write;

use super::Result;
use crate::particle::Particle;
use std::{fs::File, io::BufWriter};

pub fn output_snapshot(file: &mut File, particles: &[Particle]) -> Result<()> {
    let mut writer = BufWriter::new(file);

    // Print the number of particles
    writeln!(writer, "{}", particles.len())?;

    // Print simulation properties
    writeln!(writer, "Properties=\"id:I:1:pos:R:2:vel:R:2:theta:R:1\"",)?;

    for particle in particles {
        let (x, y) = particle.get_coordinates();
        let (v_x, v_y) = particle.get_velocity_coordinates();
        let theta = particle.get_angle();

        writeln!(
            writer,
            "{} {:.32} {:.32} {:.32} {:.32} {:.32}",
            particle.get_id(),
            x,
            y,
            v_x,
            v_y,
            theta
        )?;
    }

    Ok(())
}
