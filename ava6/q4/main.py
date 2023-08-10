from gaussxw import gaussxwab
import math

def w(t, x):
    K = 1.3806e-23
    C = 299_792_458
    H_ = 1.054571e-34
    const_part = (K**4 * t**4) / (4 * math.pi**2 * C**2 * H_**3)
    return const_part * x**3 / (math.e**x - 1)

T = 500
N = 1000
a = 0
b = 1
# Calculate the sample points and weights, then map them
# to the required integration domain
xp,wp = gaussxwab(N, a, b)
# Perform the integration
s = 0.0
for k in range(N):
    z = xp[k]
    s += wp[k] * 1/((1-z)**2) * w(T, z/(1-z))

o = s/T**4
print(f"w = {s}")
print(f"o = {o}")

