pub fn p0001() {
    let mut acc = 0u32;
    for i in 1..1000 {
        acc += if i % 3 == 0 || i % 5 == 0 { i } else { 0 };
    }
    println!("Problem 0001 Solution:");
    println!("{}", acc);
}