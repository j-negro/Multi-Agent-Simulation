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


for num_particles in range(
    NUM_PARTICLES_LOWER_BOUND,
    NUM_PARTICLES_UPPER_BOUND + NUM_PARTICLES_STEP,
    NUM_PARTICLES_STEP,
):
    # Create a folder for the data
    os.makedirs(f"./src/analysis/data/particles_{num_particles}/txt", exist_ok=True)
    os.makedirs(f"./src/analysis/data/particles_{num_particles}/graphs", exist_ok=True)

    for noise in np.arange(
        NOISE_LOWER_BOUND, NOISE_UPPER_BOUND + NOISE_STEP, NOISE_STEP
    ):
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
                f"./src/analysis/data/particles_{num_particles}/txt/noise_{round(noise, 2)}.txt",
                "-g",
                f"./src/analysis/data/particles_{num_particles}/graphs/noise_{round(noise, 2)}.png",
            ]
        )
