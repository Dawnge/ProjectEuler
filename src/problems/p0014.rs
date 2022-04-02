fn collatz_step(n: u32) -> u32 {
    if n % 2 == 0 {
        n/2
    } else {
        3*n + 1
    }
}


pub fn p0014() {
    use std::time::Instant;
    let now = Instant::now();
    const MAX: usize = 999_999;
    let mut lookup_table = [0u32; MAX];
    let mut i = 3u32;
    for i in 0..=(MAX as f64).log2() as usize {
        lookup_table[2u64.pow(i as u32) as usize] = (i as u32) + 1u32;
    }

    loop {
        if i >= MAX as u32 {
            break;
        }
        if lookup_table[i as usize] == 0 { // we havent seen this number yet, lets trace its chain.
            let mut trace: Vec<u32> = vec![i];
            let mut end_weight: u32;
            let mut next = i;
            loop {
                next = collatz_step(next);
                if next < MAX as u32 && lookup_table[next as usize] != 0{
                    end_weight = lookup_table[next as usize];
                    break;
                } else {
                    trace.push(next);
                }
            }
            for v in trace.iter().rev() {
                end_weight+=1;
                if (*v as usize) < MAX {
                    lookup_table[*v as usize] = end_weight;
                }
            }
        }
        i+=1;
    }
    let mut iter = lookup_table.iter().enumerate();
    let init = iter.next().unwrap();
    let res = iter.fold(init, |acc, num| {
        if num.1 > acc.1 {
            num
        } else {
            acc
        }
    });
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    println!("Problem 0014 Solution:");
    println!("{:?}", res.0);

}