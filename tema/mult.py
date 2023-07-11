import random
import time

class Matrix:
    def zero(row, col):
        v = []
        for i in range(row*col):
            v.append(0.0)
        return v

    def get(self, row, col):
        return self.v[row * self.col + col]
    
    def set(self, row, col, val):
        self.v[row * self.col + col] = val

    def __init__(self, row, col):
        self.row = row
        self.col = col
        self.v = Matrix.zero(row, col)

    def __mul__(self, rhs):
        m = self.col
        assert m == rhs.row
        n = self.row
        p = rhs.col
        out = Matrix(n, p)
        for i in range(n): 
            for j in range(p):
                s = 0.0
                for k in range(m):
                    s = s + self.get(i, k) * rhs.get(k, j)
                out.set(i, j, s)
        return out


# sizes = [16, 32, 64, 128, 256, 512, 1024, 2048]
# iters = [100, 75, 50, 25, 20, 10, 1, 1]
sizes = [512, 1024, 2048]
iters = [1, 1, 1]
for (size, iters_) in zip(sizes, iters):
    m1 = Matrix(size, size)
    m2 = Matrix(size, size)
    vals = [0, 0.5, 1.0, 1.5]
    for i in range(m1.row):
        for j in range(m1.col):
            m1.set(i, j, random.choice(vals))
            m2.set(i, j, random.choice(vals))

    begin = time.perf_counter_ns()
    for i in range(iters_):
        o = m1 * m2
    end = time.perf_counter_ns()
    elapsed = end-begin
    print(size, elapsed/iters_)