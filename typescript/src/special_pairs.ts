function isPrime(num: number) {
  if (num <= 1) {
    return false;
  }
  if (num === 2) {
    return true; // 2 is prime
  }
  if (num % 2 === 0) {
    return false; // other even numbers are not prime
  }
  // Check for odd divisors from 3 up to the square root of num
  for (let i = 3; i <= Math.floor(Math.sqrt(num)); i += 2) {
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
