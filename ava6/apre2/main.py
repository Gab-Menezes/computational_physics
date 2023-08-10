from gaussxw import gaussxwab
import numpy as np
import math

def f(x, y, z):
    return abs(x)*abs(y)


target_point = np.array([0, 0, 2])
N = 100
a = -1
b = 1
# Calculate the sample points and weights, then map them
# to the required integration domain
xp,xwp = gaussxwab(N, a, b)
yp,ywp = gaussxwab(N, a, b)
zp,zwp = gaussxwab(N, a, b)
# Perform the integration
s = np.zeros(3)
for i in range(N):
    x = xp[i]
    xw = xwp[i]
    for j in range(N):
        y = yp[j]
        yw = ywp[j]
        for k in range(N):
            z = zp[k]
            zw = zwp[k]
            integral_val = xw * yw * zw * f(x, y, z)
            current_point = np.array([x, y, z])
            dir_vec = target_point - current_point
            mag = np.linalg.norm(dir_vec)
            r = dir_vec / mag
            s += integral_val/mag**2 * r

print(s)