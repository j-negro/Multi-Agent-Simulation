START = 20

with open("orders_output.txt", "r") as f:
    count = int(f.readline())

    order_sum = 0
    for i in range(count):
        if i > START:
            order_sum += float(f.readline())

order = order_sum/(count - START)

print(order)