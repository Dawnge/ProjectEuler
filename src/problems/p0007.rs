
pub fn p0007() {
    // Since i dont know how large my Sieve of Eratosthenes is supposed to be
    // we approach it from a different direction e.g. building it up.
    // Less compuationally efficient, but better space efficiency. for 10001 prime numbers acceptable.
    let mut primes: Vec<u32> = vec![2];
    let mut i = 3;
    let mut count = 1;
    while count < 10001 {
        let log2_i = (i as f32).sqrt() as u32 + 1;
        let mut is_prime = true;
        for p in primes.iter() {
            if p > &log2_i {
                break;
            } else if i % p == 0 {
                    is_prime = false;
                    break;
            }
        }
        if is_prime {
            count +=1;
            primes.push(i);
        }
        i+=1;
    }
    
    println!("Problem 0007 Solution:");
    println!("{:?}", primes.iter().last().unwrap());
}