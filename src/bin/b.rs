use proconio::input;
#[allow(unused_imports)]
use proconio::source::auto::AutoSource;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};
#[allow(unused_imports)]
use num::integer::{sqrt, gcd, lcm};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};

fn main() {
    // let source = AutoSource::from("0");
    input!{
        // from source,
        n: usize,
        a: [usize; n]
    };

    for i in 0..n{
        if a[i] % 2 == 0{
            if a[i] % 3 != 0 && a[i] % 5 != 0{
                println!("DENIED");
                return
            }
        }
    }
    println!("APPROVED");
}
