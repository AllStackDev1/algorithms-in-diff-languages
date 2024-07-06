def is_prime(num):
    if num <= 1:
        return False
    if num == 2:
        return True
    if num % 2 == 0:
        return False
    
    for i in range(3, int(num**0.5) + 1, 2):
        if num % i == 0:
            return False
    return True
    
def special_pairs(n):
    primes = set()
    res = []
    
    for i in range(1, n-1):
        if is_prime(i):
            primes.add(i)
    
    # Find special pairs
    for p in primes:
        complement = n - p
        if complement in primes and p < complement:
            res.append([p, complement])
    
    return res

print("Special Pairs - PY %s" % special_pairs(18))