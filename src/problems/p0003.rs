pub fn p0003() {
    let mut nbr = 600851475143u64;
    let mut limit = (nbr as f64).sqrt() as u64 + 1;
    let mut primes: Vec<u64> = vec![];

    let mut i = 3u64;
    while nbr >= limit {
        if nbr % i == 0 {
            primes.push(i);
            nbr = nbr / i;
            limit = (nbr as f64).sqrt() as u64 + 1;
        } else {
            i += 2;
        }
    }
    println!("Problem 0003 Solution:");
    println!("{}", primes.iter().max().unwrap());
}