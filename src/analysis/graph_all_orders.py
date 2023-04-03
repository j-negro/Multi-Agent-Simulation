import os

import matplotlib.pyplot as plt
import numpy as np

START = 1000

# Noise properties
NOISE_LOWER_BOUND = 0.0
NOISE_UPPER_BOUND = 3.0
NOISE_STEP = 0.25

# Density properties
density_values = [
    (100, 5),
    (400, 10),
    (900, 15),
    (1600, 20),
    (2500, 25),
    (3600, 30),
    (4900, 35),
]

metrics = []

for num_particles, length in density_values:
    directory = f"./data/particles_{num_particles}_lenght_{length}"

    for noise in np.arange(
        NOISE_LOWER_BOUND, NOISE_UPPER_BOUND + NOISE_STEP, NOISE_STEP
    ):
        try:
            with open(
                f"{directory}/noise_{round(noise, 2)}.txt",
                "r",
            ) as f:
                orders: list[float] = []
                # Skip first START lines
                for _ in range(START):
                    f.readline()

                for line in f:
                    orders.append(float(line))

        except FileNotFoundError:
            print("File not found")
            raise SystemExit

        np_orders = np.array(orders)
        metrics.append(
            {
                "num_particles": num_particles,
                "noise": noise,
                "mean": np.mean(np_orders),
            }
        )

plt.rcParams["font.family"] = "serif"
plt.figure(figsize=(1280 / 108, 720 / 108), dpi=108)

os.makedirs("./data/graph_all_orders_result", exist_ok=True)

# Graph every density value with a distinct marker and color
markers = ["+", "x", "<", ">", "s", "*", "H", "X", "d"]

for num_particles, length in density_values:
    metrics_for_num_particles = [
        metric for metric in metrics if metric["num_particles"] == num_particles
    ]

    plt.scatter(
        [metric["noise"] for metric in metrics_for_num_particles],
        [metric["mean"] for metric in metrics_for_num_particles],
        label=f"N: {num_particles}",
        marker=markers.pop(),
    )

    plt.xlabel("Noise (radians)")
    plt.ylabel("Order Parameter")
    plt.grid()
plt.legend()
plt.savefig(f"./data/graph_all_orders_result/Density_4.png")
