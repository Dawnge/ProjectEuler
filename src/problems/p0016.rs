
pub fn p0016() {
    use std::time::Instant;
    let now = Instant::now();

    const POWER_OF_2: u32 = 1000;
    /*
    for an example we start with number [8, 0] = 8
    step 1: *2 and record if number produces carry
    number = [(8*2) % 10 = 6, 0] = 6
    carry =  [0, 1]
    apply carry to number and add fix last 0: [6 + 0, 0 + 1] => [6,1,0] = 16
    
    */
    let mut nbr = vec![1,0];
    for _ in 0..POWER_OF_2 {
        let mut carry:Vec<u8> = vec![0];
        for n in nbr.iter_mut() {
            if n >= &mut 5 {
                carry.push(1);
            } else {
                carry.push(0);
            }
            *n = (*n * 2) % 10;
        }
        carry.remove(carry.len()-1);
        for (i, c) in carry.iter().enumerate() {
            nbr[i] += c;
        }
        if nbr.last().unwrap() != &0 {
            nbr.push(0);
        }
    }
    let sol: u32 = nbr.iter().map(|x| {*x as u32}).sum();

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    println!("Problem 0016 Solution:");
    println!("{:?}", sol);

}