use proconio::input;
#[allow(unused_imports)]
use proconio::source::auto::AutoSource;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};
#[allow(unused_imports)]
use num::integer::{sqrt, gcd, lcm};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};
use smallvec::alloc::collections::BTreeMap;
use std::collections::HashMap;

fn main() {
    let source = AutoSource::from("7
    beat
    vet
    beet
    bed
    vet
    bet
    beet");
    input!{
        // from source,
        n: usize,
        s: [String; n]
    };

    let mut map = HashMap::new();
    for si in s{
        *map.entry(si).or_insert(0) += 1;      //or_insertは参照を返す
    }

    let mut u = map.iter().collect::<Vec<_>>();
    u.sort_by_key(|&(k,v)| k);                  // 第二ソートキー
    u.sort_by_key(|&(k,v)| Reverse(v));         // 第一ソートキー

    let max = u[0].1;

    for (k, v) in u {
        if v < max{
            break
        }
        println!("{}", k)
    }
}
