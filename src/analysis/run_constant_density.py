import os
import subprocess

import numpy as np

# Simulation properties
MAX_ITERATIONS = 1000

# Noise properties
NOISE_LOWER_BOUND = 0.0
NOISE_UPPER_BOUND = 5.0
NOISE_STEP = 0.25

# Density properties
SIMULATION_LENGTH = 5
NUM_PARTICLES_LOWER_BOUND = 100
NUM_PARTICLES_UPPER_BOUND = 1000
NUM_PARTICLES_STEP = 100

FIXED_DENSITY = 4
length_values = np.array(range(5, 25, 5))

num_particles_values = np.array(
    [FIXED_DENSITY * length**2 for length in length_values]
)
noise_values = np.arange(NOISE_LOWER_BOUND, NOISE_UPPER_BOUND + NOISE_STEP, NOISE_STEP)


for num_particles in num_particles_values:
    # Create a folder for the data
    os.makedirs(f"./data/particles_{num_particles}/txt", exist_ok=True)

    for noise in noise_values:
        subprocess.run(
            [
                "./target/release/multi-agent-simulation",
                "-p",
                str(num_particles),
                "-n",
                str(noise),
                "-l",
                str(SIMULATION_LENGTH),
                "-m",
                str(MAX_ITERATIONS),
                "-o",
                f"./data/particles_{num_particles}/txt/noise_{round(noise, 2)}.txt",
            ]
        )
