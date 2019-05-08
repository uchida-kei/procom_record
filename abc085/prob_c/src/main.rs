macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}
macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let mut $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

fn main() {
    /*
    input! {
        n: isize,
        e: isize
    }
    */

    //test
    //
    let n: isize = 1000;
    let e: isize = 1234000;

    const K10: isize = 10000;
    const K5: isize = 5000;
    const K1: isize = 1000;

    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut z: isize = 0;

    println!("{}", e);

    x = e / K10;
    let e = e % K10;
    y = e / K5;
    let e = e % K5;
    z = e / K1;
    let e = e % K1;

    //println!("{} {} {}", x, y, z);

    let mut xi: isize = x;
    let mut yi: isize = y;
    let mut yj: isize = y;
    let mut zj: isize = z;

    for i in 0..x + 1 {
        if (xi + yj + zj) == n {
            break;
        }
        if x < i {
            //continue;
        }
        xi = x - i;
        yi = y + 2 * i;
        for j in 0..yi + 1 {
            if (n - xi + yi + z) < 4 {
                //break;
            }
            if y < j {
                //continue;
            }
            yj = yi - j;
            zj = z + 5 * j;
            println!("{} {} {}", xi, yj, zj);
            if (xi + yj + zj) == n {
                break;
            } else if (xi + yj + zj) > n {
                break;
            }
        }
    }

    if (xi + yj + zj) == n {
        println!("{} {} {}", xi, yj, zj);
    } else {
        println!("-1 -1 -1");
    }
}
