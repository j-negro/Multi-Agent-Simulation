import numpy as np

START = 20

try:
    with open("orders_output.txt", "r") as f:
        count = int(f.readline())

        orders: list[float] = []
        for i in range(count):
            if i > START:
                orders.append(float(f.readline()))
except FileNotFoundError:
    print("File not found")
    raise SystemExit

np_orders = np.array(orders)
print("Mean: ", np.mean(np_orders))
print("Std: ", np.std(np_orders))
