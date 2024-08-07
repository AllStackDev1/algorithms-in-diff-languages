def is_prime(num):
 # Corner case
    if (num <= 1):
        return False

    # Check from 2 to sqrt(n)
    for i in range(2, int(num**0.5)+1):
        if (num % i == 0):
            return False

    return True
    
def special_pairs(n):
    primes = set(i for i in range(1, n-1) if is_prime(i))
    res = []
    
    # for i in range(1, n-1):
    #     if is_prime(i):
    #         primes.add(i)
    
    # Find special pairs
    for p in primes:
        complement = n - p
        if complement in primes and p < complement:
            res.append([p, complement])
    
    return res
