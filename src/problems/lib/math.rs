
/*
Custom math lib.
Mostly deals with primes.
*/

#[derive(Debug)]
pub struct SieveOfEratosthenes {
    pub sieve: Vec<u8>, // bit vector, 1 = prime, 0 = not prime.
    pub capacity: u64, // largest number in the sieve, capacity = sieve.len() * 8.
    pub primes: Vec<u64>,
}

impl SieveOfEratosthenes {

    fn is_prime_unsafe(&self, n: u64,  limit: u64) -> bool {
        // unsafe because it assumes all primes up to the limit are in primes[].
        let mut is_prime = true;
        for p in &self.primes {
            if p > &limit {break;}
            if n % p == 0 {
                is_prime = false;
                break;
            }
        }
        is_prime
    }

    fn extend_capacity_to(&mut self, c:u64) {
        if self.capacity >= c {
            return; // capacity is already large enough.
        }
        let end_index = ((c as f64) / 8.0f64).ceil() as u64;
        self.sieve.resize_with(end_index as usize, || {255});
        let start_index = self.capacity / 8;

        for i in start_index..end_index  {
            for j in 0..8u8 {
                let n = i*8 + j as u64;
                let limit = (n as f64).sqrt().ceil() as u64;
                if self.is_prime_unsafe(n, limit) {
                    self.primes.push(n);
                } else {
                    self.sieve[i as usize] &= 255u8 - (1u8<<j);
                }
            }
        }
        self.capacity = end_index * 8;
    }

    pub fn new() -> SieveOfEratosthenes {
        let sieve = vec![172];
        // 01234567
        // 00110101 <-> 10101100 = 172
        let capacity = 8u64;
        let primes = vec![2,3,5,7];

        SieveOfEratosthenes {
            sieve,
            capacity,
            primes,
        }
    }

    pub fn precompute(&mut self, n: u64) {
        self.extend_capacity_to(n); // this makes the next step safe
    }

    pub fn is_prime(&mut self, n: u64) -> bool {
        if n < self.capacity {
            // bit
            // the number is in the sieve -> extract it, if 1 => return true else false.
            let i = n / 8;
            let bit_shift = n % 8;
            if self.sieve[i as usize] & 1u8<<bit_shift == 0 {
                false
            } else {
                true
            }
        } else {
            let sqrt_n = (n as f64).sqrt().ceil() as u64;
            self.extend_capacity_to(sqrt_n); // this makes the next step safe
            
            self.is_prime_unsafe(n,sqrt_n)
        }
    }

    pub fn prime_factors(&mut self, n: u64) -> Vec<u64> {
        // this is an advanced algo that increases its prime count as it goes.
        let sqroot = (n as f64).sqrt().ceil() as u64;

        let mut nbr = n;
        let mut nbr_sqrt = sqroot;
        let mut i = 0;
        let mut p = self.primes[i];
        let mut factors: Vec<u64> = vec![];

        while p <= nbr_sqrt {
            if nbr % p == 0 {
                factors.push(p);
                nbr = nbr / p;
                nbr_sqrt = (nbr as f64).sqrt() as u64 + 1;
            } else {
                if i == self.primes.len() - 1 {
                    // we need more prime numbers, lets 2x our capacity.
                    //println!("DOUBLING CAPACITY {} {}", i, self.primes[i-1]);
                    self.extend_capacity_to(self.capacity*2);
                }
                p = self.primes[i+1];
                i += 1;
            }
        }
        if nbr > 1 {
            factors.push(nbr);
        }
        factors
    }
}