
pub fn prime_factors(nbr: u64) -> Vec<u64> {
    let mut nbr = nbr;
    let mut limit = (nbr as f64).sqrt() as u64 + 1;
    let mut primes: Vec<u64> = vec![];

    let mut i = 2u64;
    while nbr >= limit {
        if nbr % i == 0 {
            primes.push(i);
            nbr /= i;
            limit = (nbr as f64).sqrt() as u64 + 1;
        } else {
            i += 1;
        }
    }
    primes
}

pub fn p0005() {

    /*
    ill start with explanations:
    Note: In this case sets respect multiple occurances, e.g. {0,0} != {0},
        however order is not important e.g. {0_a, 0_b} == {0_b, 0_a} where 0_a = 0_b = 0.
        the size of a set is determined by its member count e.g. |{0,0,0}| = 3; |{6,2}| = 2

    if you look at prime factorization of 2520 you will see:
    2520-set = {2, 2, 2, 3, 3, 5, 7}
    multiple criteria:
        now let a be a number in [2,...,10] with prime factorization F_A = {...}
        F_A is a subset of the 2520-set.
    For the smallest criteria:
        the 2520-set is the smallest set that statisfies the multiple criteria.
    -> 2520 is the smallest positive number that is evenly divisible by all of the numbers from 1 to 10.

    Extend this idea to 20 and you got an efficient solution.
    */
    //println!("{:?}", prime_factors(2520));
    let limit = 20;
    let mut v: Vec<u64> = vec![0; limit+1];

    for i in 2..limit {
        let factors = prime_factors(i as u64);
        let mut count = 0;
        let mut prev_value = factors[0];
        for fac in factors {
            if fac == prev_value {
                count += 1;
            } else {
                v[prev_value as usize] = v[prev_value as usize].max(count);
                count = 1;
                prev_value = fac;
            }
        }
        v[prev_value as usize] = v[prev_value as usize].max(count); // last step
    }
    
    let mut res = 1;

    for (i, val) in v.iter().enumerate().take(limit+1).skip(2) {
        if *val != 0 {
            res *= (i as u64).checked_pow(*val as u32).unwrap();
        } 
    }


    println!("Problem 0005 Solution:");
    println!("{:?}", res);
}