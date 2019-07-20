fn main() {
    let s: &str = "125";
    const dec: usize = 10;

    let mut nums = Vec::new();

    for c in s.to_string().chars() {
        nums.push(c as usize - 0x30);
    }

    let mut sum: usize = 0;

    let n: usize = nums.len();

    for i in 0..(1 << n - 1) {
        let mut last: usize = 0;
        for j in 0..n {
            if (i >> j) == 1 {
                let sl = &nums[last..j];
                last = j;
                for (i, n) in sl.iter().enumerate() {
                    sum += n * dec.pow(i as u32);
                    println!("{}", dec.pow(i as u32));
                }
            }
        }
    }

    //println!("{}", sum);
}
