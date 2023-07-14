import time

def chebyshev(pol, x):
    if pol == 0:
        return 1.0
    elif pol == 1:
        return x
    else:
        return 2.0 * x * chebyshev(pol - 1, x) - chebyshev(pol - 2, x)

def bisection(pol, a, b, precision):
    assert chebyshev(pol, a) * chebyshev(pol, b) < 0.0

    c = a
    while b - a >= precision:
        c = (a + b) / 2.0

        fc = chebyshev(pol, c)
        if fc == 0.0:
            break

        if fc * chebyshev(pol, a) < 0.0:
            b = c
        else:
            a = c

    return c, chebyshev(pol, c)

precisions = [1e-1, 1e-2, 1e-3, 1e-4, 1e-5, 1e-6]
ab = [(-1.0, 1.0), (0.2, 1.0), (-1.0, 1.0), (-0.8, 1.0)]

for p, (a, b) in enumerate(ab):
    p += 1
    for precision in precisions:
        begin = time.time()
        val = bisection(p, a, b, precision)
        end = time.time()
        print(f"{p} | {precision} | {end - begin:.6f} | ({val[0]:.6f}, {val[1]:.6f})")

    print("")
