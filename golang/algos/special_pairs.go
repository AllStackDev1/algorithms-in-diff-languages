package special_pairs

import (
	"math"
)

func isPrime(n int32) bool {
	if n <= 1 {
		return false
	}

	if n == 2 {
		return true
	}

	if n % 2 == 0 {
		return false
	}

	for i := int32(3); i <= int32(math.Sqrt(float64(n))); i += 2 {
		if n % i == 0 {
			return false
		}
	}

	return true
}

// Get finds all unique pairs of prime numbers that sum up to n.
func Get(n int32) [][]int32 {
	primes := []int32{}
	primeSet := make(map[int32]struct{}) // To quickly check for the presence of a prime

	// Find all primes less than n and store them in a set for quick lookup
	for i := int32(2); i < n; i++ {
		if isPrime(i) {
			primes = append(primes, i)
			primeSet[i] = struct{}{}
		}
	}

	res := [][]int32{}
	seen := make(map[int32]struct{}) // To avoid adding duplicate pairs

	// Find special pairs
	for _, p := range primes {
		compliment := n - p
		if _, exists := primeSet[compliment]; exists && p < compliment {
			if _, pairSeen := seen[p]; !pairSeen {
				res = append(res, []int32{p, compliment})
				seen[compliment] = struct{}{} // Mark the compliment as seen
			}
		}
	}

	return res
}