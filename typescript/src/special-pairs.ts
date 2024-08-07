function isPrime(num: number) {
  if (num <= 1) {
    return false;
  }

  // Check for odd divisors from 2 up to the square root of num
  for (let i = 2; i <= Math.sqrt(num); i++) {
    if (num % i === 0) {
      return false;
    }
  }

  return true;
}

export const specialPairs = (n: number) => {
  const primes = new Set<number>();
  const res: [number, number][] = [];

  // Collect prime numbers less than n
  for (let i = 1; i < n; i++) {
    if (isPrime(i)) {
      primes.add(i);
    }
  }

  // Find special pairs
  primes.forEach((i) => {
    const complement = n - i;
    if (primes.has(complement) && i < complement) {
      res.push([i, complement]);
    }
  });

  return res;
};
