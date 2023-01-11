import numpy as np
coprime_count = 1

def euclid_algorithm(a, b):
    if b == 0:
        return a
    else:
        return euclid_algorithm(b, a % b)
running_total = 1

# Pi is the square root of 6 times the total / co-prime count
for i in range(100000000):
    a = np.random.randint(1, 999999999)
    b = np.random.randint(1, 999999999)
    if b > a:
        a,b = b,a
    if a == b:
        continue
    result = euclid_algorithm(a, b)
    if result == 1:
        coprime_count += 1
    running_total += 1
print(np.sqrt(6 * (running_total / coprime_count)))