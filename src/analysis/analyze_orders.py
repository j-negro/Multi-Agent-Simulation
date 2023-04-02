import matplotlib.pyplot as plt
import numpy as np

START = 100

# Noise properties
NOISE_LOWER_BOUND = 0.0
NOISE_UPPER_BOUND = 5.0
NOISE_STEP = 0.25

# Density properties
NUM_PARTICLES_LOWER_BOUND = 100
NUM_PARTICLES_UPPER_BOUND = 1000
NUM_PARTICLES_STEP = 100

metrics = []

for num_particles in np.arange(
    NUM_PARTICLES_LOWER_BOUND,
    NUM_PARTICLES_UPPER_BOUND + NUM_PARTICLES_STEP,
    NUM_PARTICLES_STEP,
):
    for noise in np.arange(
        NOISE_LOWER_BOUND, NOISE_UPPER_BOUND + NOISE_STEP, NOISE_STEP
    ):
        try:
            with open(
                f"./data/particles_{num_particles}/txt/noise_{round(noise, 2)}.txt",
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
                "std": np.std(np_orders),
            }
        )

plt.rcParams["font.family"] = "serif"
plt.figure(figsize=(1280 / 108, 720 / 108), dpi=108)
plt.grid()

# For every number of particles, plot the mean and std of the orders as a function of noise
for num_particles in np.arange(
    NUM_PARTICLES_LOWER_BOUND,
    NUM_PARTICLES_UPPER_BOUND + NUM_PARTICLES_STEP,
    NUM_PARTICLES_STEP,
):
    metrics_for_num_particles = [
        metric for metric in metrics if metric["num_particles"] == num_particles
    ]

    # Plot the mean as scatter plot, and the std as error bars
    plt.errorbar(
        [metric["noise"] for metric in metrics_for_num_particles],
        [metric["mean"] for metric in metrics_for_num_particles],
        yerr=[metric["std"] for metric in metrics_for_num_particles],
        fmt="bx",
        ecolor="r",
        capsize=5,
    )

    plt.xlabel("Noise (radians)")
    plt.ylabel("Order")
    plt.savefig(f"./results/particles_{num_particles}.png")
    plt.clf()
