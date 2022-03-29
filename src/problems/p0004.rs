fn is_pal(mut i: u32) -> bool {
    let mut v = vec![];
    while i != 0 {
        v.push(i % 10);
        i -= i % 10;
        i /= 10;
    }
    let mut r = v.clone();
    r.reverse();
    return r == v;
}

pub fn p0004() {
    let mut n1 = 0;
    let mut n2 = 0;
    let mut pal = 0;
    for a in 900..999 {
        for b in 900..a {
            let v = a*b;
            if is_pal(v) && v > pal {
                pal = v;
                n1 = a;
                n2 = b;
            }
        }
    }
    println!("Problem 0004 Solution:");
    println!("{} a={} b={}", pal, n1, n2);
}