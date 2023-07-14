import matplotlib.pyplot as plt
import math

def up(n):
    s = 0
    for i in range(1, n+1):
        s += 1 / i
    return s

def down(n):
    s = 0
    for i in range(n, 0, -1):
        s += 1 / i
    return s

xs = []
ys = []
for i in range(1, 10_000):
    u = up(i)
    d = down(i)
    y = (u - d)/(abs(u) + abs(d))
    xs.append(i)
    ys.append(y)

plt.loglog(xs, ys)
plt.savefig("output/p.png")