from gaussxw import gaussxwab
import math

def hermite(n, x):
    if n == 0:
        return 1
    if n == 1:
        return 2*x
    return 2*x*hermite(n - 1, x) - 2*(n-1)*hermite(n - 2, x)

def harmonic(n, x):
    bottom = math.sqrt((2**n) * math.factorial(n) * math.sqrt(math.pi))
    return 1/bottom * (math.e**(-1*(x**2)/2)) * hermite(n, x)

def f(n,x):
    return x*x*abs(harmonic(n, x))**2

N = 100
a = -1
b = 1
# Calculate the sample points and weights, then map them
# to the required integration domain
xp,wp = gaussxwab(N, a, b)
# Perform the integration
x2 = 0.0
for k in range(N):
    z = xp[k]
    x2 += wp[k] * (1+z*z)/((1-z*z)**2) * f(5, z/(1-z*z))

x = math.sqrt(x2)
print(f"<x^2> = {x2}")
print(f"sqrt(<x^2>) = {x}")
