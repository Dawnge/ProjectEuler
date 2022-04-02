
/*
pub fn prime_factors(nbr: u64) -> Vec<u64> {
    let mut nbr = nbr;
    let mut limit = (nbr as f64).sqrt() as u64 + 1;
    let mut primes: Vec<u64> = vec![];

    let mut i = 2u64;
    // div by 2
    loop {
        if nbr % i == 0 {
            primes.push(i);
            nbr = nbr / i;
            limit = (nbr as f64).sqrt() as u64 + 1;
        } else {
            break;
        }
    }
    // div by uneven numbers, todo: figure frequencies out and use it here?
    i = 3;
    while nbr >= limit {
        if nbr % i == 0 {
            primes.push(i);
            nbr = nbr / i;
            limit = (nbr as f64).sqrt() as u64 + 1;
        } else {
            i += 2;
        }
    }
    return primes;
}



fn get_threshold(n: u64) -> u64 {
    
    // Handle lower end edge case:
    //assert!(n > 0);
    if n == 1 {
        return 1u64;
    }
    // This is the smallest config for n >= 2:
    let mut primes: Vec<u64> = vec![2,3];
    let mut prime_factors: Vec<u64> = vec![1];
    let mut factors: Vec<u64> = vec![2,1];
    let mut factors_len = 2;
    let mut current_nbr = 2;
    let mut done = false;
    loop {
        let f = factors.iter().fold(1, |mut acc, val| {acc*=val; acc});
        if done {
            let mut ret = 1;
            for i in 0..prime_factors.len() {
                ret *=  primes[i].pow(prime_factors[i] as u32);
            }
            return ret;
        }
        // not enough factors yet, lets improve.
        let mut new_max_index = 0;
        let mut new_max_value: f64 = ((current_nbr*primes[0]) as f64) / ((f / factors[0] * (factors[0] + 1) - f) as f64) ;
        for i in 1..factors.len() {
            let nfc = f / factors[i] * (factors[i] + 1);
            if nfc > n {
                new_max_index = i;
                done = true;
                break;
            }
            let v = ((current_nbr*primes[i]) as f64) / ((nfc - f) as f64) ;
            if v < new_max_value {
                new_max_index = i;
                new_max_value = v;
            }
        }
        if done {
            continue;
        }
        // case 1: we add a new prime number:
        if new_max_index == prime_factors.len() {
            prime_factors.push(1);
            factors[factors_len-1] += 1;
            factors.push(1);
            // get next prime number and add it to primes.
            let mut i = primes[factors_len-1] + 2;
            loop {
                let mut is_prime =  true;
                for p in &primes {
                    if i % p == 0 {
                        is_prime = false;
                        break;
                    }
                }
                if is_prime {
                    primes.push(i);
                    break;
                }
                i+=2;
            }
            current_nbr *= primes[factors_len - 1];
            factors_len +=1;

        } else {
            // case 2: we increment an already existing number
            prime_factors[new_max_index] += 1;
            factors[new_max_index] += 1;
            current_nbr *= primes[new_max_index];
        }
    }
}
 */

fn triangle_number(idx: u64) -> u64 {
    (idx+1) * idx /2
}
fn factor_count(prime_factors: Vec<u64>) -> u64 {
    // pf = [2,2,2,5,13]
    // f = 3,1,1

    // edge case
    if prime_factors.len() == 0 {
        return 1;
    }

    let mut f: Vec<u64> = vec![];
    let mut last_p = prime_factors[0];
    let mut last_p_count = 1;
    for i in 1..prime_factors.len() {
        if prime_factors[i] == last_p {
            last_p_count+=1;
        } else {
            f.push(last_p_count);
            last_p_count = 1;
            last_p = prime_factors[i];
        }
    }
    f.push(last_p_count);
    f.iter().fold(1, |mut acc, val| {acc*=(val+1); acc})
   
}

use crate::problems::lib::math::SieveOfEratosthenes;

pub fn p0012() {
    use std::time::Instant;
    let now = Instant::now();

   
    let min_divisor_count = 501;

    let mut sieve = SieveOfEratosthenes::new();

    /*
    let threshold =  get_threshold(min_divisor_count);
    let mut i = (0.5f64 * ((8.0f64*(threshold as f64) + 1.0f64).sqrt() - 1.0f64)) as u64;
    if i % 2 != 0 {
        i-=1;
    }
    let mut fc_old = factor_count(sieve.prime_factors(i/2));
 */
    let mut i = 2;
    let mut fc_old = factor_count(sieve.prime_factors(i/2));

    loop {
        let fc_new;
        if i % 2 == 0 {
            fc_new = factor_count(sieve.prime_factors(i+1));
        } else {
            fc_new = factor_count(sieve.prime_factors((i+1)/2));
        }
        let fc = fc_new * fc_old;
        if fc > min_divisor_count {
            break;
        }
        fc_old = fc_new;
        i+=1;
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    println!("Problem 0012 Solution:");
    println!("{:?}", triangle_number(i));


}