use std::collections::BTreeSet;

fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }

    if n == 2 {
        return true;
    }

    if n % 2 == 0 {
        return false;
    }

    let limit = (n as f64).sqrt() as i32;
    for i in (3..=limit).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }

    // let mut i = 3;
    // while i <= f64::sqrt(n.into()).floor() as i32 {
    //     if n % i == 0 {
    //         return false;
    //     }
    //     i += 2
    // }

    true
}

pub fn get(n: i32) -> Vec<Vec<i32>> {
    // let mut primes: HashSet<i32> = HashSet::new();
    // Collect all primes less than n into a BTreeSet
    let primes: BTreeSet<i32> = (2..n).filter(|&x| is_prime(x)).collect();

    // Find all pairs of primes that sum to n
    let mut res: Vec<Vec<i32>> = Vec::new();

    // for i in 2..n {
    //     if is_prime(i) {
    //         primes.insert(i);
    //     }
    // }

    // for p in primes.iter() {
    //     let compliment = n - p;
    //     if primes.contains(&compliment) && p < &compliment {
    //         res.push(vec![*p, compliment])
    //     }
    // }

    for &p in &primes {
        let compliment = n - p;

        if compliment > p && primes.contains(&compliment) {
            res.push(vec![p, compliment])
        }
    }

    res
}
