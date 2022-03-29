pub fn p0002() {
    let mut t1 = 1u32;
    let mut t2 = 2u32;
    let mut acc = t2;
    loop {
        let t3 = t1 + t2;
        if t3 > 4000000{break;}
        if t3 % 2 == 0 {
            acc += t3;
        }
        t1 = t2;
        t2 = t3;
    }
    println!("Problem 0002 Solution:");
    println!("{}", acc);
}