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

FIXED_LENGTH = 20
FIXED_NOISE = 1.0

DIRECTORY_PATH = f"./data/noise_{FIXED_NOISE}_length_{FIXED_LENGTH}"

orders_per_num_particles = {}
for file in os.listdir(DIRECTORY_PATH):
    if file.endswith(".txt"):
        num_particles = int(file.split(".")[0].split("_")[1])

        if num_particles != 200 and num_particles != 1000 and num_particles != 2000:
            continue

        with open(f"{DIRECTORY_PATH}/{file}", "r") as f:
            orders: list[float] = []

            # Skip the first line
            for _ in range(0, 2):
                f.readline()

            for line in f:

                orders.append(float(line))

        np_orders = np.array(orders)
        orders_per_num_particles[num_particles] = np_orders
# Sort the dictionary by the number of particles ascending
orders_per_num_particles = dict(sorted(orders_per_num_particles.items()))
for num_particles, np_orders in orders_per_num_particles.items():
    plt.plot(np.arange(0, MAX_ITERATIONS), np_orders, label=f"N: {num_particles}")
plt.xlabel("Iterations")
plt.ylabel("Order parameter")

plt.legend()
plt.savefig(
    f"./data/graph_orders_results/Order_Parameters_Noise_{FIXED_NOISE}_Length_{FIXED_LENGTH}.png"
)
plt.clf()
