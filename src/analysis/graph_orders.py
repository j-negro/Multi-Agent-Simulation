import os

import numpy as np
from matplotlib import pyplot as plt

START = 100

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

os.makedirs("./data/constant_order_graphs", exist_ok=True)

# Plot the order parameter as a function of noise for different densities
for noise in noise_values:
    for num_particles in num_particles_values:
        try:
            with open(
                f"./data/particles_{num_particles}/txt/noise_{round(noise, 2)}.txt",
                "r",
            ) as f:
                orders: list[float] = []

                # Skip the first line
                for _ in range(0, 2):
                    f.readline()

                for line in f:

                    orders.append(float(line))

        except FileNotFoundError:
            print("File not found")
            raise SystemExit

        np_orders = np.array(orders)

        plt.plot(
            np.arange(0, MAX_ITERATIONS), np_orders, label=f"{num_particles} particles"
        )

    plt.xlabel("Iterations")
    plt.ylabel("Order parameter")

    plt.legend()
    plt.savefig(
        f"./data/constant_order_graphs/Order_Parameters_Noise_{round(noise, 2)}.png"
    )
    plt.clf()
