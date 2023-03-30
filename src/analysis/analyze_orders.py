import numpy as np

START = 20

try:
    with open("orders_output.txt", "r") as f:
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
print("Mean: ", np.mean(np_orders))
print("Std: ", np.std(np_orders))
