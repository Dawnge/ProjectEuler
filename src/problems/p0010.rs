
fn sieve_of_eratosthenes(limit: u64) -> Vec<u64> {
    let limit = limit -1;
    // not memory efficient but fast.
    let mut sieve: Vec<u8> = vec![1; limit as usize];
    let limit_log = (limit as f64).sqrt() as u64 + 1;
    for i in 0..limit_log {
        let base = i + 2; // first entry is 2, second is 3...
        if sieve[i as usize] > 0 {
            // it is a prime number, eliminate every entry that is a multiple of it.
            let mut nbr = i + base;
            while nbr < limit {
                sieve[nbr as usize] = 0;
                nbr += base;
            }
        }
    }
    let mut primes: Vec<u64> = vec![];
    for i in 0..limit {
        if sieve[i as usize] > 0 {
            primes.push(i + 2);
        }
    }
    
    primes
}

pub fn p0010() {
    let sieve = sieve_of_eratosthenes(2_000_000);
    let s: u64 = sieve.iter().sum();
    println!("Problem 0010 Solution:");
    println!("{:?}", s);
}