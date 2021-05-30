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
        mut n: [i64;3]
    };

    n.sort();

    if n[0] == n[1] && n[1] != n[2] {
        println!("Yes");
        return
    }
    if n[0] != n[1] && n[1] == n[2] {
        println!("Yes");
        return
    }

    println!("No");

}
