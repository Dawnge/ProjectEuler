
pub fn p0015() {
    use std::time::Instant;
    let now = Instant::now();

    /*
    Its the tree thingy:

   0       1
   1      1 1
   2     1 2 1       <-- 2 for 1x1
   3    1 3 3 1
   4   1 4 6 4 1     <-- 6 for 2x2
   5  1 5 10105 1
   6       20        <-- 20 for 3x3

          ...
    */
    let square_side_length = 20;

    let mut v = vec![1u64];

    for _ in 0..square_side_length*2 {
        let mut tmp = vec![1u64];
        for i in 1..v.len() {
            tmp.push(v[i-1] + v[i]);
        }
        tmp.push(1);
        v = tmp;
    }
    let sol = v[square_side_length];

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    println!("Problem 0015 Solution:");
    println!("{:?}", sol);

}