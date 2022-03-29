
pub fn p0006() {
    let mut qsum = 0;
    let mut sum = 0;
    for i in 1..=100 {
        qsum += i*i;
        sum += i;
    }
    
    println!("Problem 0006 Solution:");
    println!("{:?}", sum*sum - qsum);
}