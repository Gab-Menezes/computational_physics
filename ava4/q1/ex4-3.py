from numpy import zeros, random
N = 10
C = zeros([N,N],float)
A = random.rand(N,N)
B = random.rand(N,N)
for i in range(N):
    for j in range(N):
        for k in range(N):
            C[i,j] += A[i,k]*B[k,j]
print(C)
