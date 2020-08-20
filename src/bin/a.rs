#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n:usize,
        mut bottuns: [(usize, usize); n],
    }
    let mut ans = 0;
    for i in 0..n {
        let amari = (bottuns[n - 1 - i].0 + ans) % bottuns[n - 1 - i].1;
        if amari != 0 {
            ans = ans + bottuns[n - 1 - i].1 - amari;
        }
    }
    println!("{}", ans);
}
