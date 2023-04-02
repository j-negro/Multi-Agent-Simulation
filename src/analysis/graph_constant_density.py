import os

import numpy as np
from matplotlib import pyplot as plt

# Simulation properties
MAX_ITERATIONS = 2000

os.makedirs("./data/graph_orders_results", exist_ok=True)
plt.rcParams["font.family"] = "serif"
plt.figure(figsize=(1280 / 108, 720 / 108), dpi=108)
plt.grid()

DIRECTORY_PATH = f"./data/particles_1600_lenght_20"

orders_per_noise_value = {}
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

        print(f"Processing file: {file} with noise value: {noise_value}")

        with open(f"{DIRECTORY_PATH}/{file}", "r") as f:
            orders: list[float] = []

            # Skip the first line
            f.readline()

            for line in f:
                orders.append(float(line))

        np_orders = np.array(orders)
        orders_per_noise_value[noise_value] = np_orders
# Sort the dictionary by the number of particles ascending
orders_per_noise_value = dict(sorted(orders_per_noise_value.items()))
for noise_value, np_orders in orders_per_noise_value.items():
    plt.plot(
        np.arange(0, MAX_ITERATIONS + 1),
        np_orders,
        label="η: {:.2f}".format(noise_value),
    )
plt.xlabel("Iterations")
plt.ylabel("Order parameter")

plt.legend()
plt.savefig(
    f"./data/graph_orders_results/Order_Parameters_Particles_1600_Length_20.png"
)
plt.clf()
