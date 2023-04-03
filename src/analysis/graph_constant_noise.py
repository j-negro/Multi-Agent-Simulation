import os

import numpy as np
from matplotlib import pyplot as plt

START = 100

# Simulation properties
MAX_ITERATIONS = 2000

os.makedirs("./data/graph_orders_results", exist_ok=True)
plt.rcParams["font.family"] = "serif"
plt.figure(figsize=(1280 / 108, 720 / 108), dpi=108)
plt.grid()

FIXED_NOISE = 2.0
parameters = [
    (100, 5),
    # (400, 10),
    (900, 15),
    (1600, 20),
    # (2500, 25),
    # (3600, 30),
    (4900, 35),
]

orders_per_num_particles = {}
for num_particles, length in parameters:
    DIRECTORY_PATH = f"./data/particles_{num_particles}_lenght_{length}"
    for file in os.listdir(DIRECTORY_PATH):
        if file.endswith(".txt"):
            # The filenames are in the format: "noise_0.0.txt". Parse the noise value
            noise_with_extension = file.split("_")[-1]
            noise_value_str = (
                noise_with_extension.split(".")[0]
                + "."
                + noise_with_extension.split(".")[1]
            )
            noise_value = float(noise_value_str)
            with open(f"{DIRECTORY_PATH}/{file}", "r") as f:
                orders: list[float] = []

                # Skip the first line
                f.readline()

                for line in f:
                    orders.append(float(line))

            np_orders = np.array(orders)
            orders_per_num_particles[num_particles] = np_orders

# Sort the dictionary by the number of particles ascending
orders_per_num_particles = dict(sorted(orders_per_num_particles.items()))
for num_particles, np_orders in orders_per_num_particles.items():
    plt.plot(np.arange(0, MAX_ITERATIONS + 1), np_orders, label=f"N: {num_particles}")
plt.xlabel("Iterations")
plt.ylabel("Order Parameter")

plt.legend()
plt.savefig(
    "./data/graph_orders_results/Density_4_Noise_{:.2f}.png".format(FIXED_NOISE)
)
plt.clf()
