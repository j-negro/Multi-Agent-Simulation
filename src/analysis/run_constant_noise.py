import os
import subprocess

import numpy as np

# Simulation properties
MAX_ITERATIONS = 2000


FIXED_LENGTH = 20
FIXED_NOISE = 1.0
num_particles_values = np.array(range(200, 2000 + 200, 200))


for num_particles in num_particles_values:
    length = FIXED_LENGTH

    # Create a folder for the data
    os.makedirs(
        f"./data/noise_{round(FIXED_NOISE, 2)}_length_{FIXED_LENGTH}", exist_ok=True
    )

    subprocess.run(
        [
            "./target/release/multi-agent-simulation",
            "-p",
            str(num_particles),
            "-n",
            str(round(FIXED_NOISE, 2)),
            "-l",
            str(length),
            "-m",
            str(MAX_ITERATIONS),
            "-o",
            f"./data/noise_{round(FIXED_NOISE, 2)}_length_{FIXED_LENGTH}/particles_{num_particles}.txt",
        ]
    )
