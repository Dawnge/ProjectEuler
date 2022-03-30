
pub fn p0009() {
    let mut sol = 0;
    for a in 0..1000 {
        for b in (a+1)..(1000-a) {
            let c = 1000 - a - b;
            if a*a + b*b == c*c {
                sol = a*b*c;
            }
        }
    }
    println!("Problem 0009 Solution:");
    println!("{:?}", sol);
}